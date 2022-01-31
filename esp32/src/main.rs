mod kv_store;
mod rt_system;
mod wifi;

#[cfg(feature = "telnet")]
use bricc::debug::telnet::TelnetModule;
use bricc::{
    network::wifi::{WifiModule, WifiModuleInterface},
    Bricc,
};
use esp_idf_svc::nvs::EspDefaultNvs;
use wifi::{EspWifiModule, EspWifiModuleInterface};

use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::{sync::Arc, time::Duration};

use crate::{kv_store::EspKvStore, rt_system::EspRtSystemControl};

fn main() {
    esp_idf_sys::link_patches();

    println!("Bricc booted");
    println!("Starting wifi");

    let default_nvs = Arc::new(match EspDefaultNvs::new() {
        Ok(nvs) => nvs,
        Err(_) => panic!("Couldn't create EspDefaultNvs"),
    });

    #[allow(unused)]
    let wifi_module = EspWifiModule::init(default_nvs.clone());
    println!("Wifi up and running");
    println!("Starting telnet session");
    if wifi_module
        .get_interface()
        .set_ap_wpa2_psk("kyp".into(), "killyourphone".into())
        .is_err()
    {
        println!("Issue setting up kyp AP");
        panic!()
    }

    esp_idf_sys::esp!(unsafe {
        esp_idf_sys::esp_vfs_eventfd_register(&esp_idf_sys::esp_vfs_eventfd_config_t {
            max_fds: 50,
            ..Default::default()
        })
    })
    .unwrap();

    #[cfg(feature = "telnet")]
    let (mut display_module, input_module_interface) = {
        let tmp = TelnetModule::new::<EspWifiModuleInterface, EspRtSystemControl>(
            "kyp".into(),
            "killyourphone".into(),
            23,
            wifi_module.get_interface(),
        );
        println!("Telnet ready");
        tmp
    };

    #[cfg(feature = "telnet")]
    let mut bricc_system = Bricc::new::<TelnetModule>(
        EspKvStore::new(default_nvs.clone()),
        wifi_module,
        input_module_interface,
    );

    bricc_system.init();

    loop {
        #[cfg(feature = "telnet")]
        bricc_system.bricc_loop(&mut display_module);
        #[cfg(not(feature = "telnet"))]
        println!("No I/O");
        std::thread::sleep(Duration::from_millis(20));
    }
}

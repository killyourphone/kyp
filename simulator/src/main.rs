use std::{thread, time::Duration};

use dummy_rt_system::DummyRtSystemControl;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use bricc::{debug::telnet::TelnetModule, network::wifi::WifiModule};

mod dummy_rt_system;
mod dummy_wifi;

mod input;

use crate::input::SimulatorInput;
use dummy_wifi::{DummyWifiInterface, DummyWifiModule};

#[allow(unused)]
fn main_simulator() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(84, 48));

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut win = Window::new("Hello World", &output_settings);

    let (input_impl, sender) = SimulatorInput::new();
    let mut bricc_system =
        bricc::Bricc::new::<SimulatorDisplay<BinaryColor>>(DummyWifiModule::new(), input_impl);

    loop {
        win.update(&display);
        bricc_system.bricc_loop(&mut display);
        std::thread::sleep(Duration::from_millis(10));
        for event in win.events() {
            sender.send(event).unwrap();
        }
    }
}

#[allow(unused)]
fn main_telnet() -> Result<(), core::convert::Infallible> {
    let dummy_wifi_mod = DummyWifiModule::new();
    let (mut display, input_interface) =
        TelnetModule::new::<DummyWifiInterface, DummyRtSystemControl>(
            "kyp".into(),
            "killyourphone".into(),
            2223,
            dummy_wifi_mod.get_interface(),
        );

    thread::sleep(Duration::from_millis(1000));

    let mut bricc_system =
        bricc::Bricc::new::<SimulatorDisplay<BinaryColor>>(DummyWifiModule::new(), input_interface);

    loop {
        bricc_system.bricc_loop(&mut display);
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    #[cfg(feature = "telnet")]
    return main_telnet();
    #[cfg(not(feature = "telnet"))]
    return main_simulator();
}

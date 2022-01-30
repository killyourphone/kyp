use crate::wifi::mpsc::{Receiver, Sender};
use bricc::network::wifi::{ApStatus, ClientStatus, WifiModule, WifiModuleInterface};
use bricc::network::wifi::{PSKKey, WifiCommand, WifiError, WifiStatus, SSID};
use embedded_svc::wifi::AccessPointConfiguration;
use embedded_svc::wifi::AuthMethod;
use embedded_svc::wifi::ClientConfiguration;
use embedded_svc::wifi::Configuration;
use embedded_svc::wifi::Wifi;
use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::wifi::*;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::SendError;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

const WIFI_COMMAND_CHECK_PERIOD: Duration = Duration::from_millis(10);
const WIFI_THREAD_STACK_SIZE_BYTES: usize = 16384usize;

struct WifiModuleConfig {
    client_configs: HashMap<SSID, ClientConfiguration>,
    ap_config: Option<AccessPointConfiguration>,
}

pub struct EspWifiModule {
    connection_thread: JoinHandle<()>,
    status_receiver: Receiver<WifiStatus>,
    status_cache: WifiStatus,
    interface_seed: EspWifiModuleInterface,
}

impl WifiModule for EspWifiModule {
    type Interface = EspWifiModuleInterface;

    fn signal_terminate(&mut self) {
        todo!()
    }

    fn join(self) {
        self.connection_thread.join().unwrap();
    }

    fn get_status(&mut self) -> WifiStatus {
        let new_status = self.status_receiver.try_recv();
        if new_status.is_ok() {
            self.status_cache = new_status.unwrap();
        }
        self.status_cache.clone()
    }

    fn get_interface(&self) -> Self::Interface {
        self.interface_seed.clone()
    }
}

#[derive(Clone)]
pub struct EspWifiModuleInterface {
    command_sender: Sender<WifiCommand>,
}

impl WifiModuleInterface for EspWifiModuleInterface {
    fn add_network_wpa2_psk(
        &mut self,
        ssid: SSID,
        key: PSKKey,
    ) -> Result<(), SendError<WifiCommand>> {
        self.command_sender
            .send(WifiCommand::ConnectWPA2PSK(ssid, key))
    }

    fn set_ap_wpa2_psk(&mut self, ssid: SSID, key: PSKKey) -> Result<(), SendError<WifiCommand>> {
        self.command_sender
            .send(WifiCommand::CreateApWPA2PSK(ssid, key))
    }
}

impl EspWifiModule {
    pub fn init(default_nvs: Arc<EspDefaultNvs>) -> EspWifiModule {
        let (command_sender, command_receiver) = mpsc::channel::<WifiCommand>();
        let (status_sender, status_receiver) = mpsc::sync_channel::<WifiStatus>(0);
        let thread_builder = thread::Builder::new().stack_size(WIFI_THREAD_STACK_SIZE_BYTES);

        EspWifiModule {
            interface_seed: EspWifiModuleInterface { command_sender },
            status_receiver,
            connection_thread: thread_builder
                .spawn(move || {
                    let netif_stack = Arc::new(match EspNetifStack::new() {
                        Ok(stack) => stack,
                        Err(_) => panic!("Couldn't create EspNetifStack"),
                    });
                    let sys_loop_stack = Arc::new(match EspSysLoopStack::new() {
                        Ok(stack) => stack,
                        Err(_) => panic!("Couldn't create EspSysLoopStack"),
                    });

                    let mut esp_wifi =
                        EspWifi::new(netif_stack, sys_loop_stack, default_nvs).unwrap();

                    let mut configs: WifiModuleConfig = WifiModuleConfig {
                        client_configs: HashMap::new(),
                        ap_config: None,
                    };

                    loop {
                        let status = match command_receiver.try_recv() {
                            Ok(c) => match c {
                                WifiCommand::ConnectWPA2PSK(ssid, key) => {
                                    configs = EspWifiModule::connect_wpa2_psk(configs, ssid, key);
                                    let result =
                                        EspWifiModule::reconfigure_wifi(&configs, esp_wifi);

                                    esp_wifi = result.0;

                                    match result.1 {
                                        Ok(status) => status,
                                        Err(err) => WifiStatus::Error(err),
                                    }
                                }
                                WifiCommand::CreateApWPA2PSK(ssid, key) => {
                                    configs = EspWifiModule::make_ap_wpa2_psk(configs, ssid, key);
                                    let result =
                                        EspWifiModule::reconfigure_wifi(&configs, esp_wifi);

                                    esp_wifi = result.0;

                                    match result.1 {
                                        Ok(status) => status,
                                        Err(err) => WifiStatus::Error(err),
                                    }
                                }
                            },
                            Err(_) => WifiStatus::Disabled,
                        };
                        if status_sender.try_send(status).is_err() {
                            // no-op.
                        };
                        thread::sleep(Duration::from_millis(10));
                    }
                })
                .unwrap(),

            status_cache: WifiStatus::Disabled,
        }
    }

    fn make_ap_wpa2_psk(
        mut configs: WifiModuleConfig,
        ssid: SSID,
        key: PSKKey,
    ) -> WifiModuleConfig {
        configs.ap_config = Some(AccessPointConfiguration {
            ssid,
            channel: 1,
            password: key,
            auth_method: AuthMethod::WPA2Personal,
            ..Default::default()
        });
        configs
    }

    fn connect_wpa2_psk(
        mut configs: WifiModuleConfig,
        ssid: SSID,
        key: PSKKey,
    ) -> WifiModuleConfig {
        let config = ClientConfiguration {
            ssid: ssid.clone().into(),
            password: key.into(),
            channel: None,
            ..Default::default()
        };
        configs.client_configs.insert(ssid, config);
        configs
    }

    fn reconfigure_wifi(
        config: &WifiModuleConfig,
        mut esp_wifi: EspWifi,
    ) -> (EspWifi, Result<WifiStatus, WifiError>) {
        if config.client_configs.is_empty() {
            let ap_config = config.ap_config.clone();
            if ap_config.is_some() {
                let is_error = esp_wifi
                    .set_configuration(&Configuration::AccessPoint(ap_config.clone().unwrap()))
                    .is_err();
                if !is_error {
                    return (
                        esp_wifi,
                        Ok(WifiStatus::Ap(ApStatus::Enabled(
                            ap_config.clone().unwrap().ssid,
                        ))),
                    );
                } else {
                    return (
                        esp_wifi,
                        Err(WifiError::Unknown("Failed to create AP".into())),
                    );
                }
            } else {
                (esp_wifi, Ok(WifiStatus::Disabled))
            }
        } else {
            let scan_result = esp_wifi.scan();

            if scan_result.is_err() {
                return (
                    esp_wifi,
                    Err(WifiError::Unknown(scan_result.unwrap_err().to_string())),
                );
            }

            let aps = scan_result.unwrap().into_iter();

            for ap in aps {
                let client_config = config.client_configs.get(&ap.ssid);
                if client_config.is_some() {
                    let ap_config = config.ap_config.clone();
                    let overall_config = if ap_config.is_some() && client_config.is_some() {
                        Configuration::Mixed(
                            client_config.unwrap().clone(),
                            ap_config.clone().unwrap(),
                        )
                    } else if ap_config.is_some() {
                        Configuration::AccessPoint(ap_config.clone().unwrap())
                    } else if client_config.is_some() {
                        Configuration::Client(client_config.unwrap().clone())
                    } else {
                        return (esp_wifi, Ok(WifiStatus::Disabled));
                    };

                    let is_error = esp_wifi.set_configuration(&overall_config).is_err();
                    if !is_error {
                        if ap_config.is_some() {
                            return (
                                esp_wifi,
                                Ok(WifiStatus::Client(ClientStatus::Connected(
                                    client_config.unwrap().ssid.clone(),
                                    ap.signal_strength,
                                ))),
                            );
                        }
                    }
                }
            }
            (esp_wifi, Ok(WifiStatus::Disabled))
        }
    }
}

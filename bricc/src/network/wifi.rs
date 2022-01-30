use std::sync::mpsc::SendError;

pub type SSID = String;
pub type PSKKey = String;
pub type WifiSignalStrength = u8;

#[derive(Clone)]
pub enum WifiCommand {
    ConnectWPA2PSK(SSID, PSKKey),
    CreateApWPA2PSK(SSID, PSKKey),
}

#[derive(Clone)]
pub enum ClientStatus {
    Connected(SSID, WifiSignalStrength),
    Disconnected,
    Standby,
    Error(WifiError),
}

#[derive(Clone)]
pub enum ApStatus {
    Enabled(SSID),
    Disabled,
    Error(WifiError),
}

#[derive(Clone)]
pub enum WifiStatus {
    Client(ClientStatus),
    Ap(ApStatus),
    Mixed(ClientStatus, ApStatus),
    Disabled,
    Error(WifiError),
}

#[derive(Clone)]
pub enum WifiError {
    Unknown(String),
}

// The event loop keeps a copy of this guy around to help shut down the system cleanly.
pub trait WifiModule {
    type Interface: WifiModuleInterface;

    fn signal_terminate(&mut self);
    fn join(self);
    fn get_status(&mut self) -> WifiStatus;
    fn get_interface(&self) -> Self::Interface;
}

pub trait WifiModuleInterface: Clone {
    fn add_network_wpa2_psk(
        &mut self,
        ssid: SSID,
        key: PSKKey,
    ) -> Result<(), SendError<WifiCommand>>;

    fn set_ap_wpa2_psk(&mut self, ssid: SSID, key: PSKKey) -> Result<(), SendError<WifiCommand>>;
}

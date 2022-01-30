use bricc::network::wifi::{WifiModule, WifiModuleInterface, WifiStatus};

#[derive(Clone)]
pub struct DummyWifiModule {}

impl WifiModule for DummyWifiModule {
    type Interface = DummyWifiInterface;

    fn signal_terminate(&mut self) {}

    fn join(self) {}

    fn get_status(&mut self) -> WifiStatus {
        WifiStatus::Disabled
    }

    fn get_interface(&self) -> Self::Interface {
        DummyWifiInterface {}
    }
}

impl DummyWifiModule {
    pub fn new() -> DummyWifiModule {
        DummyWifiModule {}
    }
}

#[derive(Clone)]
pub struct DummyWifiInterface {}

impl WifiModuleInterface for DummyWifiInterface {
    fn add_network_wpa2_psk(
        &mut self,
        _ssid: bricc::network::wifi::SSID,
        _key: bricc::network::wifi::PSKKey,
    ) -> Result<(), std::sync::mpsc::SendError<bricc::network::wifi::WifiCommand>> {
        Ok(())
    }

    fn set_ap_wpa2_psk(
        &mut self,
        _ssid: bricc::network::wifi::SSID,
        _key: bricc::network::wifi::PSKKey,
    ) -> Result<(), std::sync::mpsc::SendError<bricc::network::wifi::WifiCommand>> {
        Ok(())
    }
}

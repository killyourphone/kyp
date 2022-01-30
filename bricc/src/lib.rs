#![feature(async_closure)]

#[cfg(feature = "debug")]
pub mod debug;
pub mod display;
pub mod gui;
pub mod input;
pub mod network;
pub mod prefs;
pub mod realtime;
pub mod traits;
pub mod voip;

use std::time::Duration;

use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::OriginDimensions,
};
use input::traits::InputModule;
use network::wifi::WifiModule;
use voip::sip::generate_register;

use crate::{
    gui::{
        panes::lockscreen::RootPane,
        traits::{GuiElement, Pane},
    },
    prefs::kv_store::KvStore,
};

pub struct Bricc<KvStoreImpl: KvStore, WifiModuleImpl: WifiModule, InputModuleImpl: InputModule> {
    root_pane: RootPane,
    wifi_module: WifiModuleImpl,
    input_module: InputModuleImpl,
    kv_store: KvStoreImpl,
    screen_needs_update: bool,
}

impl<
        KvStoreImpl: KvStore,
        WifiModuleImpl: network::wifi::WifiModule,
        InputModuleImpl: input::traits::InputModule,
    > Bricc<KvStoreImpl, WifiModuleImpl, InputModuleImpl>
{
    pub fn new<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>(
        kv_store: KvStoreImpl,
        wifi_impl: WifiModuleImpl,
        input_impl: InputModuleImpl,
    ) -> Bricc<KvStoreImpl, WifiModuleImpl, InputModuleImpl> {
        println!("Bricc::new");

        Bricc {
            root_pane: RootPane::new::<Display>(),
            wifi_module: wifi_impl,
            input_module: input_impl,
            kv_store,
            screen_needs_update: true,
        }
    }
    pub fn bricc_loop<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>(
        &mut self,
        display: &mut Display,
    ) {
        loop {
            generate_register();
            match self.input_module.get_input() {
                Some(user_input) => match self.root_pane.process_input::<Display>(user_input) {
                    gui::traits::GuiAction::ScreenUpdated => {
                        self.screen_needs_update = true;
                    }
                    gui::traits::GuiAction::InvalidInput => {
                        todo!()
                    }
                    gui::traits::GuiAction::Nothing => {
                        continue;
                    }
                    gui::traits::GuiAction::PopPane => {
                        self.root_pane.pop_deepest();
                        self.screen_needs_update = true;
                    }
                },
                None => break,
            }
        }
        if self.screen_needs_update {
            self.screen_needs_update = false;
            display.clear(BinaryColor::Off);
            self.root_pane.render(display);
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

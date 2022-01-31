use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::OriginDimensions;

use crate::gui::menu::Menu;
use crate::gui::menu::MenuElement;
use crate::gui::menu::MenuElementType;
use crate::gui::menu::MenuInputEventResult;
use crate::gui::traits::GuiAction;
use crate::gui::traits::GuiElement;
use crate::gui::traits::Pane;
use crate::input::traits::UserInput;

#[derive(Clone, Copy)]
pub enum SettingsOptions {
    Wifi,
    Cellular,
    Voip,
    Sound,
    Security,
    Accessibility,
    About,
}

impl ToString for SettingsOptions {
    fn to_string(&self) -> String {
        match self {
            SettingsOptions::Wifi => "WiFi".into(),
            SettingsOptions::Cellular => "Cellular".into(),
            SettingsOptions::Voip => "VOIP".into(),
            SettingsOptions::Sound => "Sound".into(),
            SettingsOptions::Security => "Security".into(),
            SettingsOptions::Accessibility => "Accessibility".into(),
            SettingsOptions::About => "About Device".into(),
        }
    }
}

impl MenuElement for SettingsOptions {
    fn menu_item_type(&self) -> MenuElementType {
        match self {
            SettingsOptions::Wifi => MenuElementType::Button,
            SettingsOptions::Cellular => MenuElementType::Button,
            SettingsOptions::Voip => MenuElementType::Button,
            SettingsOptions::Sound => MenuElementType::Button,
            SettingsOptions::Security => MenuElementType::Button,
            SettingsOptions::Accessibility => MenuElementType::Button,
            SettingsOptions::About => MenuElementType::Button,
        }
    }
}

pub struct SettingsPane {
    menu: Menu<SettingsOptions>,
}

impl<Display: OriginDimensions + DrawTarget<Color = BinaryColor>> GuiElement<Display>
    for SettingsPane
{
    fn render(&mut self, framebuffer: &mut Display) {
        self.menu.render(framebuffer)
    }
}

impl Pane for SettingsPane {
    fn process_input<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>(
        &mut self,
        input: UserInput,
    ) -> GuiAction {
        match self.menu.process_input(input) {
            MenuInputEventResult::MenuItemSelected(item) => match item {
                SettingsOptions::Wifi => todo!(),
                SettingsOptions::Cellular => todo!(),
                SettingsOptions::Voip => todo!(),
                SettingsOptions::Sound => todo!(),
                SettingsOptions::Security => todo!(),
                SettingsOptions::Accessibility => todo!(),
                SettingsOptions::About => todo!(),
            },
            MenuInputEventResult::WrappedGuiAction(action) => action,
        }
    }

    fn is_preventing_lock(&self) -> bool {
        false
    }

    fn tick(&mut self) -> GuiAction {
        GuiAction::Nothing
    }

    fn pop_deepest(&mut self) -> bool {
        false
    }
}

impl SettingsPane {
    pub fn new<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>() -> SettingsPane {
        SettingsPane {
            menu: Menu::<SettingsOptions>::new::<Display>(vec![
                SettingsOptions::Wifi,
                SettingsOptions::Cellular,
                SettingsOptions::Voip,
                SettingsOptions::Sound,
                SettingsOptions::Security,
                SettingsOptions::Accessibility,
                SettingsOptions::About,
            ]),
        }
    }
}

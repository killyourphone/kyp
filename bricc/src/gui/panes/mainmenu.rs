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

use super::contacts::ContactsPane;
use super::settings::SettingsPane;

enum ChildPane {
    Contacts(ContactsPane),
    Settings(SettingsPane),
    None,
}

#[derive(Clone, Copy)]
pub enum MainMenuOptions {
    Contacts,
    Settings,
}

impl ToString for MainMenuOptions {
    fn to_string(&self) -> String {
        match self {
            MainMenuOptions::Contacts => "Contacts".into(),
            MainMenuOptions::Settings => "Settings".into(),
        }
    }
}

impl MenuElement for MainMenuOptions {
    fn menu_item_type(&self) -> MenuElementType {
        match self {
            MainMenuOptions::Contacts => MenuElementType::Button,
            MainMenuOptions::Settings => MenuElementType::Button,
        }
    }
}

pub struct MainMenuPane {
    menu: Menu<MainMenuOptions>,
    child_pane: ChildPane,
}

impl<Display: OriginDimensions + DrawTarget<Color = BinaryColor>> GuiElement<Display>
    for MainMenuPane
{
    fn render(&mut self, framebuffer: &mut Display) {
        match &mut self.child_pane {
            ChildPane::Contacts(pane) => pane.render(framebuffer),
            ChildPane::Settings(pane) => pane.render(framebuffer),
            ChildPane::None => self.menu.render(framebuffer),
        }
    }
}

impl Pane for MainMenuPane {
    fn process_input<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>(
        &mut self,
        input: UserInput,
    ) -> GuiAction {
        match &mut self.child_pane {
            ChildPane::Contacts(pane) => pane.process_input::<Display>(input),
            ChildPane::Settings(pane) => pane.process_input::<Display>(input),
            ChildPane::None => match self.menu.process_input(input) {
                MenuInputEventResult::MenuItemSelected(item) => match item {
                    MainMenuOptions::Contacts => {
                        self.child_pane = ChildPane::Contacts(ContactsPane::new::<Display>());
                        GuiAction::ScreenUpdated
                    }
                    MainMenuOptions::Settings => {
                        self.child_pane = ChildPane::Settings(SettingsPane::new::<Display>());
                        GuiAction::ScreenUpdated
                    }
                },
                MenuInputEventResult::WrappedGuiAction(action) => action,
            },
        }
    }

    fn is_preventing_lock(&self) -> bool {
        false
    }

    fn tick(&mut self) -> GuiAction {
        match &mut self.child_pane {
            ChildPane::Contacts(c) => c.tick(),
            ChildPane::Settings(s) => s.tick(),
            ChildPane::None => GuiAction::Nothing,
        }
    }

    fn pop_deepest(&mut self) -> bool {
        let child_did_pop = match &mut self.child_pane {
            ChildPane::Contacts(p) => p.pop_deepest(),
            ChildPane::Settings(p) => p.pop_deepest(),
            ChildPane::None => return false,
        };
        if !child_did_pop {
            self.child_pane = ChildPane::None;
        }
        true
    }
}

impl MainMenuPane {
    pub fn new<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>() -> MainMenuPane {
        MainMenuPane {
            menu: Menu::<MainMenuOptions>::new::<Display>(vec![
                MainMenuOptions::Contacts,
                MainMenuOptions::Settings,
            ]),
            child_pane: ChildPane::None,
        }
    }
}

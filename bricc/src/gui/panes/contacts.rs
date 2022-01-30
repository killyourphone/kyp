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

#[derive(Clone)]
pub struct Contact {
    name: String,
}

impl ToString for Contact {
    fn to_string(&self) -> String {
        return self.name.clone();
    }
}

impl MenuElement for Contact {
    fn menu_item_type(&self) -> MenuElementType {
        return MenuElementType::Callable;
    }
}

pub struct ContactsPane {
    menu: Menu<Contact>,
}

impl<Display: OriginDimensions + DrawTarget<Color = BinaryColor>> GuiElement<Display>
    for ContactsPane
{
    fn render(&mut self, framebuffer: &mut Display) {
        self.menu.render(framebuffer)
    }
}

impl Pane for ContactsPane {
    fn process_input<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>(
        &mut self,
        input: UserInput,
    ) -> GuiAction {
        match self.menu.process_input(input) {
            MenuInputEventResult::MenuItemSelected(item) => {
                println!("Calling {}", item.name);
                GuiAction::Nothing
            }
            MenuInputEventResult::WrappedGuiAction(action) => action,
        }
    }

    fn is_preventing_lock(&self) -> bool {
        false
    }

    fn tick(&mut self) -> GuiAction {
        GuiAction::Nothing
    }

    fn pop_deepest(&mut self) {
        todo!()
    }
}

impl ContactsPane {
    pub fn new<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>() -> ContactsPane {
        ContactsPane {
            menu: Menu::<Contact>::new::<Display>(vec![
                Contact {
                    name: "John Smith".into(),
                },
                Contact {
                    name: "Jane Doe".into(),
                },
                Contact {
                    name: "Ellen Poe".into(),
                },
                Contact {
                    name: "Whoever Else".into(),
                },
            ]),
        }
    }
}

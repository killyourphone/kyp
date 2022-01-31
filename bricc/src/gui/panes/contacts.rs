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

use super::edit_contact_pane::EditContactPane;

#[derive(Clone)]
enum ContactsPaneItem {
    Contact(Contact),
    AddNewButton,
}

impl ToString for ContactsPaneItem {
    fn to_string(&self) -> String {
        match self {
            ContactsPaneItem::Contact(contact) => contact.name.clone(),
            ContactsPaneItem::AddNewButton => "Add New".into(),
        }
    }
}

impl MenuElement for ContactsPaneItem {
    fn menu_item_type(&self) -> MenuElementType {
        match self {
            ContactsPaneItem::Contact(contact) => MenuElementType::Callable,
            ContactsPaneItem::AddNewButton => MenuElementType::Button,
        }
    }
}

#[derive(Clone)]
pub struct Contact {
    pub name: String,
    pub phone_number: String,
}

pub struct ContactsPane {
    menu: Menu<ContactsPaneItem>,
    child: Option<EditContactPane>,
}

impl<Display: OriginDimensions + DrawTarget<Color = BinaryColor>> GuiElement<Display>
    for ContactsPane
{
    fn render(&mut self, framebuffer: &mut Display) {
        match &mut self.child {
            Some(c) => c.render(framebuffer),
            None => self.menu.render(framebuffer),
        }
    }
}

impl Pane for ContactsPane {
    fn process_input<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>(
        &mut self,
        input: UserInput,
    ) -> GuiAction {
        match &mut self.child {
            Some(c) => c.process_input::<Display>(input),
            None => match self.menu.process_input(input) {
                MenuInputEventResult::MenuItemSelected(item) => match item {
                    ContactsPaneItem::Contact(contact) => GuiAction::Nothing,
                    ContactsPaneItem::AddNewButton => {
                        self.child = Some(EditContactPane::new::<Display>(Contact {
                            name: "".into(),
                            phone_number: "".into(),
                        }));
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
        match &mut self.child {
            Some(c) => c.tick(),
            None => GuiAction::Nothing,
        }
    }

    fn pop_deepest(&mut self) -> bool {
        let child_did_pop = match &mut self.child {
            Some(c) => c.pop_deepest(),
            None => return false,
        };
        if !child_did_pop {
            self.child = None;
        }
        true
    }
}

impl ContactsPane {
    pub fn new<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>() -> ContactsPane {
        ContactsPane {
            menu: Menu::<ContactsPaneItem>::new::<Display>(vec![
                ContactsPaneItem::Contact(Contact {
                    name: "John Smith".into(),
                    phone_number: "".into(),
                }),
                ContactsPaneItem::Contact(Contact {
                    name: "Jane Doe".into(),
                    phone_number: "".into(),
                }),
                ContactsPaneItem::Contact(Contact {
                    name: "Ellen Poe".into(),
                    phone_number: "".into(),
                }),
                ContactsPaneItem::Contact(Contact {
                    name: "Whoever Else".into(),
                    phone_number: "".into(),
                }),
                ContactsPaneItem::AddNewButton,
            ]),
            child: None,
        }
    }
}

use std::time::Duration;

use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::OriginDimensions,
};

use crate::gui::{
    menu::{Menu, MenuElement},
    text_input::TextInputHelper,
    traits::{GuiAction, GuiElement, Pane},
};

use super::contacts::Contact;
use crate::gui::menu::MenuElementType;

#[derive(Clone)]
enum ContactAttribute {
    Name,
    PhoneNumber,
}

impl ToString for ContactAttribute {
    fn to_string(&self) -> String {
        match self {
            ContactAttribute::Name => "Name".into(),
            ContactAttribute::PhoneNumber => "Phone#".into(),
        }
    }
}

impl MenuElement for ContactAttribute {
    fn menu_item_type(&self) -> MenuElementType {
        match self {
            ContactAttribute::Name => MenuElementType::TextEntry,
            ContactAttribute::PhoneNumber => MenuElementType::NumberEntry,
        }
    }
}

pub struct EditContactPane {
    name: String,
    phone_number: String,
    menu: Menu<ContactAttribute>,
    text_edit: Option<(ContactAttribute, TextInputHelper)>,
}

impl EditContactPane {
    pub fn new<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>(
        contact: Contact,
    ) -> EditContactPane {
        EditContactPane {
            name: contact.name,
            phone_number: contact.phone_number,
            menu: Menu::new::<Display>(vec![ContactAttribute::Name, ContactAttribute::PhoneNumber]),
            text_edit: None,
        }
    }
}

impl<Display: OriginDimensions + DrawTarget<Color = BinaryColor>> GuiElement<Display>
    for EditContactPane
{
    fn render(&mut self, framebuffer: &mut Display) {
        match &mut self.text_edit {
            Some((_, helper)) => helper.render(framebuffer),
            None => self.menu.render(framebuffer),
        }
    }
}

impl Pane for EditContactPane {
    fn process_input<
        Display: embedded_graphics::prelude::OriginDimensions
            + embedded_graphics::draw_target::DrawTarget<
                Color = embedded_graphics::pixelcolor::BinaryColor,
            >,
    >(
        &mut self,
        input: crate::input::traits::UserInput,
    ) -> crate::gui::traits::GuiAction {
        match &mut self.text_edit {
            Some((attrib, helper)) => match helper.process_input(input) {
                Some(result) => match result {
                    crate::gui::text_input::TextInputResult::Edited(val) => {
                        match attrib {
                            ContactAttribute::Name => self.name = val,
                            ContactAttribute::PhoneNumber => self.phone_number = val,
                        }
                        GuiAction::PopPane
                    }
                    crate::gui::text_input::TextInputResult::Canceled => GuiAction::PopPane,
                },
                None => GuiAction::ScreenUpdated,
            },
            None => match self.menu.process_input(input) {
                crate::gui::menu::MenuInputEventResult::MenuItemSelected(item) => {
                    let child = match item {
                        ContactAttribute::Name => TextInputHelper::new::<Display>(
                            "Enter name:".into(),
                            self.name.clone(),
                            crate::gui::text_input::KeyboardType::TextStartLower,
                            Duration::from_millis(1000),
                        ),
                        ContactAttribute::PhoneNumber => TextInputHelper::new::<Display>(
                            "Enter phone#:".into(),
                            self.phone_number.clone(),
                            crate::gui::text_input::KeyboardType::Numbers,
                            Duration::from_millis(1000),
                        ),
                    };
                    self.text_edit = Some((item, child));
                    GuiAction::ScreenUpdated
                }
                crate::gui::menu::MenuInputEventResult::WrappedGuiAction(action) => action,
            },
        }
    }

    fn is_preventing_lock(&self) -> bool {
        todo!()
    }

    fn tick(&mut self) -> crate::gui::traits::GuiAction {
        match &mut self.text_edit {
            Some((_, helper)) => helper.tick(),
            None => GuiAction::Nothing,
        }
    }

    fn pop_deepest(&mut self) -> bool {
        false
    }
}

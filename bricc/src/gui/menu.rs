use crate::gui::traits::GuiElement;
use crate::input::traits::UserInput;

use super::traits::GuiAction;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::Size;
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::primitives::StyledDrawable;
use embedded_graphics::text::Baseline;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use profont::PROFONT_7_POINT;

#[derive(Debug, PartialEq)]
pub enum MenuElementType {
    Button,
    Callable,
    NumberEntry,
    TextEntry,
    CheckBox,
}

pub enum MenuInputEventResult<MenuOption: ToString + MenuElement> {
    MenuItemSelected(MenuOption),
    WrappedGuiAction(GuiAction),
}

pub trait MenuElement {
    fn menu_item_type(&self) -> MenuElementType;
}

pub struct Menu<MenuOption: ToString + MenuElement + Clone> {
    options: Vec<MenuOption>,
    cursor: usize,
    y_offset_pixels: i32,
}

impl<
        MenuOption: ToString + MenuElement + Clone,
        Display: OriginDimensions + DrawTarget<Color = BinaryColor>,
    > GuiElement<Display> for Menu<MenuOption>
{
    fn render(&mut self, framebuffer: &mut Display) {
        let style = MonoTextStyle::new(&PROFONT_7_POINT, BinaryColor::On);
        let selected_style = MonoTextStyle::new(&PROFONT_7_POINT, BinaryColor::Off);
        let selected_box_primitive_style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::On)
            .build();
        let mut ord = 0usize;
        let mut y = PROFONT_7_POINT.character_size.width as i32;
        let x = PROFONT_7_POINT.character_size.width as i32;
        let predicted_top_baseline_y =
            (self.cursor as i32) * (PROFONT_7_POINT.character_size.height as i32);
        let screen_size = framebuffer.size();
        let max_top_baseline_y = (screen_size.height as i32)
            - (2 * PROFONT_7_POINT.character_size.width as i32)
            - (PROFONT_7_POINT.character_size.height as i32);

        if predicted_top_baseline_y < self.y_offset_pixels {
            self.y_offset_pixels = predicted_top_baseline_y;
        }
        if predicted_top_baseline_y - self.y_offset_pixels > max_top_baseline_y {
            self.y_offset_pixels = predicted_top_baseline_y - max_top_baseline_y;
        }

        for option in &self.options {
            let str = if ord < 9 {
                let mut tmp = (ord + 1).to_string();
                tmp.push_str(": ");
                tmp.push_str(&option.to_string());
                tmp
            } else {
                String::from("")
            };
            let pt = Point::new(x, y - self.y_offset_pixels);
            if ord == self.cursor {
                if Rectangle::new(
                    pt,
                    Size::new(screen_size.width, PROFONT_7_POINT.character_size.height),
                )
                .draw_styled(&selected_box_primitive_style, framebuffer)
                .is_err()
                {
                    println!("Failed to draw box around selected item.");
                }
                let text = Text::with_baseline(&str, pt, selected_style, Baseline::Top);
                if text.draw(framebuffer).is_err() {
                    println!("Failed to draw text in menu.")
                }
            } else {
                let text = Text::with_baseline(&str, pt, style, Baseline::Top);
                if text.draw(framebuffer).is_err() {
                    println!("Failed to draw text in menu.")
                }
            }
            ord += 1;
            y += PROFONT_7_POINT.character_size.height as i32;
        }
    }
}

impl<MenuOption: ToString + MenuElement + Clone> Menu<MenuOption> {
    pub fn new<Display: DrawTarget>(options: Vec<MenuOption>) -> Menu<MenuOption> {
        Menu {
            options,
            cursor: 0,
            y_offset_pixels: 0,
        }
    }

    pub fn process_input(&mut self, input: UserInput) -> MenuInputEventResult<MenuOption> {
        match input {
            UserInput::Up => {
                if self.cursor == 0 {
                    self.cursor = self.options.len() - 1;
                } else {
                    self.cursor -= 1;
                }
                MenuInputEventResult::WrappedGuiAction(GuiAction::ScreenUpdated)
            }
            UserInput::Down => {
                if self.cursor == self.options.len() - 1 {
                    self.cursor = 0;
                } else {
                    self.cursor = (self.cursor + 1) % self.options.len();
                }
                MenuInputEventResult::WrappedGuiAction(GuiAction::ScreenUpdated)
            }
            UserInput::Call => {
                MenuInputEventResult::WrappedGuiAction(GuiAction::PopPane)
                // if self.options[self.cursor].menu_item_type() == MenuElementType::Callable {
                //     MenuInputEventResult::MenuItemSelected(self.options[self.cursor].clone())
                // } else {
                //     MenuInputEventResult::WrappedGuiAction(GuiAction::ScreenUpdated)
                // }
            }
            UserInput::SoftKey => {
                MenuInputEventResult::MenuItemSelected(self.options[self.cursor].clone())
            }
            UserInput::Number(num) => {
                if num != 0u8 && (num as usize) <= self.options.len() {
                    MenuInputEventResult::MenuItemSelected(self.options[(num - 1) as usize].clone())
                } else {
                    MenuInputEventResult::WrappedGuiAction(GuiAction::ScreenUpdated)
                }
            }
            UserInput::Star => MenuInputEventResult::WrappedGuiAction(GuiAction::Nothing),
            UserInput::Hash => MenuInputEventResult::WrappedGuiAction(GuiAction::Nothing),
            UserInput::Power => MenuInputEventResult::WrappedGuiAction(GuiAction::Nothing),
        }
    }
}

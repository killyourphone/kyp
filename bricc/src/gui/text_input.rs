use std::time::{Duration, Instant};

use embedded_graphics::geometry::Dimensions;
use embedded_graphics::Drawable;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{OriginDimensions, Point, Size},
    primitives::Rectangle,
};
use embedded_text::plugin::ansi::Ansi;
use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use profont::PROFONT_7_POINT;

use crate::input::traits::UserInput;

use super::traits::{GuiAction, GuiElement};

pub enum TextInputResult {
    Edited(String),
    Canceled,
}

#[derive(PartialEq)]
pub enum KeyboardType {
    Numbers,
    TextStartCaps,
    TextStartLower,
}

pub struct TextInputHelper {
    prompt: String,
    text: String,
    keyboard: KeyboardType,
    last_input: Instant,
    multi_press_timeout: Duration,
    multi_press_count: u8,
    multi_press_sequence: String,
}

impl<Display: OriginDimensions + DrawTarget<Color = BinaryColor>> GuiElement<Display>
    for TextInputHelper
{
    fn render(&mut self, framebuffer: &mut Display) {
        let character_style = MonoTextStyle::new(&PROFONT_7_POINT, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Left)
            .build();

        let margin = PROFONT_7_POINT.character_size.width as i32;
        let actual_bounds = {
            let bounds = Rectangle::new(Point::new(margin, margin), Size::new(128, 0));

            let text_box =
                TextBox::with_textbox_style(&self.prompt, bounds, character_style, textbox_style);

            match text_box.draw(framebuffer) {
                Ok(_) => text_box.bounding_box(),
                Err(_) => {
                    println!("Failed to draw prompt");
                    return;
                }
            }
        };

        {
            let bounds = Rectangle::new(
                Point::new(margin, actual_bounds.bottom_right().unwrap().y + margin),
                Size::new(128, 0),
            );

            let mut text_to_draw = self.text.clone();
            text_to_draw.push_str("\x1b[4m");
            let next_char_str = self
                .multi_press_sequence
                .get((self.multi_press_count as usize)..((self.multi_press_count as usize) + 1));
            if next_char_str.is_some() && next_char_str.unwrap().len() > 0 {
                text_to_draw.push_str(next_char_str.unwrap());
            } else {
                text_to_draw.push('_');
            }

            let text_box =
                TextBox::with_textbox_style(&text_to_draw, bounds, character_style, textbox_style)
                    .add_plugin(Ansi::new());

            if text_box.draw(framebuffer).is_err() {
                println!("Failed to draw text");
            }
        }
    }
}

impl TextInputHelper {
    pub fn new<Display: DrawTarget>(
        prompt: String,
        initial_text: String,
        keyboard: KeyboardType,
        timeout_duration: Duration,
    ) -> TextInputHelper {
        TextInputHelper {
            prompt,
            text: initial_text,
            keyboard,
            last_input: Instant::now(),
            multi_press_timeout: timeout_duration,
            multi_press_count: 0,
            multi_press_sequence: String::from(""),
        }
    }

    pub fn tick(&mut self) -> GuiAction {
        if Instant::now() - self.last_input > self.multi_press_timeout
            && self.multi_press_sequence.len() != 0
        {
            self.last_input = Instant::now();
            // The old value must be put into the string.
            let next_char_str = self
                .multi_press_sequence
                .get((self.multi_press_count as usize)..((self.multi_press_count as usize) + 1));
            if next_char_str.is_some() {
                self.text.push_str(next_char_str.unwrap());
            }
            self.multi_press_sequence = "".into();
            self.multi_press_count = 0;
            GuiAction::ScreenUpdated
        } else {
            GuiAction::Nothing
        }
    }

    pub fn process_input(&mut self, input: UserInput) -> Option<TextInputResult> {
        if self.keyboard == KeyboardType::Numbers {
            return match input {
                UserInput::Number(num) => {
                    self.text.push_str(&format!("{}", num));
                    None
                }
                UserInput::Star => None,
                UserInput::Hash => None,
                UserInput::Up => None,
                UserInput::Down => None,
                UserInput::SoftKey => Some(TextInputResult::Edited(self.text.clone())),
                UserInput::Call => Some(TextInputResult::Canceled),
                UserInput::Power => None,
            };
        } else {
            let next_seq = match self.keyboard {
                KeyboardType::Numbers => panic!(),
                KeyboardType::TextStartLower => match input {
                    UserInput::Number(num) => Some(String::from(match num {
                        0 => " ",
                        1 => ".,",
                        2 => "abcABC",
                        3 => "defDEF",
                        4 => "ghiGHI",
                        5 => "jklJKL",
                        6 => "mnoMNO",
                        7 => "pqrsPQRS",
                        8 => "tuvTUV",
                        9 => "wxyzWXYZ",
                        _ => {
                            println!("Invalid user input, number oob");
                            panic!();
                        }
                    })),
                    UserInput::Star => Some("+-*/".into()),
                    UserInput::Hash => {
                        Some("#".into()) // TODO deal with shift characters
                    }
                    UserInput::Up => None,
                    UserInput::Down => None,
                    UserInput::SoftKey => None,
                    UserInput::Call => None,
                    UserInput::Power => None,
                },
                KeyboardType::TextStartCaps => {
                    match input {
                        UserInput::Number(num) => Some(String::from(match num {
                            0 => " ",
                            1 => ".,",
                            2 => "ABCabc",
                            3 => "DEFdef",
                            4 => "GHIghi",
                            5 => "JKLjkl",
                            6 => "MNOmno",
                            7 => "PQRSpqrs",
                            8 => "TUVtuv",
                            9 => "WXYZwxyz",
                            _ => {
                                println!("Invalid user input, number oob");
                                panic!();
                            }
                        })),
                        UserInput::Star => Some("+-*/".into()),
                        UserInput::Hash => {
                            Some("#".into()) // TODO deal with shift characters
                        }
                        UserInput::Up => None,
                        UserInput::Down => None,
                        UserInput::SoftKey => None,
                        UserInput::Call => None,
                        UserInput::Power => None,
                    }
                }
            };
            match input {
                UserInput::SoftKey => return Some(TextInputResult::Edited(self.text.clone())),
                UserInput::Call => return Some(TextInputResult::Canceled),
                _ => {}
            }
            self.last_input = Instant::now();
            if next_seq.is_some() && next_seq.clone().unwrap() == self.multi_press_sequence {
                self.multi_press_count += 1;
                if self.multi_press_count as usize >= self.multi_press_sequence.len() {
                    self.multi_press_count = 0;
                }
                None
            } else if next_seq.is_some() {
                let next_char_str = self.multi_press_sequence.get(
                    (self.multi_press_count as usize)..((self.multi_press_count as usize) + 1),
                );
                if next_char_str.is_some() {
                    self.text.push_str(next_char_str.unwrap());
                }
                self.multi_press_sequence = next_seq.unwrap();
                self.multi_press_count = 0;
                None
            } else {
                None
            }
        }
    }
}

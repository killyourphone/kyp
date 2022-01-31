use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::OriginDimensions,
};

use crate::input::traits::UserInput;

pub enum GuiAction {
    ScreenUpdated,
    InvalidInput,
    PopPane,
    Nothing,
}

pub trait GuiElement<Display: DrawTarget<Color = BinaryColor>> {
    fn render(&mut self, framebuffer: &mut Display);
}

pub trait Pane: Send {
    fn process_input<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>(
        &mut self,
        input: UserInput,
    ) -> GuiAction;
    fn is_preventing_lock(&self) -> bool;
    fn tick(&mut self) -> GuiAction;
    fn pop_deepest(&mut self) -> bool;
}

use std::time::Instant;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::OriginDimensions;

use crate::gui::traits::{GuiAction, GuiElement, Pane};
use crate::input::traits::UserInput;

use super::mainmenu::MainMenuPane;

pub struct RootPane {
    last_input_instant: Instant,
    child: MainMenuPane,
    is_unlocked: bool,
}

impl<Display: OriginDimensions + DrawTarget<Color = BinaryColor>> GuiElement<Display> for RootPane {
    fn render(&mut self, framebuffer: &mut Display) {
        if self.is_unlocked {
            self.child.render(framebuffer)
        } else {
            if framebuffer.clear(BinaryColor::Off).is_err() {
                println!("Failed to clear display!")
            }
        }
    }
}

impl Pane for RootPane {
    fn process_input<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>(
        &mut self,
        input: UserInput,
    ) -> GuiAction {
        if self.is_unlocked {
            self.last_input_instant = Instant::now();
            self.child.process_input::<Display>(input)
        } else {
            // Check if should unlock.
            self.is_unlocked = true;
            GuiAction::ScreenUpdated
        }
    }

    fn is_preventing_lock(&self) -> bool {
        todo!()
    }

    fn tick(&mut self) -> GuiAction {
        todo!()
    }

    fn pop_deepest(&mut self) {
        self.child.pop_deepest();
    }
}

impl RootPane {
    pub fn new<Display: OriginDimensions + DrawTarget<Color = BinaryColor>>() -> RootPane {
        RootPane {
            last_input_instant: Instant::now(),
            child: MainMenuPane::new::<Display>(),
            is_unlocked: false,
        }
    }
}

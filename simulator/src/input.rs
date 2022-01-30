use std::sync::mpsc::{channel, Receiver, Sender};

use bricc::input::traits::{InputModule, UserInput};
use embedded_graphics_simulator::sdl2::Keycode;
use embedded_graphics_simulator::SimulatorEvent;

pub struct SimulatorInput {
    receiver: Receiver<SimulatorEvent>,
}

impl InputModule for SimulatorInput {
    fn get_input(&mut self) -> Option<bricc::input::traits::UserInput> {
        loop {
            match self.receiver.try_recv() {
                Ok(sim_event) => match sim_event {
                    SimulatorEvent::KeyUp {
                        keycode,
                        keymod: _,
                        repeat: _,
                    } => match keycode {
                        Keycode::Up => return Some(UserInput::Up),
                        Keycode::Down => return Some(UserInput::Down),
                        Keycode::Num0 => return Some(UserInput::Number(0)),
                        Keycode::Num1 => return Some(UserInput::Number(1)),
                        Keycode::Num2 => return Some(UserInput::Number(2)),
                        Keycode::Num3 => return Some(UserInput::Number(3)),
                        Keycode::Num4 => return Some(UserInput::Number(4)),
                        Keycode::Num5 => return Some(UserInput::Number(5)),
                        Keycode::Num6 => return Some(UserInput::Number(6)),
                        Keycode::Num7 => return Some(UserInput::Number(7)),
                        Keycode::Num8 => return Some(UserInput::Number(8)),
                        Keycode::Num9 => return Some(UserInput::Number(9)),
                        Keycode::Hash => return Some(UserInput::Hash),
                        Keycode::Asterisk => return Some(UserInput::Star),
                        Keycode::C => return Some(UserInput::Call),
                        Keycode::Space => return Some(UserInput::SoftKey),
                        _ => {
                            continue;
                        }
                    },
                    SimulatorEvent::KeyDown {
                        keycode: _,
                        keymod: _,
                        repeat: _,
                    } => {
                        continue;
                    }
                    SimulatorEvent::MouseButtonUp {
                        mouse_btn: _,
                        point: _,
                    } => todo!(),
                    SimulatorEvent::MouseButtonDown {
                        mouse_btn: _,
                        point: _,
                    } => todo!(),
                    SimulatorEvent::MouseWheel {
                        scroll_delta: _,
                        direction: _,
                    } => {
                        continue;
                    }
                    SimulatorEvent::MouseMove { point: _ } => {
                        continue;
                    }
                    SimulatorEvent::Quit => {
                        panic!();
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
        None
    }
}

impl SimulatorInput {
    pub fn new() -> (SimulatorInput, Sender<SimulatorEvent>) {
        let (sender, receiver) = channel();
        return (SimulatorInput { receiver }, sender);
    }
}

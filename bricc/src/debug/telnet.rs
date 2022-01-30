use crate::input::traits::{InputModule, UserInput};
use crate::network::wifi::{PSKKey, WifiModuleInterface, SSID};
use crate::realtime::rt_ctl::RtSystemControl;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::prelude::PointsIter;
use embedded_graphics::Pixel;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::{OriginDimensions, Size},
};
use std::io::{Read, Write};
use std::iter::FromIterator;
use std::net::{Shutdown, TcpListener};
use std::sync::mpsc::{channel, Sender};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::time::Duration;
use std::{thread, thread::JoinHandle};

struct TelnetFramebuffer {
    thread_handle: Option<JoinHandle<()>>,
}

const TELNET_THREAD_STACK_SIZE_BYTES: usize = 8192usize;

impl TelnetFramebuffer {
    pub fn setup_thread<RtSystem: RtSystemControl>(
        &mut self,
        recv: Receiver<Vec<Pixel<BinaryColor>>>,
        send: SyncSender<[u64; 84]>,
    ) {
        let thread_builder = thread::Builder::new().stack_size(TELNET_THREAD_STACK_SIZE_BYTES);

        let result = thread_builder.spawn(move || {
            RtSystem::wdt_unsubscribe_me();
            RtSystem::set_low_priority();
            let mut frame = [0u64; 84];
            if send.try_send(frame.clone()).is_err() {
                println!("Failed to init screen, oops");
            }
            loop {
                let recv_result = recv.recv_timeout(Duration::from_micros(100));
                if recv_result.is_err() {
                    if send.try_send(frame.clone()).is_err() {}
                    continue;
                }
                for pixel in recv_result.unwrap() {
                    if pixel.0.x < 0 || pixel.0.x >= 84 || pixel.0.y < 0 || pixel.0.y >= 48 {
                        continue;
                    }
                    if pixel.1 == BinaryColor::On {
                        frame[pixel.0.x as usize] |= 1 << pixel.0.y
                    } else {
                        frame[pixel.0.x as usize] &= !(1 << pixel.0.y)
                    }
                }
            }
        });
        if result.is_err() {
            println!("Failed to create framebuffer thread");
            panic!()
        }
        self.thread_handle = Some(result.unwrap());
        println!("Spawned telnet framebuffer listener thread");
    }
}

pub struct TelnetModule {
    #[allow(unused)]
    telnet_framebuffer: TelnetFramebuffer,
    draw_call_sender: Sender<Vec<Pixel<BinaryColor>>>,
}

impl DrawTarget for TelnetModule {
    type Color = BinaryColor;

    fn draw_iter<PixelIter: IntoIterator<Item = embedded_graphics::Pixel<BinaryColor>>>(
        &mut self,
        pixels: PixelIter,
    ) -> Result<(), Self::Error> {
        if self.draw_call_sender.send(Vec::from_iter(pixels)).is_err() {
            println!("nobody wants my draw calls");
        }
        Ok(())
    }

    fn fill_contiguous<I>(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        colors: I,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.draw_iter(
            area.points()
                .zip(colors)
                .map(|(pos, color)| embedded_graphics::Pixel(pos, color)),
        )
    }

    fn fill_solid(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        color: Self::Color,
    ) -> Result<(), Self::Error> {
        self.fill_contiguous(area, core::iter::repeat(color))
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.fill_solid(&self.bounding_box(), color)
    }

    type Error = ();
}

impl OriginDimensions for TelnetModule {
    fn size(&self) -> embedded_graphics::prelude::Size {
        return Size::new(84, 48);
    }
}

pub struct TelnetModuleInputInterface {
    input_receiver: Receiver<UserInput>,
}

impl InputModule for TelnetModuleInputInterface {
    fn get_input(&mut self) -> Option<UserInput> {
        match self.input_receiver.try_recv() {
            Ok(user_input) => Some(user_input),
            Err(_) => None,
        }
    }
}

impl TelnetModule {
    pub fn new<WifiModuleInterfaceImpl: WifiModuleInterface, RtSystem: RtSystemControl>(
        ssid: SSID,
        key: PSKKey,
        _port: u16,
        wifi_interface: WifiModuleInterfaceImpl,
    ) -> (TelnetModule, TelnetModuleInputInterface) {
        println!("Setting up AP");
        wifi_interface
            .clone()
            .set_ap_wpa2_psk(ssid.clone(), key.clone())
            .unwrap();

        thread::sleep(Duration::from_millis(3000));

        println!("Starting listener");

        let (frame_sender, frame_listener) = sync_channel(1);
        let (input_sender, input_listener) = sync_channel(20);
        let (draw_call_sender, draw_call_receiver) = channel();
        let mut framebuffer = TelnetFramebuffer {
            thread_handle: None,
        };
        let thread_builder = thread::Builder::new().stack_size(TELNET_THREAD_STACK_SIZE_BYTES);

        println!("Spawning TCP listener thread");
        let result = thread_builder.spawn(move || {
            RtSystem::wdt_unsubscribe_me();
            RtSystem::set_low_priority();
            println!("Init listener");

            let listener_result = TcpListener::bind("0.0.0.0:23");

            if listener_result.is_err() {
                println!("Failed to start tcp listener");
                panic!()
            }

            let listener = listener_result.unwrap();

            println!("Starting tcp loop");

            loop {
                println!("Listening for tcp connection.");
                let result = listener.accept();
                match result {
                    Ok((mut stream, _)) => {
                        println!("Have connection");

                        let mut kill_sesh = false;
                        while !kill_sesh {
                            let mut should_render = false;
                            let mut frame = [0u64; 84];
                            loop {
                                frame = match frame_listener.try_recv() {
                                    Ok(frame) => {
                                        should_render = true;
                                        frame
                                    }
                                    Err(_) => {
                                        break;
                                    }
                                };
                            }
                            if should_render {
                                if stream.write("\x1b[2J\x1b[3J".as_bytes()).is_err() {
                                    kill_sesh = true;
                                    break;
                                }
                                for row in 0u8..48u8 {
                                    for column in frame {
                                        if column & (1u64 << row) != 0 {
                                            if stream.write("X".as_bytes()).is_err() {
                                                kill_sesh = true;
                                                break;
                                            }
                                        } else {
                                            if stream.write(b".").is_err() {
                                                kill_sesh = true;
                                                break;
                                            }
                                        }
                                    }
                                    if stream.write(b"\n").is_err() {
                                        kill_sesh = true;
                                        break;
                                    }
                                }
                            }
                            loop {
                                let mut input = [0u8];
                                let read_result = stream.read(&mut input);
                                let read = read_result;
                                if read.is_err() || read.unwrap() == 0 {
                                    break;
                                }
                                let user_input = match input[0] {
                                    b'0' => Some(UserInput::Number(0)),
                                    b'1' => Some(UserInput::Number(1)),
                                    b'2' => Some(UserInput::Number(2)),
                                    b'3' => Some(UserInput::Number(3)),
                                    b'4' => Some(UserInput::Number(4)),
                                    b'5' => Some(UserInput::Number(5)),
                                    b'6' => Some(UserInput::Number(6)),
                                    b'7' => Some(UserInput::Number(7)),
                                    b'8' => Some(UserInput::Number(8)),
                                    b'9' => Some(UserInput::Number(9)),
                                    b' ' => Some(UserInput::SoftKey),
                                    b'#' => Some(UserInput::Hash),
                                    b'*' => Some(UserInput::Star),
                                    b'c' => Some(UserInput::Call),
                                    b'C' => Some(UserInput::Call),
                                    b'u' => Some(UserInput::Up),
                                    b'U' => Some(UserInput::Up),
                                    b'd' => Some(UserInput::Down),
                                    b'D' => Some(UserInput::Down),
                                    b'q' => {
                                        kill_sesh = true;
                                        break;
                                    }
                                    _ => None,
                                };
                                match user_input {
                                    Some(thing) => {
                                        if input_sender.try_send(thing).is_err() {
                                            println!("Nobody wants my input");
                                        }
                                    }
                                    None => {
                                        break;
                                    }
                                }
                                loop {
                                    match frame_listener.try_recv() {
                                        Ok(_) => {}
                                        Err(_) => {
                                            break;
                                        }
                                    };
                                }
                            }
                        }
                        println!("Lost connection");
                        if stream.shutdown(Shutdown::Both).is_err() {
                            println!("Failed to shut down stream.");
                        }
                    }
                    Err(err) => {
                        println!("Err listening {}", err.to_string());
                    }
                }
            }
        });
        if result.is_err() {
            println!("Failed to create telnet thread");
            panic!()
        }

        framebuffer.setup_thread::<RtSystem>(draw_call_receiver, frame_sender.clone());

        return (
            TelnetModule {
                telnet_framebuffer: framebuffer,
                draw_call_sender,
            },
            TelnetModuleInputInterface {
                input_receiver: input_listener,
            },
        );
    }
}

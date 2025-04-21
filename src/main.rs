use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use std::ffi::CString;
use std::ptr;

use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use image::ImageReader;

// Constants
const WINDOW_TITLE: &'static str = "Walker Engine";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct WalkerEngine {
    _entry: ash::Entry,
    instance: ash::Instance,
}

impl WalkerEngine {

    fn new() -> WalkerEngine {
        // init vulkan stuff
        let entry = ash::Entry::new().unwrap();
        let instance = WalkerEngine::create_instance(&entry);

        // cleanup(); the 'drop' function will take care of it.
        WalkerEngine {
            _entry: entry,
            instance,
        }
    }
    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window.")
    }

    pub fn main_loop(event_loop: EventLoop<()>) {

        event_loop.run(move |event, _, control_flow| {

            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            dbg!();
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | _ => {},
                    }
                },
                _ => (),
            }

        })
    }
}

fn read_icon(path : &str) -> Result<image::DynamicImage, image::ImageError> {
    println!("Reading icon");
    let img = ImageReader::open(path)?.decode()?;
    match img {
        image::DynamicImage::ImageRgb8(ref j) => {
            println!("{0} width", j.width());
            return Ok(img);
        }
        image::DynamicImage::ImageRgba16(ref j) => {
            println!("{0} width", j.width());
            return Ok(img);
        }
        image::DynamicImage::ImageRgb16(ref j) => {
            println!("{0} width", j.width());
            return Ok(img);
        }
        image::DynamicImage::ImageRgb32F(ref j) => {
            println!("{0} width", j.width());
            return Ok(img);
        }
        image::DynamicImage::ImageRgba32F(ref j) => {
            println!("{0} width", j.width());
            return Ok(img);
        }
        image::DynamicImage::ImageRgba8(ref j) => {
            println!("{0} width", j.width());
            return Ok(img);
        }
        _ => {
            println!("Unexpected format!");
            return Err(image::ImageError::Decoding(image::error::DecodingError::new(image::error::ImageFormatHint::Unknown, std::io::Error::new(std::io::ErrorKind::Unsupported, "Oh no"))));
        }
    }
}

fn main() {

    let event_loop = EventLoop::new();
    let window = WalkerEngine::init_window(&event_loop);
    
    let img = read_icon("skywalker_icon.png");

    if let Ok(j) = img {
        let icon = winit::window::Icon::from_rgba(j.as_bytes().to_vec(), j.width(), j.height());
        if let Ok(i) = icon {
            window.set_window_icon(Some(i));
        }
    }

    WalkerEngine::main_loop(event_loop);
    
}
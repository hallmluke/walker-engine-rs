use walker_engine::utility;
use walker_engine::utility::constants::*;

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
// const WINDOW_WIDTH: u32 = 800;
// const WINDOW_HEIGHT: u32 = 600;

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

    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        
        let app_name = CString::new(WINDOW_TITLE).unwrap();
        let engine_name = CString::new("Vulkan Engine").unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: APPLICATION_VERSION,
            p_engine_name: engine_name.as_ptr(),
            engine_version: ENGINE_VERSION,
            api_version: API_VERSION,
        };

        let extension_names = utility::platforms::required_extension_names();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
        };

        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance!")
        };

        instance
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
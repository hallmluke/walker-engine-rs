//use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
//use winit::event::{Event, ElementState, WindowEvent};
//use winit::event::WindowEvent::KeyboardInput;
//use winit::keyboard::KeyCode;
use winit::event_loop::EventLoop;
use image::ImageReader;
use winit_input_helper::WinitInputHelper;


const IS_PAINT_FPS_COUNTER: bool = true;

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

pub fn init_window(
    event_loop: &EventLoop<()>,
    title: &str,
    width: u32,
    height: u32,
) -> winit::window::Window {
    

    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .build(event_loop)
        .expect("Failed to create window.");

    let img = read_icon("skywalker_icon.png");

    if let Ok(j) = img {
        let icon = winit::window::Icon::from_rgba(j.as_bytes().to_vec(), j.width(), j.height());
        if let Ok(i) = icon {
            window.set_window_icon(Some(i));
        }
    }

    return window;
}

pub trait VulkanApp {
    fn draw_frame(&mut self, delta_time: f32, input: &WinitInputHelper);
    fn recreate_swapchain(&mut self);
    fn cleanup_swapchain(&self);
    fn wait_device_idle(&self);
    fn resize_framebuffer(&mut self);
    fn window_ref(&self) -> &winit::window::Window;
}

pub struct ProgramProc {
    pub event_loop: EventLoop<()>,
    pub input: WinitInputHelper
}

impl ProgramProc {

    pub fn new() -> ProgramProc {
        // init window stuff
        let event_loop = EventLoop::new().unwrap();
        let input = WinitInputHelper::new();

        ProgramProc { event_loop, input }
    }

    pub fn main_loop<A: 'static + VulkanApp>(mut self, mut vulkan_app: A) {

        let mut tick_counter = super::fps_limiter::FPSLimiter::new();

        //self.event_loop.run(move |event, _, control_flow| {
        self.event_loop.run(move |event, elwt| {

            if self.input.update(&event) {

                if self.input.close_requested() {
                    vulkan_app.wait_device_idle();
                    elwt.exit();
                    return;
                }

                if let Some(_resolution) = self.input.resolution() {
                    vulkan_app.wait_device_idle();
                    vulkan_app.resize_framebuffer();
                }

                tick_counter.tick_frame();
                let delta_time = tick_counter.delta_time();

                vulkan_app.draw_frame(delta_time, &self.input);
            }

            /*match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            vulkan_app.wait_device_idle();
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(KeyCode::Escape), ElementState::Pressed) => {
                                            vulkan_app.wait_device_idle();
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | WindowEvent::Resized(_new_size) => {
                            vulkan_app.wait_device_idle();
                            vulkan_app.resize_framebuffer();
                        },
                        | _ => {},
                    }
                },
                | Event::MainEventsCleared => {
                    vulkan_app.window_ref().request_redraw();
                },
                | Event::RedrawRequested(_window_id) => {
                    tick_counter.tick_frame();
                    tick_counter.keep_fps();
                    let delta_time = tick_counter.delta_time();
                    
                    vulkan_app.draw_frame(delta_time, self.input);

                    if IS_PAINT_FPS_COUNTER {
                        print!("FPS: {}\r", tick_counter.fps());
                    }
                },
                | Event::LoopDestroyed => {
                    vulkan_app.wait_device_idle();
                },
                _ => (),
            }*/

        });
    }

}
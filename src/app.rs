use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use crate::draw;
use crate::utils::log_error;

// the Number struct is arguably the worst decision of my life
pub struct Number {
    pub number: usize
}

impl Number {

    pub fn usize(&self) -> usize {
        self.number
    }

    pub fn u8(&self) -> u8 {
        self.number as u8
    }

    pub fn u32(&self) -> u32 {
        self.number as u32
    }

    pub fn f32(&self) -> f32 {
        self.number as f32
    }

    pub fn f64(&self) -> f64 {
        self.number as f64
    }
}

const width: f32= 800.;
const height: f32= 600.;
pub const WIDTH: Number = Number{number: width as usize};
pub const HEIGHT: Number = Number{number: height as usize};
pub const ASP_RAT: f32 = width/height;

pub struct App {
    event_loop: EventLoop<()>,
    input: WinitInputHelper,
    window: Window,
    pixels: Pixels,
}

impl App {
    pub fn new() -> Result<Self, pixels::Error> {
        let event_loop = EventLoop::new();
        let input = WinitInputHelper::new();
        let window = Self::create_window(&event_loop);
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WIDTH.u32(), HEIGHT.u32(), surface_texture)?;

        Ok(Self {
            event_loop,
            input,
            window,
            pixels,
        })
    }

    pub fn create_window(event_loop: &EventLoop<()>) -> Window {
        let size = LogicalSize::new(WIDTH.f64(), HEIGHT.f64());
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(event_loop)
            .unwrap()
    }

    pub fn run(mut self) -> Result<(), pixels::Error> {
        self.event_loop.run(move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {
                if let Err(err) = draw::draw_frame(&mut self.pixels) {
                    log_error("renderer.draw_frame", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            if self.input.update(&event) {
                if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let Some(size) = self.input.window_resized() {
                    if let Err(err) = self.pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                self.window.request_redraw();
            }
        });

        Ok(())
    }
}
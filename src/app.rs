use std::time::{Duration, Instant};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use crate::camera::Camera;
use crate::renderer::Renderer;
use crate::utils::log_error;

pub const width: f32 = 800.;
pub const height: f32 = 600.;
pub const aspect_ratio: f32 = width/height;

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
        let pixels = Pixels::new(width as u32, height as u32, surface_texture)?;

        Ok(Self {
            event_loop,
            input,
            window,
            pixels,
        })
    }

    pub fn create_window(event_loop: &EventLoop<()>) -> Window {
        let size = LogicalSize::new(width, height);
        WindowBuilder::new()
            .with_title("Ray Tracer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(event_loop)
            .unwrap()
    }

    pub fn run(mut self, renderer: Renderer) -> Result<(), pixels::Error> {

        let previous_frame_time = Instant::now();

        self.event_loop.run(move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {

                let timestep = previous_frame_time.elapsed();
                renderer.on_update(timestep.as_secs_f32(), &self.input);

                if let Err(err) = renderer.render(&mut self.pixels) {
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
    }
}
// src/window_init.rs
use glfw::{Action, Context, Key};
use gl::types::GLubyte;

pub struct WindowInit {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl WindowInit {
    // Create a new WindowInit instance with a specified title and window size
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        // Create the GLFW window with the given title and size
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        // Enable key polling for the window
        window.set_key_polling(true);
        // Make the window the current OpenGL context
        window.make_current();

        // Load OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Self { glfw, window, events }
    }

    // Run the main loop of the WindowInit instance
    pub fn run(&mut self) {
        while !self.window.should_close() {
            // Poll GLFW events
            self.glfw.poll_events();
            // Flush the event queue and handle events
            for (_, event) in glfw::flush_messages(&self.events) {
                self.handle_event(event);
            }

            // Draw a frame and swap the buffers
            self.draw_frame();
            self.window.swap_buffers();
        }
    }

    // Handle GLFW window events
    fn handle_event(&mut self, event: glfw::WindowEvent) {
        match event {
            // Close the window when the ESC key is pressed
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.window.set_should_close(true)
            }
            _ => {}
        }
    }

    // Draw a frame with a blue background
    fn draw_frame(&mut self) {
        unsafe {
            let color: [GLubyte; 4] = [0, 191, 255, 255];
            gl::ClearColor(
                color[0] as f32 / 255.0,
                color[1] as f32 / 255.0,
                color[2] as f32 / 255.0,
                color[3] as f32 / 255.0,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
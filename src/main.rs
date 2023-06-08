mod window_init;
use window_init::WindowInit;

fn main() {
    let window_title = "Hello, GLFW!";
    let window_width = 800;
    let window_height = 600;

    let mut window_init = WindowInit::new(window_title, window_width, window_height);
    window_init.run();
}
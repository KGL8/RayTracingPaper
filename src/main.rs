mod app;
mod utils;
mod renderer;
mod camera;

use pixels::Error;
use app::App;
use camera::Camera;
use renderer::Renderer;

fn main() -> Result<(), Error> {
    env_logger::init();
    let window = cgmath::Vector2::new(80.0,600.0);
    let camera = Camera::new(45.0,0.1,100.0);
    let renderer = Renderer::new(window, camera);
    let mut app = App::new()?;
    app.run(renderer)?;
    Ok(())
}
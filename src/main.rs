mod app;
mod utils;
mod renderer;
mod camera;

use std::{rc::Rc, cell::RefCell};

use pixels::Error;
use app::App;
use camera::Camera;
use renderer::Renderer;

fn main() -> Result<(), Error> {
    env_logger::init();
    let window = cgmath::Vector2::new(80.0,600.0);
    let mut camera = Camera::new(45.0,0.1,100.0);
    let mut renderer = Rc::new(RefCell::new(Renderer::new(window, camera))); // Wrap Renderer in Rc<RefCell<_>>
    let app = App::new()?;
    app.run(renderer)?;
    Ok(())
}
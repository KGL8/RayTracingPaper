mod app;
mod draw;
mod utils;
mod renderer;

use pixels::Error;
use app::App;

fn main() -> Result<(), Error> {
    env_logger::init();
    let mut app = App::new()?;
    app.run()?;
    Ok(())
}
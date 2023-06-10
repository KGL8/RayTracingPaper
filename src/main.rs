mod app;
mod renderer;
mod utils;

use pixels::Error;
use app::App;

fn main() -> Result<(), Error> {
    env_logger::init();
    let mut app = App::new()?;
    app.run()?;
    Ok(())
}
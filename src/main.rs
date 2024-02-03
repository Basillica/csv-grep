use color_eyre:: Result;
use terminal::{
    app::models, utils
};
use std::error::Error;


mod terminal;


fn main() -> Result<(), Box<dyn Error>> {
    utils::init_error_hooks()?;
    let mut terminal = utils::init_terminal()?;
    models::App::default().run(&mut terminal)?;
    utils::restore_terminal()?;
    Ok(())

    // terminal::table::main::main()
}
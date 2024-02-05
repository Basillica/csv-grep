use color_eyre:: Result;
use std::error::Error;

mod tui;
mod components;


fn main() -> Result<(), Box<dyn Error>> {
    let file_path = std::env::args().nth(1).expect("no file path provided");
    tui::main::main(file_path)
}
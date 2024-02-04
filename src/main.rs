use color_eyre:: Result;
use std::error::Error;

mod terminal;


fn main() -> Result<(), Box<dyn Error>> {
    let file_path = std::env::args().nth(1).expect("no file path provided");
    terminal::table::main::main(file_path)
}
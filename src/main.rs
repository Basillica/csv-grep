use color_eyre::eyre::Ok;
use color_eyre:: Result;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

use terminal::utils::create_struct;

mod terminal;



fn main() -> Result<(), Box<dyn Error>> {
    terminal::table::main::main()
}
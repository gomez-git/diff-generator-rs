mod cli;
mod difference_generator;
mod format;
mod tree;

use self::cli::Options;
use self::difference_generator::generate_difference;
use std::error::Error;
use std::{env, process};

fn main() {
    run().unwrap_or_else(|err| {
        println!("Error: {err}");
        process::exit(1);
    });
}

fn run() -> Result<(), Box<dyn Error>> {
    let arguments = env::args().skip(1);

    let options: Options = Options::build(arguments)?;

    let difference: String = generate_difference(options)?;

    println!("{difference}");

    Ok(())
}

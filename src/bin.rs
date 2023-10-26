use clap::{App, Arg};
use std::fs;
use std::path::Path;

use recipe_parser;

fn main() {
    let matches = App::new("recipes")
        .version("0.1.0")
        .author("Silas Baronda")
        .arg(
            Arg::with_name("file_path")
                .takes_value(true)
                .help("Input recipe file path"),
        )
        .get_matches();

    // check if input file argument exists
    let filepath = match matches.value_of("file_path") {
        None => {
            println!("Input File is necessary argument...\nExiting.");
            std::process::exit(1);
        }
        Some(a) => a,
    };

    // check if the input file exists
    if !Path::new(filepath).exists() {
        println!("Given Input File does not exist...\nExiting");
        std::process::exit(1);
    }

    // read file
    let input = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            println!("Error Reading file : {}\nExiting", e);
            std::process::exit(1);
        }
    };

    recipe_parser::test();
    println!("{:?}", input);
    //Recipe::parse()
}

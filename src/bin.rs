use clap::{App, Arg};
use std::{fs, println};
use std::path::Path;
use serde_json::{Result};

use recipe_parser::{self, Recipe};

fn main() {
    let matches = App::new("recipes")
        .version("0.1.0")
        .author("Silas Baronda")
        .arg(
            Arg::with_name("file_path")
                .takes_value(true)
                .help("Input recipe file path"),
        )
        .arg(
            Arg::with_name("json")
            .long("json")
            .takes_value(false)
            .help("output recipe in json")
        )
        .arg(
            Arg::with_name("debug")
            .long("debug")
            .takes_value(false)
            .help("output more debug information")
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

    let Ok((_, targets)) = recipe_parser::Recipe::parse(&input) else { todo!() };
    //println!("{:?}", i);

    if matches.is_present("json") {
        format_for_json(targets);
    } else {
        format_for_stdout(targets);
    }
}

fn format_for_stdout(targets: Vec<recipe_parser::Target>) {
    for target in targets.iter() {
        for comment in target.comments.iter() {
            println!("# {}", comment);
        }

        print!("{}: ", target.name);

        if let Some(ingredients) = &target.ingredients {
            let mut ingredient_list = Vec::<String>::new();

            for ingredient in ingredients.iter() {
                let mut out = String::new();
                if ingredient.amount != "" {
                    out.push_str("(");
                    out.push_str(&ingredient.amount);
                    if ingredient.unit == "" {
                        out.push_str(")");
                    }
                }

                if ingredient.unit != "" {
                    out.push_str(&ingredient.unit);
                    out.push_str(")");
                }

                if out == "" {
                    out.insert_str(0, &ingredient.name);
                    //print!("{}", ingredient.name);
                } else {
                    out.insert_str(0, " ");
                    out.insert_str(0, &ingredient.name);
                    //print!("{} {}", ingredient.name, out);
                }

                ingredient_list.push(out);
            }

            println!("{}", ingredient_list.join(", "));
        }

        if let Some(instructions) = &target.instructions {
            for instruction in instructions.iter() {
                println!("\t{}", instruction.body);
            }
        }

        println!();
    }
}

fn format_for_json(targets: Vec<recipe_parser::Target>) {
    let r = Recipe {
        targets
    };

    let json = serde_json::to_string(&r).expect("could not output as json");
    println!("{}", json);
}

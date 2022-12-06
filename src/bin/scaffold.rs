use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

use aoc_common::get_aoc_input;

use chrono::Datelike;

use clap::Parser;

const MODULE_TEMPLATE: &str = r###"use aoc_common::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    match exec_part {
        ExecutionPart::Part1 => {},
        ExecutionPart::Part2 => {},
    }

    return Ok(());
}
"###;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,

    #[arg(short, long)]
    year: Option<i32>,
}

fn parse_args() -> Args {
    return Args::parse();
}

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

fn main() {
    let args = parse_args();

    let inputs_dir = format!("inputs/{}", args.day);
    let input_path = format!("inputs/{}/input.txt", args.day);
    let example_path = format!("inputs/{}/example_0.txt", args.day);
    let module_path = format!("src/bin/day{}.rs", args.day);

    let mut file = match safe_create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {}", e);
            process::exit(1);
        }
    };

    match file.write_all(MODULE_TEMPLATE.as_bytes()) {
        Ok(_) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {}", e);
            process::exit(1);
        }
    }

    match std::fs::create_dir(&inputs_dir) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to create inputs dir: \"{}\": {}", &inputs_dir, e);
            process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {}", e);
            process::exit(1);
        }
    }

    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {}", e);
            process::exit(1);
        }
    }

    match get_aoc_input(args.year.unwrap_or(chrono::Utc::now().year()), args.day) {
        Ok(input) => {
            match std::fs::write(&input_path, input) {
                Ok(_) => {
                    println!("Successfully wrote AoC input to file \"{}\"", &input_path)
                },
                Err(e) => {
                    eprintln!("Failed to write AoC input file: {}", e);
                    process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to get AoC input: {}", e);
            process::exit(1);
        }
    }

    println!("---");
    println!(
        "🎄 Type `cargo solve day{}` to run your solution.",
        &args.day
    );
}

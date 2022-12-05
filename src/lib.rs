mod cli;

use std::{io, fs, error};

type GenericResult<T> = Result<T, Box<dyn error::Error>>;
pub type NullResult = GenericResult<()>;

pub enum ExecutionPart {
    Part1,
    Part2,
}

impl From<bool> for ExecutionPart {
    fn from(item: bool) -> Self {
        if item {
            return ExecutionPart::Part2;
        } else {
            return ExecutionPart::Part1;
        }
    }
}

fn get_inputs(day: u8, example_input: Option<u8>) -> io::Result<String> {
    let file_path = match example_input {
        Some(n) => format!("./inputs/{day}/example_{n}.txt"),
        None => format!("./inputs/{day}/input.txt"),
    };

    return fs::read_to_string(file_path);
}

pub fn get_input_from_args(bin_name: &str) -> io::Result<(String, ExecutionPart)> {
    let day_str = &bin_name[3..];
    let day = match day_str.parse::<u8>() {
        Ok(n) => n,
        Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "failed to parse day from binary name")),
    };

    let args = cli::parse_args();
    return match get_inputs(day, args.example) {
        Ok(input) => Ok((input, ExecutionPart::from(args.part2))),
        Err(err) => Err(err)
    };
}

fn get_aoc_token() -> io::Result<String> {
    match dirs::home_dir() {
        Some(home_dir) => match fs::read_to_string(home_dir.join(".aoc_auth_token")) {
            Ok(token) => Ok(token.trim().into()),
            Err(err) => Err(err),
        },
        None => Err(io::Error::new(io::ErrorKind::NotFound, "home directory not found")),
    }
}

pub fn get_aoc_input(year: i32, day: u8) -> GenericResult<String> {
    let aoc_token = get_aoc_token()?;

    let client = reqwest::blocking::Client::builder()
        .user_agent("github.com/dogboy21/adventofcode-2022 by info@dogboy.xyz")
        .build()?;

    let resp = client.get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header("Cookie", format!("session={aoc_token}").as_str())
        .send()?;

    let text_content = resp.text()?;
    return Ok(text_content);
}
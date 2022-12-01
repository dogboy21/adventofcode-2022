mod cli;

type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;
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

fn get_inputs(day: u8, example_input: Option<u8>) -> std::io::Result<String> {
    let file_path = match example_input {
        Some(n) => format!("./inputs/{day}/example_{n}.txt"),
        None => format!("./inputs/{day}/input.txt"),
    };

    return std::fs::read_to_string(file_path);
}

pub fn get_input_from_args(bin_name: &str) -> std::io::Result<(String, ExecutionPart)> {
    let day_str = &bin_name[3..];
    let day = match day_str.parse::<u8>() {
        Ok(n) => n,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "failed to parse day from binary name")),
    };

    let args = cli::parse_args();
    return match get_inputs(day, args.example) {
        Ok(input) => Ok((input, ExecutionPart::from(args.part2))),
        Err(err) => Err(err)
    };
}

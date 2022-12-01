use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub example: Option<u8>,

    #[arg(short, long, default_value_t = false)]
    pub part2: bool,
}

pub fn parse_args() -> Args {
    return Args::parse();
}
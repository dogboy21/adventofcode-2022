use aoc_common::*;

use std::collections::HashSet;

const BIN: &str = env!("CARGO_BIN_NAME");

fn find_first_marker_position(input: &str, window_size: usize) -> Option<usize> {
    for i in (window_size - 1)..input.len() {
        let window = &input[i-(window_size - 1)..=i];
        let unique_chars: HashSet<_> = window.chars().collect();
        if unique_chars.len() == window_size {
            return Some(i + 1);
        }
    }

    return None;
}

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    match exec_part {
        ExecutionPart::Part1 => {
            println!("The first start-of-packet marker was detected after {} characters", find_first_marker_position(&input, 4).unwrap_or(0));
        },
        ExecutionPart::Part2 => {
            println!("The first start-of-message marker was detected after {} characters", find_first_marker_position(&input, 14).unwrap_or(0));
        },
    }

    return Ok(());
}

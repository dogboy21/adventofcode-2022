use aoc_common::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn are_pairs_overlapping(only_full_overlap: bool, first_elf: (u8, u8), second_elf: (u8, u8)) -> bool {
    match only_full_overlap {
        true =>(first_elf.0 >= second_elf.0 && first_elf.1 <= second_elf.1) || (second_elf.0 >= first_elf.0 && second_elf.1 <= first_elf.1),
        false => ((first_elf.0 >= second_elf.0 && first_elf.0 <= second_elf.1) || (second_elf.0 >= first_elf.0 && second_elf.0 <= first_elf.1)),
    }
}

fn read_pairs(line: &str) -> ((u8, u8), (u8, u8)) {
    let pairs: Vec<(u8, u8)> = line.split_terminator(",")
        .map(|pair| pair.split_terminator("-").map(|limit_str| limit_str.parse::<u8>().unwrap_or(0)).collect::<Vec<u8>>())
        .map(|pair| (pair.get(0).unwrap_or(&0).to_owned(), pair.get(1).unwrap_or(&0).to_owned()))
        .collect();

    return (pairs.get(0).unwrap_or(&(0, 0)).to_owned(), pairs.get(1).unwrap_or(&(0, 0)).to_owned());
}

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    let elf_pairs: Vec<((u8, u8), (u8, u8))> = input.lines().map(|line| read_pairs(line)).collect();

    let check_full_overlap = match exec_part {
        ExecutionPart::Part1 => true,
        ExecutionPart::Part2 => false,
    };

    let overlapping = elf_pairs.into_iter().filter(|pair| are_pairs_overlapping(check_full_overlap, pair.0, pair.1)).count();
    println!("{} pairs contain the other either fully or partially", overlapping);

    return Ok(());
}
use aoc_common::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    let elves: Vec<_> = input.split("\n\n").collect();

    let mut elf_calories_sum: Vec<u32> = elves.iter().map(|elf_block| {
        return elf_block.lines().into_iter().map(|calorie_item| match calorie_item.parse::<u32>() {
                Ok(n) => n,
                Err(_) => 0,
        }).sum();
    }).collect();

    elf_calories_sum.sort_by(|a, b| b.cmp(a));

    match exec_part {
        ExecutionPart::Part1 => {
            println!("The Elf carrying the most Calories carries {} Calories.", elf_calories_sum.get(0).unwrap_or(&0));
        },
        ExecutionPart::Part2 => {
            let sum: u32 = elf_calories_sum.iter().take(3).sum();

            println!("The top three Elves carry {} Calories in total", sum)
        },
    }

    return Ok(());
}
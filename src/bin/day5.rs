use aoc_common::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    let input_split: Vec<_> = input.split("\n\n").collect();
    let stack_count = input_split[0].lines().last().unwrap_or("").split_whitespace().map(|stack_num| stack_num.parse::<u8>().unwrap_or(0)).max().unwrap_or(0);
    let mut stacks = Vec::with_capacity(stack_count as usize);
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    input_split.get(0).unwrap_or(&"").lines().for_each(|line| {
        line.chars().collect::<Vec<_>>().chunks(4).map(|a| a.iter().collect::<String>()).enumerate().for_each(|(idx, val)| {
            let stack_char = val.chars().nth(1).unwrap_or(' ');
            if stack_char.is_alphabetic() {
                stacks[idx].push(stack_char);
            }
        });
    });

    for i in 0..stack_count {
        stacks[i as usize].reverse();
    }

    let instructions: Vec<_> = input_split[1].lines().map(|insn_line| {
        let insn = insn_line.split_whitespace().collect::<Vec<_>>();

        (insn[1].parse::<u8>().unwrap_or(0), insn[3].parse::<usize>().unwrap_or(0), insn[5].parse::<usize>().unwrap_or(0))
    }).collect();

    match exec_part {
        ExecutionPart::Part1 => {
            for instruction in instructions {
                for _ in 0..instruction.0 {
                    let moving_obj = stacks[instruction.1 - 1].pop().unwrap();
                    stacks[instruction.2 - 1].push(moving_obj);
                }
            }
        },
        ExecutionPart::Part2 => {
            for instruction in instructions {
                let mut items: Vec<_> = (0..instruction.0).map(|_| stacks[instruction.1 - 1].pop().unwrap()).collect();
                items.reverse();
                items.iter().for_each(|item| stacks[instruction.2 - 1].push(*item));
            }
        },
    }

    let stack_tops: String = stacks.into_iter().map(|mut stack| stack.pop().unwrap()).collect();
    println!("The crates on top of each stack are: {}", stack_tops);

    return Ok(());
}
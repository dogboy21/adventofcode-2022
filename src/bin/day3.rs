use aoc_common::*;

use std::collections::HashSet;

const BIN: &str = env!("CARGO_BIN_NAME");

fn get_item_priority(item: &char) -> Option<u32> {
    match item {
        'A'..='Z' => Some(item.to_owned() as u32 - 'A' as u32 + 27),
        'a'..='z' => Some(item.to_owned() as u32 - 'a' as u32 + 1),
        _ => None
    }
}

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    let backpacks = input.lines();

    match exec_part {
        ExecutionPart::Part1 => {
            let sum: u32 = backpacks.map(|backpack| {
                let middle_idx: usize = backpack.len() / 2;

                let first_compartment = backpack[..middle_idx].chars().into_iter().collect::<HashSet<_>>();
                let second_compartment = backpack[middle_idx..].chars().into_iter().collect::<HashSet<_>>();
                let intersecting_items = first_compartment.intersection(&second_compartment);

                return intersecting_items.into_iter().map(|item| get_item_priority(item).unwrap_or(0)).sum::<u32>();
            })
            .sum();

            println!("The sum of priorities is {}", sum);
        },
        ExecutionPart::Part2 => {
            let sum: u32 = backpacks.collect::<Vec<_>>().chunks(3).map(|group| {
                let intersecting_items = group.iter()
                    .map(|str| str.chars().into_iter().collect::<HashSet<_>>())
                    .reduce(|accum, item| accum.intersection(&item).map(|item| item.to_owned()).collect::<HashSet<_>>());

                let item_sum: u32 = intersecting_items.map(|intersection| intersection.into_iter().map(|item| get_item_priority(&item).unwrap_or(0)).sum()).unwrap_or(0);

                return item_sum;
            })
            .sum();

            println!("The sum of priorities of the item types is {}", sum);
        },
    }

    return Ok(());
}
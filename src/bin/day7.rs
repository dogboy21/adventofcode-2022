use aoc_common::*;

use std::collections::HashMap;

const BIN: &str = env!("CARGO_BIN_NAME");

const MAX_DIR_SIZE: u64 = 100000;
const FS_SIZE: u64 = 70000000;
const NEEDED_UPDATE_SIZE: u64 = 30000000;

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    let lines: Vec<_> = input.lines().collect();
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();
    let mut current_dir: Vec<String> = Vec::new();
    current_dir.push(String::from("/"));

    for line in lines {
        let split_line: Vec<_> = line.split_whitespace().collect();
        match split_line[0] {
            "$" => {
                match split_line[1] {
                    "cd" => {
                        match split_line[2] {
                            "/" => {
                                current_dir.clear();
                                current_dir.push(String::from("/"));
                            },
                            ".." => {
                                current_dir.pop();
                            },
                            dir_name => {
                                current_dir.push(String::from(dir_name));
                            }
                        }
                    },
                    _ => {},
                }
            }
            "dir" => {}, // ignore dir lines
            file_size => {
                let file_size_parsed: u64 = file_size.parse()?;
                for i in 0..current_dir.len() {
                    let dir_path = current_dir[0..=i].join("/");
                    let current_dir_size = dir_sizes.get(dir_path.as_str()).unwrap_or(&0);
                    dir_sizes.insert(dir_path.to_owned(), current_dir_size + file_size_parsed);
                }
            },
        }
    }

    match exec_part {
        ExecutionPart::Part1 => {
            let dir_size_sum: u64 = dir_sizes.into_values().into_iter().filter(|size| size < &MAX_DIR_SIZE).sum();
            println!("The sum of the total sizes of directories smaller than {} is {}", MAX_DIR_SIZE, dir_size_sum);
        },
        ExecutionPart::Part2 => {
            let unused_space = FS_SIZE - dir_sizes.get("/").unwrap_or(&0);
            let needed_space = NEEDED_UPDATE_SIZE - unused_space;
            let smallest_dir_to_delete: u64 = dir_sizes.into_values().into_iter().filter(|size| size >= &needed_space).min().unwrap_or(0);
            println!("The total size of the smallest directory to delete is {}", smallest_dir_to_delete);
        },
    }

    return Ok(());
}

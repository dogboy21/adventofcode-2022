use aoc_common::*;

use std::collections::HashSet;

const BIN: &str = env!("CARGO_BIN_NAME");

enum Movement {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32)
}

impl Movement {
    fn get_direction_pos_offset(self: &Self) -> (i32, i32) {
        match self {
            Self::Left(_) => (-1, 0),
            Self::Right(_) => (1, 0),
            Self::Up(_) => (0, 1),
            Self::Down(_) => (0, -1),
        }
    }

    fn get_distance(self: &Self) -> i32 {
        match self {
            Self::Left(distance) => *distance,
            Self::Right(distance) => *distance,
            Self::Up(distance) => *distance,
            Self::Down(distance) => *distance,
        }
    }
}

impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left(distance) => write!(f, "L {}", distance),
            Self::Right(distance) => write!(f, "R {}", distance),
            Self::Up(distance) => write!(f, "U {}", distance),
            Self::Down(distance) => write!(f, "D {}", distance),
        }
    }
}

impl TryFrom<String> for Movement {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let movement_args: Vec<_> = value.split_whitespace().collect();
        if movement_args.len() != 2 {
            return Err("invalid movement arg count");
        }

        match movement_args[1].parse::<i32>() {
            Ok(distance) => {
                match movement_args[0] {
                    "L" => Ok(Movement::Left(distance)),
                    "R" => Ok(Movement::Right(distance)),
                    "U" => Ok(Movement::Up(distance)),
                    "D" => Ok(Movement::Down(distance)),
                    _ => Err("unknown movement direction")
                }
            },
            Err(_) => Err("failed to parse movement distance"),
        }
    }
}

fn chebyshev_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    return std::cmp::max((p2.1 - p1.1).abs(), (p2.0 - p1.0).abs());
}

fn adjust_tail_pos(head_pos: &(i32, i32), tail_pos: &mut (i32, i32)) {
    if chebyshev_distance(*head_pos, *tail_pos) == 1 {
        return;
    }

    if head_pos.0 > tail_pos.0 {
        tail_pos.0 += 1;
    } else if head_pos.0 < tail_pos.0 {
        tail_pos.0 -= 1;
    }

    if head_pos.1 > tail_pos.1 {
        tail_pos.1 += 1;
    } else if head_pos.1 < tail_pos.1 {
        tail_pos.1 -= 1;
    }
}

fn simulate_rope(rope_length: usize, movements: Vec<Movement>) {
    let mut positions = Vec::new();
    for _ in 0..rope_length {
        positions.push((0, 0));
    }

    let mut visited_tail_positions = HashSet::new();

    for movement in movements {
        let offset = movement.get_direction_pos_offset();
        for _ in 0..movement.get_distance() {
            let mut head = positions[0];
            head.0 += offset.0;
            head.1 += offset.1;
            positions[0] = head;

            for tail_idx in 1..rope_length {
                let previous = positions[tail_idx - 1];
                let mut tail = positions[tail_idx];
                adjust_tail_pos(&previous, &mut tail);
                positions[tail_idx] = tail;
            }
            visited_tail_positions.insert(positions[rope_length - 1]);
        }
    }

    println!("The tail of the rope of length {} visited {} unique positions", rope_length, visited_tail_positions.len());
}

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    let movements: Vec<_> = input.lines().map(|line| Movement::try_from(String::from(line)).unwrap()).collect();

    match exec_part {
        ExecutionPart::Part1 => simulate_rope(2, movements),
        ExecutionPart::Part2 => simulate_rope(10, movements),
    }

    return Ok(());
}

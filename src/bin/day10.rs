use aoc_common::*;

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl TryFrom<String> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let args: Vec<_> = value.split_whitespace().collect();
        match args[0] {
            "noop" => Ok(Self::Noop),
            "addx" => Ok(Self::Addx(args.get(1).ok_or("no argument for addx instruction")?.parse::<i32>()?)),
            insn => Err(format!("invalid instruction {}", insn).as_str())?,
        }
    }
}

fn execute_instructions<F>(instructions: Vec<Instruction>, mut cycle_callback: F) where F: FnMut(i32, i32) -> () {
    let mut cycle = 1;
    let mut reg_x = 1;

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle_callback(cycle, reg_x);
                cycle += 1;
            },
            Instruction::Addx(add) => {
                cycle_callback(cycle, reg_x);
                cycle += 1;
                cycle_callback(cycle, reg_x);
                reg_x += add;
                cycle += 1;
            }
        }
    }
}

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    let instructions: Vec<_> = input.lines().map(|line| Instruction::try_from(String::from(line)).unwrap()).collect();

    match exec_part {
        ExecutionPart::Part1 => {
            let mut signal_strength_sum = 0;

            execute_instructions(instructions, |cycle, reg_x| {
                if (cycle - 20) % 40 == 0 {
                    signal_strength_sum += (cycle as i32) * reg_x;
                }
            });

            println!("The sum of the signal strengths is {}", signal_strength_sum);
        },
        ExecutionPart::Part2 => {
            execute_instructions(instructions, |cycle, sprite_center_pos| {
                let x = (cycle - 1) % 40;
                let pixel = match x >= (sprite_center_pos - 1) && x <= (sprite_center_pos + 1) {
                    true => "██",
                    false => "░░",
                };

                print!("{}", pixel);

                if x == 39 {
                    print!("\n");
                }
            });
        },
    }

    return Ok(());
}
use aoc_common::*;

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(PartialEq, Eq)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn get_score(&self) -> u32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }
}

impl TryFrom<char> for GameResult {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(GameResult::Lose),
            'Y' => Ok(GameResult::Draw),
            'Z' => Ok(GameResult::Win),
            _ => Err("invalid game result character")
        }
    }
}

#[derive(PartialEq, Eq)]
enum GameInput {
    Rock,
    Paper,
    Scissors,
}

impl GameInput {
    fn get_score(&self) -> u32 {
        match self {
            GameInput::Rock => 1,
            GameInput::Paper => 2,
            GameInput::Scissors => 3,
        }
    }

    fn get_game_result(&self, input: &GameInput) -> GameResult {
        match (self, input) {
            (GameInput::Rock, GameInput::Rock) => GameResult::Draw,
            (GameInput::Paper, GameInput::Paper) => GameResult::Draw,
            (GameInput::Scissors, GameInput::Scissors) => GameResult::Draw,

            (GameInput::Rock, GameInput::Scissors) => GameResult::Win,
            (GameInput::Scissors, GameInput::Paper) => GameResult::Win,
            (GameInput::Paper, GameInput::Rock) => GameResult::Win,

            _ => GameResult::Lose,
        }
    }

    fn get_self_input_for_result(&self, result: GameResult) -> &GameInput {
        match result {
            GameResult::Draw => self,
            GameResult::Win => match self {
                GameInput::Rock => &GameInput::Paper,
                GameInput::Paper => &GameInput::Scissors,
                GameInput::Scissors => &GameInput::Rock,
            },
            GameResult::Lose => match self {
                GameInput::Rock => &GameInput::Scissors,
                GameInput::Paper => &GameInput::Rock,
                GameInput::Scissors => &GameInput::Paper,
            }
        }
    }
}

impl TryFrom<char> for GameInput {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(GameInput::Rock),
            'B' | 'Y' => Ok(GameInput::Paper),
            'C' | 'Z' => Ok(GameInput::Scissors),
            _ => Err("invalid game input character")
        }
    }
}

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    let rounds = input.lines();

    let total_score: u32 = match exec_part {
        ExecutionPart::Part1 => {
            let part_score: u32 = rounds.into_iter()
                .map(|line| {
                    let opponent_char: char = line.chars().nth(0).unwrap_or(' ');
                    let self_char: char = line.chars().nth(2).unwrap_or(' ');
                    let opponent_input = GameInput::try_from(opponent_char)?;
                    let self_input = GameInput::try_from(self_char)?;

                    let score = self_input.get_score() + self_input.get_game_result(&opponent_input).get_score();

                    return Ok(score);
                })
                .map(|result: Result<u32, &'static str>| result.unwrap_or(0))
                .sum();

            part_score
        },
        ExecutionPart::Part2 => {
            let part_score: u32 = rounds.into_iter()
                .map(|line| {
                    let opponent_char: char = line.chars().nth(0).unwrap_or(' ');
                    let result_char: char = line.chars().nth(2).unwrap_or(' ');
                    let opponent_input = GameInput::try_from(opponent_char)?;
                    let expected_result = GameResult::try_from(result_char)?;

                    let score = expected_result.get_score() + opponent_input.get_self_input_for_result(expected_result).get_score();

                    return Ok(score);
                })
                .map(|result: Result<u32, &'static str>| result.unwrap_or(0))
                .sum();

            part_score
        },
    };

    println!("The total score for the Rock Paper Scissors game would be {}", total_score);

    return Ok(());
}
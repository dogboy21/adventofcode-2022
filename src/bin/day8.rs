use aoc_common::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn trace_direction(trees: &Vec<Vec<u32>>, x: usize, y: usize, direction: (i32, i32)) -> Vec<u32> {
    let grid_size: i32 = trees.len().try_into().unwrap();
    let mut traced: Vec<u32> = Vec::new();

    let mut c_x: i32 = x.try_into().unwrap();
    let mut c_y: i32 = y.try_into().unwrap();

    for _ in 0..grid_size {
        c_x += direction.0;
        c_y += direction.1;
        if c_x < 0 || c_y < 0 || c_x >= grid_size || c_y >= grid_size {
            break;
        }

        traced.push(trees[c_y as usize][c_x as usize]);
    }

    return traced;
}

fn get_visible_trees(tree_height: u32, traced_trees: &Vec<u32>) -> usize {
    for i in 0..traced_trees.len() {
        if traced_trees[i] >= tree_height {
            return i + 1;
        }
    }

    return traced_trees.len();
}

fn main() -> NullResult {
    let (input, exec_part) = get_input_from_args(BIN)?;

    let trees: Vec<_> = input.lines().map(|line| line.chars().map(|tree_char| tree_char.to_digit(10).unwrap_or(0)).collect::<Vec<_>>()).collect();
    let grid_size = trees.len();
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    match exec_part {
        ExecutionPart::Part1 => {
            let mut visible_trees: u32 = 0;

            for x in 0..grid_size {
                for y in 0..grid_size {
                    let tree_height: i32 = trees[y][x].try_into().unwrap();
                    for direction in directions.iter() {
                        if trace_direction(&trees, x, y, direction.to_owned()).into_iter().map(|h| i32::try_from(h).unwrap()).max().unwrap_or(-1) < tree_height {
                            visible_trees += 1;
                            break;
                        }
                    }
                }
            }

            println!("Number of trees visible from outside the grid: {}", visible_trees);
        },
        ExecutionPart::Part2 => {
            let mut scenic_scores = Vec::new();

            for x in 0..grid_size {
                for y in 0..grid_size {
                    let tree_height = trees[y][x];
                    let scenic_score = directions.iter()
                        .map(|direction| {
                            let traced = trace_direction(&trees, x, y, direction.to_owned());
                            return get_visible_trees(tree_height, &traced);
                        })
                        .reduce(|accum, item| accum * item)
                        .unwrap_or(0);

                    scenic_scores.push(scenic_score);
                }
            }

            println!("The highest scenic score is {:?}", scenic_scores.into_iter().max().unwrap_or(0));
        },
    }

    return Ok(());
}

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

use color_eyre::Result;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct DigInstruction {
    direction: Direction,
    distance: i32,
}

impl FromStr for DigInstruction {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let direction = match s.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };
        let distance = s.next().unwrap().parse::<i32>()?;

        Ok(Self {
            direction,
            distance,
        })
    }
}

enum TrenchDir {
    Horizontal,
    Vertical,
    TurnUp,
    TurnDown,
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();

    let mut intructions = input
        .iter()
        .map(|s| s.parse::<DigInstruction>())
        .collect::<Result<Vec<DigInstruction>>>()?;

    let mut trench = HashMap::new();
    let mut current_block = (0, 0);
    let mut current_dir = None;
    while let Some(instruction) = intructions.pop() {
        match instruction.direction {
            Direction::Up => {
                if let Some(Direction::Left) | Some(Direction::Right) = current_dir {
                    trench.insert(current_block, TrenchDir::TurnUp);
                } else {
                    trench.insert(current_block, TrenchDir::Vertical);
                }
                for _ in 0..instruction.distance {
                    current_block.1 += 1;
                    trench.insert(current_block, TrenchDir::Vertical);
                }
                current_dir = Some(Direction::Up);
            }
            Direction::Down => {
                if let Some(Direction::Left) | Some(Direction::Right) = current_dir {
                    trench.insert(current_block, TrenchDir::TurnDown);
                } else {
                    trench.insert(current_block, TrenchDir::Vertical);
                }
                for _ in 0..instruction.distance {
                    current_block.1 -= 1;
                    trench.insert(current_block, TrenchDir::Vertical);
                }
                current_dir = Some(Direction::Down);
            }
            Direction::Left => {
                if let Some(Direction::Up) = current_dir {
                    trench.insert(current_block, TrenchDir::TurnUp);
                } else if let Some(Direction::Down) = current_dir {
                    trench.insert(current_block, TrenchDir::TurnDown);
                } else {
                    trench.insert(current_block, TrenchDir::Horizontal);
                }
                for _ in 0..instruction.distance {
                    current_block.0 -= 1;
                    trench.insert(current_block, TrenchDir::Horizontal);
                }
                current_dir = Some(Direction::Left);
            }
            Direction::Right => {
                if let Some(Direction::Up) = current_dir {
                    trench.insert(current_block, TrenchDir::TurnUp);
                } else if let Some(Direction::Down) = current_dir {
                    trench.insert(current_block, TrenchDir::TurnDown);
                } else {
                    trench.insert(current_block, TrenchDir::Horizontal);
                }
                for _ in 0..instruction.distance {
                    current_block.0 += 1;
                    trench.insert(current_block, TrenchDir::Horizontal);
                }
                current_dir = Some(Direction::Right);
            }
        }
    }
    let min_x = *trench.iter().map(|((x, _), _)| x).min().unwrap();
    let max_x = *trench.iter().map(|((x, _), _)| x).max().unwrap();
    let min_y = *trench.iter().map(|((_, y), _)| y).min().unwrap();
    let max_y = *trench.iter().map(|((_, y), _)| y).max().unwrap();
    let mut inside = HashSet::new();

    for y in min_y..=max_y {
        let mut is_inside = false;
        let mut on_pipe = false;
        let mut last_turn = None;
        for x in min_x..=max_x {
            if let Some(TrenchDir::TurnUp) = trench.get(&(x, y)) {
                if let Some(TrenchDir::TurnUp) = last_turn {
                    if on_pipe {
                        is_inside = !is_inside;
                    }
                }
                on_pipe = !on_pipe;
                last_turn = Some(TrenchDir::TurnUp);
                // eprint!("U")
            }
            if let Some(TrenchDir::TurnDown) = trench.get(&(x, y)) {
                if let Some(TrenchDir::TurnDown) = last_turn {
                    if on_pipe {
                        is_inside = !is_inside;
                    }
                }
                on_pipe = !on_pipe;
                last_turn = Some(TrenchDir::TurnDown);
                // eprint!("D")
            } else if let Some(TrenchDir::Vertical) = trench.get(&(x, y)) {
                is_inside = !is_inside;
                // eprint!("#")
            } else {
                if on_pipe {
                    // inside.insert((x, y));
                    // eprint!("-")
                } else if is_inside {
                    inside.insert((x, y));
                    // eprint!("*")
                } else {
                    // eprint!(".")
                }
            }
        }
        // eprintln!();
    }
    let sol = inside.len() as i32 + trench.len() as i32;
    eprintln!("x [{},{}]", min_x, max_x);
    eprintln!("y [{},{}]", min_y, max_y);
    eprintln!("{:?}", Instant::now() - start_time);
    Ok(sol)
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    eprintln!("{:?}", Instant::now() - start_time);
    todo!()
}

#[cfg(test)]
mod test {

    use color_eyre::Result;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    fn get_file(filename: PathBuf) -> Result<Vec<String>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let lines: std::result::Result<Vec<String>, _> = reader.lines().collect();
        Ok(lines?)
    }

    use crate::{solve_task_one, solve_task_two};

    #[test]
    fn test_case_one_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_one(file)?, 62);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        assert_eq!(
            solve_task_one(get_file(cargo_manifest_dir.join("inputs/full.txt"))?)?,
            0
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        assert_eq!(
            solve_task_two(get_file(PathBuf::from("inputs/example_2.txt"))?)?,
            0
        );
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        assert_eq!(
            solve_task_two(get_file(PathBuf::from("inputs/full.txt"))?)?,
            0
        );
        Ok(())
    }
}

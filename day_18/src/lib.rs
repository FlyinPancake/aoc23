use std::{str::FromStr, time::Instant};

use color_eyre::Result;
use rayon::prelude::*;

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
    distance: i64,
}

impl DigInstruction {
    fn from_hex_str(s: &str) -> Self {
        let mut s = s.chars();
        let last = s.by_ref().clone().last().unwrap();
        let direction = match last {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => unreachable!(),
        };
        let distance = i64::from_str_radix(&s.take(5).collect::<String>(), 16).unwrap();

        Self {
            direction,
            distance,
        }
    }
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
        let distance = s.next().unwrap().parse::<i64>()?;

        Ok(Self {
            direction,
            distance,
        })
    }
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();

    let instructions = input
        .iter()
        .map(|s| s.parse::<DigInstruction>())
        .collect::<Result<Vec<DigInstruction>>>()?;

    let (vertices, bounding_points) =
        instructions
            .iter()
            .fold((vec![(0, 0)], 0), |(mut acc, b), i| {
                let prev = acc.last().unwrap();

                let next = match i.direction {
                    Direction::Up => (prev.0, prev.1 + i.distance),
                    Direction::Down => (prev.0, prev.1 - i.distance),
                    Direction::Left => (prev.0 - i.distance, prev.1),
                    Direction::Right => (prev.0 + i.distance, prev.1),
                };

                acc.push(next);
                (acc, b + i.distance)
            });
    // let vertices: HashSet<(i64, i64)> = HashSet::from_iter(vertices.iter().cloned());
    let area: i64 = vertices
        .par_windows(3)
        .map(|window| {
            let ((_, y_m1), (x_n, _), (_, y_p1)) = (window[0], window[1], window[2]);

            x_n * (y_p1 - y_m1)
        })
        .sum();
    let area = area.abs() / 2;

    let interior_points = area - bounding_points / 2 + 1;

    eprintln!("{:?}", Instant::now() - start_time);
    Ok(interior_points + bounding_points)
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();

    let instructions = input
        .iter()
        .map(|s| {
            let start = s.find("(#").unwrap();
            let end = s.find(")").unwrap();
            DigInstruction::from_hex_str(&s[start + 2..end])
        })
        .collect::<Vec<DigInstruction>>();

    let (vertices, bounding_points) =
        instructions
            .iter()
            .fold((vec![(0, 0)], 0), |(mut acc, b), i| {
                let prev = acc.last().unwrap();

                let next = match i.direction {
                    Direction::Up => (prev.0, prev.1 + i.distance),
                    Direction::Down => (prev.0, prev.1 - i.distance),
                    Direction::Left => (prev.0 - i.distance, prev.1),
                    Direction::Right => (prev.0 + i.distance, prev.1),
                };

                acc.push(next);
                (acc, b + i.distance)
            });

    let area: i64 = vertices
        .windows(3)
        .map(|window| {
            let ((_, y_m1), (x_n, _), (_, y_p1)) = (window[0], window[1], window[2]);

            x_n * (y_p1 - y_m1)
        })
        .sum();
    let area = area.abs() / 2;

    let interior_points = area - bounding_points / 2 + 1;

    eprintln!("{:?}", Instant::now() - start_time);
    Ok(interior_points + bounding_points)
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
            solve_task_two(get_file(PathBuf::from("inputs/example_1.txt"))?)?,
            952408144115
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

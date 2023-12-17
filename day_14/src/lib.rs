use std::time::Instant;

use color_eyre::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Square,
    Round,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Square,
            'O' => Self::Round,
            _ => panic!("Invalid tile"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn from_lines(lines: Vec<String>) -> Self {
        let tiles = lines
            .iter()
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect();
        Self { tiles }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[y][x] = tile;
    }

    fn load(&self, direction: Direction) -> i32 {
        match direction {
            Direction::Up => self.tiles.iter().enumerate().fold(0, |acc, (y, row)| {
                let multiplier = self.tiles.len() - y;
                acc + row.iter().filter(|tile| **tile == Tile::Round).count() as i32
                    * multiplier as i32
            }),
            Direction::Down => todo!(),
            Direction::Left => todo!(),
            Direction::Right => todo!(),
        }
    }
    fn print(&self) {
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Empty => eprint!("."),
                    Tile::Square => eprint!("#"),
                    Tile::Round => eprint!("O"),
                }
            }
            eprintln!();
        }
    }

    fn roll(mut self, direction: Direction) -> Self {
        match direction {
            Direction::Up => {
                for x in 0..self.tiles.len() {
                    let rounds = self
                        .tiles
                        .iter()
                        .enumerate()
                        .filter(|(_, row)| row[x] == Tile::Round)
                        .map(|(y, _)| y)
                        .collect::<Vec<_>>();
                    for y in rounds {
                        let mut y = y;
                        while y > 0 && self.get(x, y - 1).unwrap() == &Tile::Empty && y > 0 {
                            self.set(x, y, Tile::Empty);
                            self.set(x, y - 1, Tile::Round);
                            y -= 1;
                        }
                    }
                }
                self
            }
            Direction::Down => todo!(),
            Direction::Left => todo!(),
            Direction::Right => todo!(),
        }
    }
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let map = Map::from_lines(input);
    // map.print();
    let map = map.roll(Direction::Up);
    let sol = map.load(Direction::Up);
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
        assert_eq!(solve_task_one(file)?, 136);
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

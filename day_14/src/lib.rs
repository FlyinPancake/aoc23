use std::time::Instant;

use color_eyre::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    #[allow(dead_code)]
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

    fn roll(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                for x in 0..self.tiles[0].len() {
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
            }
            Direction::Down => {
                for x in 0..self.tiles[0].len() {
                    let rounds = self
                        .tiles
                        .iter()
                        .enumerate()
                        .filter(|(_, row)| row[x] == Tile::Round)
                        .map(|(y, _)| y)
                        .rev()
                        .collect::<Vec<_>>();
                    for y in rounds {
                        let mut y = y;
                        while y < self.tiles.len() - 1
                            && self.get(x, y + 1).unwrap() == &Tile::Empty
                        {
                            self.set(x, y, Tile::Empty);
                            self.set(x, y + 1, Tile::Round);
                            y += 1;
                        }
                    }
                }
            }
            Direction::Left => {
                for y in 0..self.tiles.len() {
                    let rounds = self.tiles[y]
                        .iter()
                        .enumerate()
                        .filter(|(_, tile)| **tile == Tile::Round)
                        .map(|(x, _)| x)
                        .collect::<Vec<_>>();
                    for x in rounds {
                        let mut x = x;
                        while x > 0 && self.get(x - 1, y).unwrap() == &Tile::Empty {
                            self.set(x, y, Tile::Empty);
                            self.set(x - 1, y, Tile::Round);
                            x -= 1;
                        }
                    }
                }
            }
            Direction::Right => {
                for y in 0..self.tiles.len() {
                    let rounds = self.tiles[y]
                        .iter()
                        .enumerate()
                        .filter(|(_, tile)| **tile == Tile::Round)
                        .map(|(x, _)| x)
                        .rev()
                        .collect::<Vec<_>>();
                    for x in rounds {
                        let mut x = x;
                        while x < self.tiles[0].len() - 1
                            && self.get(x + 1, y).unwrap() == &Tile::Empty
                        {
                            self.set(x, y, Tile::Empty);
                            self.set(x + 1, y, Tile::Round);
                            x += 1;
                        }
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
        .iter()
        .for_each(|direction| {
            self.roll(*direction);
        });
    }
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let mut map = Map::from_lines(input);
    // map.print();
    map.roll(Direction::Up);
    let sol = map.load(Direction::Up);
    eprintln!("{:?}", Instant::now() - start_time);
    Ok(sol)
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let map = Map::from_lines(input);

    // map.print();
    let mut map = map;
    let mut prev = vec![];
    let mut cycle_start = None;
    for _ in 0..1_000_000_000 {
        prev.push(map.clone());
        map.cycle();
        if prev.contains(&map) {
            cycle_start = Some(prev.iter().position(|m| *m == map).unwrap());
            break;
        }
    }

    let cycle_start = cycle_start.unwrap();
    let cycle_length = prev.len() - cycle_start;
    let cycle_index = (1_000_000_000 - cycle_start) % cycle_length;

    let map = prev[cycle_start..][cycle_index].clone();
    let sol = map.load(Direction::Up) as i32;
    eprintln!("{:?}", Instant::now() - start_time);
    Ok(sol)
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
            110274
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_2.txt"))?;
        assert_eq!(solve_task_two(file)?, 64);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 90982);
        Ok(())
    }
}

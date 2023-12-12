use std::time::Instant;

use color_eyre::{
    eyre::{anyhow, Report},
    Result,
};
#[derive(Debug)]
enum Pipe {
    NorthSouth,
    WestEast,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl TryFrom<&char> for Pipe {
    type Error = Report;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::NorthSouth),
            '-' => Ok(Pipe::WestEast),
            'L' => Ok(Pipe::NorthEast),
            'J' => Ok(Pipe::NorthWest),
            'F' => Ok(Pipe::SouthEast),
            '7' => Ok(Pipe::SouthWest),
            _ => Err(anyhow!("Invalid pipe character")),
        }
    }
}

impl Pipe {
    fn get_directions(&self) -> Vec<Direction> {
        match self {
            Pipe::NorthSouth => vec![Direction::North, Direction::South],
            Pipe::WestEast => vec![Direction::West, Direction::East],
            Pipe::NorthWest => vec![Direction::North, Direction::West],
            Pipe::NorthEast => vec![Direction::North, Direction::East],
            Pipe::SouthWest => vec![Direction::South, Direction::West],
            Pipe::SouthEast => vec![Direction::South, Direction::East],
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug)]
enum Tile {
    Ground,
    Pipe(Pipe),
    Start,
}

impl TryFrom<&char> for Tile {
    type Error = Report;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Ground),
            '|' | '-' | 'L' | 'J' | 'F' | '7' => Ok(Tile::Pipe(Pipe::try_from(value)?)),
            'S' => Ok(Tile::Start),
            _ => Err(anyhow!("Invalid tile character")),
        }
    }
}

impl Tile {
    fn get_directions(&self) -> Vec<Direction> {
        match self {
            Tile::Ground => vec![],
            Tile::Pipe(pipe) => pipe.get_directions(),
            Tile::Start => vec![
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
        }
    }

    fn can_connect(&self, other: &Direction) -> bool {
        match self {
            Tile::Ground => false,
            Tile::Pipe(pipe) => pipe.get_directions().contains(&other),
            Tile::Start => true,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl TryFrom<Vec<String>> for Map {
    type Error = Report;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut tiles = Vec::new();
        for line in value {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Tile::try_from(&c)?);
            }
            tiles.push(row);
        }
        Ok(Map { tiles })
    }
}

impl Map {
    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    fn get_start(&self) -> (usize, usize) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::Start = tile {
                    return (x, y);
                }
            }
        }
        panic!("No start found");
    }
}

fn get_loop(map: &Map, path: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let (x, y) = path[path.len() - 1];
    let tile = map.get_tile(x, y).unwrap();
    let mut paths = vec![path.clone()];
    let directions = tile.get_directions();
    for direction in directions {
        let (x, y) = match direction {
            Direction::North => {
                if y == 0 {
                    continue;
                }
                (x, y - 1)
            }
            Direction::South => (x, y + 1),
            Direction::West => {
                if x == 0 {
                    continue;
                }
                (x - 1, y)
            }
            Direction::East => (x + 1, y),
        };
        let tile = map.get_tile(x, y).unwrap();
        if let Tile::Start = tile {
            continue;
        }
        if tile.can_connect(&direction.get_opposite()) && !path.contains(&(x, y)) {
            let mut new_path = path.clone();
            new_path.push((x, y));
            paths.push(get_loop(map, new_path));
        }
    }
    paths.sort_by(|a, b| b.len().cmp(&a.len()));
    paths[0].clone()
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let map = Map::try_from(input)?;
    let start = map.get_start();
    let longest_path = get_loop(&map, vec![start]).len();
    eprintln!("Longest path: {}", longest_path);

    eprintln!("{:?}", Instant::now() - start_time);
    match longest_path % 2 {
        0 => Ok((longest_path / 2) as i32),
        _ => Ok((longest_path / 2 + 1) as i32),
    }
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
    fn test_case_one_example_1() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_2.txt"))?;
        assert_eq!(solve_task_one(file)?, 4);
        Ok(())
    }

    #[test]
    fn test_case_one_example_2() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_3.txt"))?;
        assert_eq!(solve_task_one(file)?, 4);
        Ok(())
    }

    #[test]
    fn test_case_one_example_3() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_5.txt"))?;
        assert_eq!(solve_task_one(file)?, 8);
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

    #[test]
    fn test_can_connect() {
        let map = crate::Map::try_from(vec![
            String::from("S|"),
            String::from("L."),
            String::from("J."),
        ])
        .unwrap();
        assert!(map
            .get_tile(0, 0)
            .unwrap()
            .can_connect(&crate::Direction::North));
        assert!(map
            .get_tile(0, 0)
            .unwrap()
            .can_connect(&crate::Direction::South));
        assert!(map
            .get_tile(0, 0)
            .unwrap()
            .can_connect(&crate::Direction::West));
        assert!(map
            .get_tile(0, 0)
            .unwrap()
            .can_connect(&crate::Direction::East));
        assert!(map
            .get_tile(1, 0)
            .unwrap()
            .can_connect(&crate::Direction::North));
        assert!(map
            .get_tile(1, 0)
            .unwrap()
            .can_connect(&crate::Direction::South));
        assert!(!map
            .get_tile(0, 1)
            .unwrap()
            .can_connect(&crate::Direction::West));
    }
}

use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    time::Instant,
};

use color_eyre::{
    eyre::{anyhow, Report},
    Result,
};

#[derive(Debug, Hash, PartialEq, Eq)]
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

    fn pipes() -> [Pipe; 6] {
        [
            Pipe::NorthSouth,
            Pipe::WestEast,
            Pipe::NorthWest,
            Pipe::NorthEast,
            Pipe::SouthWest,
            Pipe::SouthEast,
        ]
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

#[derive(Debug, PartialEq, Eq)]
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

    fn get_adjacent_tile(&self, (x, y): (usize, usize), direction: &Direction) -> Option<&Tile> {
        match direction {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    self.get_tile(x, y - 1)
                }
            }
            Direction::South => self.get_tile(x, y + 1),
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    self.get_tile(x - 1, y)
                }
            }
            Direction::East => self.get_tile(x + 1, y),
        }
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

    fn get_start_pipe_type(&self) -> Pipe {
        let start_loc = self.get_start();
        let (x, y) = start_loc;
        let tile = self.get_tile(x, y).unwrap();
        match tile {
            Tile::Start => {
                let mut maybe_start: HashSet<Pipe> = HashSet::from_iter(Pipe::pipes().into_iter());
                let north_tile = self
                    .get_adjacent_tile(start_loc, &Direction::North)
                    .unwrap();
                if north_tile.can_connect(&Direction::South) {
                    maybe_start.remove(&Pipe::SouthEast);
                    maybe_start.remove(&Pipe::SouthWest);
                    maybe_start.remove(&Pipe::WestEast);
                }
                let east_tile = self.get_adjacent_tile(start_loc, &Direction::East).unwrap();
                if east_tile.can_connect(&Direction::West) {
                    maybe_start.remove(&Pipe::NorthWest);
                    maybe_start.remove(&Pipe::SouthWest);
                    maybe_start.remove(&Pipe::NorthSouth);
                }
                let south_tile = self
                    .get_adjacent_tile(start_loc, &Direction::South)
                    .unwrap();
                if south_tile.can_connect(&Direction::North) {
                    maybe_start.remove(&Pipe::NorthEast);
                    maybe_start.remove(&Pipe::NorthWest);
                    maybe_start.remove(&Pipe::WestEast);
                }
                if let Some(west_tile) = self.get_adjacent_tile(start_loc, &Direction::West) {
                    if west_tile.can_connect(&Direction::East) {
                        maybe_start.remove(&Pipe::NorthEast);
                        maybe_start.remove(&Pipe::SouthEast);
                        maybe_start.remove(&Pipe::NorthSouth);
                    }
                };

                if maybe_start.len() != 1 {
                    panic!("Invalid start pipe");
                }
                maybe_start.into_iter().next().unwrap()
            }
            _ => panic!("Invalid start pipe"),
        }
    }

    fn update_tile(&mut self, (x, y): (usize, usize), tile: Tile) {
        self.tiles[y][x] = tile;
    }
}

fn get_loop(map: &Map, start_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::from(vec![start_pos]);
    let mut visited = HashSet::new();
    visited.insert(start_pos);

    while let Some(pos) = queue.pop_front() {
        let (x, y) = pos;
        let tile = map.get_tile(x, y).unwrap();
        for direction in tile.get_directions() {
            let other_pos = match direction {
                Direction::North => (x, if y > 0 { y - 1 } else { continue }),
                Direction::South => (x, y + 1),
                Direction::West => (
                    if x > 0 {
                        x - 1
                    } else {
                        continue;
                    },
                    y,
                ),
                Direction::East => (x + 1, y),
            };
            if visited.contains(&other_pos) {
                continue;
            }
            if let Some(other_tile) = map.get_adjacent_tile(pos, &direction) {
                if other_tile.can_connect(&direction.get_opposite()) {
                    visited.insert(other_pos);
                    queue.push_back(other_pos);
                }
            }
        }
    }

    visited.into_iter().collect()
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let map = Map::try_from(input)?;
    let start = map.get_start();
    let longest_path = get_loop(&map, start).len();
    eprintln!("Longest path: {}", longest_path);

    eprintln!("{:?}", Instant::now() - start_time);
    match longest_path % 2 {
        0 => Ok((longest_path / 2) as i32),
        _ => Ok((longest_path / 2 + 1) as i32),
    }
}

fn find_outside_points(map: &Map) -> HashSet<(usize, usize)> {
    let mut outside = HashSet::new();

    for (y, row) in map.tiles.iter().enumerate() {
        let mut indise = false;
        let mut facing_up = None;
        for (x, tile) in row.iter().enumerate() {
            match tile {
                Tile::Pipe(Pipe::NorthSouth) => {
                    if facing_up != None {
                        panic!("Invalid map 1");
                    }
                    indise = !indise;
                }
                Tile::Pipe(Pipe::WestEast) => {
                    if facing_up == None {
                        panic!("Invalid map 2");
                    }
                }
                Tile::Pipe(Pipe::NorthEast) | Tile::Pipe(Pipe::SouthEast) => {
                    if facing_up != None {
                        panic!("Invalid map 3");
                    }
                    if let Tile::Pipe(Pipe::NorthEast) = tile {
                        facing_up = Some(true);
                    } else {
                        facing_up = Some(false);
                    }
                }
                Tile::Pipe(Pipe::NorthWest) | Tile::Pipe(Pipe::SouthWest) => {
                    if facing_up == None {
                        panic!("Invalid map 4");
                    }
                    if let Some(facing_up) = facing_up {
                        if tile
                            != &if facing_up {
                                Tile::Pipe(Pipe::NorthWest)
                            } else {
                                Tile::Pipe(Pipe::SouthWest)
                            }
                        {
                            indise = !indise;
                        }
                    }
                    facing_up = None;
                }
                Tile::Ground => {}
                _ => panic!("Unexpected character"),
            }
            if !indise {
                outside.insert((x, y));
            }
        }
    }

    outside
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let mut map = Map::try_from(input)?;
    let start = map.get_start();
    let start_type = map.get_start_pipe_type();
    let path = get_loop(&map, start);
    for y in 0..map.tiles.len() {
        for x in 0..map.tiles[y].len() {
            if !path.contains(&(x, y)) {
                map.update_tile((x, y), Tile::Ground);
            }
        }
    }
    map.update_tile(start, Tile::Pipe(start_type));
    let map = map;
    let outside = find_outside_points(&map);

    let map_size: i32 = (map.tiles.len() * map.tiles[0].len()).try_into().unwrap();
    let path = HashSet::from_iter(path.iter().cloned());

    let sol = map_size - outside.union(&path).count() as i32;

    eprintln!("{:?}", Instant::now() - start_time);
    Ok(sol as i32)
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
            6800
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_8.txt"))?;
        assert_eq!(solve_task_two(file)?, 1);
        Ok(())
    }

    #[test]
    fn test_case_two_example_2() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_5.txt"))?;
        assert_eq!(solve_task_two(file)?, 1);
        Ok(())
    }

    #[test]
    fn test_case_two_example_3() -> Result<()> {
        assert_eq!(
            solve_task_two(get_file(PathBuf::from("inputs/example_10.txt"))?)?,
            4
        );
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 483);
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

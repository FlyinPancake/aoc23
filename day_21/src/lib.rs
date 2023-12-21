use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use color_eyre::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapTile {
    Start,
    Garden,
    Rock,
}

impl MapTile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Garden,
            '#' => Self::Rock,
            'S' => Self::Start,
            _ => panic!("Invalid char"),
        }
    }
}
#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<MapTile>>,
    start: (usize, usize),
    mem: HashMap<(usize, usize), VecDeque<(usize, usize)>>,
}

impl Map {
    fn from_lines(lines: Vec<String>) -> Self {
        let mut start = None;
        let tiles = lines
            .iter()
            .enumerate()
            .map(|(y, line)| {
                let line: Vec<MapTile> = line.chars().map(MapTile::from_char).collect();
                if line.contains(&MapTile::Start) {
                    if start.is_some() {
                        panic!("Multiple starts");
                    }
                    start = Some((line.iter().position(|&x| x == MapTile::Start).unwrap(), y));
                }
                line
            })
            .collect();
        Self {
            tiles,
            start: start.unwrap(),
            mem: HashMap::new(),
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<MapTile> {
        self.tiles.get(y).and_then(|row| row.get(x)).copied()
    }

    fn get_neighbors(&mut self, x: usize, y: usize) -> VecDeque<(usize, usize)> {
        if let Some(neighbors) = self.mem.get(&(x, y)) {
            return neighbors.clone();
        }
        let mut neighbors = VecDeque::new();
        if x > 0 && !matches!(self.get_tile(x - 1, y), Some(MapTile::Rock)) {
            neighbors.push_back((x - 1, y));
        }
        if y > 0 && !matches!(self.get_tile(x, y - 1), Some(MapTile::Rock)) {
            neighbors.push_back((x, y - 1));
        }
        if x < self.tiles.len() - 1 && !matches!(self.get_tile(x + 1, y), Some(MapTile::Rock)) {
            neighbors.push_back((x + 1, y));
        }
        if y < self.tiles[0].len() - 1 && !matches!(self.get_tile(x, y + 1), Some(MapTile::Rock)) {
            neighbors.push_back((x, y + 1));
        }
        self.mem.insert((x, y), neighbors.clone());
        neighbors
    }
}

pub fn solve_task_one(input: Vec<String>, steps: i32) -> Result<i32> {
    let start_time = Instant::now();
    let mut map = Map::from_lines(input);
    let mut queue = map
        .get_neighbors(map.start.0, map.start.1)
        .into_iter()
        .map(|(x, y)| (x, y, 1))
        .collect::<VecDeque<_>>();
    let mut visited = HashSet::new();
    visited.insert(map.start);
    let mut current_step = HashSet::new();
    while let Some((x, y, cur_steps)) = queue.pop_front() {
        if cur_steps > steps {
            break;
        }
        if current_step.insert(cur_steps) {
            eprintln!("Step: {}", cur_steps);
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        if cur_steps % 2 == 0 {
            visited.insert((x, y));
        }
        if matches!(map.get_tile(x, y), Some(MapTile::Garden)) {
            queue.extend(
                map.get_neighbors(x, y)
                    .into_iter()
                    .map(|(x, y)| (x, y, cur_steps + 1)),
            );
        }
    }

    eprintln!("{:?}", Instant::now() - start_time);
    Ok(visited.len() as i32)
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
        assert_eq!(solve_task_one(file, 6)?, 16);
        Ok(())
    }

    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_one(file, 64)?, 0);
        Ok(())
    }
    #[ignore = "Not implemented yet"]
    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_two(file)?, 0);
        Ok(())
    }
    #[ignore = "Not implemented yet"]
    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 0);
        Ok(())
    }
}

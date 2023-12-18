use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use color_eyre::Result;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum Element {
    BackMirror,
    ForwardMirror,
    Empty,
    HorizontalSplitter,
    VerticalSplitter,
}

impl Element {
    fn from_char(c: char) -> Self {
        match c {
            '\\' => Self::BackMirror,
            '/' => Self::ForwardMirror,
            '.' => Self::Empty,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => panic!("Invalid element"),
        }
    }

    fn get_egress_dirs(&self, beam_direction: Direction) -> Vec<Direction> {
        match self {
            Element::BackMirror => vec![match beam_direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }],
            Element::ForwardMirror => vec![match beam_direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            }],
            Element::Empty => vec![beam_direction],
            Element::HorizontalSplitter => match beam_direction {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => vec![beam_direction],
            },
            Element::VerticalSplitter => match beam_direction {
                Direction::Up | Direction::Down => vec![beam_direction],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            },
        }
    }
}
#[derive(Debug, Clone, PartialEq, Hash)]
struct Map {
    elements: Vec<Vec<Element>>,
}

impl Map {
    fn from_input(input: Vec<String>) -> Self {
        let elements = input
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|c| Element::from_char(c))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { elements }
    }

    fn get_element(&self, (x, y): (usize, usize)) -> Option<Element> {
        self.elements.get(y).and_then(|row| row.get(x)).copied()
    }

    fn get_visited_locations(&self, entry: ((usize, usize), Direction)) -> HashSet<(usize, usize)> {
        let mut queue = VecDeque::new();
        queue.push_back(entry);
        let mut visited = HashSet::new();
        visited.insert(entry);

        while let Some(((x, y), beam_dir)) = queue.pop_front() {
            let element = self.get_element((x, y)).unwrap();
            let egress_dirs = element.get_egress_dirs(beam_dir);
            for dir in egress_dirs {
                let (x, y) = match dir {
                    Direction::Up => {
                        if y == 0 {
                            continue;
                        }
                        (x, y - 1)
                    }
                    Direction::Down => {
                        if y == self.elements.len() - 1 {
                            continue;
                        }
                        (x, y + 1)
                    }
                    Direction::Left => {
                        if x == 0 {
                            continue;
                        }
                        (x - 1, y)
                    }
                    Direction::Right => {
                        if x == self.elements[0].len() - 1 {
                            continue;
                        }
                        (x + 1, y)
                    }
                };
                if visited.insert(((x, y), dir)) {
                    queue.push_back(((x, y), dir));
                }
            }
        }

        visited.into_iter().map(|(pos, _)| pos).collect()
    }
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let map = Map::from_input(input);
    let visited_locations = &map.get_visited_locations(((0, 0), Direction::Right));
    let sol = visited_locations.len();
    eprintln!("{:?}", Instant::now() - start_time);
    Ok(sol as i32)
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let map = Map::from_input(input);
    let mut entry_locations = vec![];
    for y in 0..map.elements.len() {
        entry_locations.push(((0, y), Direction::Right));
        entry_locations.push(((map.elements[y].len() - 1, y), Direction::Left));
    }
    for x in 0..map.elements[0].len() {
        entry_locations.push(((x, 0), Direction::Down));
        entry_locations.push(((x, map.elements.len() - 1), Direction::Up));
    }
    let visited_locations: Vec<usize> = entry_locations
        .into_par_iter()
        .map(|entry| map.get_visited_locations(entry).len())
        .collect();

    let sol = *visited_locations.iter().max().unwrap();

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
    fn test_case_one_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_one(file)?, 46);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        assert_eq!(
            solve_task_one(get_file(cargo_manifest_dir.join("inputs/full.txt"))?)?,
            6816
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_two(file)?, 51);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 8163);
        Ok(())
    }
}

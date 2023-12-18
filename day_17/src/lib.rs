use std::{
    collections::{binary_heap::BinaryHeap, HashSet},
    time::Instant,
};

use color_eyre::Result;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    pos: (i32, i32),
    moving_dir: (i32, i32),
    moving_since: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct QueueNode {
    heat_loss: i32,
    node: Node,
}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.heat_loss.cmp(&other.heat_loss).reverse())
    }
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let input: Vec<Vec<u32>> = input
        .par_iter()
        .map(|x| {
            x.chars()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(QueueNode {
        heat_loss: 0,
        node: Node {
            pos: (0, 0),
            moving_dir: (0, 0),
            moving_since: 0,
        },
    });
    let mut sol = None;
    let mut i = 0;
    while let Some(node) = queue.pop() {
        i += 1;
        if node.node.pos == (input[0].len() as i32 - 1, input.len() as i32 - 1) {
            sol = Some(node.heat_loss);
            break;
        }
        if seen.contains(&node.node) {
            continue;
        }
        seen.insert(node.node);

        let next_pos = (
            node.node.pos.0 + node.node.moving_dir.0,
            node.node.pos.1 + node.node.moving_dir.1,
        );

        if node.node.moving_since < 3 && node.node.moving_dir != (0, 0) {
            if 0 <= next_pos.0
                && next_pos.0 < input[0].len() as i32
                && 0 <= next_pos.1
                && next_pos.1 < input.len() as i32
            {
                queue.push(QueueNode {
                    heat_loss: node.heat_loss
                        + input[next_pos.1 as usize][next_pos.0 as usize] as i32,
                    node: Node {
                        pos: next_pos,
                        moving_dir: node.node.moving_dir,
                        moving_since: node.node.moving_since + 1,
                    },
                });
            }
        }
        for (next_dir_row, next_dir_col) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if (next_dir_col, next_dir_row) != node.node.moving_dir
                && (-next_dir_col, -next_dir_row) != node.node.moving_dir
            {
                let next_pos = (
                    node.node.pos.0 + next_dir_col,
                    node.node.pos.1 + next_dir_row,
                );
                let next_node = Node {
                    pos: next_pos,
                    moving_dir: (next_dir_col, next_dir_row),
                    moving_since: 1,
                };
                if 0 <= next_pos.0
                    && next_pos.0 < input[0].len() as i32
                    && 0 <= next_pos.1
                    && next_pos.1 < input.len() as i32
                {
                    queue.push(QueueNode {
                        heat_loss: node.heat_loss
                            + input[next_pos.1 as usize][next_pos.0 as usize] as i32,
                        node: next_node,
                    });
                }
            }
        }
    }
    eprintln!("Iterations: {}", i);

    eprintln!("⏱️  Took: {:?}", Instant::now() - start_time);
    Ok(sol.unwrap())
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let input: Vec<Vec<u32>> = input
        .par_iter()
        .map(|x| {
            x.chars()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(QueueNode {
        heat_loss: 0,
        node: Node {
            pos: (0, 0),
            moving_dir: (0, 0),
            moving_since: 0,
        },
    });
    let mut sol = None;
    let mut i = 0;
    while let Some(node) = queue.pop() {
        i += 1;
        if node.node.pos == (input[0].len() as i32 - 1, input.len() as i32 - 1)
            && node.node.moving_since >= 4
        {
            sol = Some(node.heat_loss);
            break;
        }
        if seen.contains(&node.node) {
            continue;
        }
        seen.insert(node.node);

        let next_pos = (
            node.node.pos.0 + node.node.moving_dir.0,
            node.node.pos.1 + node.node.moving_dir.1,
        );

        if node.node.moving_since < 10 && node.node.moving_dir != (0, 0) {
            if 0 <= next_pos.0
                && next_pos.0 < input[0].len() as i32
                && 0 <= next_pos.1
                && next_pos.1 < input.len() as i32
            {
                queue.push(QueueNode {
                    heat_loss: node.heat_loss
                        + input[next_pos.1 as usize][next_pos.0 as usize] as i32,
                    node: Node {
                        pos: next_pos,
                        moving_dir: node.node.moving_dir,
                        moving_since: node.node.moving_since + 1,
                    },
                });
            }
        }

        if node.node.moving_dir == (0, 0) || node.node.moving_since >= 4 {
            for (next_dir_row, next_dir_col) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if (next_dir_col, next_dir_row) != node.node.moving_dir
                    && (-next_dir_col, -next_dir_row) != node.node.moving_dir
                {
                    let next_pos = (
                        node.node.pos.0 + next_dir_col,
                        node.node.pos.1 + next_dir_row,
                    );
                    let next_node = Node {
                        pos: next_pos,
                        moving_dir: (next_dir_col, next_dir_row),
                        moving_since: 1,
                    };
                    if 0 <= next_pos.0
                        && next_pos.0 < input[0].len() as i32
                        && 0 <= next_pos.1
                        && next_pos.1 < input.len() as i32
                    {
                        queue.push(QueueNode {
                            heat_loss: node.heat_loss
                                + input[next_pos.1 as usize][next_pos.0 as usize] as i32,
                            node: next_node,
                        });
                    }
                }
            }
        }
    }
    eprintln!("Iterations: {}", i);

    eprintln!("⏱️  Took: {:?}", Instant::now() - start_time);
    Ok(sol.unwrap())
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
        assert_eq!(solve_task_one(file)?, 102);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        assert_eq!(
            solve_task_one(get_file(cargo_manifest_dir.join("inputs/full.txt"))?)?,
            785
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_two(file)?, 94);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 922);
        Ok(())
    }
}

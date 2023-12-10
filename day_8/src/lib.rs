use color_eyre::Result;
use rayon::prelude::*;
use std::{collections::HashMap, time::Instant};

static START_NODE: &str = "AAA";
static END_NODE: &str = "ZZZ";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    pub value: String,
    left: String,
    right: String,
}

impl<'a> Node {
    fn new(value: String, left: String, right: String) -> Self {
        Self { value, left, right }
    }
}

pub fn solve_task_one(input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();
    let mut o_input = input.iter();
    let dirs = o_input.next().unwrap().clone();

    let raw_nodes = input
        .into_iter()
        .skip(2)
        .map(|el| {
            let mut parts = el.split("=");
            let pars: &[_] = &['(', ')'];
            (
                parts.next().unwrap().trim().to_string(),
                parts
                    .next()
                    .unwrap()
                    .trim()
                    .trim_matches(pars)
                    .split(',')
                    .map(|part| part.trim().to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(v, lrs)| (v, (lrs[0].clone(), lrs[1].clone())))
        .collect::<Vec<_>>();

    let mut nodes: HashMap<String, Node> = HashMap::new();

    for (v, (l, r)) in raw_nodes.into_iter() {
        nodes.insert(v.clone(), Node::new(v, l, r));
    }

    let mut reps = 0;
    let mut cur_node = &nodes[START_NODE];
    let end_node = &nodes[END_NODE];

    while cur_node != end_node {
        reps += 1;
        for d in dirs.chars() {
            match d {
                'L' => cur_node = &nodes[&nodes[&cur_node.value].left],
                'R' => cur_node = &nodes[&nodes[&cur_node.value].right],
                _ => panic!(),
            }
        }
    }

    eprintln!("{:?}", Instant::now() - start_time);
    Ok(reps * dirs.len() as i64)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a.abs() / gcd(a, b)) * b.abs()
    }
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();
    let mut o_input = input.iter();
    let dirs = o_input.next().unwrap().clone();

    let raw_nodes = input
        .into_iter()
        .skip(2)
        .map(|el| {
            let mut parts = el.split("=");
            let pars: &[_] = &['(', ')'];
            (
                parts.next().unwrap().trim().to_string(),
                parts
                    .next()
                    .unwrap()
                    .trim()
                    .trim_matches(pars)
                    .split(',')
                    .map(|part| part.trim().to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(v, lrs)| (v, (lrs[0].clone(), lrs[1].clone())))
        .collect::<Vec<_>>();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    for (v, (l, r)) in raw_nodes.into_iter() {
        nodes.insert(v.clone(), Node::new(v, l, r));
    }

    let start_nodes: Vec<_> = nodes
        .iter()
        .filter(|n| n.0.ends_with('A'))
        .map(|n| n.1.clone())
        .collect();

    let dirs: Vec<char> = dirs.chars().collect();

    let circle_lens = start_nodes.iter().map(|n| {
        let mut n = n;
        let mut end_node = None;
        let end_node = loop {
            let mut dirs = dirs.iter().enumerate();

            while let Some((idx, dir)) = dirs.next() {
                if n.value.ends_with("Z") {
                    end_node = Some((idx, n));
                    break;
                }
                match dir {
                    'L' => n = &nodes[&n.left],
                    'R' => n = &nodes[&n.right],
                    _ => panic!("wrong char in dirs {dir}"),
                }
            }

            if let Some(end_node) = end_node {
                break end_node;
            }
        };

        let (end_idx, end_node) = end_node;
        let mut circle_nodes = vec![end_node];
        let mut n = end_node;
        let mut ldirs = dirs.iter().skip(end_idx);
        let circle_nodes = loop {
            if let Some(dir) = ldirs.next() {
                match dir {
                    'L' => n = &nodes[&n.left],
                    'R' => n = &nodes[&n.right],
                    _ => panic!(),
                }
                if end_node == n {
                    break circle_nodes;
                } else {
                    circle_nodes.push(n);
                }
            } else {
                ldirs = dirs.iter().skip(0);
            }
        };

        circle_nodes.len() as i64
    });

    eprintln!("{:?}", circle_lens);
    let lcm = circle_lens.into_iter().fold(1, lcm);
    eprintln!("Took {:?}", Instant::now() - start_time);
    Ok(lcm)
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
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_one(file)?, 2);
        Ok(())
    }
    #[test]
    fn test_case_one_example_2() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_2.txt"))?;
        assert_eq!(solve_task_one(file)?, 6);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        assert_eq!(
            solve_task_one(get_file(cargo_manifest_dir.join("inputs/full.txt"))?)?,
            12737
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        assert_eq!(
            solve_task_two(get_file(PathBuf::from("inputs/example_3.txt"))?)?,
            6
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

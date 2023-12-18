use std::{collections::HashMap, str::FromStr, time::Instant};

use color_eyre::{eyre::anyhow, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum HotSpringState {
    Operating,
    Damaged,
    Unknown,
}

impl TryFrom<char> for HotSpringState {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(HotSpringState::Operating),
            '#' => Ok(HotSpringState::Damaged),
            '?' => Ok(HotSpringState::Unknown),
            _ => Err(anyhow!("Invalid character")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct HotSpringRow {
    states: Vec<HotSpringState>,
    broken_spans: Vec<usize>,
}

impl FromStr for HotSpringRow {
    type Err = color_eyre::Report;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = value.split_ascii_whitespace();
        let states: Result<Vec<HotSpringState>> = parts
            .next()
            .unwrap()
            .chars()
            .into_iter()
            .map(HotSpringState::try_from)
            .collect();

        let broken_spans: Vec<usize> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self {
            states: states?,
            broken_spans,
        })
    }
}

fn count_solutions(row: HotSpringRow, cache: &mut HashMap<HotSpringRow, i64>) -> i64 {
    if row.states.is_empty() {
        return if row.broken_spans.is_empty() { 1 } else { 0 };
    }
    if row.broken_spans.is_empty() {
        return if row.states.contains(&HotSpringState::Damaged) {
            0
        } else {
            1
        };
    }

    if let Some(&cached) = cache.get(&row) {
        return cached;
    }

    let mut result = 0;
    if &HotSpringState::Damaged != row.states.first().unwrap() {
        let mut new_row = row.clone();
        new_row.states = new_row.states[1..].to_vec();
        result += count_solutions(new_row, cache);
    }
    let first_span = row.broken_spans[0];
    if row.states.len() >= first_span
        && !row.states[..first_span].contains(&HotSpringState::Operating)
    {
        if row.states.len() == first_span {
            let mut new_row = row.clone();
            new_row.broken_spans = new_row.broken_spans[1..].to_vec();
            new_row.states = vec![];
            result += count_solutions(new_row, cache);
        } else if row.states[first_span] != HotSpringState::Damaged {
            let mut new_row = row.clone();
            new_row.broken_spans = new_row.broken_spans[1..].to_vec();
            new_row.states = row.states[(first_span + 1)..].to_vec();
            result += count_solutions(new_row, cache);
        }
    }
    cache.insert(row, result);
    result
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();
    let mut cache = HashMap::new();
    let rows = input
        .into_iter()
        .map(|line| line.parse::<HotSpringRow>().unwrap())
        .collect::<Vec<HotSpringRow>>();
    let totals: Vec<i64> = rows
        .into_iter()
        .map(|el| count_solutions(el, &mut cache))
        .collect();
    let total = totals.iter().sum();
    eprintln!("{:?}", Instant::now() - start_time);
    Ok(total)
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();
    let mut cache = HashMap::new();

    let input = input
        .iter()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let states = parts.next().unwrap();
            let broken_spans = parts.next().unwrap();
            let unpacked_states = [states; 5].into_iter().fold("".to_string(), |acc, s| {
                if acc.is_empty() {
                    acc + s
                } else {
                    acc + "?" + s
                }
            });
            let unpacked_brocken_spans =
                [broken_spans; 5]
                    .into_iter()
                    .fold("".to_string(), |acc, s| {
                        if !acc.is_empty() {
                            acc + "," + s
                        } else {
                            acc + s
                        }
                    });

            format!("{} {}", unpacked_states, unpacked_brocken_spans)
        })
        .collect::<Vec<String>>();

    let rows = input
        .into_iter()
        .map(|line| line.parse::<HotSpringRow>().unwrap())
        .collect::<Vec<HotSpringRow>>();
    let totals: Vec<i64> = rows
        .into_iter()
        .map(|el| count_solutions(el, &mut cache))
        .collect();
    let total = totals.iter().sum();

    eprintln!("{:?}", Instant::now() - start_time);
    Ok(total)
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
        let file = get_file(cargo_manifest_dir.join("inputs/example_2.txt"))?;
        assert_eq!(solve_task_one(file)?, 21);
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
            525152
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

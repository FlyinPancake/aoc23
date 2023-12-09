use std::time::Instant;

use color_eyre::Result;

fn get_first_value(nums: Vec<i32>) -> i32 {
    let mut differences = vec![nums];
    let mut current: &Vec<i32> = differences.last_mut().unwrap();
    loop {
        let mut diffs = vec![];
        let mut last_num = current.first().unwrap();
        for n in current.into_iter().skip(1) {
            diffs.push(*n - *last_num);
            last_num = n;
        }
        differences.push(diffs);
        current = differences.last_mut().unwrap();
        if current.iter().all(|v| *v == 0) {
            break;
        }
    }
    let mut next_differences = differences.clone();
    next_differences.last_mut().unwrap().insert(0, 0);
    let mut prev = next_differences.pop().unwrap();
    prev.reverse();
    while let Some(mut cur) = next_differences.pop() {
        cur.reverse();
        let cur_last = *cur.last().unwrap();
        let prev_last = prev.last_mut().unwrap();
        cur.push(*prev_last + cur_last);
        prev = cur;
    }
    *prev.last().unwrap()
}

fn get_next_value(nums: Vec<i32>) -> i32 {
    let mut differences = vec![nums];
    let mut current: &Vec<i32> = differences.last_mut().unwrap();
    loop {
        let mut diffs = vec![];
        let mut last_num = current.first().unwrap();
        for n in current.into_iter().skip(1) {
            diffs.push(*n - *last_num);
            last_num = n;
        }
        differences.push(diffs);
        current = differences.last_mut().unwrap();
        if current.iter().all(|v| *v == 0) {
            break;
        }
    }
    // differences.reverse();
    let mut next_differences = differences.clone();
    next_differences.last_mut().unwrap().push(0);
    let mut prev = next_differences.pop().unwrap();
    while let Some(mut cur) = next_differences.pop() {
        let cur_last = *cur.last().unwrap();
        let prev_last = prev.last_mut().unwrap();
        cur.push(*prev_last - cur_last);
        prev = cur;
    }
    *prev.last().unwrap()
}

pub fn solve_task_one(input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let sensor_value_histories = input
        .into_iter()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // eprintln!("{:?}", sensor_value_histories);
    let sol = sensor_value_histories.into_iter().map(get_next_value).sum();
    eprintln!("{:?}", Instant::now() - start_time);
    Ok(sol)
}

pub fn solve_task_two(input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let sensor_value_histories = input
        .into_iter()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // eprintln!("{:?}", sensor_value_histories);
    let sol = sensor_value_histories
        .into_iter()
        .map(get_first_value)
        .sum();
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
        assert_eq!(solve_task_one(file)?, 114);
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

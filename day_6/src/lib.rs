use std::time::Instant;

use color_eyre::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn solve_task_one(input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let times = input[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|t| t.parse::<i32>().unwrap());

    let distances = input[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(|t| t.parse::<i32>().unwrap());

    let races = times.zip(distances);
    let sol = races
        .map(|(t, d)| {
            (1..=t)
                .map(|charge| {
                    let race = t - charge;
                    let dist = race * charge;
                    dist
                })
                .filter(|race_dist| *race_dist > d)
                .count() as i32
        })
        .product();
    eprintln!("⏱️  Took {:#?}", Instant::now() - start_time);
    Ok(sol)
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();
    let time: i64 = input[0]
        .split_ascii_whitespace()
        .skip(1)
        .fold("".to_string(), |a, b| format!("{}{}", a, b))
        .parse()?;

    let record_distance: i64 = input[1]
        .split_ascii_whitespace()
        .skip(1)
        .fold("".to_string(), |a, b| format!("{}{}", a, b))
        .parse()?;

    let sol: i64 = (1..=time)
        .into_par_iter()
        .map(|ch_time| ch_time * (time - ch_time))
        .filter(|race_dist| record_distance < *race_dist)
        .count() as i64;

    eprintln!("⏱️  Took {:#?}", Instant::now() - start_time);
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
        assert_eq!(solve_task_one(file)?, 288);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        assert_eq!(
            solve_task_one(get_file(cargo_manifest_dir.join("inputs/full.txt"))?)?,
            170000
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_2.txt"))?;
        assert_eq!(solve_task_two(file)?, 71503);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 20537782);
        Ok(())
    }
}

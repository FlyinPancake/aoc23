use color_eyre::{eyre::anyhow, Result};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn solve_task_one(input: Vec<String>) -> Result<i32> {
    let lines = input;

    let lines = lines
        .into_par_iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sol = lines
        .into_par_iter()
        .map(|line| line.first().unwrap() * 10 + line.last().unwrap())
        .sum::<u32>();
    i32::try_from(sol).map_err(|_| anyhow!("Number doesn't fit!"))
}

fn solve_task_two(input: Vec<String>) -> Result<i32> {
    let lines = input
        .into_par_iter()
        .map(|line| {
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .collect::<Vec<_>>();

    let lines = lines
        .into_par_iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sol = lines
        .into_par_iter()
        .map(|line| line.first().unwrap() * 10 + line.last().unwrap())
        .sum::<u32>();
    i32::try_from(sol).map_err(|_| anyhow!("Number doesn't fit!"))
}

fn main() {
    unreachable!();
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
        assert_eq!(
            solve_task_one(get_file(PathBuf::from("inputs/example_1.txt"))?)?,
            142
        );
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        assert_eq!(
            solve_task_one(get_file(PathBuf::from("inputs/full.txt"))?)?,
            55017
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        assert_eq!(
            solve_task_two(get_file(PathBuf::from("inputs/example_2.txt"))?)?,
            281
        );
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        assert_eq!(
            solve_task_two(get_file(PathBuf::from("inputs/full.txt"))?)?,
            53539
        );
        Ok(())
    }
}

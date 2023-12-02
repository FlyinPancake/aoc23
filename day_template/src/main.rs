use color_eyre::Result;

fn solve_task_one(input: Vec<String>) -> Result<i32> {
    todo!()
}

fn solve_task_two(input: Vec<String>) -> Result<i32> {
    todo!()
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
            0
        );
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        assert_eq!(
            solve_task_one(get_file(PathBuf::from("inputs/full.txt"))?)?,
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
}

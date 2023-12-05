use color_eyre::Result;

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    todo!()
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
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
        assert_eq!(solve_task_one(file)?, 0);
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

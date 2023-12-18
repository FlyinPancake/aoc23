use std::time::Instant;

use color_eyre::Result;

pub fn find_hor_reflections(pattern: Vec<String>) -> Vec<usize> {
    let mut reflections = vec![];
    for i in 1..pattern.len() {
        let lines_above = i;
        let lines_below = pattern.len() - i;
        let reflection_size = if lines_above > lines_below {
            lines_below
        } else {
            lines_above
        };
        let mut bottom_part = pattern[i..i + reflection_size].to_vec();
        bottom_part.reverse();
        if pattern[i - reflection_size..i] == bottom_part {
            reflections.push(i * 100);
            break;
        }
    }
    reflections
}

pub fn find_ver_reflections(pattern: Vec<String>) -> Vec<usize> {
    let mut reflections = vec![];
    let width = pattern[0].len();
    for i in 1..width {
        let bars_left = i;
        let bars_right = width - i;
        let reflection_size = if bars_left > bars_right {
            bars_right
        } else {
            bars_left
        };
        let left_part = pattern
            .iter()
            .map(|l| l[i - reflection_size..i].to_string())
            .collect::<Vec<String>>();
        let right_part = pattern
            .iter()
            .map(|l| l[i..i + reflection_size].chars().rev().collect::<String>())
            .collect::<Vec<String>>();

        if left_part == right_part {
            reflections.push(i);
        }
    }
    reflections
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();

    let patterns: Vec<Vec<String>> = input.split(|x| x == "").map(|el| el.to_vec()).collect();
    // eprintln!("{:?}", patterns.len());
    let sol: usize = patterns
        .iter()
        .map(|pattern| {
            let hor_reflections = find_hor_reflections(pattern.to_vec());
            let ver_reflections = find_ver_reflections(pattern.to_vec());

            hor_reflections.get(0).unwrap_or(&0) + ver_reflections.get(0).unwrap_or(&0)
        })
        .sum();

    eprintln!("{:?}", Instant::now() - start_time);

    Ok(sol as i32)
}

pub fn find_smudged_hor_reflections(pattern: Vec<String>) -> Vec<usize> {
    let mut reflections = vec![];
    for i in 1..pattern.len() {
        let lines_above = i;
        let lines_below = pattern.len() - i;
        let reflection_size = if lines_above > lines_below {
            lines_below
        } else {
            lines_above
        };
        let mut bottom_part = pattern[i..i + reflection_size].to_vec();
        bottom_part.reverse();
        let differences: usize = bottom_part
            .iter()
            .zip(pattern[i - reflection_size..i].iter())
            .map(|(a, b)| a.chars().zip(b.chars()).filter(|(a, b)| a != b).count())
            .sum();

        if differences == 1 {
            reflections.push(i * 100);
            break;
        }
    }
    reflections
}

pub fn find_smudged_ver_reflections(pattern: Vec<String>) -> Vec<usize> {
    let mut reflections = vec![];
    let width = pattern[0].len();
    for i in 1..width {
        let bars_left = i;
        let bars_right = width - i;
        let reflection_size = if bars_left > bars_right {
            bars_right
        } else {
            bars_left
        };
        let left_part = pattern
            .iter()
            .map(|l| l[i - reflection_size..i].to_string())
            .collect::<Vec<String>>();
        let right_part = pattern
            .iter()
            .map(|l| l[i..i + reflection_size].chars().rev().collect::<String>())
            .collect::<Vec<String>>();
        let differences: usize = left_part
            .iter()
            .zip(right_part.iter())
            .map(|(a, b)| a.chars().zip(b.chars()).filter(|(a, b)| a != b).count())
            .sum();
        if differences == 1 {
            reflections.push(i);
        }
    }
    reflections
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let patterns: Vec<Vec<String>> = input.split(|x| x == "").map(|el| el.to_vec()).collect();
    let sol: usize = patterns
        .iter()
        .map(|pattern| {
            let hor_reflections = find_smudged_hor_reflections(pattern.to_vec());
            let ver_reflections = find_smudged_ver_reflections(pattern.to_vec());
            if hor_reflections.len() + ver_reflections.len() != 1 {
                eprintln!("{:#?}", pattern);
                eprintln!("{:?}", hor_reflections);
                eprintln!("{:?}", ver_reflections);
            }
            hor_reflections.get(0).unwrap_or(&0) + ver_reflections.get(0).unwrap_or(&0)
        })
        .sum();
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
        assert_eq!(solve_task_one(file)?, 405);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        assert_eq!(
            solve_task_one(get_file(cargo_manifest_dir.join("inputs/full.txt"))?)?,
            34911
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_two(file)?, 400);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 33183);
        Ok(())
    }
}

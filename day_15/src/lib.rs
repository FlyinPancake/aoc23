use std::time::Instant;

use color_eyre::Result;

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let sol = input[0].split(",").map(hash).sum::<u32>() as i32;
    eprintln!("{:?}", Instant::now() - start_time);
    Ok(sol)
}
#[derive(Debug, Clone)]
enum LensOperation {
    Add(Lens),
    Remove(String),
}

impl LensOperation {
    fn from_str(input: &str) -> Self {
        let split = &mut input.split(&['=', '-']);
        let label = split.next().unwrap().to_string();

        if input.contains('=') {
            let focal_length = split.next().unwrap().parse::<u32>().unwrap();
            Self::Add(Lens {
                label,
                focal_length,
            })
        } else {
            Self::Remove(label)
        }
    }
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    pub focal_length: u32,
}
#[derive(Debug, Clone)]
struct LensSystem {
    lenses: [Vec<Lens>; 256],
}

const EMPTY_VEC: Vec<Lens> = vec![];

impl Default for LensSystem {
    fn default() -> Self {
        Self {
            lenses: [EMPTY_VEC; 256],
        }
    }
}

impl LensSystem {
    fn add_lens(&mut self, lens: Lens) {
        let idx = hash(&lens.label) as usize;
        let add_idx = self.lenses[idx].iter().position(|l| l.label == lens.label);
        if let Some(add) = add_idx {
            self.lenses[idx][add].focal_length = lens.focal_length;
        } else {
            self.lenses[idx].push(lens);
        }
    }

    fn remove_lens(&mut self, label: String) {
        let idx = hash(&label) as usize;
        let rm_idx = self.lenses[idx].iter().position(|l| l.label == label);
        if let Some(rm) = rm_idx {
            self.lenses[idx].remove(rm);
        }
    }

    fn perform_operation(&mut self, operation: LensOperation) {
        match operation {
            LensOperation::Add(lens) => self.add_lens(lens),
            LensOperation::Remove(label) => self.remove_lens(label),
        }
    }

    fn get_focal_length(&self) -> i32 {
        self.lenses.iter().enumerate().fold(0, |acc, (idx, lens)| {
            acc + lens
                .iter()
                .enumerate()
                .fold(0, |lens_acc, (lens_idx, lens)| {
                    lens_acc + (idx as i32 + 1) * (lens_idx as i32 + 1) * lens.focal_length as i32
                })
        })
    }
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let operations: Vec<_> = input[0].split(",").map(LensOperation::from_str).collect();
    let mut lens_system = LensSystem::default();
    for operation in operations {
        lens_system.perform_operation(operation);
    }
    let sol = lens_system.get_focal_length();
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
        assert_eq!(solve_task_one(file)?, 1320);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        assert_eq!(
            solve_task_one(get_file(cargo_manifest_dir.join("inputs/full.txt"))?)?,
            519603
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_two(file)?, 145);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 244342);
        Ok(())
    }
}

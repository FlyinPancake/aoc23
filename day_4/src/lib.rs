use std::{collections::HashSet, str::FromStr, time::Instant};

use color_eyre::{eyre::anyhow, Result};

#[derive(Debug)]
struct ScratchCard {
    #[allow(dead_code)]
    id: i32,
    winning_numbers: HashSet<i32>,
    own_numbers: HashSet<i32>,
}

impl FromStr for ScratchCard {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let id: i32 = parts.next().unwrap()[5..].trim().parse()?;

        let mut parts = parts.next().unwrap().split("|");
        let winning_numbers = parts
            .next()
            .ok_or(anyhow!("No winning numbers found"))?
            .split_ascii_whitespace()
            .map(str::trim)
            .map(i32::from_str)
            .collect::<Result<HashSet<i32>, _>>()?;
        let own_numbers = parts
            .next()
            .ok_or(anyhow!("No own numbers"))?
            .split_ascii_whitespace()
            .map(str::trim)
            .map(i32::from_str)
            .collect::<Result<HashSet<_>, _>>()?;
        Ok(Self {
            id,
            winning_numbers,
            own_numbers,
        })
    }
}

impl ScratchCard {
    fn get_points(&self) -> Result<i32> {
        let matched: u32 = self.matches();

        if matched == 0 {
            Ok(0)
        } else {
            Ok(2_i32.pow(matched - 1))
        }
    }

    fn matches(&self) -> u32 {
        self.winning_numbers.intersection(&self.own_numbers).count() as u32
    }
}

pub fn solve_task_one(input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let sol = input
        .iter()
        .filter_map(|line| match line.parse::<ScratchCard>() {
            Ok(c) => match c.get_points() {
                Ok(n) => Some(n),
                Err(_) => None,
            },
            Err(_) => {
                panic!("Failed on line {}", line);
            }
        })
        .sum();
    eprintln!("⏱️ Took: {:?}", Instant::now() - start_time);
    Ok(sol)
}

pub fn solve_task_two(input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    // HashMap<CardId, (matches)>
    let cards = input.iter().map(|s| {
        let c = s.parse::<ScratchCard>().unwrap();
        c.matches()
    });

    let mut card_multiples: Vec<i32> = vec![1; cards.len()];

    for (card_id, points) in cards.enumerate() {
        for other_card_id in (card_id + 1)..(card_id + points as usize + 1) {
            let card_multiplier = card_multiples[card_id];
            let other_card_multiplier = card_multiples[other_card_id];
            card_multiples[other_card_id] = card_multiplier + other_card_multiplier;
        }
    }
    eprintln!("⏱️ Took: {:?}", Instant::now() - start_time);
    Ok(card_multiples.iter().sum())
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
        assert_eq!(solve_task_one(file)?, 13);
        Ok(())
    }

    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_one(file)?, 24848);
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_2.txt"))?;
        assert_eq!(solve_task_two(file)?, 30);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 7258152);
        Ok(())
    }
}

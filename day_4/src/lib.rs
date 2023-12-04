use std::{collections::HashMap, str::FromStr, time::Instant};

use color_eyre::{eyre::anyhow, Result};

#[derive(Debug)]
struct ScratchCard {
    id: i32,
    winning_numbers: Vec<i32>,
    own_numbers: Vec<i32>,
}

impl FromStr for ScratchCard {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let id: i32 = parts
            .next()
            .ok_or(anyhow!("No content in string"))?
            .replace("Card ", "")
            .trim()
            .parse()?;

        let mut parts = parts.next().ok_or(anyhow!("No numbers part"))?.split("|");
        let winning_numbers: Vec<i32> = parts
            .next()
            .ok_or(anyhow!("No winning numbers found"))?
            .split_ascii_whitespace()
            .map(str::trim)
            .map(i32::from_str)
            .collect::<Result<Vec<i32>, _>>()?;
        let own_numbers = parts
            .next()
            .ok_or(anyhow!("No own numbers"))?
            .split_ascii_whitespace()
            .map(str::trim)
            .map(i32::from_str)
            .collect::<Result<Vec<_>, _>>()?;
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
        self.winning_numbers
            .iter()
            .filter(|n| self.own_numbers.contains(n))
            .count() as u32
    }
}

pub fn solve_task_one(input: Vec<String>) -> Result<i32> {
    let sol = input
        .iter()
        .filter_map(|line| match line.parse::<ScratchCard>() {
            Ok(c) => Some(c),
            Err(_) => {
                panic!("Failed on line {}", line);
            }
        })
        .filter_map(|sc| match sc.get_points() {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .sum();
    Ok(sol)
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let cards: HashMap<i32, ScratchCard> = input
        .iter()
        .filter_map(|line| match line.parse::<ScratchCard>() {
            Ok(c) => Some((c.id, c)),
            Err(_) => {
                panic!("Failed on line {}", line);
            }
        })
        .collect();

    let solutions: HashMap<i32, u32> = cards.iter().map(|(i, sc)| (*i, sc.matches())).collect();

    let mut own_cards: Vec<&ScratchCard> = cards.values().collect();
    let mut total_cards = 0;
    while let Some(card) = own_cards.pop() {
        total_cards += 1;
        for i in 1..((solutions[&card.id] as i32) + 1) {
            own_cards.push(&cards[&(card.id + i)]);
        }
    }
    eprintln!("⏱️ Took: {:?}", Instant::now() - start_time);
    Ok(total_cards)
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
        assert_eq!(
            solve_task_one(get_file(PathBuf::from("inputs/full.txt"))?)?,
            24848
        );
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
        assert_eq!(
            solve_task_two(get_file(PathBuf::from("inputs/full.txt"))?)?,
            7258152
        );
        Ok(())
    }
}

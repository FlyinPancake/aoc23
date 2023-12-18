use std::str::FromStr;

use color_eyre::{eyre::anyhow, Result};
#[derive(Debug)]
pub enum GameCube {
    Red,
    Green,
    Blue,
}
impl FromStr for GameCube {
    type Err = color_eyre::Report;

    fn from_str(value: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match value {
            "red" => Ok(GameCube::Red),
            "green" => Ok(GameCube::Green),
            "blue" => Ok(GameCube::Blue),
            _ => Err(anyhow!("Color not found")),
        }
    }
}
#[derive(Debug)]
pub struct RoundData {
    pub red_cubes: i32,
    pub green_cubes: i32,
    pub blue_cubes: i32,
}

impl RoundData {
    pub fn satisfies_constraints(&self, r: i32, g: i32, b: i32) -> bool {
        self.red_cubes <= r && self.green_cubes <= g && self.blue_cubes <= b
    }

    pub fn pow(&self) -> i32 {
        self.red_cubes * self.green_cubes * self.blue_cubes
    }
}

impl Default for RoundData {
    fn default() -> Self {
        RoundData {
            red_cubes: 0,
            green_cubes: 0,
            blue_cubes: 0,
        }
    }
}

impl FromStr for RoundData {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut initial_rd = RoundData::default();
        s.split(",").try_for_each(|s| {
            let parts = s.trim().split_whitespace().collect::<Vec<_>>();
            let (balls, color) = (
                parts
                    .get(0)
                    .ok_or(anyhow!("No number part"))?
                    .parse::<i32>()?,
                parts
                    .get(1)
                    .ok_or(anyhow!("No color part"))?
                    .parse::<GameCube>()?,
            );
            match color {
                GameCube::Red => initial_rd.red_cubes += balls,
                GameCube::Green => initial_rd.green_cubes += balls,
                GameCube::Blue => initial_rd.blue_cubes += balls,
            }
            Result::<_, Self::Err>::Ok(())
        })?;
        Ok(initial_rd)
    }
}
#[derive(Debug)]
pub struct GameData {
    pub id: i32,
    pub rounds: Vec<RoundData>,
}

impl FromStr for GameData {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let game_str_parts: Vec<&str> = s.split(":").collect();
        let id = game_str_parts[0].replace("Game ", "").parse::<i32>()?;
        let rounds = game_str_parts[1]
            .split(";")
            .map(RoundData::from_str)
            .collect::<Result<Vec<_>>>()?;
        Ok(GameData { id, rounds })
    }
}

impl GameData {
    pub fn satisfies_constraints(&self, r: i32, g: i32, b: i32) -> bool {
        self.rounds
            .iter()
            .all(|round| round.satisfies_constraints(r, g, b))
    }

    pub fn minimal_possible_cubes(&self) -> Option<RoundData> {
        Some(RoundData {
            red_cubes: self.rounds.iter().map(|r| r.red_cubes).max()?,
            green_cubes: self.rounds.iter().map(|r| r.green_cubes).max()?,
            blue_cubes: self.rounds.iter().map(|r| r.blue_cubes).max()?,
        })
    }
}

static MAX_RED_CUBES: i32 = 12;
static MAX_GREEN_CUBES: i32 = 13;
static MAX_BLUE_CUBES: i32 = 14;

pub fn solve_task_one(input: Vec<String>) -> Result<i32> {
    let sum: i32 = input
        .into_iter()
        .filter_map(|line| match line.parse::<GameData>() {
            Ok(gd) => Some(gd),
            Err(_) => None,
        })
        .filter_map(|gd| {
            if gd.satisfies_constraints(MAX_RED_CUBES, MAX_GREEN_CUBES, MAX_BLUE_CUBES) {
                Some(gd.id)
            } else {
                None
            }
        })
        .sum();
    Ok(sum)
}

pub fn solve_task_two(input: Vec<String>) -> Result<i32> {
    let sum: i32 = input
        .into_iter()
        .filter_map(|line| match line.parse::<GameData>() {
            Ok(gd) => Some(gd),
            Err(_) => None,
        })
        .filter_map(|gd| gd.minimal_possible_cubes())
        .map(|rd| rd.pow())
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod test {

    use color_eyre::Result;
    use std::path::PathBuf;

    use crate::{solve_task_one, solve_task_two};
    use common::get_file;

    #[test]
    fn test_case_one_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_one(file)?, 8);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_one(file)?, 2632);
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_two(file)?, 2286);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 69629);
        Ok(())
    }
}

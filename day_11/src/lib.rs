use std::time::Instant;

use color_eyre::{
    eyre::{anyhow, Error},
    Result,
};

#[derive(Debug)]
struct Map {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    empty_mul_scale: i64,
}

impl TryFrom<Vec<String>> for Map {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let rows = value.len();
        let columns = value[0].len();
        let mut galaxies = Vec::new();
        for (row, line) in value.into_iter().enumerate() {
            for (col, c) in line.chars().into_iter().enumerate() {
                match c {
                    '.' => (),
                    '#' => galaxies.push((row, col)),

                    _ => return Err(anyhow!("Invalid map character")),
                }
            }
        }
        let galaxy_rows = &galaxies.iter().map(|(g_r, _)| g_r).collect::<Vec<_>>();
        let empty_rows = (0..rows)
            .into_iter()
            .filter(|r| !galaxy_rows.contains(&r))
            .collect::<Vec<_>>();

        let galaxy_cols = &galaxies.iter().map(|(_, g_c)| g_c).collect::<Vec<_>>();
        let empty_cols = (0..columns)
            .into_iter()
            .filter(|c| !galaxy_cols.contains(&c))
            .collect::<Vec<_>>();

        Ok(Map {
            galaxies,
            empty_cols,
            empty_rows,
            empty_mul_scale: 1,
        })
    }
}

impl Map {
    fn get_galaxy_pairings(&self) -> Vec<((usize, usize), (usize, usize))> {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, g1)| {
                self.galaxies
                    .clone()
                    .into_iter()
                    .enumerate()
                    .skip(i + 1)
                    .map(move |(_, g2)| (*g1, g2))
            })
            .collect()
    }

    fn set_empty_mul_scale(&mut self, empty_mul_scale: i64) {
        self.empty_mul_scale = empty_mul_scale;
    }

    fn get_total_distances(&self) -> i64 {
        let pairings = self.get_galaxy_pairings();
        let distances: Vec<_> = pairings
            .iter()
            .map(|(g1, g2)| {
                let (r1, c1) = g1;
                let (r2, c2) = g2;
                let empty_rows_between = self
                    .empty_rows
                    .iter()
                    .filter(|&r| {
                        if r1 <= r2 {
                            (r1..r2).contains(&r)
                        } else {
                            (r2..r1).contains(&r)
                        }
                    })
                    .count() as i64;
                let empty_cols_between = self
                    .empty_cols
                    .iter()
                    .filter(|&c| {
                        if c1 <= c2 {
                            (c1..c2).contains(&c)
                        } else {
                            (c2..c1).contains(&c)
                        }
                    })
                    .count() as i64;

                let r = (*r1 as i64 - *r2 as i64).abs()
                    + empty_rows_between * (self.empty_mul_scale - 1);
                let c = (*c1 as i64 - *c2 as i64).abs()
                    + empty_cols_between * (self.empty_mul_scale - 1);
                r + c
            })
            .collect();
        distances.iter().sum()
    }
}
pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();
    let mut map = Map::try_from(input)?;
    map.set_empty_mul_scale(2);

    let distances = map.get_total_distances();
    eprintln!("{:?}", Instant::now() - start_time);
    Ok(distances)
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();

    let mut map = Map::try_from(input)?;
    map.set_empty_mul_scale(1_000_000);
    let distances = map.get_total_distances();
    eprintln!("{:?}", Instant::now() - start_time);
    Ok(distances)
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
        assert_eq!(solve_task_one(file)?, 374);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        assert_eq!(
            solve_task_one(get_file(cargo_manifest_dir.join("inputs/full.txt"))?)?,
            9445168
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_two(file)?, 82000210);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 742305960572);
        Ok(())
    }
}

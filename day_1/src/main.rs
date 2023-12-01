use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use color_eyre::eyre::Result;
use common::CommonCli;
use rayon::prelude::*;

fn main() -> Result<()> {
    let args = CommonCli::parse();
    if !args.file.is_file() {
        panic!("{} is not a file", args.file.display());
    }
    let file = File::open(args.file)?;
    let reader = BufReader::new(file);
    let lines: std::result::Result<Vec<String>, _> = reader.lines().collect();
    let lines = lines?;

    let lines = lines
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

    println!("Solution: {}", sol);

    Ok(())
}

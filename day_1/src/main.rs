use std::{
    fs::File,
    io::{BufRead, BufReader},
    slice::Iter,
};

use clap::Parser;
use color_eyre::eyre::Result;
use common::CommonCli;
use rayon::prelude::*;

#[derive(Debug)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    // fn from_text(text: &str) -> Option<Self> {
    //     match text {
    //         "one" => Some(Self::One),
    //         "two" => Some(Self::Two),
    //         "three" => Some(Self::Three),
    //         "four" => Some(Self::Four),
    //         "five" => Some(Self::Five),
    //         "six" => Some(Self::Six),
    //         "seven" => Some(Self::Seven),
    //         "eight" => Some(Self::Eight),
    //         "nine" => Some(Self::Nine),
    //         _ => None,
    //     }
    // }

    fn as_digit_str(&self) -> &'static str {
        match self {
            Self::One => "1",
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::One => "one",
            Self::Two => "two",
            Self::Three => "three",
            Self::Four => "four",
            Self::Five => "five",
            Self::Six => "six",
            Self::Seven => "seven",
            Self::Eight => "eight",
            Self::Nine => "nine",
        }
    }

    fn iter() -> Iter<'static, Self> {
        [
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
        ]
        .iter()
    }

    fn replace_in_string_at(&self, input: &mut String, at: usize) -> usize {
        input.replace_range(at..at + self.as_str().len(), self.as_digit_str());
        // println!("{} -> {}", self.as_str(), self.as_digit_str()

        self.as_str().len() - 1
    }
}

fn replace_digit_strings(input: String) -> String {
    if Digit::iter().all(|d| !input.contains(d.as_str())) {
        return input;
    }
    let mut matches: Vec<_> = Digit::iter()
        .map(|d| {
            input
                .match_indices(d.as_str())
                .map(|(at, _content)| (at, d))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    matches.sort_by(|(at_a, _d_a), (at_b, _d_b)| at_a.cmp(at_b));
    let mut input = input;
    let mut offset = 0;
    // println!("{:?}", matches);
    for (at, digigt) in matches {
        offset += digigt.replace_in_string_at(&mut input, at - offset);
        // println!("{}", input);
    }
    input
}

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
        .map(|line| replace_digit_strings(line));
    let solution: u32 = lines
        .into_par_iter()
        .map(|line| {
            let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            let num = nums.first().unwrap() * 10 + nums.last().unwrap();
            println!("{} ", num);
            num
        })
        .sum();

    println!("Solution: {}", solution);
    Ok(())
}

#[cfg(test)]
mod test {
    use color_eyre::Result;

    use crate::replace_digit_strings;
    #[test]
    fn test_remove_digits() -> Result<()> {
        assert_eq!(replace_digit_strings("sixsixsix".to_string()), "666");
        assert_eq!(replace_digit_strings("sixsixsixsix".to_string()), "6666");
        assert_eq!(replace_digit_strings("fasz23".to_string()), "fasz23");
        assert_eq!(
            replace_digit_strings("onetwothreefourfivesixseveneightnine".to_string()),
            "123456789"
        );

        Ok(())
    }
}

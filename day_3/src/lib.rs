use std::{slice::Iter, time::Instant};

use color_eyre::{
    eyre::{anyhow, Error},
    Result,
};
#[derive(Debug, Clone, Copy)]
enum Symbol {
    Star,
    Hash,
    Plus,
    Dollar,
    Slash,
    Ampersand,
    Percent,
    At,
    Equals,
    Dash,
}

impl From<Symbol> for char {
    fn from(value: Symbol) -> char {
        match value {
            Symbol::Star => '*',
            Symbol::Hash => '#',
            Symbol::Plus => '+',
            Symbol::Dollar => '$',
            Symbol::Slash => '/',
            Symbol::Ampersand => '&',
            Symbol::Percent => '%',
            Symbol::At => '@',
            Symbol::Equals => '=',
            Symbol::Dash => '-',
        }
    }
}

impl TryFrom<char> for Symbol {
    type Error = Error;
    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            '*' => Ok(Symbol::Star),
            '#' => Ok(Symbol::Hash),
            '+' => Ok(Symbol::Plus),
            '$' => Ok(Symbol::Dollar),
            '/' => Ok(Symbol::Slash),
            '&' => Ok(Symbol::Ampersand),
            '%' => Ok(Symbol::Percent),
            '@' => Ok(Symbol::At),
            '=' => Ok(Symbol::Equals),
            '-' => Ok(Symbol::Dash),
            _ => Err(anyhow!("Symbol not known {}", value)),
        }
    }
}

impl Symbol {
    pub fn iterator() -> Iter<'static, Self> {
        static SYMBOLS: [Symbol; 10] = [
            Symbol::Ampersand,
            Symbol::At,
            Symbol::Dollar,
            Symbol::Hash,
            Symbol::Percent,
            Symbol::Plus,
            Symbol::Slash,
            Symbol::Star,
            Symbol::Equals,
            Symbol::Dash,
        ];
        SYMBOLS.iter()
    }
}
#[derive(Debug)]
struct NumberLocation {
    pub row_idx: usize,
    /// id of the first char
    pub start: usize,
    /// id of the last char (not the one after that)
    pub end: usize,
}
#[derive(Debug, Copy, Clone)]
struct SymolLocation {
    pub row_idx: usize,
    pub col_idx: usize,
}
#[derive(Debug)]
enum SchematicPoi {
    Number(i32, NumberLocation),
    Symbol(Symbol, SymolLocation),
}

struct SchematicParser {
    pub pois: Vec<SchematicPoi>,
    current_num: Option<String>,
    current_row: usize,
    current_col: usize,
}

impl SchematicParser {
    pub fn new() -> Self {
        Self {
            pois: vec![],
            current_col: 0,
            current_row: 0,
            current_num: None,
        }
    }

    fn store_number(&mut self, number: String) -> Result<()> {
        self.current_num = None;
        self.pois.push(SchematicPoi::Number(
            number.parse()?,
            NumberLocation {
                row_idx: self.current_row,
                start: self.current_col - number.len(),
                end: self.current_col - 1,
            },
        ));
        Ok(())
    }

    fn store_symbol(&mut self, symbol: Symbol) {
        self.pois.push(SchematicPoi::Symbol(
            symbol,
            SymolLocation {
                row_idx: self.current_row,
                col_idx: self.current_col,
            },
        ))
    }

    pub fn parse(mut self, shematic: &Vec<Vec<char>>) -> Result<Vec<SchematicPoi>> {
        for (row, line) in shematic.iter().enumerate() {
            self.current_row = row;
            for (col, char) in line.iter().enumerate() {
                self.current_col = col;
                match char {
                    '0'..='9' => match self.current_num.clone() {
                        Some(num_string) => {
                            self.current_num = Some(format!("{num_string}{char}").to_string())
                        }
                        None => self.current_num = Some(char.to_string()),
                    },
                    c if Symbol::iterator().any(|s| char::from(*s) == *c) => {
                        match self.current_num.clone() {
                            Some(num) => self.store_number(num)?,
                            None => (),
                        }
                        self.store_symbol((*char).try_into()?)
                    }
                    '.' => match self.current_num.clone() {
                        Some(num) => self.store_number(num)?,
                        None => (),
                    },
                    _ => return Err(anyhow!("unknown character found: {char}")),
                }
            }
            match self.current_num.clone() {
                Some(num) => self.store_number(num)?,
                None => (),
            }
        }
        Ok(self.pois)
    }
}

pub fn solve_task_one(input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let schematic = input
        .into_iter()
        .map(|l| l.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let parser = SchematicParser::new();
    let pois = parser.parse(&schematic)?;
    let symbols = pois
        .iter()
        .filter_map(|p| {
            if let SchematicPoi::Symbol(s, loc) = p {
                Some((*s, *loc))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let sum: i32 = pois
        .into_iter()
        .filter_map(|p| {
            if let SchematicPoi::Number(num, loc) = p {
                Some((num, loc))
            } else {
                None
            }
        })
        .filter(|(_, num_loc)| {
            symbols
                .iter()
                .any(|(_sym, SymolLocation { col_idx, row_idx })| {
                    let in_row_range = row_idx.abs_diff(num_loc.row_idx) <= 1;
                    let in_col_range = (num_loc.start == 0 || num_loc.start - 1 <= *col_idx)
                        && num_loc.end + 1 >= *col_idx;
                    in_row_range && in_col_range
                })
        })
        .map(|(n, _)| n)
        .sum();

    let end_time = Instant::now();

    println!("Took {:#?}", end_time - start_time);
    Ok(sum)
}

pub fn solve_task_two(input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();

    let schematic = input
        .into_iter()
        .map(|l| l.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let parser = SchematicParser::new();
    let pois = parser.parse(&schematic)?;
    let gears = pois
        .iter()
        .filter_map(|p| {
            if let SchematicPoi::Symbol(s, loc) = p {
                if let Symbol::Star = *s {
                    Some((*s, *loc))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let numbers = pois
        .into_iter()
        .filter_map(|p| {
            if let SchematicPoi::Number(num, loc) = p {
                Some((num, loc))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let ratios: Vec<(i32, i32)> = gears
        .into_iter()
        .filter_map(|(_, gear_loc)| {
            let ratios = numbers
                .iter()
                .filter(|(_, num_loc)| {
                    let in_row_range = gear_loc.row_idx.abs_diff(num_loc.row_idx) <= 1;
                    let in_col_range = (num_loc.start == 0
                        || num_loc.start - 1 <= gear_loc.col_idx)
                        && num_loc.end + 1 >= gear_loc.col_idx;
                    in_row_range && in_col_range
                })
                .collect::<Vec<_>>();
            if ratios.len() == 2 {
                Some((ratios[0].0, ratios[1].0))
            } else {
                None
            }
        })
        .collect();

    let sol: i32 = ratios.iter().map(|(a, b)| a * b).sum();
    let end_time = Instant::now();

    println!("Took {:#?}", end_time - start_time);
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
        assert_eq!(
            solve_task_one(get_file(PathBuf::from("inputs/example_1.txt"))?)?,
            4361
        );
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        assert_eq!(
            solve_task_one(get_file(PathBuf::from("inputs/full.txt"))?)?,
            527364
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        assert_eq!(
            solve_task_two(get_file(PathBuf::from("inputs/example_2.txt"))?)?,
            467835
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

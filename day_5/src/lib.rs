use std::{ops::Range, str::FromStr, time::Instant};

use color_eyre::Result;
use rayon::prelude::*;
#[derive(Debug)]
struct Conversion {
    dest_range_start: i64,
    src_range_start: i64,
    range_len: i64,
}

impl Conversion {
    pub fn src_range(&self) -> Range<i64> {
        self.src_range_start..(self.src_range_start + self.range_len)
    }

    pub fn offset(&self) -> i64 {
        self.dest_range_start - self.src_range_start
    }
}

impl FromStr for Conversion {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut nums = s
            .split_ascii_whitespace()
            .map(|e| e.parse::<i64>().unwrap());
        let conv = Conversion {
            dest_range_start: nums.next().unwrap(),
            src_range_start: nums.next().unwrap(),
            range_len: nums.next().unwrap(),
        };
        Ok(conv)
    }
}
#[derive(Debug)]
struct ConversionTable {
    conversions: Vec<Conversion>,
}

impl ConversionTable {
    pub fn from_string_vec(strings: Vec<String>) -> Self {
        ConversionTable {
            conversions: strings
                .iter()
                .map(|s| Conversion::from_str(s).unwrap())
                .collect(),
        }
    }
    pub fn convert(&self, n: i64) -> i64 {
        for conv in &self.conversions {
            if conv.src_range().contains(&n) {
                return n - conv.src_range_start + conv.dest_range_start;
            }
        }
        n
    }

    pub fn convert_range(&self, range: Range<i64>) -> Vec<Range<i64>> {
        let mut ranges = vec![];
        let mut cur_ranges = vec![range];
        for conv in &self.conversions {
            let src_range = conv.src_range();

            let mut new_ranges = vec![];
            for range in cur_ranges.iter() {
                // [ssssssss]
                //    [rr]
                if range.start >= src_range.start && range.end <= src_range.end {
                    ranges.push(range.start + conv.offset()..range.end + conv.offset());
                    return ranges;
                //    [ss]
                // [rrrrrrrrrr]
                } else if range.start < src_range.start && range.end > src_range.end {
                    ranges.push(src_range.start + conv.offset()..src_range.end + conv.offset());
                    new_ranges.push(range.start..src_range.start);
                    new_ranges.push(src_range.end..range.end);
                // [ssssss]
                //    [rrrrrr]
                } else if range.start >= src_range.end
                    && range.start < src_range.end
                    && range.end > src_range.end
                {
                    ranges.push(range.start + conv.offset()..src_range.end + conv.offset());
                    new_ranges.push(src_range.end..range.end);
                //     [ssssss]
                // [rrrrrr]
                } else if range.end > src_range.start
                    && range.end <= src_range.end
                    && range.start < src_range.start
                {
                    ranges.push(src_range.start + conv.offset()..range.end + conv.offset());
                    new_ranges.push(range.start..src_range.start);
                } else {
                    new_ranges = cur_ranges.clone();
                }
            }
            cur_ranges = new_ranges;
        }
        ranges.append(&mut cur_ranges);
        let ranges = ranges;
        let result = merge_ranges(ranges);

        result
    }
}

fn merge_ranges(ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
    let mut ranges = ranges;
    ranges.sort_by_key(|r| r.start);
    let mut result = vec![];
    let mut current_range = ranges[0].clone();

    for range in ranges.iter().skip(1) {
        if range.start <= current_range.end {
            // Ranges overlap, merge them
            current_range = current_range.start..range.end.max(current_range.end);
        } else {
            // Ranges don't overlap, push the current merged range and update
            result.push(current_range.clone());
            current_range = range.clone();
        }
    }
    result.push(current_range);
    result
}

pub fn solve_task_one(input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();

    let mut result = vec![];
    let mut current_block = vec![];
    for line in input {
        if line.is_empty() {
            if !current_block.is_empty() {
                result.push(current_block.clone());
                current_block.clear();
            }
        } else {
            current_block.push(line);
        }
    }
    if !current_block.is_empty() {
        result.push(current_block);
    }
    let conversions = result;

    let seeds = conversions[0][0][6..]
        .split_ascii_whitespace()
        .map(|e| e.parse::<i64>().unwrap());

    let seed_to_soils = ConversionTable::from_string_vec(conversions[1][1..].into());
    let soils = seeds.map(|s| seed_to_soils.convert(s));
    let soil_to_fertilizer = ConversionTable::from_string_vec(conversions[2][1..].into());
    let fertilizers = soils.map(|s| soil_to_fertilizer.convert(s));
    let fertilizer_to_water = ConversionTable::from_string_vec(conversions[3][1..].into());
    let waters = fertilizers.map(|f| fertilizer_to_water.convert(f));
    let water_to_light = ConversionTable::from_string_vec(conversions[4][1..].into());
    let lights = waters.map(|w| water_to_light.convert(w));
    let light_to_temperature = ConversionTable::from_string_vec(conversions[5][1..].into());
    let temperatures = lights.map(|l| light_to_temperature.convert(l));
    let temperature_to_humidity = ConversionTable::from_string_vec(conversions[6][1..].into());
    let humidities = temperatures.map(|t| temperature_to_humidity.convert(t));
    let humidity_to_location = ConversionTable::from_string_vec(conversions[7][1..].into());
    let locations = humidities.map(|h| humidity_to_location.convert(h));
    let sol = locations.min().unwrap();
    eprintln!("⏱️  Took {:#?}", Instant::now() - start_time);
    Ok(sol)
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();

    let mut result = vec![];
    let mut current_block = vec![];
    for line in input {
        if line.is_empty() {
            if !current_block.is_empty() {
                result.push(current_block.clone());
                current_block.clear();
            }
        } else {
            current_block.push(line);
        }
    }
    if !current_block.is_empty() {
        result.push(current_block);
    }
    let conversions = result;

    let seeds: Vec<_> = conversions[0][0][6..]
        .split_ascii_whitespace()
        .map(|e| e.parse::<i64>().unwrap())
        .collect();

    let seeds = seeds.par_chunks_exact(2).map(|ch| ch[0]..ch[0] + ch[1]);
    let seeds_v = seeds.clone().collect::<Vec<_>>();
    eprintln!("{:#?}", seeds_v);
    let seeds = seeds.into_par_iter();

    let seed_to_soils = ConversionTable::from_string_vec(conversions[1][1..].into());
    let soils = merge_ranges(
        seeds
            .flat_map(|s| seed_to_soils.convert_range(s))
            .collect::<Vec<_>>(),
    );
    eprintln!("{:#?}", soils);
    panic!();

    let soil_to_fertilizer = ConversionTable::from_string_vec(conversions[2][1..].into());
    let fertilizers: Vec<_> = soils
        .into_par_iter()
        .flat_map(|s| soil_to_fertilizer.convert_range(s))
        .collect();
    let fertilizers = merge_ranges(fertilizers);
    eprintln!("{:#?}", fertilizers);
    let fertilizer_to_water = ConversionTable::from_string_vec(conversions[3][1..].into());
    let waters = fertilizers
        .into_par_iter()
        .flat_map(|f| fertilizer_to_water.convert_range(f));
    let waters = merge_ranges(waters.collect());
    let water_to_light = ConversionTable::from_string_vec(conversions[4][1..].into());
    let lights = waters
        .into_par_iter()
        .flat_map(|w| water_to_light.convert_range(w));
    let lights = merge_ranges(lights.collect()).into_par_iter();
    let light_to_temperature = ConversionTable::from_string_vec(conversions[5][1..].into());
    let temperatures = lights.flat_map(|l| light_to_temperature.convert_range(l));
    let temperatures = merge_ranges(temperatures.collect()).into_par_iter();
    let temperature_to_humidity = ConversionTable::from_string_vec(conversions[6][1..].into());
    let humidities = temperatures.flat_map(|t| temperature_to_humidity.convert_range(t));
    let humidities = merge_ranges(humidities.collect()).into_par_iter();
    let humidity_to_location = ConversionTable::from_string_vec(conversions[7][1..].into());
    let locations = humidities.flat_map(|h| humidity_to_location.convert_range(h));
    let locations = merge_ranges(locations.collect()).into_par_iter();
    let locations = locations.map(|r| r.start);
    // eprintln!("{:#?}", locations);
    let sol = locations.min().unwrap();
    eprintln!("⏱️  Took {:#?}", Instant::now() - start_time);
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

    use crate::{merge_ranges, solve_task_one, solve_task_two, ConversionTable};

    #[test]
    fn test_case_one_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_one(file)?, 35);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        assert_eq!(
            solve_task_one(get_file(cargo_manifest_dir.join("inputs/full.txt"))?)?,
            214922730
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        assert_eq!(
            solve_task_two(get_file(PathBuf::from("inputs/example_1.txt"))?)?,
            46
        );
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        assert_eq!(
            solve_task_two(get_file(PathBuf::from("inputs/full.txt"))?)?,
            148041808
        );
        Ok(())
    }

    #[test]
    fn test_range_merge() -> Result<()> {
        let merged = merge_ranges(vec![50..52, 52..100, 1..50]);
        assert_eq!(merged, vec![1..100]);
        Ok(())
    }

    #[test]
    fn test_range_convert() -> Result<()> {
        let ct =
            ConversionTable::from_string_vec(vec!["50 98 2".to_string(), "52 50 48".to_string()]);
        // let r1 = 98..100;
        // assert_eq!(ct.convert_range(r1), vec![50..52]);

        // let r2 = 90..99;
        // assert_eq!(ct.convert_range(r2), vec![50..51, 92..100]);

        // let r3 = 1..100;
        // assert_eq!(ct.convert_range(r3), vec![1..100]);

        let r4 = 79..93;
        assert_eq!(ct.convert_range(r4), vec![81..95]);

        Ok(())
    }

    #[test]
    fn test_merge_and_convert() {
        let ct = ConversionTable::from_string_vec(vec![
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
        ]);

        let ranges = vec![57..70, 81..95];

        assert_eq!(
            merge_ranges(
                ranges
                    .clone()
                    .into_iter()
                    .flat_map(|r| ct.convert_range(r))
                    .collect()
            ),
            ranges
        );
    }
}

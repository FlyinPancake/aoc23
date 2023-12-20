use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
    time::Instant,
};

use color_eyre::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
}

impl FromStr for ModuleType {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match &s[..1] {
            "b" => Ok(Self::Broadcast),
            "%" => Ok(Self::FlipFlop(false)),
            "&" => Ok(Self::Conjunction(HashMap::new())),
            _ => Err(color_eyre::eyre::anyhow!("Invalid module type")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    name: String,
    module_type: ModuleType,
    destinations: VecDeque<String>,
}

impl Module {
    fn process_pulse(&mut self, pulse: Pulse, from: &String) -> VecDeque<(String, String, Pulse)> {
        match self.module_type {
            ModuleType::Broadcast => self
                .destinations
                .iter()
                .map(|d| (self.name.clone(), d.clone(), Pulse::Low))
                .collect(),
            ModuleType::FlipFlop(mut state) => {
                if pulse == Pulse::Low {
                    state = !state;
                    self.destinations
                        .iter()
                        .map(|d| {
                            (
                                self.name.clone(),
                                d.clone(),
                                if state { Pulse::High } else { Pulse::Low },
                            )
                        })
                        .collect()
                } else {
                    VecDeque::new()
                }
            }
            ModuleType::Conjunction(ref mut states) => {
                let _last_state = states.get(from).expect(&format!("Invalid state {}", from));
                states.insert(from.to_string(), pulse);
                if states.values().all(|s| *s == Pulse::High) {
                    self.destinations
                        .iter()
                        .map(|d| (self.name.clone(), d.clone(), Pulse::High))
                        .collect()
                } else {
                    self.destinations
                        .iter()
                        .map(|d| (self.name.clone(), d.clone(), Pulse::Low))
                        .collect()
                }
            }
        }
    }
}

impl FromStr for Module {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut split = s.split(" -> ");
        let module_type_src = split.next().unwrap();
        let module_type = ModuleType::from_str(module_type_src)?;
        let name = match module_type {
            ModuleType::Broadcast => "broadcast".to_string(),
            _ => module_type_src[1..].to_string(),
        };
        let destinations = split
            .next()
            .unwrap()
            .split(", ")
            .map(String::from)
            .collect();
        Ok(Self {
            name,
            module_type,
            destinations,
        })
    }
}

pub fn solve_task_one(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let modules = input
        .iter()
        .map(|s| Module::from_str(s))
        .map(|m| m.map(|m| (m.name.clone(), m)))
        .collect::<Result<HashMap<_, _>>>()?;
    let connections: HashMap<String, VecDeque<String>> = modules
        .values()
        .filter(|m| matches!(m.module_type, ModuleType::Conjunction(_)))
        // .map(|m| (m, VecDeque::new()))
        .map(|module| {
            let froms = modules
                .iter()
                .filter(|(_, m)| m.destinations.contains(&module.name))
                .map(|(name, _)| name.clone())
                .collect();
            (module.name.clone(), froms)
        })
        .collect();

    let mut modules: HashMap<String, Module> = modules
        .into_iter()
        .map(|(name, mut module)| {
            if let ModuleType::Conjunction(ref mut states) = module.module_type {
                // states.insert(name.clone(), Pulse::Low);
                connections.get(&name).unwrap().iter().for_each(|d| {
                    states.insert(d.clone(), Pulse::Low);
                });
                states.insert("broadcast".to_string(), Pulse::Low);
            }
            (name, module)
        })
        .collect();
    eprintln!("{:?}", modules);

    let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
    let mut first_pulse = VecDeque::new();
    first_pulse.push_back(("button".to_string(), "broadcast".to_string(), Pulse::Low));
    queue.append(&mut first_pulse);
    let mut lows = 0;
    let mut highs = 0;

    while let Some((from, to, pulse)) = queue.pop_front() {
        eprintln!("{} -{:?}-> {:?}", from, pulse, to);
        match pulse {
            Pulse::High => highs += 1,
            Pulse::Low => lows += 1,
        }
        let module = modules.get_mut(&to).unwrap();
        let mut new_pulses = module.process_pulse(pulse, &from);
        queue.append(&mut new_pulses);
    }

    eprintln!("Lows: {}", lows);
    eprintln!("Highs: {}", highs);

    eprintln!("{:?}", Instant::now() - start_time);
    todo!()
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    eprintln!("{:?}", Instant::now() - start_time);
    todo!()
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
        assert_eq!(solve_task_one(file)?, 0);
        Ok(())
    }

    #[ignore = "Not implemented yet"]
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_one(file)?, 0);
        Ok(())
    }
    #[ignore = "Not implemented yet"]
    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_two(file)?, 0);
        Ok(())
    }
    #[ignore = "Not implemented yet"]
    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 0);
        Ok(())
    }
}

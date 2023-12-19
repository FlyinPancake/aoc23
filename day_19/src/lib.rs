use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
    str::FromStr,
    time::Instant,
};

use color_eyre::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Attribute {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Attribute {
    fn char_repr(&self) -> char {
        match self {
            Attribute::ExtremelyCoolLooking => 'x',
            Attribute::Musical => 'm',
            Attribute::Aerodynamic => 'a',
            Attribute::Shiny => 's',
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'x' => Self::ExtremelyCoolLooking,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => panic!("Invalid attribute"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn total_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let s = s.replace(&['{', '}'], "");
        let mut split = s.split(',');
        let x = split.next().unwrap().trim()[2..].parse()?;
        let m = split.next().unwrap().trim()[2..].parse()?;
        let a = split.next().unwrap().trim()[2..].parse()?;
        let s = split.next().unwrap().trim()[2..].parse()?;
        Ok(Self { x, m, a, s })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    GreaterThan(Attribute, usize),
    LessThan(Attribute, usize),
}

impl Condition {
    fn apply_condition(&self, part: &Part) -> bool {
        match self {
            Self::GreaterThan(attribute, value) => match attribute {
                Attribute::ExtremelyCoolLooking => part.x > *value,
                Attribute::Musical => part.m > *value,
                Attribute::Aerodynamic => part.a > *value,
                Attribute::Shiny => part.s > *value,
            },
            Self::LessThan(attribute, value) => match attribute {
                Attribute::ExtremelyCoolLooking => part.x < *value,
                Attribute::Musical => part.m < *value,
                Attribute::Aerodynamic => part.a < *value,
                Attribute::Shiny => part.s < *value,
            },
        }
    }
}

impl FromStr for Condition {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        if s.contains('<') {
            let mut split = s.split('<');
            let attribute =
                Attribute::from_char(split.next().unwrap().trim().chars().last().unwrap());
            let value = split.next().unwrap().trim().parse()?;
            Ok(Self::LessThan(attribute, value))
        } else if s.contains('>') {
            let mut split = s.split('>');
            let attribute =
                Attribute::from_char(split.next().unwrap().trim().chars().last().unwrap());
            let value = split.next().unwrap().trim().parse()?;
            Ok(Self::GreaterThan(attribute, value))
        } else {
            panic!("Invalid condition")
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NextStep {
    Workflow(String),
    Accepted,
    Rejected,
}

impl NextStep {
    fn apply_next_step(&self, part: Part, workflows: &HashMap<String, Workflow>) -> Self {
        match self {
            Self::Workflow(s) => {
                let workflow = s.clone();
                let workflow = workflows.get(&workflow).unwrap();
                for rule in &workflow.rules {
                    if let Some(next_step) = rule.apply_rule(&part) {
                        return next_step;
                    }
                }
                workflow.default_next_step.clone()
            }
            Self::Accepted => Self::Accepted,
            Self::Rejected => Self::Rejected,
        }
    }
}

impl FromStr for NextStep {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accepted),
            "R" => Ok(Self::Rejected),
            _ => Ok(Self::Workflow(s.to_string())),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    condition: Condition,
    next_step: NextStep,
}

impl Rule {
    fn apply_rule(&self, part: &Part) -> Option<NextStep> {
        if self.condition.apply_condition(part) {
            Some(self.next_step.clone())
        } else {
            None
        }
    }
    /// Returns the next step with its new constraint and the remaining part constraint
    fn apply_rule_part_constraint(
        &self,
        part_constraint: &PartConstraint,
    ) -> (Option<(PartConstraint, NextStep)>, PartConstraint) {
        match &self.condition {
            Condition::GreaterThan(attr, val) => {
                let val = *val as i64;
                let mut new_constraint = part_constraint.clone();
                let mut old_constraint = part_constraint.clone();
                match attr {
                    Attribute::ExtremelyCoolLooking => {
                        if new_constraint.x.contains(&val) {
                            new_constraint.x.start = val + 1;
                            old_constraint.x.end = val + 1;
                        } else {
                            return (None, old_constraint);
                        }
                    }
                    Attribute::Musical => {
                        if new_constraint.m.contains(&val) {
                            new_constraint.m.start = val + 1;
                            old_constraint.m.end = val + 1;
                        } else {
                            return (None, old_constraint);
                        }
                    }
                    Attribute::Aerodynamic => {
                        if new_constraint.a.contains(&val) {
                            new_constraint.a.start = val + 1;
                            old_constraint.a.end = val + 1;
                        } else {
                            return (None, old_constraint);
                        }
                    }
                    Attribute::Shiny => {
                        if new_constraint.s.contains(&val) {
                            new_constraint.s.start = val + 1;
                            old_constraint.s.end = val + 1;
                        } else {
                            return (None, old_constraint);
                        }
                    }
                }
                (
                    Some((new_constraint, self.next_step.clone())),
                    old_constraint,
                )
            }
            Condition::LessThan(attr, val) => {
                let val = *val as i64;
                let mut new_constraint = part_constraint.clone();
                let mut old_constraint = part_constraint.clone();
                match attr {
                    Attribute::ExtremelyCoolLooking => {
                        if new_constraint.x.contains(&val) {
                            new_constraint.x.end = val;
                            old_constraint.x.start = val;
                        } else {
                            return (None, old_constraint);
                        }
                    }
                    Attribute::Musical => {
                        if new_constraint.m.contains(&val) {
                            new_constraint.m.end = val;
                            old_constraint.m.start = val;
                        } else {
                            return (None, old_constraint);
                        }
                    }
                    Attribute::Aerodynamic => {
                        if new_constraint.a.contains(&val) {
                            new_constraint.a.end = val;
                            old_constraint.a.start = val;
                        } else {
                            return (None, old_constraint);
                        }
                    }
                    Attribute::Shiny => {
                        if new_constraint.s.contains(&val) {
                            new_constraint.s.end = val;
                            old_constraint.s.start = val;
                        } else {
                            return (None, old_constraint);
                        }
                    }
                }
                (
                    Some((new_constraint, self.next_step.clone())),
                    old_constraint,
                )
            }
        }
    }
}

impl FromStr for Rule {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut split = s.split(':');
        let condition = split.next().unwrap().trim().parse()?;
        let next_step = split.next().unwrap().trim().parse()?;
        Ok(Rule {
            condition,
            next_step,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_next_step: NextStep,
}

impl FromStr for Workflow {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut splits = s.split(&['{', '}']);
        let name = splits.next().unwrap().trim().to_string();
        let mut rules: Vec<String> = splits
            .next()
            .unwrap()
            .split(',')
            .map(String::from)
            .collect();

        let default_next_step: NextStep = rules.pop().unwrap().trim().to_string().parse()?;

        let rules = rules
            .into_iter()
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<Rule>>>()?;
        Ok(Workflow {
            name,
            rules,
            default_next_step,
        })
    }
}

pub fn solve_task_one(input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();
    let mut parts = input.split(|s| s.is_empty());
    let workflows: HashMap<String, Workflow> = parts
        .next()
        .unwrap()
        .into_iter()
        .map(|s| {
            let workflow: Workflow = s.parse().unwrap();
            (workflow.name.clone(), workflow)
        })
        .collect();

    let parts: Vec<Part> = parts
        .next()
        .unwrap()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut accepted_parts = vec![];
    for part in parts {
        let mut current_workflow = NextStep::Workflow("in".to_string());
        while let NextStep::Workflow(_) = &current_workflow {
            current_workflow = current_workflow.apply_next_step(part, &workflows);
        }
        if current_workflow == NextStep::Accepted {
            accepted_parts.push(part);
        }
    }
    let sol = accepted_parts
        .iter()
        .map(|p| p.total_rating())
        .sum::<usize>();

    eprintln!("{:?}", Instant::now() - start_time);
    Ok(sol as i64)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PartConstraint {
    x: Range<i64>,
    m: Range<i64>,
    a: Range<i64>,
    s: Range<i64>,
}
impl PartConstraint {
    fn total_acceptable_parts(&self) -> usize {
        self.x.clone().count()
            * self.m.clone().count()
            * self.a.clone().count()
            * self.s.clone().count()
    }
}

impl Workflow {
    fn apply_part_constraint(&self, constraint: PartConstraint) -> Vec<(PartConstraint, NextStep)> {
        let mut results = vec![];
        let mut current_constraint = constraint;
        for rule in &self.rules {
            let (result, remaining_constraint) =
                rule.apply_rule_part_constraint(&current_constraint);
            if let Some((new_constraint, next_step)) = result {
                results.push((new_constraint, next_step));
            }
            current_constraint = remaining_constraint;
        }
        results.push((current_constraint, self.default_next_step.clone()));
        results
    }
}

impl Default for PartConstraint {
    fn default() -> Self {
        Self {
            x: MIN_ATTRIBUTE_VALUE..MAX_ATTRIBUTE_VALUE,
            m: MIN_ATTRIBUTE_VALUE..MAX_ATTRIBUTE_VALUE,
            a: MIN_ATTRIBUTE_VALUE..MAX_ATTRIBUTE_VALUE,
            s: MIN_ATTRIBUTE_VALUE..MAX_ATTRIBUTE_VALUE,
        }
    }
}

const MIN_ATTRIBUTE_VALUE: i64 = 1;
const MAX_ATTRIBUTE_VALUE: i64 = 4001;

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i64> {
    let start_time = Instant::now();
    let mut parts = input.split(|s| s.is_empty());
    let workflows: HashMap<String, Workflow> = parts
        .next()
        .unwrap()
        .into_iter()
        .map(|s| {
            let workflow: Workflow = s.parse().unwrap();
            (workflow.name.clone(), workflow)
        })
        .collect();

    let mut constraints = VecDeque::from(vec![(
        PartConstraint::default(),
        NextStep::Workflow("in".to_string()),
    )]);
    let mut accepted_constraints = vec![];
    while let Some((constraint, next_step)) = constraints.pop_front() {
        if let NextStep::Accepted = next_step {
            accepted_constraints.push(constraint);
        } else if let NextStep::Workflow(workflow) = next_step {
            let workflow = workflows.get(&workflow).unwrap();
            let mut new_constraints = workflow
                .apply_part_constraint(constraint)
                .try_into()
                .unwrap();

            // let mut new_constraints = VecDeque::from(new_constraints);
            constraints.append(&mut new_constraints);
        }
    }
    let sol = accepted_constraints
        .iter()
        .map(|c| c.total_acceptable_parts())
        .sum::<usize>();
    eprintln!("{:?}", Instant::now() - start_time);
    Ok(sol as i64)
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
        assert_eq!(solve_task_one(file)?, 19114);
        Ok(())
    }

    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_one(file)?, 575412);
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_two(file)?, 167409079868000);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 126107942006821);
        Ok(())
    }
}

use std::{cmp::Ordering, collections::HashMap, str::FromStr, time::Instant};

use color_eyre::Result;
use rayon::slice::ParallelSliceMut;

static ALL_CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

static ALL_CARDS_TWO: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
struct Hand {
    cards: [char; 5],
}

impl FromStr for Hand {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [0 as char; 5];
        for (i, c) in s.chars().enumerate() {
            cards[i] = c;
        }
        Ok(Hand { cards })
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let mut counts: HashMap<_, _> = ALL_CARDS.iter().map(|c| (*c, 0)).collect();
        for c in self.cards.iter() {
            counts.insert(*c, counts[c] + 1);
        }
        let mut pairs = 0;
        let mut three_of_a_kind = false;
        let mut four_of_a_kind = false;
        let mut five_of_a_kind = false;
        for (_c, instances) in counts.into_iter() {
            if instances == 2 {
                pairs += 1;
            } else if instances == 3 {
                three_of_a_kind = true;
            } else if instances == 4 {
                four_of_a_kind = true;
            } else if instances == 5 {
                five_of_a_kind = true;
            }
        }
        if five_of_a_kind {
            HandType::FiveOfAKind
        } else if four_of_a_kind {
            HandType::FourOfAKind
        } else if three_of_a_kind && pairs == 1 {
            HandType::FullHouse
        } else if three_of_a_kind {
            HandType::ThreeOfAKind
        } else if pairs == 2 {
            HandType::TwoPair
        } else if pairs == 1 {
            HandType::Pair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(ordering) = self.hand_type().partial_cmp(&other.hand_type()) {
            match ordering {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Greater => Some(Ordering::Greater),
                Ordering::Equal => {
                    for (s, o) in self.cards.into_iter().zip(other.cards) {
                        let (self_idx, _) = ALL_CARDS
                            .iter()
                            .enumerate()
                            .find(|(_idx, ch)| **ch == s)
                            .unwrap();
                        let (other_idx, _) = ALL_CARDS
                            .iter()
                            .enumerate()
                            .find(|(_, ch)| **ch == o)
                            .unwrap();
                        match self_idx.cmp(&other_idx) {
                            Ordering::Less => return Some(Ordering::Greater),
                            Ordering::Equal => (),
                            Ordering::Greater => return Some(Ordering::Less),
                        }
                    }
                    Some(Ordering::Equal)
                }
            }
        } else {
            None
        }
    }
}
#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
struct HandWithJoker {
    cards: [char; 5],
}

impl FromStr for HandWithJoker {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [0 as char; 5];
        for (i, c) in s.chars().enumerate() {
            cards[i] = c;
        }
        Ok(HandWithJoker { cards })
    }
}

impl HandWithJoker {
    pub fn hand_type(&self) -> HandType {
        let mut counts: HashMap<_, _> = ALL_CARDS_TWO.iter().map(|c| (*c, 0)).collect();
        for c in self.cards.iter() {
            counts.insert(*c, counts[c] + 1);
        }
        let mut pairs = 0;
        let mut three_of_a_kind = false;
        let mut four_of_a_kind = false;
        let mut five_of_a_kind = false;
        let mut jokers = 0;
        for (c, instances) in counts.into_iter() {
            if c == 'J' {
                jokers = instances
            } else if instances == 2 {
                pairs += 1;
            } else if instances == 3 {
                three_of_a_kind = true;
            } else if instances == 4 {
                four_of_a_kind = true;
            } else if instances == 5 {
                five_of_a_kind = true;
            }
        }

        if five_of_a_kind
            || four_of_a_kind && jokers == 1
            || three_of_a_kind && jokers == 2
            || pairs == 1 && jokers == 3
            || jokers >= 4
        {
            HandType::FiveOfAKind
        } else if four_of_a_kind
            || three_of_a_kind && jokers == 1
            || pairs == 1 && jokers == 2
            || pairs == 0 && jokers == 3
        {
            HandType::FourOfAKind
        } else if three_of_a_kind && pairs == 1
            || pairs == 2 && jokers == 1
            || pairs == 1 && jokers == 2
        {
            HandType::FullHouse
        } else if three_of_a_kind || pairs == 1 && jokers == 1 || pairs == 0 && jokers == 2 {
            HandType::ThreeOfAKind
        } else if pairs == 2 {
            HandType::TwoPair
        } else if pairs == 1 || pairs == 0 && jokers == 1 {
            HandType::Pair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ordering = self.hand_type().cmp(&other.hand_type());
        match ordering {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => {
                for (s, o) in self.cards.into_iter().zip(other.cards) {
                    let (self_idx, _) = ALL_CARDS_TWO
                        .iter()
                        .enumerate()
                        .find(|(_idx, ch)| **ch == s)
                        .unwrap();
                    let (other_idx, _) = ALL_CARDS_TWO
                        .iter()
                        .enumerate()
                        .find(|(_, ch)| **ch == o)
                        .unwrap();
                    match self_idx.cmp(&other_idx) {
                        Ordering::Less => return Some(Ordering::Greater),
                        Ordering::Equal => (),
                        Ordering::Greater => return Some(Ordering::Less),
                    }
                }
                Some(Ordering::Equal)
            }
        }
    }
}

pub fn solve_task_one(input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let mut hands: Vec<_> = input
        .iter()
        .map(|s| {
            let mut parts = s.split_ascii_whitespace();
            let hand: Hand = parts.next().unwrap().parse().unwrap();
            let bid: i32 = parts.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect();

    hands.par_sort_by_key(|(h, _)| *h);
    let sol: i32 = hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx as i32 + 1) * bid)
        .sum();
    eprintln!("⏱️ Took: {:?}", start_time.elapsed());
    Ok(sol)
}

pub fn solve_task_two(#[allow(unused_variables)] input: Vec<String>) -> Result<i32> {
    let start_time = Instant::now();
    let mut hands: Vec<_> = input
        .iter()
        .map(|s| {
            let mut parts = s.split_ascii_whitespace();
            let hand: HandWithJoker = parts.next().unwrap().parse().unwrap();
            let bid: i32 = parts.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect();

    hands.par_sort_by_key(|(h, _)| *h);
    let sol: i32 = hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx as i32 + 1) * bid)
        .sum();
    eprintln!("⏱️ Took: {:?}", start_time.elapsed());
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

    use crate::{solve_task_one, solve_task_two, Hand, HandType, HandWithJoker};

    #[test]
    fn test_case_one_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_1.txt"))?;
        assert_eq!(solve_task_one(file)?, 6440);
        Ok(())
    }
    #[test]
    fn test_case_one_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        assert_eq!(
            solve_task_one(get_file(cargo_manifest_dir.join("inputs/full.txt"))?)?,
            250347426
        );
        Ok(())
    }

    #[test]
    fn test_case_two_example() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/example_2.txt"))?;
        assert_eq!(solve_task_two(file)?, 5905);
        Ok(())
    }

    #[test]
    fn test_case_two_solve() -> Result<()> {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
        assert_eq!(solve_task_two(file)?, 251224870);
        Ok(())
    }

    #[test]
    fn test_enum_order() {
        use crate::HandType::*;
        assert!(FiveOfAKind > FourOfAKind);
        assert!(FourOfAKind > FullHouse);
        assert!(FullHouse > ThreeOfAKind);
        assert!(ThreeOfAKind > TwoPair);
        assert!(TwoPair > Pair);
        assert!(Pair > HighCard);
    }

    #[test]
    fn test_hand_sort() {
        let hands = [
            (
                Hand {
                    cards: ['K', 'K', '6', '7', '7'],
                },
                28,
            ),
            (
                Hand {
                    cards: ['K', 'T', 'J', 'J', 'T'],
                },
                220,
            ),
            (
                Hand {
                    cards: ['T', '5', '5', 'J', '5'],
                },
                684,
            ),
            (
                Hand {
                    cards: ['Q', 'Q', 'Q', 'J', 'A'],
                },
                483,
            ),
            (
                Hand {
                    cards: ['3', '2', 'T', '3', 'K'],
                },
                765,
            ),
        ];
        // assert!(hands[4] < hands[0]);
        // assert!(hands[4] < hands[1]);
        // assert!(hands[4] < hands[2]);
        // assert!(hands[4] < hands[3]);
        assert!(hands[0] > hands[1]);
    }

    #[test]
    fn test_joker() {
        let hand: HandWithJoker = "J9Q77".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);
        let hand: HandWithJoker = "7227J".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::FullHouse);
        let hand: HandWithJoker = "J44JJ".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::FiveOfAKind);
        let hand: HandWithJoker = "4J888".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::FourOfAKind);
        let hand: HandWithJoker = "KTJJT".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::FourOfAKind);

        let hand: HandWithJoker = "J9Q77".parse().unwrap();
        let hand_2: HandWithJoker = "9JQ77".parse().unwrap();
        assert!(hand < hand_2);
    }
}

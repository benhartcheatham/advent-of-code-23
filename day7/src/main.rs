use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone, Copy, Eq)]
struct Hand<'a> {
    hand: &'a str,
    strength: u32,
    bid: u32,
}

impl<'a> Hand<'a> {
    fn calculate_card_strength(card: char) -> u32 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => {
                if card.is_ascii_digit() {
                    card.to_digit(10).unwrap()
                } else {
                    0
                }
            }
        }
    }

    fn calculate_strength(hand: &str) -> u32 {
        let mut map = HashMap::new();

        for c in hand.chars() {
            map.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }

        // transform jokers into best card
        if map.len() > 1 && map.contains_key(&'J') {
            let j_num: u32 = map.remove(&'J').unwrap();

            // find max card
            let (mut max_key, mut max_val) = ('0', 0);
            for (k, v) in map.iter() {
                if *v > max_val {
                    max_key = *k;
                    max_val = *v;
                }
            }

            map.entry(max_key).and_modify(|n| *n += j_num);
        }

        match map.len() {
            1 => 7,
            2 => {
                let mut iter = map.iter();
                let (_, f) = iter.next().unwrap();
                let (_, s) = iter.next().unwrap();

                match (f, s) {
                    // four of a kind
                    (4, 1) | (1, 4) => 6,
                    // full house
                    (3, 2) | (2, 3) => 5,
                    _ => {
                        println!(
                            "Hand strength couldn't be calculated: {} map: {:?}",
                            hand, map
                        );
                        0
                    }
                }
            }
            3 => {
                let mut iter = map.iter();
                let (_, f) = iter.next().unwrap();
                let (_, s) = iter.next().unwrap();

                match (f, s) {
                    // three of a kind
                    (3, 1) | (1, 3) | (1, 1) => 4,
                    // two pair
                    (1, 2) | (2, 1) | (2, 2) => 3,
                    _ => {
                        println!(
                            "Hand strength couldn't be calculated: {} map: {:?}",
                            hand, map
                        );
                        0
                    }
                }
            }
            4 => 2,
            5 => 1,
            _ => 0,
        }
    }

    fn new(hand: &'a str, bid: u32) -> Hand {
        Hand {
            hand,
            strength: Self::calculate_strength(hand),
            bid,
        }
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.bid == other.bid
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.strength == other.strength {
            for (c0, c1) in self
                .hand
                .chars()
                .map(Self::calculate_card_strength)
                .zip(other.hand.chars().map(Self::calculate_card_strength))
            {
                let cmp = c0.cmp(&c1);

                if cmp != Ordering::Equal {
                    return cmp;
                }
            }

            return Ordering::Equal;
        }

        self.strength.cmp(&other.strength)
    }
}

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");

    println!("solution: {}", solution(input));
    Ok(())
}

fn solution(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut hands: Vec<Hand> = Vec::new();
    let mut sum = 0;

    for l in lines {
        let parts: Vec<&str> = l.split_whitespace().collect();

        if parts.len() != 2 {
            continue;
        }

        hands.push(Hand::new(parts[0], parts[1].parse::<u32>().unwrap()));
    }

    hands.sort();

    for (i, h) in hands.iter().enumerate() {
        sum += (i + 1) as u32 * h.bid;
    }

    sum
}

advent_of_code::solution!(7);
use std::{marker::PhantomData, num::ParseIntError, str::FromStr};

pub fn part_one(input: &str) -> Option<u32> {
    let hands = input
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<Hand<Part1Parser>>, _>>()
        .expect("Invalid input");

    Some(total_winnings(hands))
}

struct Part1Parser;

impl Parser for Part1Parser {
    fn char_to_value(c: char) -> u8 {
        let v = c as u8;
        match v {
            0x32..=0x39 => v - 0x32,
            0x54 => 8,   // T
            0x4A => 9,   // J
            0x51 => 0xA, // Q
            0x4B => 0xB, // K
            0x41 => 0xC, // A
            _ => unreachable!(),
        }
    }

    fn counter_to_value(counter: [u8; 13]) -> u8 {
        let mut pairs: u8 = 0;
        let mut tree: u8 = 0;
        for count in counter {
            match count {
                5 => return 6, // Five of kind
                4 => return 5, // Four of kind
                3 => tree += 1,
                2 => pairs += 1,
                _ => continue,
            }
        }
        match (pairs, tree) {
            (1, 1) => 4, // Fullhouse
            (_, 1) => 3, // Tree of kind
            (2, _) => 2, // Two pairs
            (1, _) => 1, // pair
            _ => 0,      // high card
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands = input
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<Hand<Part2Parser>>, _>>()
        .expect("Invalid input");

    Some(total_winnings(hands))
}

struct Part2Parser;

impl Parser for Part2Parser {
    fn char_to_value(c: char) -> u8 {
        let v = c as u8;
        match v {
            0x4A => 0, // J
            0x32..=0x39 => v - 0x31,
            0x54 => 9,   // T
            0x51 => 0xA, // Q
            0x4B => 0xB, // K
            0x41 => 0xC, // A
            _ => unreachable!(),
        }
    }

    fn counter_to_value(counter: [u8; 13]) -> u8 {
        let jokers = counter[0];
        let mut counter: Vec<u8> = counter[1..].to_vec();
        counter.sort_unstable();

        let counter = (counter[11], counter[10]);
        match counter {
            (x, _) if x + jokers == 5 => 6,
            (x, _) if x + jokers == 4 => 5,
            (x, 2) if x + jokers == 3 => 4,
            (x, _) if x + jokers == 3 => 3,
            (2, 2) => 2,
            (x, _) if x + jokers == 2 => 1,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct Hand<U> {
    value: u32,
    bid: u32,
    marker: PhantomData<U>,
}

pub fn total_winnings<U>(mut hands: Vec<Hand<U>>) -> u32 {
    hands.sort_unstable_by_key(Hand::value);
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid() * (rank as u32 + 1))
        .sum()
}

impl<U> Hand<U> {
    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn bid(&self) -> u32 {
        self.bid
    }
}

impl<U> FromStr for Hand<U>
where
    U: Parser,
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (counter, value) = s[..5].chars().map(U::char_to_value).fold(
            ([0u8; 0xD], 0),
            |(mut counter, mut value), v| {
                counter[v as usize] += 1;
                value <<= 4;
                value ^= v as u32;
                (counter, value)
            },
        );

        let value = value ^ ((U::counter_to_value(counter) as u32) << 20);
        let bid = s[6..].parse::<u32>()?;

        Ok(Self {
            value,
            bid,
            marker: PhantomData,
        })
    }
}

pub trait Parser {
    fn char_to_value(c: char) -> u8;
    fn counter_to_value(counter: [u8; 13]) -> u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}

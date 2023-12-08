advent_of_code::solution!(4);
use std::{
    collections::{HashSet, VecDeque},
    num::ParseIntError,
};

pub fn part_one(input: &str) -> Option<u16> {
    Some(input.lines().map(card_value).sum())
}

fn card_value(card: &str) -> u16 {
    let n = count_corresponding_numbers(card) as u32;
    if n == 0 {
        0
    } else {
        2u16.pow(n - 1)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(total_cards(input))
}

fn total_cards(cards: &str) -> u32 {
    cards
        .lines()
        .map(count_corresponding_numbers)
        .fold(
            (0, VecDeque::with_capacity(25)),
            |(total_card, mut next_copies), n| {
                let n_copies: u32 = next_copies.pop_front().unwrap_or_default();
                for i in 0..n {
                    match next_copies.get_mut(i as usize) {
                        Some(n) => *n += n_copies + 1,
                        None => next_copies.push_back(n_copies + 1),
                    }
                }
                (total_card + 1 + n_copies, next_copies)
            },
        )
        .0
}

fn parse_numbers(n: &str) -> impl Iterator<Item = Result<u16, ParseIntError>> + '_ {
    (0..=n.len() / 3).map(|i| {
        let i = i * 3;
        n[i..i + 2].trim().parse::<u16>()
    })
}

fn count_corresponding_numbers(card: &str) -> u16 {
    let i = card
        .find(':')
        .expect("Invalid input, cannot find the start of card");
    let card = &card[i..];
    let i = card
        .find('|')
        .expect("Invalid input, cannot find the separator of winning numbers & numbers we have");
    let winning_numbers = &card[2..i - 1];
    let numbers_you_have = &card[i + 2..];

    let winning_numbers: Result<HashSet<u16>, ParseIntError> =
        parse_numbers(winning_numbers).collect();
    let winning_numbers = winning_numbers.expect("Invalid input, cannot parse winning numbers");

    parse_numbers(numbers_you_have)
        .filter(|n| {
            let n = n
                .as_ref()
                .expect("Invalid input, cannot parse numbers we have");
            winning_numbers.get(n).is_some()
        })
        .count() as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}

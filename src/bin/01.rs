advent_of_code::solution!(1);
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u16> {
    Some(input.lines().map(calibration_value_part_one).sum())
}

fn calibration_value_part_one(line: &str) -> u16 {
    let digits = line.as_bytes();

    let first = digits
        .iter()
        .filter(|&d| d.is_ascii_digit())
        .map(Digit::from)
        .next()
        .expect("Invalid input");
    let last = digits
        .iter()
        .rev()
        .filter(|&d| d.is_ascii_digit())
        .map(Digit::from)
        .next()
        .expect("Invalid input");

    first.make_number(last)
}

pub fn part_two(input: &str) -> Option<u16> {
    Some(input.lines().map(calibration_value_part_two).sum())
}

fn calibration_value_part_two(line: &str) -> u16 {
    let window_size = 5;

    let mut digits = (0..line.len())
        .filter_map(|i| Digit::from_str(&line[i..(i + window_size).min(line.len())]).ok());

    let first = digits
        .next()
        .expect("Expect to have at least 1 digit in line");
    let last = digits.last().unwrap_or(first);

    first.make_number(last)
}

#[derive(Debug, Clone, Copy)]
pub struct Digit(u8);

impl Digit {
    pub fn make_number(self, d: Digit) -> u16 {
        (self.0 * 10 + d.0) as u16
    }
}

impl FromStr for Digit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_char = s.chars().next().unwrap();
        let spelled_digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        if first_char.is_ascii_digit() {
            Ok(Digit(first_char as u8 - 48))
        } else {
            for (i, &spelled_digit) in spelled_digits.iter().enumerate() {
                if s.starts_with(spelled_digit) {
                    return Ok(Digit(i as u8 + 1));
                }
            }
            Err(())
        }
    }
}

impl From<&u8> for Digit {
    fn from(value: &u8) -> Self {
        Self(value - 48)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}

use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum_of_part_numbers(input))
}

fn sum_of_part_numbers(s: &str) -> u32 {
    let mut numbers_map: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    let mut numbers: Vec<u16> = Vec::new();

    let mut symbol_indexs: Vec<(usize, usize)> = Vec::new();
    let mut part_numbers: HashSet<usize> = HashSet::new();
    let mut reading_number = false;

    for (row_index, l) in s.lines().enumerate() {
        for (col_index, c) in l.chars().enumerate() {
            if c.is_ascii_digit() {
                match reading_number {
                    true => {
                        let n = numbers.last_mut().unwrap();
                        *n *= 10;
                        *n += digit_to_u16(c);
                    }
                    false => {
                        reading_number = true;
                        numbers.push(digit_to_u16(c));
                    }
                };
                numbers_map
                    .entry(row_index)
                    .or_default()
                    .entry(col_index)
                    .or_insert(numbers.len() - 1);
            } else {
                reading_number = false;
                if c != '.' {
                    symbol_indexs.push((row_index, col_index));
                }
            }
        }
        reading_number = false;
    }

    let mut sum = 0u32;
    for (row_index, col_index) in symbol_indexs {
        let adjacents = adjacents(row_index, col_index);

        for (row_index, col_index) in adjacents {
            if let Some(row) = numbers_map.get(&row_index) {
                if let Some(n_index) = row.get(&col_index) {
                    if part_numbers.insert(*n_index) {
                        sum += numbers[*n_index] as u32;
                    }
                }
            }
        }
    }

    sum
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(sum_of_gear_ratios(input))
}

fn sum_of_gear_ratios(s: &str) -> u32 {
    let mut numbers_map: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    let mut numbers: Vec<u16> = Vec::new();

    let mut gears_index: Vec<(usize, usize)> = Vec::new();
    let mut reading_number = false;

    for (row_index, l) in s.lines().enumerate() {
        for (col_index, c) in l.chars().enumerate() {
            if c.is_ascii_digit() {
                match reading_number {
                    true => {
                        let n = numbers.last_mut().unwrap();
                        *n *= 10;
                        *n += digit_to_u16(c);
                    }
                    false => {
                        reading_number = true;
                        numbers.push(digit_to_u16(c));
                    }
                };
                numbers_map
                    .entry(row_index)
                    .or_default()
                    .entry(col_index)
                    .or_insert(numbers.len() - 1);
            } else {
                reading_number = false;
                if c == '*' {
                    gears_index.push((row_index, col_index));
                }
            }
        }
        reading_number = false;
    }

    let mut sum = 0u32;
    for (row_index, col_index) in gears_index {
        let adjacents = adjacents(row_index, col_index);

        let adjacent_numbers = adjacents.iter().fold(
            HashSet::new(),
            |mut acc: HashSet<usize>, (row_index, col_index)| {
                if let Some(row) = numbers_map.get(row_index) {
                    if let Some(n_index) = row.get(col_index) {
                        acc.insert(*n_index);
                    }
                }

                acc
            },
        );

        if adjacent_numbers.len() == 2 {
            let gear_ratio = adjacent_numbers
                .iter()
                .map(|&i| numbers[i] as u32)
                .product::<u32>();
            sum += gear_ratio;
        }
    }

    sum
}

fn digit_to_u16(d: char) -> u16 {
    (d as u8 - 48) as u16
}

fn adjacents(x: usize, y: usize) -> [(usize, usize); 8] {
    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}

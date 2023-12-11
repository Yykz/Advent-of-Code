use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let next_values = input.lines().map(|history| {
        let numbers = history
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap());
        next_value(numbers)
    });

    Some(next_values.sum())
}

fn next_value<T: Iterator<Item = i32>>(sequence: T) -> i32 {
    let (vec, last) =
        sequence
            .tuple_windows()
            .fold((Vec::new(), 0), |(mut sub_seq, _last), (current, next)| {
                sub_seq.push(next - current);
                (sub_seq, next)
            });
    if last == 0 {
        last
    } else {
        last + next_value(vec.iter().copied())
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let last_value = input.lines().map(|history| {
        let numbers = history
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap());
        last_value(numbers)
    });

    Some(last_value.sum())
}

fn last_value<T: Iterator<Item = i32> + DoubleEndedIterator>(sequence: T) -> i32 {
    let (vec, last) = sequence.rev().tuple_windows().fold(
        (Vec::new(), 0),
        |(mut sub_seq, _last), (current, next)| {
            sub_seq.push(next - current);
            (sub_seq, next)
        },
    );

    if vec.iter().all(|&x| x == 0) {
        last
    } else {
        last + last_value(vec.iter().rev().copied())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}

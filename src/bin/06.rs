advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let product_of_solutions = parse_part_one(input)
        .map(|(time, distance)| {
            let (x_1, x_2) = solve_eq(time, distance);
            x_2 - x_1 + 1
        })
        .product::<u64>();
    Some(product_of_solutions)
}

fn parse_part_one(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    let mut ite = input.lines();
    let times = ite.next().map(|l| 
        l.split_whitespace().filter_map(|n| n.parse::<u64>().ok())
    ).expect("Invalid input");
    let distances = ite.next().map(|l| 
        l.split_whitespace().filter_map(|n| n.parse::<u64>().ok())
    ).expect("Invalid input");

    times.zip(distances)
}


pub fn part_two(input: &str) -> Option<u64> {
    let (time, distance) = parse_part_two(input);
    let nb_of_solutions = {
        let (x_1, x_2) = solve_eq(time, distance);
        x_2 - x_1 + 1
    };
    Some(nb_of_solutions)
}

fn parse_part_two(input: &str) -> (u64, u64) {
    let mut ite = input.lines();
    let time = ite.next().map(|l| 
        l.split_whitespace().fold(String::new(), |mut n, s| {
            if s.chars().all(|c| c.is_ascii_digit()) {
                n.push_str(s)
            }
            n
        })
    ).expect("Invalid input");

    let distance = ite.next().map(|l| 
        l.split_whitespace().fold(String::new(), |mut n, s| {
            if s.chars().all(|c| c.is_ascii_digit()) {
                n.push_str(s)
            }
            n
        })
    ).expect("Invalid input");

    let time = time.parse::<u64>().expect("Invalid input");
    let distance = distance.parse::<u64>().expect("Invalid input");

    (time, distance)
}

pub fn solve_eq(b: u64, c: u64) -> (u64, u64) {
    #[allow(non_snake_case)]
    let Δ: f64 = ((b.pow(2) - 4 * c) as f64).sqrt();

    let x_1 = ((b as f64 - Δ)/2.0).trunc() as u64 + 1;
    let x_2 = ((b as f64 + Δ)/2.0).ceil() as u64 - 1;

    (x_1, x_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}

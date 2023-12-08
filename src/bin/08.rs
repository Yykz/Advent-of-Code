use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let mut ite = input.lines();
    let directions: Vec<usize> = ite
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        })
        .collect();

    let mut nodes: HashMap<&str, [&str; 2]> = HashMap::new();
    for line in ite.skip(1) {
        let name = &line[..3];
        let left = &line[7..10];
        let right = &line[12..15];

        nodes.insert(name, [left, right]);
    }

    let mut current_node = "AAA";
    let mut steps = 0;
    while current_node != "ZZZ" {
        let dir = directions[steps % directions.len()];
        current_node = nodes.get(current_node).unwrap()[dir];
        steps += 1;
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ite = input.lines();
    let directions: Vec<usize> = ite
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        })
        .collect();

    let mut nodes: HashMap<&str, [&str; 2]> = HashMap::new();
    let mut start_nodes: Vec<&str> = Vec::new();
    for line in ite.skip(1) {
        let name = &line[..3];
        let left = &line[7..10];
        let right = &line[12..15];

        if name.ends_with('A') {
            start_nodes.push(name);
        }
        nodes.insert(name, [left, right]);
    }

    let mut steps = Vec::new();
    for node in start_nodes {
        let mut current_node = node;
        let mut current_steps = 0;
        while !current_node.ends_with('Z') {
            let dir = directions[current_steps % directions.len()];
            current_node = nodes.get(current_node).unwrap()[dir];
            current_steps += 1;
        }
        steps.push(current_steps as u64);
    }

    let lcm = steps.iter().fold(1, |acc, &n| lcm(acc, n));
    Some(lcm)
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}

advent_of_code::solution!(8);

fn map_byte(c: u8) -> u16 {
    (c - b'A') as u16
}

fn to_value(s: &str) -> u16 {
    let s = s.as_bytes();
    map_byte(s[0])* 676 + map_byte(s[1]) * 26 + map_byte(s[2])
}

pub fn part_one(input: &str) -> Option<usize> {
    let (directions, nodes) = parse_part_one(input);

    let mut current_node = 0;
    let end = to_value("ZZZ");
    for (i, dir) in directions.chars().cycle().enumerate() {
        if current_node == end {
            return Some(i);
        }
        current_node = match dir {
            'L' => nodes[current_node as usize].0,
            'R' => nodes[current_node as usize].1,
            _ => unreachable!(),
        };
    }

    unreachable!()
}

fn parse_part_one(input: &str) -> (&str, [(u16, u16); 17_576]) {
    let mut ite = input.lines();
    let directions = ite.next().unwrap();

    let mut nodes = [(0u16, 0u16); 17_576];
    for line in ite.skip(1) {
        let name = to_value(&line[..3]);
        let left = to_value(&line[7..10]);
        let right = to_value(&line[12..15]);

        nodes[name as usize] = (left, right);
    }
    (directions, nodes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, nodes, start_nodes) = parse_part_two(input);

    let mut steps = Vec::new();
    for node in start_nodes {
        let mut current_node = node;
        for (i, dir) in directions.chars().cycle().enumerate() {
            if current_node % 26 == 25 {
                steps.push(i as u64);
                break;
            }
            current_node = match dir {
                'L' => nodes[current_node as usize].0,
                'R' => nodes[current_node as usize].1,
                _ => unreachable!(),
            };
        }
    }

    let lcm = steps.iter().fold(1, |acc, &n| lcm(acc, n));
    Some(lcm)
}

fn parse_part_two(input: &str) -> (&str, [(u16, u16); 17_576], Vec::<u16>) {
    let mut ite = input.lines();
    let directions = ite.next().unwrap();

    let mut nodes = [(0u16, 0u16); 17_576];
    let mut start_nodes = Vec::new();
    for line in ite.skip(1) {
        let name = to_value(&line[..3]);
        let left = to_value(&line[7..10]);
        let right = to_value(&line[12..15]);

        if name % 26 == 0 {
            start_nodes.push(name);
        }

        nodes[name as usize] = (left, right);
    }

    (directions, nodes, start_nodes)
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

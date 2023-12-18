advent_of_code::solution!(15);

// Part 1: 504449 (76.0µs @ 10000 samples)
// Part 2: 262044 (178.7µs @ 5307 samples)

#[derive(Debug, Default)]
struct Box<'a> {
    inner: Vec<(&'a str, u16)>,
}

impl<'a> Box<'a> {
    fn add(&mut self, s: &'a str, n: u16) {
        for i in 0..self.inner.len() {
            if s == self.inner[i].0 {
                self.inner[i].1 = n;
                return;
            }
        }
        self.inner.push((s, n))
    }

    fn remove(&mut self, ss: &str) {
        if let Some((i, _)) = self.inner.iter().enumerate().find(|(_i, &(s, _n))| s == ss) {
            self.inner.remove(i);
        }
    }

    fn value(&self) -> u16 {
        self.inner
            .iter()
            .enumerate()
            .map(|(i, &(_s, n))| (i + 1) as u16 * n)
            .sum()
    }
}

struct BoxArray<'a> {
    boxes: [Box<'a>; 256],
}

impl<'a> BoxArray<'a> {
    fn new() -> Self {
        Self {
            boxes: core::array::from_fn(|_| Box::default()),
        }
    }

    #[inline(always)]
    fn get_i_unchecked(&mut self, i: u8) -> &mut Box<'a> {
        // SAFETY: The `boxes` array has a fixed size of 256, and `i` is of type `u8`, ensuring that `i` is in the range 0 to 255 (inclusive).
        unsafe { self.boxes.get_unchecked_mut(i as usize) }
    }

    fn operate_label(&mut self, s: &'a str) {
        let delim_index = s.len()
            - s.chars()
                .rev()
                .enumerate()
                .find(|(_i, c)| ['-', '='].contains(c))
                .unwrap()
                .0
            - 1;
        let ss = &s[..delim_index];
        let i = hash(ss);
        match &s[delim_index..delim_index + 1] {
            "=" => {
                let n = s[delim_index + 1..].parse::<u16>().unwrap();
                self.get_i_unchecked(i).add(ss, n);
            }
            "-" => {
                self.get_i_unchecked(i).remove(ss);
            }
            _ => unreachable!(),
        }
    }

    fn sum_values(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, b)| ((i + 1) as u16 * b.value()) as u32)
            .sum()
    }
}

fn hash(s: &str) -> u8 {
    s.chars()
        .fold(0, |acc, c| ((acc as u16 + (c as u16)) * 17) as u8)
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input[..input.len() - 1]
        .split(',')
        .map(hash)
        .fold(0, |acc, n| acc + (n as u32));
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes = BoxArray::new();
    for label in input[..input.len() - 1].split(',') {
        boxes.operate_label(label);
    }

    Some(boxes.sum_values())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let r = hash("HASH");
        assert_eq!(r, 52);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u16> {
    Some(
        input
            .lines()
            .map(|l| Game::try_from(l).expect("Invalid input"))
            .filter(|game| game.is_possible(12, 13, 14))
            .map(|game| game.game_id as u16)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| Game::try_from(l).expect("Invalid input").minimum_power())
            .sum(),
    )
}

#[derive(Debug)]
struct Set(u8, u8, u8);

impl TryFrom<&str> for Set {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (red, green, blue) = value.split(", ").try_fold((0, 0, 0), |acc, s| {
            let space_index = s.find(' ').ok_or(())?;
            let (n, color) = s.split_at(space_index);
            let n = n.parse::<u8>().map_err(|_err| ())?;

            let colors = match &color[1..] {
                "red" => (n, acc.1, acc.2),
                "green" => (acc.0, n, acc.2),
                "blue" => (acc.0, acc.1, n),
                _ => unreachable!(),
            };
            Ok(colors)
        })?;

        Ok(Self(red, green, blue))
    }
}

impl Set {
    fn is_possible(&self, red: u8, green: u8, blue: u8) -> bool {
        self.0 <= red && self.1 <= green && self.2 <= blue
    }
}

#[derive(Debug)]
pub struct Game {
    pub game_id: u8,
    sets: Vec<Set>,
}

impl TryFrom<&str> for Game {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let colon_index = value.find(':').ok_or(())?;
        let game_id: u8 = value[5..colon_index].parse().map_err(|_err| ())?;
        let sets: Result<Vec<Set>, _> = value[colon_index + 2..]
            .split("; ")
            .map(Set::try_from)
            .collect();
        let sets = sets?;

        Ok(Self { game_id, sets })
    }
}

impl Game {
    pub fn is_possible(&self, red: u8, green: u8, blue: u8) -> bool {
        !self
            .sets
            .iter()
            .any(|set| !set.is_possible(red, green, blue))
    }

    pub fn minimum_power(&self) -> u32 {
        let (red, green, blue) = self.sets.iter().fold((0, 0, 0), |acc, set| {
            (acc.0.max(set.0), acc.1.max(set.1), acc.2.max(set.2))
        });

        (red as u32) * (green as u32) * (blue as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}

advent_of_code::solution!(10);

use Facing::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn opposite(self) -> Self {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pipe(Facing, Facing);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Pipe(Pipe),
    Start,
    Ground,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Pipe(Pipe(South, North)),
            '-' => Tile::Pipe(Pipe(West, East)),
            'L' => Tile::Pipe(Pipe(North, East)),
            'J' => Tile::Pipe(Pipe(West, North)),
            '7' => Tile::Pipe(Pipe(South, West)),
            'F' => Tile::Pipe(Pipe(South, East)),
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pipes: [[Option<Tile>; 140]; 140] = [[None; 140]; 140];
    let mut start_index: (usize, usize) = (0, 0);
    for (row_index, line) in input.lines().enumerate() {
        for (col_index, tile) in line.chars().enumerate() {
            let tile = Tile::from(tile);
            if let Tile::Start = tile {
                start_index = (row_index, col_index);
            }
            pipes[row_index][col_index] = Some(tile);
        }
    }

    let ((mut next_x, mut next_y), mut origin) =
        connected_tiles(start_index.0, start_index.1, &[North, South, East, West])
            .zip([North, South, East, West])
            .find(|&((i, j), origin)| {
                if let Some(Tile::Pipe(Pipe(a, b))) = pipes[i][j] {
                    a == origin.opposite() || b == origin.opposite()
                } else {
                    false
                }
            })
            .unwrap();

    let mut count = 1;
    while let Some(Tile::Pipe(Pipe(a, b))) = pipes[next_x][next_y] {
        let dir = if a != origin.opposite() { a } else { b };
        (next_x, next_y) = coords_from_dir(next_x, next_y, &dir);
        origin = dir;
        count += 1;
    }
    Some(count / 2)
}

fn coords_from_dir(row_index: usize, col_index: usize, dir: &Facing) -> (usize, usize) {
    match dir {
        North => (row_index - 1, col_index),
        South => (row_index + 1, col_index),
        East => (row_index, col_index + 1),
        West => (row_index, col_index - 1),
    }
}

fn connected_tiles(
    row_index: usize,
    col_index: usize,
    dir: &[Facing],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    dir.iter()
        .map(move |d| coords_from_dir(row_index, col_index, d))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

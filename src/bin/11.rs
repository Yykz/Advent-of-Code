advent_of_code::solution!(11);

fn adjusted_coords(
    x: usize,
    y: usize,
    expansions: (&[usize], &[usize]),
    expansion_speed: usize,
) -> (usize, usize) {
    (
        x + (expansions
            .0
            .iter()
            .take_while(|&&expansion_x| x > expansion_x)
            .count()
            * expansion_speed),
        y + (expansions
            .1
            .iter()
            .take_while(|&&expansion_y| y > expansion_y)
            .count()
            * expansion_speed),
    )
}

fn path_len(from: &(usize, usize), to: &(usize, usize)) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

fn parse_expansion(galaxies: &[(usize, usize)]) -> (Vec<usize>, Vec<usize>) {
    let expansions = galaxies
        .iter()
        .fold(([true; 140], [true; 140]), |mut acc, &(x, y)| {
            acc.0[x] = false;
            acc.1[y] = false;
            acc
        });

    let map_expansion = |expansion: [bool; 140]| {
        expansion
            .iter()
            .enumerate()
            .filter(|(_i, &e)| e)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    };

    (map_expansion(expansions.0), map_expansion(expansions.1))
}

fn adjust_galaxies_coords(
    galaxies: Vec<(usize, usize)>,
    expansions: (Vec<usize>, Vec<usize>),
    expansion_speed: usize,
) -> Vec<(usize, usize)> {
    galaxies
        .iter()
        .map(|&(x, y)| adjusted_coords(x, y, (&expansions.0, &expansions.1), expansion_speed))
        .collect()
}

fn paths_len(adjusted_galaxies: &[(usize, usize)]) -> impl Iterator<Item = usize> + '_ {
    adjusted_galaxies
        .iter()
        .enumerate()
        .flat_map(|(n, galaxy)| {
            adjusted_galaxies[n + 1..]
                .iter()
                .map(|to_galaxy| path_len(galaxy, to_galaxy))
        })
}

fn parse_galaxies(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| char == &'#')
                .map(move |(col_index, _)| (col_index, row_index))
        })
        .collect()
}

fn sum_of_galaxies_pairs_paths_len(input: &str, expansion_speed: usize) -> Option<usize> {
    let galaxies = parse_galaxies(input);
    let expansion = parse_expansion(&galaxies);
    let galaxies = adjust_galaxies_coords(galaxies, expansion, expansion_speed - 1);
    let paths_len = paths_len(&galaxies);
    let sum = paths_len.sum();

    Some(sum)
}

pub fn part_one(input: &str) -> Option<usize> {
    sum_of_galaxies_pairs_paths_len(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    sum_of_galaxies_pairs_paths_len(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        let result = sum_of_galaxies_pairs_paths_len(input, 100);
        assert_eq!(result, Some(8410));
        let result = sum_of_galaxies_pairs_paths_len(input, 10);
        assert_eq!(result, Some(1030));
    }
}

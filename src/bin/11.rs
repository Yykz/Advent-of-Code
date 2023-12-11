advent_of_code::solution!(11);

fn adjusted_coords(
    x: usize,
    y: usize,
    expand: (&[usize], &[usize]),
    expand_speed: usize,
) -> (usize, usize) {
    (
        x + (expand
            .0
            .iter()
            .take_while(|&&expand_x| x > expand_x)
            .count()
            * expand_speed),
        y + (expand
            .1
            .iter()
            .take_while(|&&expand_y| y > expand_y)
            .count()
            * expand_speed),
    )
}

fn path_len(from: &(usize, usize), to: &(usize, usize)) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

fn parse_expands(galaxies: &[(usize, usize)]) -> (Vec<usize>, Vec<usize>) {
    let expands = galaxies
        .iter()
        .fold(([true; 140], [true; 140]), |mut acc, &(x, y)| {
            acc.0[x] = false;
            acc.1[y] = false;
            acc
        });

    let map_expand = |expand: [bool; 140]| {
        expand
            .iter()
            .enumerate()
            .filter(|(_i, &e)| e)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    };

    (map_expand(expands.0), map_expand(expands.1))
}

fn adjust_galaxies_coords(
    galaxies: Vec<(usize, usize)>,
    expand: (Vec<usize>, Vec<usize>),
    expand_speed: usize,
) -> Vec<(usize, usize)> {
    galaxies
        .iter()
        .map(|&(x, y)| adjusted_coords(x, y, (&expand.0, &expand.1), expand_speed))
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

fn run(input: &str, expand_speed: usize) -> Option<usize> {
    let galaxies = parse_galaxies(input);
    let expand = parse_expands(&galaxies);
    let galaxies = adjust_galaxies_coords(galaxies, expand, expand_speed);
    let paths_len = paths_len(&galaxies);
    let sum = paths_len.sum();

    Some(sum)
}

pub fn part_one(input: &str) -> Option<usize> {
    run(input, 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    run(input, 999_999)
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

        let result = run(input, 99);
        assert_eq!(result, Some(8410));
        let result = run(input, 9);
        assert_eq!(result, Some(1030));
    }
}

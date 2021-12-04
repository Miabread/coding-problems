#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '0' => false,
                    '1' => true,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<bool>]) -> usize {
    // Make table dimensions explicit
    let height = input.len();
    let width = input[0].len();

    // For every column
    let gamma: usize = (0..width)
        .map(|column| {
            // Get the amount of trues in that column
            let true_count = input.iter().filter(|row| row[column]).count();

            // Whatever wasn't true was false
            let false_count = height - true_count;

            // The puzzle doesn't mention the possibility of the two counts being equal,
            // so assume such input is never given to us.
            assert!(true_count != false_count);

            // Compare the counts to find the more common one
            true_count > false_count
        })
        // We want to start with the least significant bit
        .rev()
        // Keep track of place value
        .enumerate()
        // If true then get the "place value" at that position
        .filter_map(|(place, is_set)| is_set.then(|| 1 << place))
        // Sum to get the combined number
        .sum();

    // Epsilon will have the opposite booleans of gamma,
    // because it's least rather than greatest.
    // After working it through the whole binary conversion thing,
    // you can prove epsilon is just a bitwise invert of gamma.
    let epsilon = !gamma;

    // Not quite though, as we only want to consider `width` amount of bits
    // We google "how to extract n bits of a number" and it tells us to use a mask
    let mask = (1 << width) - 1;
    let epsilon = epsilon & mask;

    // Finally, compute the final answer
    gamma * epsilon
}

// INCOMPLETE
#[aoc(day3, part2)]
pub fn part2(input: &[Vec<bool>]) -> usize {
    // Make table dimensions explicit
    let height = input.len();
    let width = input[0].len();

    let mut row_flags = vec![true; height];
    let mut column = 0;

    let oxygen = loop {
        let true_count = row_flags
            .iter()
            .zip(input.iter())
            .filter(|(flag, row)| **flag && row[column])
            .count();

        let false_count = height - true_count;

        let greatest_value = true_count >= false_count;

        let rows_left = row_flags
            .iter_mut()
            .zip(input.iter())
            .filter(|(flag, _)| **flag)
            .filter_map(|(flag, row)| {
                *flag = row[column] == greatest_value;
                flag.then(|| ())
            })
            .count();

        if rows_left == 1 {
            let index = row_flags.iter().position(|it| *it).unwrap();
            break input[index]
                .iter()
                .rev()
                .enumerate()
                .filter_map(|(place, is_set)| is_set.then(|| 1 << place))
                .sum();
        }

        column += 1;
    };
    dbg!(oxygen)
}

#[cfg(test)]
pub mod test {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 198);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 23);
    }
}

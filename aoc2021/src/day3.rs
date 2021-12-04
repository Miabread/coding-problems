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
            let true_count = input
                .iter()
                .map(|row| row[column])
                .filter(|bool| *bool)
                .count();

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

#[cfg(test)]
pub mod test {
    #[test]
    pub fn part1() {
        let input = "00100
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
        assert_eq!(super::part1(&super::parse(input)), 198);
    }
}

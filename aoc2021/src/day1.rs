#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|it| it.parse().unwrap()).collect()
}

#[aoc(day1, part1, iter)]
pub fn part1_iter(input: &[usize]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(last, current)| current > last)
        .count()
}

#[aoc(day1, part1, for_loop)]
pub fn part1_for_loop(input: &[usize]) -> usize {
    let mut count = 0;

    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            count += 1;
        }
    }

    count
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(3))
        .filter(|(last, current)| current > last)
        .count()
}

#[aoc(day1, part2, for_loop)]
pub fn part2_for_loop(input: &[usize]) -> usize {
    let mut count = 0;

    for i in 3..input.len() {
        if input[i] > input[i - 3] {
            count += 1;
        }
    }

    count
}

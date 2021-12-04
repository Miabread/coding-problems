use Command::*;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(' ');

            let direction = line.next().unwrap();
            let distance = line.next().unwrap().parse().unwrap();

            match direction {
                "forward" => Forward(distance),
                "down" => Down(distance),
                "up" => Up(distance),
                _ => panic!(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> usize {
    let mut x = 0;
    let mut y = 0;

    for command in input {
        match command {
            Forward(distance) => x += distance,
            Down(distance) => y += distance,
            Up(distance) => y -= distance,
        }
    }

    x * y
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for command in input {
        match command {
            Forward(distance) => {
                x += distance;
                y += aim * distance;
            }
            Down(distance) => aim += distance,
            Up(distance) => aim -= distance,
        }
    }

    x * y
}

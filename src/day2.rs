use aoc_runner_derive::{aoc, aoc_generator};

type Day2SubDirection = (isize, isize);
#[aoc_generator(day2)]
fn day2_input(s: &str) -> Vec<Day2SubDirection> {
    s.trim()
        .lines()
        .map(|l| {
            let direction: Vec<&str> = l.trim().split(' ').collect();
            let amount = direction[1].parse().unwrap();
            match direction[0] {
                "forward" => (amount, 0),
                "down" => (0, amount),
                "up" => (0, -amount),
                _ => panic!("uh oh"),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn day2_part1(input: &[Day2SubDirection]) -> isize {
    let pos = input
        .iter()
        .fold((0isize, 0isize), |sum, val| (sum.0 + val.0, sum.1 + val.1));
    pos.0 * pos.1
}

#[aoc(day2, part2)]
fn day2_part2(input: &[Day2SubDirection]) -> isize {
    let pos = input.iter().fold((0isize, 0isize, 0isize), |sum, val| {
        (sum.0 + val.0, sum.1 + val.0 * sum.2, sum.2 + val.1)
    });
    pos.0 * pos.1
}
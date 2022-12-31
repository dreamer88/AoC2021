use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn day7_input(s: &str) -> Vec<isize> {
    s.trim().split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
fn day7_part1(input: &[isize]) -> isize {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut min_result = isize::MAX;
    for curr in min..=max {
        let sum = input.iter().fold(0isize, |sum, x| sum + (*x - curr).abs());
        if sum < min_result {
            min_result = sum;
        }
    }

    min_result
}

#[aoc(day7, part2)]
fn day7_part2(input: &[isize]) -> isize {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut min_result = isize::MAX;
    for curr in min..=max {
        let sum = input.iter().fold(0isize, |sum, x| {
            let res: isize = (*x - curr).abs();
            sum + ((res * (res+1))/2)
        });
        if sum < min_result {
            min_result = sum;
        }
    }

    min_result
}

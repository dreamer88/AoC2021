use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn day6_input(s: &str) -> Vec<usize> {
    s.trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn calc(results: &mut Vec<usize>, num_days:usize) -> usize {
    for _i in 0..num_days {
        results[7usize] += results[0usize];
        results.rotate_left(1);
    }
    
    results.iter().sum()
}

#[aoc(day6, part1)]
fn day6_part1(input: &[usize]) -> usize {
    const NUM_DAYS:usize = 80;
    let mut results: Vec<usize> = vec![0usize;9];
    input.iter().for_each(|x| { results[*x] += 1; } );

    calc(&mut results, NUM_DAYS)
}

#[aoc(day6, part2)]
fn day6_part2(input: &[usize]) -> usize {
    const NUM_DAYS:usize = 256;
    let mut results: Vec<usize> = vec![0usize;9];
    input.iter().for_each(|x| { results[*x] += 1; } );

    calc(&mut results, NUM_DAYS)
}

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn day3_input(s: &str) -> Vec<usize> {
    s.trim()
        .lines()
        .map(|x| {
            let mut result = 0usize;
            for c in x.chars() {
                result = result << 1;
                if c == '1' {
                    result += 1;
                }
            }
            result
        })
        .collect()
}

fn get_counts(input: &Vec<usize>, bit: usize) -> (Vec<usize>, Vec<usize>) {
    let mut one: Vec<usize> = vec![];
    let mut zero: Vec<usize> = vec![];

    input.iter().for_each(|x| {
        if ((x >> bit) & 1usize) > 0usize {
            one.push(*x);
        } else {
            zero.push(*x);
        }
    });

    (one, zero)
}

#[aoc(day3, part1)]
fn day3_part1(input: &[usize]) -> usize {
    let i: Vec<usize> = input.clone().iter().map(|x| *x).collect();

    let mut gamma = 0usize;
    let mut epsilon = 0usize;

    for bit in 0..12 {
        let (one, zero) = get_counts(&i, bit);
        if one.len() > zero.len() {
            gamma += 1usize << bit;
        } else {
            epsilon += 1usize << bit;
        }
    }

    gamma * epsilon
}

#[aoc(day3, part2)]
fn day3_part2(input: &[usize]) -> usize {
    let mut oxygen: Vec<usize> = input.clone().iter().map(|x| *x).collect();
    let mut co2: Vec<usize> = input.clone().iter().map(|x| *x).collect();

    for bit in (0..12).rev() {
        if oxygen.len() > 1 {
            let (oxy_one, oxy_zero) = get_counts(&oxygen, bit);
            oxygen = if oxy_one.len() >= oxy_zero.len() {
                oxy_one
            } else {
                oxy_zero
            };
        }
        if co2.len() > 1 {
            let (co2_one, co2_zero) = get_counts(&co2, bit);
            co2 = if co2_zero.len() <= co2_one.len() {
                co2_zero
            } else {
                co2_one
            };
        }
    }
    assert_eq!(oxygen.len(), 1usize);
    assert_eq!(co2.len(), 1usize);

    oxygen[0] * co2[0]
}

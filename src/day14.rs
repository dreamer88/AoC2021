use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct Puzzle {
    chars: Vec<char>,
    map: HashMap<(char, char), char>,
}

#[aoc_generator(day14)]
fn day14_input(s: &str) -> Puzzle {
    let lines: Vec<&str> = s.trim().lines().collect();

    let mut puzzle = Puzzle {
        chars: lines[0].chars().collect(),
        map: HashMap::new(),
    };

    lines[2..lines.len()].iter().for_each(|x| {
        let (pair, c) = x.split_once(" -> ").unwrap();
        let key = (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap());
        let value = c.chars().nth(0).unwrap();
        puzzle.map.insert(key, value);
    });

    puzzle
}

fn try_add<T>(counts: &mut HashMap<T, usize>, key: T, value: usize)
where
    T: std::cmp::Eq + std::hash::Hash,
{
    *counts.entry(key).or_insert(0) += value;
}

fn do_puzzle(input: &Puzzle, num_days: usize) -> usize {
    let chars = input.chars.clone();

    let mut counts: HashMap<(char, char), usize> = HashMap::new();
    let last_char = chars[chars.len()-1];
    for i in 0..chars.len() - 1 {
        let lookup = (chars[i], chars[i + 1]);
        try_add(&mut counts, lookup, 1);
    }

    for _i in 0..num_days {
        let mut new_counts: HashMap<(char, char), usize> = HashMap::new();
        for (k, v) in &counts {
            match input.map.get(&k) {
                Some(c) => {
                    try_add(&mut new_counts, (k.0, *c), *v);
                    try_add(&mut new_counts, (*c, k.1), *v);
                }
                None => {
                    try_add(&mut new_counts, *k, *v);
                }
            }
        }
        counts = new_counts;
    }

    let mut results: HashMap<char, usize> = HashMap::new();
    for (key, value) in &counts {
        try_add(&mut results, key.0, *value);
    }
    try_add(&mut results, last_char, 1);

    itertools::max(results.values()).unwrap() - itertools::min(results.values()).unwrap()
}

#[aoc(day14, part1)]
fn day14_part1(input: &Puzzle) -> usize {
    const NUM_DAYS: usize = 10;
    do_puzzle(input, NUM_DAYS)
}

#[aoc(day14, part2)]
fn day14_part2(input: &Puzzle) -> usize {
    const NUM_DAYS: usize = 40;
    do_puzzle(input, NUM_DAYS)
}

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Point = (usize, usize);
type WeightMap = Vec<Vec<u32>>;
type ResultMap = Vec<Vec<Option<u32>>>;

struct HelperContainer {
    queue: Vec<Point>,
    map: ResultMap,
    in_queue: HashSet<Point>,
}

#[aoc_generator(day15)]
fn day15_input(s: &str) -> WeightMap {
    s.trim()
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn try_point(p: &Point, last_weight: u32, help: &mut HelperContainer, weights: &WeightMap) {
    let rows = weights.len();
    let columns = weights[0].len();

    if p.0 < columns && p.1 < rows {
        let new_weight = weights[p.0][p.1] + last_weight;
        match help.map[p.0][p.1] {
            None => {
                help.map[p.0][p.1] = Some(new_weight);
                help.queue.push(*p);
                help.in_queue.insert(*p);
            }
            Some(old) => {
                if new_weight < old {
                    help.map[p.0][p.1] = Some(new_weight);

                    if !help.in_queue.contains(p) {
                        help.queue.insert(0, *p);
                        help.in_queue.insert(*p);
                    }
                }
            }
        }
    }
}

fn push_neighbors(help: &mut HelperContainer, weights: &WeightMap) {
    let p = help.queue.remove(0);
    help.in_queue.remove(&p);
    let current_weight = help.map[p.0][p.1].unwrap();

    try_point(&(p.0.wrapping_sub(1), p.1), current_weight, help, weights);
    try_point(&(p.0.wrapping_add(1), p.1), current_weight, help, weights);
    try_point(&(p.0, p.1.wrapping_sub(1)), current_weight, help, weights);
    try_point(&(p.0, p.1.wrapping_add(1)), current_weight, help, weights);
}

fn calc_basic_map(input: &WeightMap) -> ResultMap {
    let rows = input.len();
    let columns = input[0].len();

    let mut help = HelperContainer {
        queue: vec![(0, 0)],
        map: (0..rows)
            .map(|_| (0..columns).map(|_| None).collect())
            .collect(),
        in_queue: HashSet::new(),
    };

    help.map[0][0] = Some(0);

    while help.queue.len() > 0 {
        push_neighbors(&mut help, &input);
    }

    help.map
}

#[aoc(day15, part1)]
fn day15_part1(input: &WeightMap) -> u32 {
    let map = calc_basic_map(input);
    map.last().unwrap().last().unwrap().unwrap()
}

#[aoc(day15, part2)]
fn day15_part2(input: &WeightMap) -> u32 {
    let mut setup_columns: WeightMap = Vec::new();

    for row in input {
        let mut new_row: Vec<u32> = vec![];
        for i in 0u32..5 {
            for v in row {
                new_row.push((v + i - 1) % 9 + 1);
            }
        }
        setup_columns.push(new_row);
    }

    let mut new_input: WeightMap = WeightMap::new();
    for i in 0u32..5 {
        for row in &setup_columns {
            new_input.push(row.iter().map(|v| (v + i - 1) % 9 + 1).collect());
        }
    }

    assert_eq!(input.len() * 5, new_input.len());
    assert_eq!(input[0].len() * 5, new_input[0].len());

    let map = calc_basic_map(&new_input);
    map.last().unwrap().last().unwrap().unwrap()
}

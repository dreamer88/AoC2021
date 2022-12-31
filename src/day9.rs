use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day9)]
fn day9_input(s: &str) -> Vec<Vec<u32>> {
    s.trim()
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn is_low_point(row: usize, column: usize, input: &Vec<Vec<u32>>) -> bool {
    let rows = input.len();
    let cols = input[0].len();
    let val = input[row][column];

    if row > 0 && val >= input[row - 1][column] {
        false
    } else if (row + 1) < rows && val >= input[row + 1][column] {
        false
    } else if column > 0 && val >= input[row][column - 1] {
        false
    } else if (column + 1) < cols && val >= input[row][column + 1] {
        false
    } else {
        true
    }
}

#[aoc(day9, part1)]
fn day9_part1(input: &Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, v)| if is_low_point(y, x, input) { v + 1 } else { 0 })
                .collect::<Vec<u32>>()
                .iter()
                .sum()
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn calc_basin_internal(
    row: usize,
    column: usize,
    input: &Vec<Vec<u32>>,
    processed: &mut HashSet<(usize, usize)>,
) {
    let rows = input.len();
    let cols = input[0].len();
    let val = input[row][column];

    if val < 9 && !processed.contains(&(row, column)) {
        processed.insert((row, column));

        if row > 0 {
            calc_basin_internal(row - 1, column, input, processed);
        }
        if (row + 1) < rows {
            calc_basin_internal(row + 1, column, input, processed);
        }
        if column > 0 {
            calc_basin_internal(row, column - 1, input, processed);
        }
        if (column + 1) < cols {
            calc_basin_internal(row, column + 1, input, processed);
        }
    }
}

fn calc_basin(
    row: usize,
    column: usize,
    input: &Vec<Vec<u32>>,
    processed: &mut HashSet<(usize, usize)>,
) -> usize {
    let orig_len = processed.len();
    calc_basin_internal(row, column, input, processed);
    processed.len() - orig_len
}

#[aoc(day9, part2)]
fn day9_part2(input: &Vec<Vec<u32>>) -> usize {
    let mut processed: HashSet<(usize, usize)> = HashSet::new();
    let mut basins = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _v)| calc_basin(y, x, input, &mut processed))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<usize>>();
    basins.sort_by(|a, b| b.cmp(a));
    basins[0..3].iter().fold(1, |res, x| res * x)
}

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Puzzle {
    input: Vec<String>,
    output: Vec<String>,
}

#[aoc_generator(day8)]
fn day8_input(s: &str) -> Vec<Puzzle> {
    s.trim()
        .lines()
        .map(|x| {
            let (input, output) = x.split_once('|').unwrap();
            Puzzle {
                input: input.trim().split(' ').map(|x| x.chars().sorted().collect()).collect(),
                output: output.trim().split(' ').map(|x| x.chars().sorted().collect()).collect(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn day8_part1(input: &[Puzzle]) -> usize {
    let lengths_to_find: Vec<usize> = vec![2, 3, 4, 7];
    input
        .iter()
        .map(|x| {
            x.output
                .iter()
                .filter(|y| lengths_to_find.contains(&y.len()))
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn day8_part2(input: &[Puzzle]) -> usize {
    input
        .iter()
        .map(|x| {
            let ugh: Vec<String> = (0..10).map(|_x| String::new()).collect();
            let mut arr: Vec<&String> = ugh.iter().collect();

            // find our easily identifiable numbers
            // find 1
            arr[1] = x.input.iter().find(|s| s.len() == 2).unwrap();
            arr[7] = x.input.iter().find(|s| s.len() == 3).unwrap();
            arr[4] = x.input.iter().find(|s| s.len() == 4).unwrap();
            arr[8] = x.input.iter().find(|s| s.len() == 7).unwrap();

            // find 6 (length 6 and not all in 7; unlike 9 and 0)
            arr[6] = x
                .input
                .iter()
                .find(|s| s.len() == 6 && !arr[7].chars().all(|s_c| s.contains(s_c))).unwrap();

            // find 9 (length 6 and all in 4; unlike 0 and 6)
            arr[9] = x
                .input
                .iter()
                .find(|s| s.len() == 6 && arr[4].chars().all(|s_c| s.contains(s_c))).unwrap();

            // find 0 (length 6 and not 6 or 9)
            arr[0] = x
                .input
                .iter()
                .find(|s| s.len() == 6 && s != &arr[6] && s != &arr[9]).unwrap();

            // find 0 (length 5 and all in 6)
            arr[5] = x
                .input
                .iter()
                .find(|s| s.len() == 5 && s.chars().all(|s_c| arr[6].contains(s_c))).unwrap();

            // find 3 (length 5 and all in 9 and not equal to 5)
            arr[3] = x
                .input
                .iter()
                .find(|s| s.len() == 5 && s != &arr[5] && s.chars().all(|s_c| arr[9].contains(s_c))).unwrap();

            // find 2 (length 5 and not equal to 3 or 5)
            arr[2] = x
                .input
                .iter()
                .find(|s| s.len() == 5 && s != &arr[3] && s != &arr[5]).unwrap();

            let mut output:usize = 0;
            for s in &x.output {
                output *= 10;
                output += arr.iter().position(|i| i == &s).unwrap();
            }

            output
        })
        .sum()
}

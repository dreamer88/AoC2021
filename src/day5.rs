use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::{max, min};

struct Point {
    pub x: usize,
    pub y: usize,
}

struct Line {
    pub start: Point,
    pub end: Point,
}

#[aoc_generator(day5)]
fn day5_input(s: &str) -> Vec<Line> {
    s.trim()
        .lines()
        .map(|x| {
            let (s, e) = x.split_once("->").unwrap();
            let (s_x, s_y) = s.trim().split_once(",").unwrap();
            let (e_x, e_y) = e.trim().split_once(",").unwrap();
            Line {
                start: Point {
                    x: s_x.parse().unwrap(),
                    y: s_y.parse().unwrap(),
                },
                end: Point {
                    x: e_x.parse().unwrap(),
                    y: e_y.parse().unwrap(),
                },
            }
        })
        .collect()
}

#[aoc(day5, part1)]
fn day5_part1(input: &[Line]) -> usize {
    const MAP_SIZE: usize = 1000;
    let mut map = [[0u8; MAP_SIZE]; MAP_SIZE];

    for l in input {
        if l.start.x == l.end.x {
            for i in min(l.start.y, l.end.y)..=max(l.start.y, l.end.y) {
                map[i][l.start.x] += 1;
            }
        } else if l.start.y == l.end.y {
            for i in min(l.start.x, l.end.x)..=max(l.start.x, l.end.x) {
                map[l.start.y][i] += 1;
            }
        }
    }

    map.iter()
        .fold(0usize, |sum, &x| sum + x.iter().filter(|&y| *y >= 2).count())
}

#[aoc(day5, part2)]
fn day5_part2(input: &[Line]) -> usize {
    const MAP_SIZE: usize = 1000;
    let mut map = [[0u8; MAP_SIZE]; MAP_SIZE];

    for l in input {
        if l.start.x == l.end.x {
            for i in min(l.start.y, l.end.y)..=max(l.start.y, l.end.y) {
                map[i][l.start.x] += 1;
            }
        } else if l.start.y == l.end.y {
            for i in min(l.start.x, l.end.x)..=max(l.start.x, l.end.x) {
                map[l.start.y][i] += 1;
            }
        } else {
            // guaranteed to be diagonal
            let s = if l.start.x < l.end.x { &l.start } else { &l.end };
            let e = if l.start.x < l.end.x { &l.end } else { &l.start };
            let multiplier = if e.y < s.y { false } else { true };

            for (i,x) in (s.x..=e.x).enumerate() {
                if multiplier {
                    map[s.y+i][x] += 1;
                } else {
                    map[s.y-i][x] += 1;
                }
            }
        }
    }

    map.iter()
        .fold(0usize, |sum, &x| sum + x.iter().filter(|&y| *y >= 2).count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn wtf() {
        let f = fs::read_to_string(r"C:\Users\Jeff\Documents\Programming\rust\advent-of-code-2021\input\2021\day5.txt").unwrap();
        let lines = day5_input(f.as_str());
        let result = day5_part1(&lines.as_slice());
        println!("result: {}", result);
    }
}
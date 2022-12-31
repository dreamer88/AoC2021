use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;

#[derive(Debug, Copy, Clone)]
enum State {
    Empty,
    Right,
    Down,
}

impl State {
    fn from_char(c: &char) -> Self {
        match c {
            '.' => State::Empty,
            '>' => State::Right,
            'v' => State::Down,
            _ => {
                panic!("unknown char");
            }
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Empty => write!(f, "."),
            State::Right => write!(f, ">"),
            State::Down => write!(f, "v"),
        }
    }
}

#[aoc_generator(day25)]
fn day25_input(s: &str) -> Vec<Vec<State>> {
    s.trim()
        .lines()
        .map(|x| x.chars().map(|c| State::from_char(&c)).collect())
        .collect()
}

fn copy_vec(v: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    v.iter().map(|x| x.iter().map(|s| *s).collect()).collect()
}

fn alloc_vec(v: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    v.iter()
        .map(|x| x.iter().map(|_s| State::Empty).collect())
        .collect()
}

#[aoc(day25, part1)]
fn day25_part1(input: &Vec<Vec<State>>) -> usize {
    let mut step = 0;

    let mut state = copy_vec(input);
    let rows = input.len();
    let cols = input[0].len();
    loop {
        step = step + 1;
        let mut next_step = alloc_vec(&state);
        let mut made_change = false;

        for i in 0..rows {
            for j in 0..cols {
                if matches!(state[i][j], State::Right) {
                    if matches!(state[i][(j + 1) % cols], State::Empty) {
                        next_step[i][(j + 1) % cols] = State::Right;
                        made_change = true;
                    } else {
                        next_step[i][j] = State::Right;
                    }
                }
            }
        }

        for i in 0..rows {
            for j in 0..cols {
                if matches!(state[i][j], State::Down) {
                    if !matches!(state[(i + 1) % rows][j], State::Down)
                        && matches!(next_step[(i + 1) % rows][j], State::Empty)
                    {
                        next_step[(i + 1) % rows][j] = State::Down;
                        made_change = true;
                    } else {
                        next_step[i][j] = State::Down;
                    }
                }
            }
        }

        if !made_change {
            break;
        } else {
            state = next_step;
        }
    }
    step
}

#[aoc(day25, part2)]
fn day25_part2(_input: &Vec<Vec<State>>) -> usize {
    0
}

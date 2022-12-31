use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

lazy_static! {
    static ref CHAR_MAP: HashMap<char,char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>')
    ]);
}

#[aoc_generator(day10)]
fn day10_input(s: &str) -> Vec<String> {
    s.trim().lines().map(|x| x.parse().unwrap()).collect()
}

fn match_char(c: char) -> char {
    *CHAR_MAP.get(&c).unwrap()
}

fn find_corrupted(s: &String) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let matched = match stack.pop() {
                    Some(x) => Some(match_char(x)),
                    _ => None,
                };
                if matched != Some(c) {
                    return Some(c);
                }
            }
            _ => return None,
        }
    }
    None
}

#[aoc(day10, part1)]
fn day10_part1(input: &[String]) -> usize {
    let mut scores: HashMap<Option<char>, usize> = HashMap::new();
    scores.insert(None, 0);
    scores.insert(Some(')'), 3);
    scores.insert(Some(']'), 57);
    scores.insert(Some('}'), 1197);
    scores.insert(Some('>'), 25137);

    input.iter().map(|l| scores[&find_corrupted(l)]).sum()
}

fn find_incomplete(s: &String) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                stack.pop();
                ()
            }
            _ => (),
        }
    }

    stack.iter().map(|x| match_char(*x)).rev().collect()
}

#[aoc(day10, part2)]
fn day10_part2(input: &[String]) -> usize {
    let mut scores: HashMap<Option<char>, usize> = HashMap::new();
    scores.insert(None, 0);
    scores.insert(Some(')'), 1);
    scores.insert(Some(']'), 2);
    scores.insert(Some('}'), 3);
    scores.insert(Some('>'), 4);

    let mut results: Vec<usize> = input
        .iter()
        .filter(|x| find_corrupted(x) == None)
        .map(|x| {
            let result = find_incomplete(x);
            result
                .iter()
                .map(|c| scores[&Some(*c)])
                .fold(0, |fold, v| fold * 5 + v)
        })
        .collect();
    results.sort();
    results[results.len() / 2]
}

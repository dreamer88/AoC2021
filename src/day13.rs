use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

enum Fold {
    X(isize),
    Y(isize),
}

struct Puzzle {
    holes: HashSet<(isize, isize)>,
    folds: Vec<Fold>,
}

#[aoc_generator(day13)]
fn day13_input(s: &str) -> Puzzle {
    let mut output = Puzzle {
        holes: HashSet::new(),
        folds: vec![],
    };

    s.trim().lines().for_each(|x| {
        if x.starts_with("fold along") {
            let t = x[11..x.len()].split_once('=').unwrap();
            let v: isize = t.1.parse().unwrap();
            output.folds.push(match t.0 {
                "x" => Fold::X(v),
                "y" => Fold::Y(v),
                _ => panic!(),
            });
        } else if !x.is_empty() {
            let t = x.split_once(',').unwrap();
            output
                .holes
                .insert((t.0.parse::<isize>().unwrap(), t.1.parse::<isize>().unwrap()));
        }
    });

    output
}

fn perform_fold(start: &HashSet<(isize, isize)>, folds: &[Fold]) -> HashSet<(isize, isize)> {
    let mut current_set = start.clone();
    for fold in folds {
        let mut new_set: HashSet<(isize, isize)> = HashSet::new();
        match fold {
            Fold::X(v) => {
                for p in &current_set {
                    match p.0.cmp(v) {
                        std::cmp::Ordering::Less => {
                            new_set.insert(*p);
                        }
                        std::cmp::Ordering::Greater => {
                            let x = 2 * *v - p.0;
                            new_set.insert((x, p.1));
                        }
                        std::cmp::Ordering::Equal => (),
                    };
                }
            }
            Fold::Y(v) => {
                for p in &current_set {
                    match p.1.cmp(v) {
                        std::cmp::Ordering::Less => {
                            new_set.insert(*p);
                        }
                        std::cmp::Ordering::Greater => {
                            let y = 2 * *v - p.1;
                            new_set.insert((p.0, y));
                        }
                        std::cmp::Ordering::Equal => (),
                    };
                }
            }
        }

        current_set = new_set;
    }

    current_set
}

#[aoc(day13, part1)]
fn day13_part1(input: &Puzzle) -> usize {
    perform_fold(&input.holes, &input.folds[0..1]).len()
}

#[aoc(day13, part2)]
fn day13_part2(input: &Puzzle) -> usize {
    let results = perform_fold(&input.holes, &input.folds);
    let (mut max_x, mut max_y) = (0, 0);

    for x in &results {
        if x.0 > max_x {
            max_x = x.0;
        }

        if x.1 > max_y {
            max_y = x.1;
        }
    }

    max_x += 1;
    max_y += 1;

    let mut lines: Vec<Vec<char>> = (0..max_y).map(|_| vec![' '; max_x as usize]).collect();

    for x in &results {
        lines[x.1 as usize][x.0 as usize] = 'x';
    }

    for l in lines {
        println!("{}", l.iter().collect::<String>());
    }

    0
}

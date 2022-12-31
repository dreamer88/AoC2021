use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashSet<String>>;
const START: &str = "start";
const END: &str = "end";

#[aoc_generator(day12)]
fn day12_input(s: &str) -> Graph {
    let mut map: Graph = Graph::new();
    s.trim().lines().for_each(|x| {
        let s = x.split_once('-').unwrap();
        let s0: String = s.0.into();
        let s1: String = s.1.into();

        if !map.contains_key(&s0) {
            map.insert(s0.clone(), [s1.clone()].into());
        } else {
            map.get_mut(&s0).unwrap().insert(s1.clone());
        }

        if !map.contains_key(&s1) {
            map.insert(s1, [s0].into());
        } else {
            map.get_mut(&s1).unwrap().insert(s0);
        }
    });

    map
}

fn is_small_cave(name: &String) -> bool {
    *name == name.to_lowercase()
}

fn calc_paths(graph: &Graph, name: &String, previous: HashSet<&String>) -> usize {
    let paths = &graph[name];
    let mut count = 0;
    for p in paths {
        if p == END {
            count += 1;
        } else if !is_small_cave(p) || !previous.contains(p) {
            let mut copy = previous.clone();
            copy.insert(p);
            count += calc_paths(graph, p, copy);
        }
    }
    count
}

#[aoc(day12, part1)]
fn day12_part1(input: &Graph) -> usize {
    calc_paths(input, &String::from(START), [&String::from(START)].into())
}

fn calc_paths_2(
    graph: &Graph,
    name: &String,
    previous: HashSet<&String>,
    doubled: Option<&String>,
) -> usize {
    let paths = &graph[name];
    let mut count = 0;
    for p in paths {
        if p == END {
            count += 1;
        } else {
            let is_small = is_small_cave(p);
            let is_previous = is_small && previous.contains(p);
            if !is_previous || (doubled == None && p != START) {
                let mut copy = previous.clone();
                copy.insert(p);
                count += calc_paths_2(
                    graph,
                    p,
                    copy,
                    if is_previous {
                        Some(p)
                    } else {
                        doubled
                    },
                );
            }
        }
    }
    count
}

#[aoc(day12, part2)]
fn day12_part2(input: &Graph) -> usize {
    calc_paths_2(
        input,
        &String::from(START),
        [&String::from(START)].into(),
        None,
    )
}

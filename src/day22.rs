use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::cmp::min;

#[derive(Copy, Clone)]
struct Cube {
    on: bool,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

type GetModified = fn(&Cube, isize, isize) -> Cube;
type ProcessResult = Option<(
    Option<Cube>,
    (Cube, Cube),
    Option<(Option<Cube>, Option<Cube>)>,
)>;

impl Cube {
    fn process_internal(
        s_pos: &(isize, isize),
        o_pos: &(isize, isize),
        s_cube: &Cube,
        o_cube: &Cube,
        get_modified: GetModified,
    ) -> ProcessResult {
        if s_pos == o_pos {
            // we overlap so just boot us back out
            return None;
        }

        if s_pos.0 <= o_pos.0 {
            // we don't touch
            if s_pos.1 < o_pos.0 {
                return None;
            }

            Some((
                if s_pos.0 < o_pos.0 {
                    Some(get_modified(s_cube, s_pos.0, o_pos.0 - 1))
                } else {
                    None
                },
                (
                    get_modified(s_cube, o_pos.0, min(s_pos.1, o_pos.1)),
                    get_modified(o_cube, o_pos.0, min(s_pos.1, o_pos.1)),
                ),
                if o_pos.1 < s_pos.1 {
                    Some((Some(get_modified(s_cube, o_pos.1 + 1, s_pos.1)), None))
                } else if s_pos.1 < o_pos.1 {
                    Some((None, Some(get_modified(o_cube, s_pos.1 + 1, o_pos.1))))
                } else {
                    None
                },
            ))
        } else {
            match Cube::process_internal(o_pos, s_pos, o_cube, s_cube, get_modified) {
                Some(flip) => Some((
                    match flip.2 {
                        Some((_o, s)) => s,
                        None => None,
                    },
                    (flip.1 .1, flip.1 .0),
                    match flip.0 {
                        Some(o) => Some((None, Some(o))),
                        None => None,
                    },
                )),
                None => None,
            }
        }
    }

    fn process_x(&self, other: &Cube) -> ProcessResult {
        Cube::process_internal(&self.x, &other.x, self, other, |c, start, end| Cube {
            on: c.on,
            x: (start, end),
            y: c.y,
            z: c.z,
        })
    }

    fn process_y(&self, other: &Cube) -> ProcessResult {
        Cube::process_internal(&self.y, &other.y, self, other, |c, start, end| Cube {
            on: c.on,
            x: c.x,
            y: (start, end),
            z: c.z,
        })
    }

    fn process_z(&self, other: &Cube) -> ProcessResult {
        Cube::process_internal(&self.z, &other.z, self, other, |c, start, end| Cube {
            on: c.on,
            x: c.x,
            y: c.y,
            z: (start, end),
        })
    }

    fn process_all(&self, other: &Cube) -> Option<(Cube, Vec<Cube>)> {
        if let Some((left, (mid_self, mid_other), right)) = self.process_x(other) {
            if let Some((bottom, (center_self, center_other), top)) = mid_self.process_y(&mid_other)
            {
                if let Some((inner, core, outer)) = center_self.process_z(&center_other) {
                    assert_eq!(core.0.x, core.1.x);
                    assert_eq!(core.0.y, core.1.y);
                    assert_eq!(core.0.z, core.1.z);

                    let mut parts: Vec<Cube> = vec![];
                    if let Some(c) = left {
                        parts.push(c);
                    }

                    if let Some((Some(c), None)) = right {
                        parts.push(c);
                    }

                    if let Some(c) = bottom {
                        parts.push(c);
                    }

                    if let Some((Some(c), None)) = top {
                        parts.push(c);
                    }

                    if let Some(c) = inner {
                        parts.push(c);
                    }

                    if let Some((Some(c), None)) = outer {
                        parts.push(c);
                    }

                    assert_eq!(
                        core.0.get_on_count()
                            + parts.iter().map(|x| x.get_on_count()).sum::<isize>(),
                        self.get_on_count()
                    );

                    return Some((core.0, parts));
                }
            }
        }
        None
    }

    fn get_on_count(&self) -> isize {
        if self.on {
            (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
        } else {
            0
        }
    }
}

#[aoc_generator(day22)]
fn day22_input(s: &str) -> Vec<Cube> {
    let position_regex = Regex::new(r"^(?P<mode>on|off) x=(?P<x_start>-?\d+)\.\.(?P<x_end>-?\d+),y=(?P<y_start>-?\d+)\.\.(?P<y_end>-?\d+),z=(?P<z_start>-?\d+)\.\.(?P<z_end>-?\d+)$").unwrap();
    s.trim()
        .lines()
        .map(|x| {
            let captures = position_regex.captures(x).unwrap();
            Cube {
                on: captures.name("mode").unwrap().as_str() == "on",
                x: (
                    captures.name("x_start").unwrap().as_str().parse().unwrap(),
                    captures.name("x_end").unwrap().as_str().parse().unwrap(),
                ),
                y: (
                    captures.name("y_start").unwrap().as_str().parse().unwrap(),
                    captures.name("y_end").unwrap().as_str().parse().unwrap(),
                ),
                z: (
                    captures.name("z_start").unwrap().as_str().parse().unwrap(),
                    captures.name("z_end").unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        .collect()
}

type FilterFn = fn(&Cube) -> bool;
fn process_cubes(input: &[Cube], filter_fn: FilterFn) -> Vec<Cube> {
    let mut turned_on: Vec<Cube> = vec![];
    let mut to_process: Vec<Cube> = input.iter().map(|x| *x).collect();

    while to_process.len() > 0 {
        let first = to_process.remove(0);

        if filter_fn(&first) {
            let mut new_on: Vec<Cube> = vec![];
            for cube in turned_on {
                if let Some((_, parts)) = cube.process_all(&first) {
                    new_on.extend(parts);
                } else {
                    new_on.push(cube);
                }
            }

            if first.on {
                new_on.push(first);
            }
            turned_on = new_on;
        }
    }

    turned_on
}

#[aoc(day22, part1)]
fn day22_part1(input: &[Cube]) -> isize {
    let turned_on = process_cubes(input, |c| {
        c.x.0.abs() <= 50
            && c.x.1.abs() <= 50
            && c.y.0.abs() <= 50
            && c.y.1.abs() <= 50
            && c.z.0.abs() <= 50
            && c.z.1.abs() <= 50
    });

    turned_on.iter().map(|x| x.get_on_count()).sum()
}

#[aoc(day22, part2)]
fn day22_part2(input: &[Cube]) -> isize {
    let turned_on = process_cubes(input, |_c| true);

    turned_on.iter().map(|x| x.get_on_count()).sum()
}

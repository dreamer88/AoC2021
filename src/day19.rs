use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn rotate_around_x(&self) -> Self {
        Point {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    fn rotate_around_y(&self) -> Self {
        Point {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    fn rotate_around_z(&self) -> Self {
        Point {
            x: -self.y,
            y: self.x,
            z: self.z,
        }
    }

    fn manhattan_distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    _id: usize,
    pos: Option<Point>,
    beacons: Vec<Point>,
}

impl Scanner {
    fn rotate_around_x(&self) -> Self {
        Scanner {
            _id: self._id,
            pos: self.pos,
            beacons: self.beacons.iter().map(|x| x.rotate_around_x()).collect(),
        }
    }

    fn rotate_around_y(&self) -> Self {
        Scanner {
            _id: self._id,
            pos: self.pos,
            beacons: self.beacons.iter().map(|x| x.rotate_around_y()).collect(),
        }
    }

    fn rotate_around_z(&self) -> Self {
        Scanner {
            _id: self._id,
            pos: self.pos,
            beacons: self.beacons.iter().map(|x| x.rotate_around_z()).collect(),
        }
    }

    fn get_beacons_from_pos(&self, p: &Point) -> Vec<Point> {
        self.beacons.iter().map(|x| *x + *p).collect()
    }
}

#[aoc_generator(day19)]
fn day19_input(s: &str) -> Vec<Scanner> {
    let new_scanner_re = Regex::new(r"^--- scanner (?P<scanner_id>\d+) ---$").unwrap();
    let beacon_re = Regex::new(r"^(?P<x>-?\d+),(?P<y>-?\d+),(?P<z>-?\d+)$").unwrap();

    let mut scanners: Vec<Scanner> = Vec::new();
    s.trim().lines().for_each(|x| {
        if let Some(scanner) = new_scanner_re.captures(x) {
            scanners.push(Scanner {
                _id: scanner
                    .name("scanner_id")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
                pos: None,
                beacons: vec![],
            });
        } else if let Some(beacon) = beacon_re.captures(x) {
            scanners.last_mut().unwrap().beacons.push(Point {
                x: beacon.name("x").unwrap().as_str().parse().unwrap(),
                y: beacon.name("y").unwrap().as_str().parse().unwrap(),
                z: beacon.name("z").unwrap().as_str().parse().unwrap(),
            })
        }
    });

    scanners
}

fn register_scanner(scanner: &Scanner, beacons: &mut HashSet<Point>) {
    for beacon in &scanner.beacons {
        beacons.insert(*beacon + scanner.pos.unwrap());
    }
}

fn try_scanner_against_scanners(
    scanner: &Scanner,
    placed_scanners: &Vec<Scanner>,
) -> Option<Scanner> {
    for placed_scanner in placed_scanners {
        let placed_beacons = placed_scanner.get_beacons_from_pos(&placed_scanner.pos.unwrap());

        for known_beacon in &placed_beacons {
            // try each scanner beacon against each beacon
            for beacon in &scanner.beacons {
                let offset = *known_beacon - *beacon;
                let new_positions = scanner.get_beacons_from_pos(&offset);
                let count = new_positions
                    .iter()
                    .filter(|x| placed_beacons.contains(x))
                    .count();
                assert_eq!(count >= 1, true);
                if count >= 12 {
                    let mut copy = scanner.clone();
                    copy.pos = Some(offset);
                    return Some(copy);
                }
            }
        }
    }
    None
}

fn test_scanner_against_scanners(
    scanner: &Scanner,
    placed_scanners: &Vec<Scanner>,
) -> Option<Scanner> {
    let mut current_scanner_x = scanner.clone();
    for _x_rot in 0..=3 {
        current_scanner_x = current_scanner_x.rotate_around_x();

        let mut current_scanner_y = current_scanner_x.clone();
        for _y_rot in 0..=3 {
            current_scanner_y = current_scanner_y.rotate_around_y();
            let mut current_scanner_z = current_scanner_y.clone();
            for _z_rot in 0..=3 {
                current_scanner_z = current_scanner_z.rotate_around_z();

                if let Some(result) =
                    try_scanner_against_scanners(&current_scanner_z, placed_scanners)
                {
                    return Some(result);
                }
            }
        }
    }

    None
}

fn find_beacons(input: &[Scanner]) -> (Vec<Scanner>, HashSet<Point>) {
    let mut scanners: Vec<Scanner> = input.iter().map(|x| x.clone()).collect();
    let mut placed_scanners: Vec<Scanner> = Vec::new();
    let mut beacons: HashSet<Point> = HashSet::new();

    let mut base = scanners.remove(0);
    base.pos = Some(Point::default());
    register_scanner(&base, &mut beacons);
    placed_scanners.push(base);

    let mut index = 0;
    while scanners.len() > 0 {
        if let Some(scanner) = test_scanner_against_scanners(&scanners[index], &placed_scanners) {
            register_scanner(&scanner, &mut beacons);
            placed_scanners.push(scanner);
            scanners.remove(index);
            if scanners.len() > 0 {
                index = index % scanners.len();
            }
        } else {
            index = (index + 1) % scanners.len();
        }
    }

    (placed_scanners, beacons)
}

#[aoc(day19, part1)]
fn day19_part1(input: &[Scanner]) -> usize {
    let (_, beacons) = find_beacons(input);
    beacons.len()
}

#[aoc(day19, part2)]
fn day19_part2(input: &[Scanner]) -> isize {
    let (scanners, _) = find_beacons(input);
    let mut max = isize::MIN;
    for i in 0..scanners.len() - 1 {
        for j in i + 1..scanners.len() {
            let dist = scanners[i]
                .pos
                .unwrap()
                .manhattan_distance(&scanners[j].pos.unwrap());
            if dist > max {
                max = dist;
            }
        }
    }
    max
}

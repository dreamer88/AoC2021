use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

enum XResult {
    Left,
    In,
    Right,
}

enum YResult {
    Above,
    In,
    Below,
}

type Point = (isize, isize);

struct Target {
    top_left: Point,
    bottom_right: Point,
}

#[aoc_generator(day17)]
fn day17_input(s: &str) -> Target {
    let re = Regex::new(r"^target area: x=(?P<x_min>-?\d+)\.\.(?P<x_max>-?\d+), y=(?P<y_min>-?\d+)\.\.(?P<y_max>-?\d+)$").unwrap();
    let cap = re.captures(s.trim()).unwrap();
    Target {
        top_left: (
            cap.name("x_min").unwrap().as_str().parse().unwrap(),
            cap.name("y_max").unwrap().as_str().parse().unwrap(),
        ),
        bottom_right: (
            cap.name("x_max").unwrap().as_str().parse().unwrap(),
            cap.name("y_min").unwrap().as_str().parse().unwrap(),
        ),
    }
}

fn do_step(
    pos: &mut Point,
    x_vel: &mut isize,
    y_vel: &mut isize,
    local_highest: &mut isize,
    target: &Target,
) -> (XResult, YResult) {
    pos.0 += *x_vel;
    pos.1 += *y_vel;

    if pos.1 > *local_highest {
        *local_highest = pos.1;
    }

    if *x_vel > 0 {
        *x_vel -= 1;
    } else if *x_vel < 0 {
        *x_vel += 1;
    }

    *y_vel -= 1;

    (
        if pos.0 < target.top_left.0 {
            XResult::Left
        } else if pos.0 > target.bottom_right.0 {
            XResult::Right
        } else {
            XResult::In
        },
        if pos.1 > target.top_left.1 {
            YResult::Above
        } else if pos.1 < target.bottom_right.1 {
            YResult::Below
        } else {
            YResult::In
        },
    )
}

#[aoc(day17, part1)]
fn day17_part1(input: &Target) -> isize {
    let mut highest_y = isize::MIN;

    for x_vel in 0..=input.bottom_right.0 {
        'y: for y_vel in 0..1000 {
            let mut pos: Point = (0, 0);
            let mut local_highest = 0;
            let mut cur_x_vel = x_vel;
            let mut cur_y_vel = y_vel;
            loop {
                match do_step(
                    &mut pos,
                    &mut cur_x_vel,
                    &mut cur_y_vel,
                    &mut local_highest,
                    input,
                ) {
                    (XResult::Left, y) => {
                        if cur_x_vel <= 0 {
                            break 'y;
                        } else {
                            match y {
                                YResult::Below => break, // we're still going but we're too low, try a higher y
                                _ => (),                 // we're still to the left, keep going
                            }
                        }
                    }
                    (XResult::Right, YResult::Above) => break 'y, // we're too far right and above the target so stop increasing y velocity
                    (XResult::Right, _) => break, // we're to the right but we missed the target, keep trying
                    (XResult::In, YResult::Above) => (), // keep going, we may drop into the zone
                    (XResult::In, YResult::Below) => break, // we're in but too far down, try with a higher y
                    (XResult::In, YResult::In) => {
                        // found a match!
                        if local_highest > highest_y {
                            highest_y = local_highest;
                        }
                        break; // break but keep trying a new y_vel in case we can get higher
                    }
                }
            }
        }
    }

    highest_y
}

#[aoc(day17, part2)]
fn day17_part2(input: &Target) -> usize {
    let mut result = 0usize;
    for x_vel in 0..=input.bottom_right.0 {
        'y: for y_vel in input.bottom_right.1..1000 {
            let mut pos: Point = (0, 0);
            let mut local_highest = 0;
            let mut cur_x_vel = x_vel;
            let mut cur_y_vel = y_vel;
            loop {
                match do_step(
                    &mut pos,
                    &mut cur_x_vel,
                    &mut cur_y_vel,
                    &mut local_highest,
                    input,
                ) {
                    (XResult::Left, y) => {
                        if cur_x_vel <= 0 {
                            break 'y;
                        } else {
                            match y {
                                YResult::Below => break, // we're still going but we're too low, try a higher y
                                _ => (),                 // we're still to the left, keep going
                            }
                        }
                    }
                    (XResult::Right, YResult::Above) => break 'y, // we're too far right and above the target so stop increasing y velocity
                    (XResult::Right, _) => break, // we're to the right but we missed the target, keep trying
                    (XResult::In, YResult::Above) => (), // keep going, we may drop into the zone
                    (XResult::In, YResult::Below) => break, // we're in but too far down, try with a higher y
                    (XResult::In, YResult::In) => {
                        result += 1;
                        break; // break but keep trying a new y_vel in case we can get higher
                    }
                }
            }
        }
    }

    result
}

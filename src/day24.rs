use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::{HashSet,HashMap};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Field {
    W,
    X,
    Y,
    Z,
    Val(isize),
}

impl FromStr for Field {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Field::W),
            "x" => Ok(Field::X),
            "y" => Ok(Field::Y),
            "z" => Ok(Field::Z),
            _ => {
                let v = s.parse::<isize>()?;
                Ok(Field::Val(v))
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Instruction {
    Input(Field),
    Add(Field, Field),
    Mul(Field, Field),
    Div(Field, Field),
    Mod(Field, Field),
    Eql(Field, Field),
}

lazy_static! {
    static ref INSTRUCTION_RE: Regex =
        Regex::new(r"^(?P<op>\w+) (?P<field>\w)( (?P<field2>(\w|-?\d+)))?$").unwrap();
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = INSTRUCTION_RE.captures(s).unwrap();

        match captures.name("op").unwrap().as_str() {
            "inp" => Ok(Instruction::Input(
                captures.name("field").unwrap().as_str().parse().unwrap(),
            )),

            "add" => Ok(Instruction::Add(
                captures.name("field").unwrap().as_str().parse().unwrap(),
                captures.name("field2").unwrap().as_str().parse().unwrap(),
            )),

            "mul" => Ok(Instruction::Mul(
                captures.name("field").unwrap().as_str().parse().unwrap(),
                captures.name("field2").unwrap().as_str().parse().unwrap(),
            )),

            "div" => Ok(Instruction::Div(
                captures.name("field").unwrap().as_str().parse().unwrap(),
                captures.name("field2").unwrap().as_str().parse().unwrap(),
            )),

            "mod" => Ok(Instruction::Mod(
                captures.name("field").unwrap().as_str().parse().unwrap(),
                captures.name("field2").unwrap().as_str().parse().unwrap(),
            )),

            "eql" => Ok(Instruction::Eql(
                captures.name("field").unwrap().as_str().parse().unwrap(),
                captures.name("field2").unwrap().as_str().parse().unwrap(),
            )),
            _ => panic!("bad type"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl State {
    fn new() -> Self {
        State {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn process_instruction(
        &self,
        instruction: &Instruction,
        input: Option<isize>,
    ) -> Option<State> {
        let mut new_state = self.clone();

        match instruction {
            Instruction::Input(a) => {
                if let Some(v) = input {
                    *new_state.get_field_mut(a) = v;
                } else {
                    return None;
                }
            }
            Instruction::Add(a, b) => {
                *new_state.get_field_mut(a) = self.get_field(a) + self.get_field(b)
            }
            Instruction::Mul(a, b) => {
                *new_state.get_field_mut(a) = self.get_field(a) * self.get_field(b)
            }
            Instruction::Div(a, b) => {
                *new_state.get_field_mut(a) = self.get_field(a) / self.get_field(b)
            }
            Instruction::Mod(a, b) => {
                *new_state.get_field_mut(a) = self.get_field(a) % self.get_field(b)
            }
            Instruction::Eql(a, b) => {
                *new_state.get_field_mut(a) = if self.get_field(a) == self.get_field(b) {
                    1
                } else {
                    0
                }
            }
        }

        Some(new_state)
    }

    fn process_instruction_mut(&mut self, instruction: &Instruction, input: Option<isize>) -> bool {
        match instruction {
            Instruction::Input(a) => {
                if let Some(v) = input {
                    *self.get_field_mut(a) = v;
                } else {
                    return false;
                }
            }
            Instruction::Add(a, b) => {
                *self.get_field_mut(a) = self.get_field(a) + self.get_field(b)
            }
            Instruction::Mul(a, b) => {
                *self.get_field_mut(a) = self.get_field(a) * self.get_field(b)
            }
            Instruction::Div(a, b) => {
                *self.get_field_mut(a) = self.get_field(a) / self.get_field(b)
            }
            Instruction::Mod(a, b) => {
                *self.get_field_mut(a) = self.get_field(a) % self.get_field(b)
            }
            Instruction::Eql(a, b) => {
                *self.get_field_mut(a) = if self.get_field(a) == self.get_field(b) {
                    1
                } else {
                    0
                }
            }
        }

        true
    }

    fn get_field(&self, field: &Field) -> isize {
        match field {
            Field::W => self.w,
            Field::X => self.x,
            Field::Y => self.y,
            Field::Z => self.z,
            Field::Val(v) => *v,
        }
    }

    fn get_field_mut(&mut self, field: &Field) -> &mut isize {
        match field {
            Field::W => &mut self.w,
            Field::X => &mut self.x,
            Field::Y => &mut self.y,
            Field::Z => &mut self.z,
            Field::Val(_) => panic!("cannot assign to val"),
        }
    }
}

#[aoc_generator(day24)]
fn day24_input(s: &str) -> Vec<Instruction> {
    s.trim()
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn try_add_max(state_map: &mut HashMap<State, isize>, state: &State, input: isize) {
    if let Some(v) = state_map.get_mut(state) {
        if *v < input {
            *v = input;
        }
    } else {
        state_map.insert(*state, input);
    }
}

fn try_add_min(state_map: &mut HashMap<State, isize>, state: &State, input: isize) {
    if let Some(v) = state_map.get_mut(state) {
        if input < *v {
            *v = input;
        }
    } else {
        state_map.insert(*state, input);
    }
}


#[aoc(day24, part1)]
fn day24_part1(input: &[Instruction]) -> isize {
    let mut states: Vec<(State, isize)> = vec![(State::new(), 0isize)];

    let len = input.iter().count();
    for (i, instruction) in input.iter().enumerate() {
        match instruction {
            Instruction::Mod(..) | Instruction::Input(..) => {
                let mut next_states: HashMap<State, isize> = HashMap::new();

                for mut state_pair in states {
                    match state_pair.0.process_instruction_mut(instruction, None) {
                        true => {
                            try_add_max(&mut next_states, &state_pair.0, state_pair.1);
                        }
                        false => {
                            for v in 1..=9 {
                                let mut new_state = state_pair.0.clone();
                                if new_state.process_instruction_mut(instruction, Some(v)) {
                                    try_add_max(&mut next_states, &new_state, state_pair.1 * 10 + v);
                                } else {
                                    panic!("we should never get here");
                                }
                            }
                        }
                    }
                }

                states = next_states.iter().map(|(s, v)| (*s, *v)).collect();
            }
            _ => {
                states.iter_mut().for_each(|x| {
                    match x.0.process_instruction_mut(instruction, None) {
                        true => (),
                        false => {
                            panic!("we shouldn't be here");
                        }
                    }
                } );
            }
        }
    }

    states
        .iter()
        .filter(|(s, v)| s.z == 0)
        .map(|(s, v)| *v)
        .max()
        .unwrap()
}

#[aoc(day24, part2)]
fn day24_part2(input: &[Instruction]) -> isize {
    let mut states: Vec<(State, isize)> = vec![(State::new(), 0isize)];

    let len = input.iter().count();
    for (i, instruction) in input.iter().enumerate() {
        match instruction {
            Instruction::Input(..) => {
                let mut next_states: HashMap<State, isize> = HashMap::new();

                for mut state_pair in states {
                    match state_pair.0.process_instruction_mut(instruction, None) {
                        true => {
                            try_add_min(&mut next_states, &state_pair.0, state_pair.1);
                        }
                        false => {
                            for v in 1..=9 {
                                let mut new_state = state_pair.0.clone();
                                if new_state.process_instruction_mut(instruction, Some(v)) {
                                    try_add_min(&mut next_states, &new_state, state_pair.1 * 10 + v);
                                } else {
                                    panic!("we should never get here");
                                }
                            }
                        }
                    }
                }

                states = next_states.iter().map(|(s, v)| (*s, *v)).collect();
            }
            _ => {
                states.iter_mut().for_each(|x| {
                    match x.0.process_instruction_mut(instruction, None) {
                        true => (),
                        false => {
                            panic!("we shouldn't be here");
                        }
                    }
                } );
            }
        }
    }

    states
        .iter()
        .filter(|(s, v)| s.z == 0)
        .map(|(s, v)| *v)
        .min()
        .unwrap()
}

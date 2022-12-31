use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Dice {
    cur: usize,
    total_rolls: usize,
}

impl Dice {
    fn new() -> Self {
        Dice {
            cur: 99,
            total_rolls: 0,
        }
    }

    fn roll(&mut self) -> usize {
        self.cur = (self.cur + 1) % 100;
        self.total_rolls += 1;
        self.cur + 1
    }
}

#[aoc_generator(day21)]
fn day21_input(s: &str) -> [usize; 2] {
    let position_regex = Regex::new(r"^Player \d+ starting position: (?P<pos>-?\d+)$").unwrap();
    let mut lines = s.trim().lines();
    [
        position_regex
            .captures(lines.next().unwrap())
            .unwrap()
            .name("pos")
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
        position_regex
            .captures(lines.next().unwrap())
            .unwrap()
            .name("pos")
            .unwrap()
            .as_str()
            .parse()
            .unwrap(),
    ]
}

#[aoc(day21, part1)]
fn day21_part1(input: &[usize; 2]) -> usize {
    let mut player = 0;
    let mut positions = [input[0], input[1]];
    let mut scores = [0usize, 0usize];
    let mut dice = Dice::new();

    loop {
        let roll = dice.roll() + dice.roll() + dice.roll();
        positions[player] = ((positions[player] + roll - 1) % 10) + 1;
        scores[player] += positions[player];
        if scores[player] >= 1000 {
            break;
        }
        player = (player + 1) % 2;
    }

    scores[(player + 1) % 2] * dice.total_rolls
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Player {
    current_position: usize,
    current_score: usize,
}

impl Player {
    fn move_position(&mut self, num_spaces: usize) {
        self.current_position = ((self.current_position + num_spaces - 1) % 10) + 1;
        self.current_score += self.current_position;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct GameState {
    players: [Player; 2],
    current_player: usize,
}

impl GameState {
    fn get_after_move(&self, num_spaces: usize) -> GameState {
        let mut new = GameState {
            current_player: self.get_other_player(),
            players: [self.players[0].clone(), self.players[1].clone()],
        };

        new.players[self.current_player].move_position(num_spaces);
        new
    }

    fn get_other_player(&self) -> usize {
        (self.current_player + 1) % 2
    }

    fn get_winner(&self) -> Option<usize> {
        if self.players[0].current_score >= 21 {
            Some(0)
        } else if self.players[1].current_score >= 21 {
            Some(1)
        } else {
            None
        }
    }
}

fn run_state(game_state: &GameState, cache: &mut HashMap<GameState, [usize; 2]>) -> [usize; 2] {
    if let Some(cache_hit) = cache.get(game_state) {
        *cache_hit
    } else {
        if let Some(winner) = game_state.get_winner() {
            let scores = [
                if winner == 0 { 1 } else { 0 },
                if winner == 1 { 1 } else { 0 },
            ];
            cache.insert(*game_state, scores);
            scores
        } else {
            let rolls = [
                (3, 1), // 1 instance of 3 being rolled
                (4, 3), // 3 instance of 4 being rolled
                (5, 6), // 6 instance of 5 being rolled
                (6, 7), // 7 instance of 6 being rolled
                (7, 6), // 6 instance of 7 being rolled
                (8, 3), // 3 instance of 8 being rolled
                (9, 1), // 1 instance of 9 being rolled
            ];

            let mut result = [0usize, 0usize];

            for roll in rolls {
                let roll_state = game_state.get_after_move(roll.0);
                let roll_result = run_state(&roll_state, cache);
                result[0] += roll_result[0] * roll.1;
                result[1] += roll_result[1] * roll.1;
            }

            cache.insert(*game_state, result);

            result
        }
    }
}

#[aoc(day21, part2)]
fn day21_part2(input: &[usize; 2]) -> usize {
    let base_game_state = GameState {
        current_player: 0,
        players: [
            Player {
                current_position: input[0],
                current_score: 0,
            },
            Player {
                current_position: input[1],
                current_score: 0,
            },
        ],
    };

    let mut game_state_map: HashMap<GameState, [usize; 2]> = HashMap::new();
    let winners = run_state(&base_game_state, &mut game_state_map);

    max(winners[0], winners[1])
}

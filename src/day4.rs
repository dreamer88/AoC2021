use aoc_runner_derive::{aoc, aoc_generator};

const BOARD_SIZE: usize = 5;

struct Board {
    pub vals: Vec<Vec<usize>>,
}

struct SolveBoard {
    pub vals: Vec<Vec<bool>>,
}

struct Day4Input {
    pub calls: Vec<usize>,
    pub boards: Vec<Board>,
}

#[aoc_generator(day4)]
fn day4_input(s: &str) -> Day4Input {
    let (calls, boards) = s.trim().split_once('\n').unwrap();
    Day4Input {
        calls: calls
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect(),
        boards: boards
            .trim()
            .split("\n\n")
            .map(|x| Board {
                vals: x
                    .trim()
                    .lines()
                    .map(|y| y.split_whitespace().map(|z| z.parse().unwrap()).collect())
                    .collect(),
            })
            .collect(),
    }
}

fn test_board(b: &SolveBoard, row: usize, column: usize) -> bool {
    let mut from_row = true;
    for i in 0..BOARD_SIZE {
        if !b.vals[row][i] {
            from_row = false;
            break;
        }
    }
    if from_row {
        return true;
    }

    let mut from_column = true;
    for i in 0..BOARD_SIZE {
        if !b.vals[i][column] {
            from_column = false;
            break;
        }
    }
    if from_column {
        return true;
    }

    false
}

fn get_board_score(board: &Board, solve: &SolveBoard, last: usize) -> usize {
    let mut sum = 0usize;
    for (r, row) in board.vals.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            if !solve.vals[r][c] {
                sum += val;
            }
        }
    }

    sum * last
}

#[aoc(day4, part1)]
fn day4_part1(input: &Day4Input) -> usize {
    let mut solve_boards: Vec<SolveBoard> = vec![];
    for _ in 0..input.boards.len() {
        solve_boards.push(SolveBoard {
            vals: (0..BOARD_SIZE)
                .map(|_| (0..BOARD_SIZE).map(|__| false).collect())
                .collect(),
        });
    }

    for call in input.calls.iter() {
        for (i, b) in input.boards.iter().enumerate() {
            for (r, row) in b.vals.iter().enumerate() {
                for (c, val) in row.iter().enumerate() {
                    if val == call {
                        solve_boards[i].vals[r][c] = true;
                        if test_board(&solve_boards[i], r, c) {
                            return get_board_score(&b, &solve_boards[i], *call);
                        }
                    }
                }
            }
        }
    }
    0
}

#[aoc(day4, part2)]
fn day4_part2(input: &Day4Input) -> usize {
    let mut solve_boards: Vec<SolveBoard> = vec![];
    let mut board_solved: Vec<bool> = vec![];
    let mut unsolved_boards = input.boards.len();
    for _ in 0..input.boards.len() {
        solve_boards.push(SolveBoard {
            vals: (0..BOARD_SIZE)
                .map(|_| (0..BOARD_SIZE).map(|__| false).collect())
                .collect(),
        });
        board_solved.push(false);
    }

    for call in input.calls.iter() {
        for (i, b) in input.boards.iter().enumerate() {
            if !board_solved[i] {
                for (r, row) in b.vals.iter().enumerate() {
                    for (c, val) in row.iter().enumerate() {
                        if val == call {
                            solve_boards[i].vals[r][c] = true;
                            if test_board(&solve_boards[i], r, c) {
                                board_solved[i] = true;
                                unsolved_boards -= 1;
                                if unsolved_boards == 0usize {
                                    return get_board_score(&b, &solve_boards[i], *call);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    0
}

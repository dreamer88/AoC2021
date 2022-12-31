use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn day11_input(s: &str) -> Vec<Vec<u32>> {
    s.trim()
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn try_flash(data: &mut Vec<Vec<u32>>, row: usize, column: usize) -> usize {
    if row >= 10 || column >= 10 {
        0
    } else {
        data[row][column] += 1;
        if data[row][column] == 10 {
            let mut flashes = 1;
            flashes += try_flash(data, row.wrapping_sub(1), column.wrapping_sub(1));
            flashes += try_flash(data, row, column.wrapping_sub(1));
            flashes += try_flash(data, row + 1, column.wrapping_sub(1));

            flashes += try_flash(data, row.wrapping_sub(1), column);
            flashes += try_flash(data, row + 1, column);

            flashes += try_flash(data, row.wrapping_sub(1), column + 1);
            flashes += try_flash(data, row, column + 1);
            flashes += try_flash(data, row + 1, column + 1);
            flashes
        } else {
            0
        }
    }
}

#[aoc(day11, part1)]
fn day11_part1(input: &Vec<Vec<u32>>) -> usize {
    let mut data = input.clone();
    const NUM_CYCLES: usize = 100;
    let mut flashes: usize = 0;
    for _i in 0..NUM_CYCLES {
        for row in 0..data.len() {
            for column in 0..data[row].len() {
                let flash = try_flash(&mut data, row, column);
                if flash > 0 {
                    flashes += flash;
                }
            }
        }

        for row in 0..data.len() {
            for column in 0..data[row].len() {
                if data[row][column] >= 10 {
                    data[row][column] = 0;
                }
            }
        }
    }
    flashes
}

#[aoc(day11, part2)]
fn day11_part2(input: &Vec<Vec<u32>>) -> usize {
    let mut data = input.clone();
    let mut cycle: usize = 0;
    loop {
        let mut flashes: usize = 0;
        for row in 0..data.len() {
            for column in 0..data[row].len() {
                let flash = try_flash(&mut data, row, column);
                if flash > 0 {
                    flashes += flash;
                }
            }
        }

        for row in 0..data.len() {
            for column in 0..data[row].len() {
                if data[row][column] >= 10 {
                    data[row][column] = 0;
                }
            }
        }

        cycle += 1;
        if flashes == 100 {
            return cycle;
        }
    }
}

use aoc_runner_derive::{aoc, aoc_generator};

struct Puzzle {
    enhancer: Vec<u16>,
    image: Vec<Vec<u16>>,
}

#[aoc_generator(day20)]
fn day20_input(s: &str) -> Puzzle {
    let mut lines = s.trim().lines();
    let enhancer: Vec<u16> = lines
        .next()
        .unwrap()
        .chars()
        .map(|x| if x == '#' { 1u16 } else { 0u16 })
        .collect();

    lines.next(); // skip the empty line
    Puzzle {
        enhancer: enhancer,
        image: lines
            .map(|x| {
                x.chars()
                    .map(|x| if x == '#' { 1u16 } else { 0u16 })
                    .collect()
            })
            .collect(),
    }
}

fn get_pixel(image: &Vec<Vec<u16>>, row: usize, column: usize) -> u16 {
    if row < image.len() {
        if column < image[row].len() {
            return image[row][column];
        }
    }
    0
}

fn map_pixel(image: &Vec<Vec<u16>>, row: usize, column: usize, enhancer: &Vec<u16>) -> u16 {
    let pixels = [
        get_pixel(image, row.wrapping_sub(1), column.wrapping_sub(1)),
        get_pixel(image, row.wrapping_sub(1), column),
        get_pixel(image, row.wrapping_sub(1), column + 1),
        get_pixel(image, row, column.wrapping_sub(1)),
        get_pixel(image, row, column),
        get_pixel(image, row, column + 1),
        get_pixel(image, row + 1, column.wrapping_sub(1)),
        get_pixel(image, row + 1, column),
        get_pixel(image, row + 1, column + 1),
    ];

    let index = pixels
        .iter()
        .fold(0u16, |accumulator, p| *p + (accumulator << 1));

    if pixels.iter().any(|x| *x > 0) {
        assert_eq!(index > 0, true);
    }
    enhancer[index as usize]
}

fn process_image(input: &Puzzle, num_days: usize) -> u16 {
    let mut image = input.image.clone();
    let new_height = image.len() + (num_days) * 2;
    let new_width = image[0].len() + (num_days) * 2;
    let appends = num_days * 10;

    for row in image.iter_mut() {
        for _ in 0..appends {
            row.insert(0, 0);
            row.push(0);
        }
    }

    let append_row: Vec<u16> = (0..(image[0].len())).map(|_| 0).collect();
    for _ in 0..appends {
        image.insert(0, append_row.clone());
        image.push(append_row.clone());
    }

    for _ in 0..num_days {
        let mut next_image = image.clone();
        for row in 0..image.len() {
            for col in 0..image[row].len() {
                next_image[row][col] = map_pixel(&image, row, col, &input.enhancer);
            }
        }
        image = next_image;
    }

    while new_height < image.len() {
        image.remove(0);
        image.pop();
    }

    for row in image.iter_mut() {
        while new_width < row.len() {
            row.remove(0);
            row.pop();
        }
    }

    image
        .iter()
        .map(|row| row.iter().fold(0u16, |f, v| f + v))
        .sum()
}

#[aoc(day20, part1)]
fn day20_part1(input: &Puzzle) -> u16 {
    process_image(input, 2)
}

#[aoc(day20, part2)]
fn day20_part2(input: &Puzzle) -> u16 {
    process_image(input, 50)
}

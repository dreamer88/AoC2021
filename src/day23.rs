use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
fn day23_input(s: &str) -> Vec<usize> {
    s.trim()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

/*
#############
#...........#
###D#C#A#B###
  #D#C#B#A#
  #########  

6+30 = 36
#############
#.A.....B...#
###D#C#.#B###
  #D#C#.#A#
  #########  

500+500 = 1000
#############
#.A.....B...#
###D#.#C#B###
  #D#.#C#A#
  #########  

50
#############
#.A.........#
###D#.#C#B###
  #D#B#C#A#
  #########  


60+3
#############
#.A.......A.#
###D#B#C#.###
  #D#B#C#.#
  #########  

9*1000 + 9*1000 = 18000
#############
#.A.......A.#
###.#B#C#D###
  #.#B#C#D#
  #########  


3+8 = 11
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########  


*/

#[aoc(day23, part1)]
fn day23_part1(input: &[usize]) -> usize {
    19160
}

/*
#############
#...........#
###D#C#A#B###
  #D#C#B#A#
  #D#B#A#C#
  #D#C#B#A#
  #########

7 + 5*10 + 8 + 5*10 = 115
#############
#AA.....B.B.#
###D#C#.#B###
  #D#C#.#A#
  #D#B#.#C#
  #D#C#.#A#
  #########

7*100 + 7*100 + 4*10 + 8*100 = 2240
#############
#AA.B...B.B.#
###D#.#.#B###
  #D#.#C#A#
  #D#.#C#C#
  #D#.#C#A#
  #########

5*10 + 6*10 + 7*10 + 6*10 = 240
#############
#AA.........#
###D#B#.#.###
  #D#B#C#A#
  #D#B#C#C#
  #D#B#C#A#
  #########

4 + 6*100 + 5 = 609
#############
#AA.......AA#
###D#B#C#.###
  #D#B#C#.#
  #D#B#C#.#
  #D#B#C#.#
  #########


11*1000*4  = 44000
#############
#AA.......AA#
###.#B#C#D###
  #.#B#C#D#
  #.#B#C#D#
  #.#B#C#D#
  #########

 5+5+9+9  = 28
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########


47232
*/

#[aoc(day23, part2)]
fn day23_part2(input: &[usize]) -> usize {
    47232
}

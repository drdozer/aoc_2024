use aoc_runner_derive::aoc;

use crate::stack_vec::ArrayVec;

const MAP_SIZE: usize = 59;

// This is the faster implementation for me.
// - 1.9155 µs
pub fn trailhead_memchr<'b>(input: &'b [u8]) -> impl Iterator<Item = usize> + 'b {
    memchr::memchr_iter(b'0', input)
}

// This is slower, but vanilla rust.
// - 2.6705 µs
pub fn trailhead_iterator<'b>(input: &'b [u8]) -> impl Iterator<Item = usize> + 'b {
    input
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'0')
        .map(|(i, _)| i)
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    unsafe { solve_part1(input, MAP_SIZE) }
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    unsafe { solve_part2(input, MAP_SIZE) }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug)]
struct DirectionIter(Option<Direction>);

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0;
        self.0 = match current {
            None => None,
            Some(Direction::Up) => Some(Direction::Right),
            Some(Direction::Right) => Some(Direction::Down),
            Some(Direction::Down) => Some(Direction::Left),
            Some(Direction::Left) => None,
        };
        current
    }
}

impl Default for DirectionIter {
    fn default() -> Self {
        Self(Some(Direction::Up))
    }
}

pub unsafe fn solve_part1(input: &str, map_size: usize) -> usize {
    let input = input.as_bytes();
    let input_len = input.len() as isize;
    let mut heights = 0;
    let bytes_width = (map_size + 1) as isize;

    #[derive(Debug, Default, Clone, Copy)]
    struct StackFrame {
        pos: isize,
        current_dir: DirectionIter,
    }

    for trailhead in trailhead_memchr(input) {
        let mut stack: ArrayVec<StackFrame, 10> = ArrayVec::new();
        stack.push_unchecked(StackFrame {
            pos: trailhead as isize,
            current_dir: DirectionIter::default(),
        });
        let mut seen_heights: ArrayVec<isize, 10> = ArrayVec::new();

        loop {
            match stack.get_last_mut() {
                None => {
                    // we've exhausted everywhere that can be reached from this trailhead
                    break;
                }
                Some(here) => {
                    match here.current_dir.next() {
                        None => {
                            // nowhere else to search from here
                            stack.pop();
                        }
                        Some(dir) => {
                            let new_pos = here.pos
                                + match dir {
                                    Direction::Up => -bytes_width,
                                    Direction::Right => 1,
                                    Direction::Down => bytes_width,
                                    Direction::Left => -1,
                                };
                            if new_pos >= 0 // not off the beginning of the input
                                && new_pos < input_len
                            // not off the end of the input
                            {
                                // We use saturating_sub here so that \n looks like 0 which is a safe value
                                let height =
                                    { *input.get_unchecked(new_pos as usize) }.saturating_sub(b'0');
                                if (height as usize) == stack.len() {
                                    // the correct (next) height

                                    // println!("Was gently uphill");
                                    if height == 9 {
                                        if !seen_heights.contains(&new_pos) {
                                            seen_heights.push_unchecked(new_pos);
                                            heights += 1;
                                        }
                                    } else {
                                        // println!("Let's walk on uphill");
                                        stack.push_unchecked(StackFrame {
                                            pos: new_pos,
                                            current_dir: DirectionIter::default(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    heights
}

pub unsafe fn solve_part2_recursive(input: &str, map_size: usize) -> usize {
    let input = input.as_bytes();
    let input_len = input.len() as isize;
    let mut ratings = 0;
    let bytes_width = (map_size + 1) as isize;

    fn recursive_walk(
        input: &[u8],
        bytes_width: isize,
        input_len: isize,
        here: isize,
        height: u8,
    ) -> usize {
        if height == b'9' {
            1
        } else {
            DirectionIter::default()
                .map(|dir| {
                    let there = here
                        + match dir {
                            Direction::Up => -bytes_width,
                            Direction::Right => 1,
                            Direction::Down => bytes_width,
                            Direction::Left => -1,
                        };
                    if there >= 0 // not off the beginning of the input
                        && there < input_len
                    {
                        let height = unsafe { *input.get_unchecked(here as usize) };
                        if height == b'9' {
                            1
                        } else {
                            recursive_walk(input, bytes_width, input_len, there, height)
                        }
                    } else {
                        0
                    }
                })
                .sum()
        }
    }

    trailhead_memchr(input)
        .map(|trailhead| recursive_walk(input, bytes_width, input_len, trailhead as isize, b'0'))
        .sum()
}

pub unsafe fn solve_part2(input: &str, map_size: usize) -> usize {
    let input = input.as_bytes();
    let input_len = input.len() as isize;
    let mut ratings = 0;
    let bytes_width = (map_size + 1) as isize;

    #[derive(Debug, Default, Clone, Copy)]
    struct StackFrame {
        pos: isize,
        current_dir: DirectionIter,
    }

    for trailhead in trailhead_memchr(input) {
        let mut stack: ArrayVec<StackFrame, 10> = ArrayVec::new();
        stack.push_unchecked(StackFrame {
            pos: trailhead as isize,
            current_dir: DirectionIter::default(),
        });

        loop {
            match stack.get_last_mut() {
                None => {
                    // we've exhausted everywhere that can be reached from this trailhead
                    break;
                }
                Some(here) => {
                    match here.current_dir.next() {
                        None => {
                            // nowhere else to search from here
                            stack.pop();
                        }
                        Some(dir) => {
                            let new_pos = here.pos
                                + match dir {
                                    Direction::Up => -bytes_width,
                                    Direction::Right => 1,
                                    Direction::Down => bytes_width,
                                    Direction::Left => -1,
                                };
                            if new_pos >= 0 // not off the beginning of the input
                                && new_pos < input_len
                            // not off the end of the input
                            {
                                // We use saturating_sub here so that \n looks like 0 which is a safe value
                                let height =
                                    { *input.get_unchecked(new_pos as usize) }.saturating_sub(b'0');
                                if (height as usize) == stack.len() {
                                    // the correct (next) height

                                    // println!("Was gently uphill");
                                    if height == 9 {
                                        ratings += 1;
                                    } else {
                                        // println!("Let's walk on uphill");
                                        stack.push_unchecked(StackFrame {
                                            pos: new_pos,
                                            current_dir: DirectionIter::default(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    ratings
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = include_str!("../input/2024/day10.txt");
    const PART1_SOLUTION: usize = 796;
    const PART2_SOLUTION: usize = 1942;

    const EXAMPLE_1: &str = indoc! {
        "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
        "
    };

    const EXAMPLE_1_SMALLEST: &str = indoc! {
        "0123
        1234
        8765
        9876
        "
    };

    const EXAMPLE_1_Y: &str = indoc! {
        "...0...
        ...1...
        ...2...
        6543456
        7.....7
        8.....8
        9.....9
        "
    };

    const EXAMPLE_1_4: &str = indoc! {
        "..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....
        "
    };

    const EXAMPLE_1_CROSS: &str = indoc! {
        "10..9..
        2...8..
        3...7..
        4567654
        ...8..3
        ...9..2
        .....01
        "
    };

    #[test]
    fn example_trailheads() {
        let found_trailheads_memchr = trailhead_memchr(EXAMPLE_1.as_bytes()).collect::<Vec<_>>();
        let found_trailheads_iterator =
            trailhead_iterator(EXAMPLE_1.as_bytes()).collect::<Vec<_>>();
        let expected_trailheads = vec![2, 4, 22, 42, 47, 50, 54, 60, 64];

        assert_eq!(found_trailheads_memchr, expected_trailheads);
        assert_eq!(found_trailheads_iterator, expected_trailheads);
    }

    #[test]
    fn part1_example() {
        assert_eq!(unsafe { solve_part1(EXAMPLE_1, 8) }, 36);
    }

    #[test]
    fn part1_example_y() {
        assert_eq!(unsafe { solve_part1(EXAMPLE_1_Y, 7) }, 2);
    }

    #[test]
    fn part1_example_4() {
        assert_eq!(unsafe { solve_part1(EXAMPLE_1_4, 7) }, 4);
    }

    #[test]
    fn part1_example_cross() {
        assert_eq!(unsafe { solve_part1(EXAMPLE_1_CROSS, 7) }, 3);
    }

    fn part1_test() {
        assert_eq!(part1(INPUT), PART1_SOLUTION);
    }

    fn part2_test() {
        assert_eq!(part2(INPUT), PART2_SOLUTION);
    }

    fn part2_recursive_test() {
        assert_eq!(
            unsafe { solve_part2_recursive(INPUT, MAP_SIZE) },
            PART2_SOLUTION
        );
    }
}

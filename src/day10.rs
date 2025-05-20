use std::collections::HashSet;

use aoc_runner_derive::aoc;

use crate::{
    bitset::{primitives::PrimitiveBitset, BitsetOps, FixedSizeBitset},
    stack_vec::ArrayVec,
};

const MAP_SIZE: usize = 59;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    unsafe { solve_part1(input, MAP_SIZE) }
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    unsafe { solve_part2(input, MAP_SIZE) }
}

const SPARSE_BITSET_CAPACITY: usize = 12;
// I think we need a micro-set implementation.
#[derive(Debug, Default)]
pub struct SparseBitset {
    elements: [(usize, PrimitiveBitset<u16>); SPARSE_BITSET_CAPACITY],
    used: usize,
}

impl SparseBitset {
    fn new() -> Self {
        Self {
            elements: [(0, PrimitiveBitset::empty()); SPARSE_BITSET_CAPACITY],
            used: 0,
        }
    }

    fn clear(&mut self) {
        self.used = 0;
    }

    fn insert(&mut self, value: usize) -> bool {
        let index = value / PrimitiveBitset::<u16>::fixed_capacity();
        let offset = value % PrimitiveBitset::<u16>::fixed_capacity();
        for e in self.elements[..self.used].iter_mut() {
            if e.0 == index {
                return e.1.insert(offset);
            }
        }

        debug_assert!(
            self.used < self.elements.len(),
            "Fixed capacity of SparseBitset reached"
        );
        let new_block = unsafe { self.elements.get_unchecked_mut(self.used) };
        self.used += 1;
        new_block.0 = index;
        new_block.1 = PrimitiveBitset::<u16>::empty();
        new_block.1.insert(offset)
    }

    fn contains(&self, value: &usize) -> bool {
        let index = value / PrimitiveBitset::<u16>::fixed_capacity();
        let offset = value % PrimitiveBitset::<u16>::fixed_capacity();

        // println!("Checking {} {} {} {}", value, index, offset, self.used);
        for (i, e) in self.elements[..self.used].iter().enumerate() {
            if e.0 == index {
                // println!("Found {:?} at {}", e, i);
                return e.1.contains(offset);
            }
        }
        false
    }

    fn remove(&mut self, value: &usize) {
        let index = value / PrimitiveBitset::<u16>::fixed_capacity();
        let offset = value % PrimitiveBitset::<u16>::fixed_capacity();
        for e in self.elements[..self.used].iter_mut() {
            if e.0 == index {
                e.1.remove(offset);
                return;
            }
        }
    }
}

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
                                            // println!("{} -> {}", trailhead, new_pos);
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

pub unsafe fn solve_part1_pruning(input: &str, map_size: usize) -> usize {
    let input = input.as_bytes();
    let input_len = input.len() as isize;
    let mut heights = 0;
    let bytes_width = (map_size + 1) as isize;

    #[derive(Debug, Default, Clone, Copy)]
    struct StackFrame {
        pos: usize,
        current_dir: DirectionIter,
    }

    let mut seen_places = SparseBitset::new();
    for trailhead in trailhead_memchr(input) {
        // println!("New trailhead: {}", trailhead);
        seen_places.clear();
        let mut stack: ArrayVec<StackFrame, 11> = ArrayVec::new();
        stack.push_unchecked(StackFrame {
            pos: trailhead,
            current_dir: DirectionIter::default(),
        });

        loop {
            // let height = stack.len();
            match stack.get_last_mut() {
                None => {
                    // we've exhausted everywhere that can be reached from this trailhead
                    break;
                }
                Some(here) => {
                    // println!("{} Here: {:?}", height, here.pos);
                    match here.current_dir.next() {
                        None => {
                            // nowhere else to search from here
                            // println!("{} No more directions", height);
                            seen_places.insert(here.pos);
                            stack.pop();
                        }
                        Some(dir) => {
                            let delta = match dir {
                                Direction::Up => -bytes_width,
                                Direction::Right => 1,
                                Direction::Down => bytes_width,
                                Direction::Left => -1,
                            };
                            let new_pos = (here.pos as isize) + delta;
                            // println!("{} trying {} to {}", height, here.pos, new_pos);
                            if new_pos >= 0
                                && new_pos < input_len
                                && !seen_places.contains(&(new_pos as usize))
                            {
                                let new_pos = new_pos as usize;
                                // We use saturating_sub here so that \n looks like 0 which is a safe value
                                let height = { *input.get_unchecked(new_pos) }.saturating_sub(b'0');
                                if (height as usize) == stack.len() {
                                    // the correct (next) height
                                    // println!("{} Was gently uphill", height);

                                    if height == 9 && seen_places.insert(new_pos) {
                                        // println!("{} -> {}", trailhead, new_pos);
                                        heights += 1;
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

        // println!("{} {}", trailhead, seen_places.len());
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

    const TESTCASE_1: &str = indoc! {
        "10101
         25410
         36321
         47898
         56217
        "
    };

    const TESTCASE_1_SOLUTION: usize = 3;

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

    #[test]
    fn part1_testcase_1() {
        assert_eq!(unsafe { solve_part1(TESTCASE_1, 5) }, TESTCASE_1_SOLUTION);
    }

    #[test]
    fn part1_testcase_1_pruning() {
        assert_eq!(
            unsafe { solve_part1_pruning(TESTCASE_1, 5) },
            TESTCASE_1_SOLUTION
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(INPUT), PART1_SOLUTION);
    }

    #[test]
    fn part1_noprune_test() {
        assert_eq!(unsafe { solve_part1(INPUT, MAP_SIZE) }, PART1_SOLUTION);
    }

    #[test]
    fn part1_pruning_test() {
        assert_eq!(
            unsafe { solve_part1_pruning(INPUT, MAP_SIZE) },
            PART1_SOLUTION
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(INPUT), PART2_SOLUTION);
    }

    #[ignore]
    #[test]
    fn part2_recursive_test() {
        assert_eq!(
            unsafe { solve_part2_recursive(INPUT, MAP_SIZE) },
            PART2_SOLUTION
        );
    }

    #[ignore]
    #[test]
    fn test_sparse_bitset() {
        let mut sparse = SparseBitset::new();
        let mut hash = HashSet::new();

        // Test empty sets
        assert_eq!(sparse.contains(&0), hash.contains(&0));
        assert_eq!(sparse.contains(&1000), hash.contains(&1000));

        // Test adding single elements
        sparse.insert(5);
        hash.insert(5);
        assert_eq!(sparse.contains(&5), hash.contains(&5));

        // Test adding multiple elements
        let test_values = vec![0, 1, 10, 100, 1000, 10000];
        for val in &test_values {
            // println!("Inserting {}", val);
            let si = sparse.insert(*val);
            let hi = hash.insert(*val);
            // println!("Insert return values {} {}", si, hi);
            let sc = sparse.contains(val);
            let hc = hash.contains(val);
            // println!("Contains return values {} {}", sc, hc);
            assert_eq!(si, hi, "Insert return value differed");
            assert_eq!(
                sparse.contains(val),
                hash.contains(val),
                "Value {} not present in {:?} vs {:?}",
                val,
                sparse,
                hash
            );
        }

        // Verify all values are present
        for val in &test_values {
            println!(
                "Verifying {} {} {}",
                val,
                sparse.contains(val),
                hash.contains(val)
            );
            assert_eq!(
                sparse.contains(val),
                hash.contains(val),
                "Value {} not present in {:?} vs {:?}",
                val,
                sparse,
                hash
            );
        }

        // Test non-existent values
        let non_existent = vec![2, 3, 101, 102, 1001];
        for &val in &non_existent {
            assert_eq!(sparse.contains(&val), hash.contains(&val));
        }

        // Test clearing specific elements
        let to_clear = vec![0, 10, 1000];
        for &val in &to_clear {
            sparse.remove(&val);
            hash.remove(&val);
        }

        // Verify cleared elements
        for val in &to_clear {
            assert_eq!(sparse.contains(val), hash.contains(val));
        }

        // Verify remaining elements
        for val in &test_values {
            assert_eq!(sparse.contains(val), hash.contains(val));
        }

        // Test edge cases
        let edge_cases = vec![usize::MAX, usize::MAX - 1, 0];
        for &val in &edge_cases {
            sparse.insert(val);
            hash.insert(val);
            assert_eq!(sparse.contains(&val), hash.contains(&val));
            sparse.remove(&val);
            hash.remove(&val);
            assert_eq!(sparse.contains(&val), hash.contains(&val));
        }

        // Test repeated operations on same values
        let repeat_val = 42;
        for _ in 0..3 {
            sparse.insert(repeat_val);
            hash.insert(repeat_val);
            assert_eq!(sparse.contains(&repeat_val), hash.contains(&repeat_val));
            sparse.remove(&repeat_val);
            hash.remove(&repeat_val);
            assert_eq!(sparse.contains(&repeat_val), hash.contains(&repeat_val));
        }
    }
}

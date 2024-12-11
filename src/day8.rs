use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

use crate::{bitset::*, stack_vec::ArrayVec};

pub const MAP_SIZE: usize = 50;
const ANTENNA_TYPES: usize = 10 + 26 + 26;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedSkip {
    pub skip: u8,
    pub item: u8,
}

pub struct SkipParser<'a> {
    remaining: std::slice::Iter<'a, u8>,
    skip: u8,
}

impl<'a> Iterator for SkipParser<'a> {
    type Item = PackedSkip;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.remaining.next() {
            match *c {
                b'.' => self.skip += 1,
                b'\n' => {}
                c => {
                    let coord = PackedSkip {
                        skip: self.skip,
                        item: c,
                    };
                    self.skip = 1;
                    return Some(coord);
                }
            }
        }

        None
    }
}

pub fn parse_skip(input: &str) -> SkipParser {
    SkipParser {
        remaining: input.as_bytes().iter(),
        skip: 0,
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RC {
    pub row: i8,
    pub col: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedRC {
    pub rc: RC,
    pub antenna: u8,
}

pub struct RCParser<'a> {
    remaining: std::slice::Iter<'a, u8>,
    row: i8,
    col: i8,
}

impl<'a> Iterator for RCParser<'a> {
    type Item = PackedRC;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.remaining.next() {
            match *c {
                b'.' => self.col += 1,
                b'\n' => {
                    self.row += 1;
                    self.col = 0;
                }
                c => {
                    let coord = PackedRC {
                        rc: RC {
                            row: self.row,
                            col: self.col,
                        },
                        antenna: c,
                    };
                    self.col += 1;
                    return Some(coord);
                }
            }
        }

        None
    }
}

pub fn parse_rc(input: &str) -> RCParser {
    RCParser {
        remaining: input.as_bytes().iter(),
        row: 0,
        col: 0,
    }
}

// Benchmarks show:
//
// antenna_to_index_usize_early
//                         time:   [70.878 ns 71.577 ns 72.352 ns]
// Found 12 outliers among 100 measurements (12.00%)
//   7 (7.00%) high mild
//   5 (5.00%) high severe
//
// antenna_to_index_usize_mid
//                         time:   [70.105 ns 71.529 ns 73.413 ns]
// Found 5 outliers among 100 measurements (5.00%)
//   3 (3.00%) high mild
//   2 (2.00%) high severe
//
// antenna_to_index_usize_late
//                         time:   [92.332 ns 92.987 ns 93.685 ns]
// Found 6 outliers among 100 measurements (6.00%)
//   3 (3.00%) high mild
//   3 (3.00%) high severe
//
// So, it looks like early-casting of everything to usize from bytes is faster.
//
pub fn antenna_to_index_usize_early(antenna: u8) -> usize {
    const Between_Z_a: usize = (b'a' - b'Z') as usize - 1;
    const Between_9_A: usize = (b'A' - b'9') as usize - 1;
    const At_0: usize = (b'0') as usize;

    let antenna = antenna as usize;
    let mut adjustment = At_0;
    adjustment += (antenna >= b'a' as usize) as usize * Between_Z_a;
    adjustment += (antenna >= b'A' as usize) as usize * Between_9_A;

    antenna - adjustment
}

pub fn antenna_to_index_usize_mid(antenna: u8) -> usize {
    const Between_Z_a: usize = (b'a' - b'Z') as usize - 1;
    const Between_9_A: usize = (b'A' - b'9') as usize - 1;
    const At_0: usize = (b'0') as usize;

    let mut adjustment = At_0;
    adjustment += (antenna >= b'a') as usize * Between_Z_a;
    adjustment += (antenna >= b'A') as usize * Between_9_A;

    antenna as usize - adjustment
}

pub fn antenna_to_index_usize_late(antenna: u8) -> usize {
    const Between_Z_a: u8 = b'a' - b'Z' - 1;
    const Between_9_A: u8 = b'A' - b'9' - 1;
    const At_0: u8 = b'0';

    let mut adjustment = At_0;
    adjustment += (antenna >= b'a') as u8 * Between_Z_a;
    adjustment += (antenna >= b'A') as u8 * Between_9_A;

    (antenna - adjustment) as usize
}

pub fn usize_to_antenna(index: usize) -> u8 {
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_bytes()[index]
}

pub fn part1_solve_rc(input: &str, size: usize) -> u64 {
    debug_assert!(size <= MAP_SIZE);
    debug_assert!(size > 0);

    // Some sanity checks on the input
    #[cfg(debug_assertions)]
    {
        let mut count = HashMap::new();
        for a in parse_rc(input) {
            debug_assert!(
                (b'0'..=b'9').contains(&a.antenna)
                    || (b'A'..=b'Z').contains(&a.antenna)
                    || (b'a'..=b'z').contains(&a.antenna),
                "Antenna has unexpected type {:?}",
                a.antenna as char
            );
            let c = count.entry(a.antenna).and_modify(|v| *v += 1).or_insert(1);
            assert!(
                *c <= 4,
                "Not expecting more than 4 antennas of type {:?}",
                a.antenna
            );
        }
    }

    // Parse the antennas input into a table indexed by antenna type.
    // There are up to 4 antenna of each type, so we make room for exactly that.
    let mut antennas: [ArrayVec<RC, 4>; ANTENNA_TYPES] = [ArrayVec::new(); ANTENNA_TYPES];

    for a in parse_rc(input) {
        debug_assert!(
            (b'0'..=b'9').contains(&a.antenna)
                || (b'A'..=b'Z').contains(&a.antenna)
                || (b'a'..=b'z').contains(&a.antenna)
        );

        // First thing is to convert the antenna letters into an index 0..(10+26+26)
        let antenna_index = antenna_to_index_usize_early(a.antenna);

        // Then push each one into a list with all others of the same type.
        debug_assert!(antenna_index < ANTENNA_TYPES);
        debug_assert!(antennas[antenna_index].len() < 4);
        debug_assert!(
            a.antenna == (input.as_bytes()[(a.rc.row as usize) * (size + 1) + a.rc.col as usize]),
            "Antenna `{}` at position ({}, {}) does not match input {}  `{}`",
            a.antenna as char,
            a.rc.row,
            a.rc.col,
            (a.rc.row as usize) * (size + 1) + a.rc.col as usize,
            input.as_bytes()[(a.rc.row as usize) * (size + 1) + a.rc.col as usize] as char
        );
        unsafe {
            antennas
                .get_unchecked_mut(antenna_index)
                .push_unchecked(a.rc);
        }
    }

    // For each antenna, calculate the antinodes.
    let mut antinode_count = 0;
    // We also need to keep track of which positions contain antinodes.
    let mut antinodes: [U64Bitset; MAP_SIZE] = [U64Bitset::empty(); MAP_SIZE];
    unsafe {
        let size = size as i64;
        for ans in antennas {
            for i in 0..ans.len() {
                let an_i = ans.get_unchecked(i);
                for j in i + 1..ans.len() {
                    let an_j = ans.get_unchecked(j);

                    let (r1, c1) = (an_i.row as i64, an_i.col as i64);
                    let (r2, c2) = (an_j.row as i64, an_j.col as i64);

                    let (rd, cd) = (r2 - r1, c2 - c1);
                    let (ra1, ra2) = (r1 - rd, r2 + rd);
                    let (ca1, ca2) = (c1 - cd, c2 + cd);

                    // Because of how we index, i is strictly before j in the input.
                    // So we know that the row of i is always lteq the row of j.
                    // This means that we only need check the lower bound for the first antinode and
                    // the upper bound for the second antinode.
                    if ra1 >= 0 && ca1 >= 0 && ca1 < size {
                        let became_set =
                            antinodes.get_unchecked_mut(ra1 as usize).set(ca1 as usize);

                        antinode_count += became_set as u64;
                    }

                    if ca2 >= 0 && ra2 < size && ca2 < size {
                        let became_set =
                            antinodes.get_unchecked_mut(ra2 as usize).set(ca2 as usize);
                        antinode_count += became_set as u64;
                    }
                }
            }
        }
    }

    antinode_count
}

pub fn part2_solve_rc(input: &str, size: usize) -> u64 {
    debug_assert!(size <= MAP_SIZE);

    // Some sanity checks on the input
    #[cfg(debug_assertions)]
    {
        let mut count = HashMap::new();
        for a in parse_rc(input) {
            debug_assert!(
                (b'0'..=b'9').contains(&a.antenna)
                    || (b'A'..=b'Z').contains(&a.antenna)
                    || (b'a'..=b'z').contains(&a.antenna),
                "Antenna has unexpected type {:?}",
                a.antenna as char
            );
            let c = count.entry(a.antenna).and_modify(|v| *v += 1).or_insert(1);
            assert!(
                *c <= 4,
                "Not expecting more than 4 antennas of type {:?}",
                a.antenna
            );
        }
    }

    // Parse the antennas input into a table indexed by antenna type.
    // There are up to 4 antenna of each type, so we make room for exactly that.
    let mut antennas: [ArrayVec<RC, 4>; ANTENNA_TYPES] = [ArrayVec::new(); ANTENNA_TYPES];

    for a in parse_rc(input) {
        debug_assert!(
            (b'0'..=b'9').contains(&a.antenna)
                || (b'A'..=b'Z').contains(&a.antenna)
                || (b'a'..=b'z').contains(&a.antenna)
        );

        // First thing is to convert the antenna letters into an index 0..(10+26+26)
        let antenna_index = antenna_to_index_usize_early(a.antenna);

        // Then push each one into a list with all others of the same type.
        debug_assert!(antenna_index < ANTENNA_TYPES);
        debug_assert!(antennas[antenna_index].len() < 4);
        debug_assert!(
            a.antenna == (input.as_bytes()[(a.rc.row as usize) * (size + 1) + a.rc.col as usize]),
            "Antenna `{}` at position ({}, {}) does not match input {}  `{}`",
            a.antenna as char,
            a.rc.row,
            a.rc.col,
            (a.rc.row as usize) * (size + 1) + a.rc.col as usize,
            input.as_bytes()[(a.rc.row as usize) * (size + 1) + a.rc.col as usize] as char
        );
        unsafe {
            antennas
                .get_unchecked_mut(antenna_index as usize)
                .push_unchecked(a.rc);
        }
    }

    // For each antenna, calculate the antinodes.
    let mut antinode_count = 0;
    // We also need to keep track of which positions contain antinodes.
    let mut antinodes: [U64Bitset; MAP_SIZE] = [U64Bitset::empty(); MAP_SIZE];
    unsafe {
        let size = size as i64;
        for ans in antennas {
            for i in 0..ans.len() {
                let an_i = ans.get_unchecked(i);
                for j in i + 1..ans.len() {
                    let an_j = ans.get_unchecked(j);

                    let (mut r1, mut c1) = (an_i.row as i64, an_i.col as i64);
                    let (mut r2, mut c2) = (an_j.row as i64, an_j.col as i64);

                    let (rd, cd) = (r2 - r1, c2 - c1);
                    // let (ra1, ra2) = (r1 - rd, r2 + rd);
                    // let (ca1, ca2) = (c1 - cd, c2 + cd);

                    // this is the same as pt 1, except that we need to loop from r1,c1 by -rd,-cd
                    // and from r2,c2 by rd,cd until we walk off the edge of the map.

                    loop {
                        let was_updated = antinodes.get_unchecked_mut(r1 as usize).set(c1 as usize);
                        antinode_count += was_updated as u64;

                        r1 -= rd;
                        c1 -= cd;
                        if r1 < 0 || c1 < 0 || c1 >= size {
                            break;
                        }
                    }

                    loop {
                        let was_updated = antinodes.get_unchecked_mut(r2 as usize).set(c2 as usize);
                        antinode_count += was_updated as u64;

                        r2 += rd;
                        c2 += cd;
                        if r2 >= size || c2 >= size || c2 < 0 {
                            break;
                        }
                    }
                }
            }
        }
    }

    antinode_count
}

// I thought that this implementation would be faster, but it is consistently slower than the _rc implementation.
// Without profiling, I don't know why.
// It must be something to do with memory access patterns, as as far as I can tell this does less calculation.
pub fn part1_solve_enumerated(input: &str, size: usize) -> u64 {
    debug_assert!(size <= MAP_SIZE);
    debug_assert!(size > 0);

    let mut antennas: [ArrayVec<i64, 4>; ANTENNA_TYPES] = [ArrayVec::new(); ANTENNA_TYPES];

    // Loop over the input.
    // We only want the radio antennas, and their offset into the input.
    for (pos, c) in input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, &c)| c >= b'0')
    {
        let pos = pos as i64;
        debug_assert!(
            (b'0'..=b'9').contains(c) || (b'A'..=b'Z').contains(&c) || (b'a'..=b'z').contains(&c)
        );

        // First thing is to convert the antenna letters into an index 0..(10+26+26)
        let antenna_index = antenna_to_index_usize_early(*c);
        unsafe {
            antennas
                .get_unchecked_mut(antenna_index)
                .push_unchecked(pos);
        }
    }

    let mut antinode_count = 0;
    let mut antinodes = PackedU64Bitset::<40>::empty();
    unsafe {
        let size = size as i64;
        let row_byte_count = size + 1;
        for ans in antennas {
            for i in 0..ans.len() {
                let an_i = ans.get_unchecked(i);
                for j in i + 1..ans.len() {
                    let an_j = ans.get_unchecked(j);

                    // do the arithmetic to find the antinodes
                    let pi = *an_i;
                    let pj = *an_j;
                    let dist = pj - pi;
                    let ai = pi - dist;
                    let aj = pj + dist;

                    // however, we want to get rid of those that are outside the arena
                    // there are two cases:
                    // * top/bottom: ai < 0, aj >= size
                    // * left/right: the number of rows between antinode and node isn't the same as between nodes
                    //
                    // We can mod the positions with the row length (inclusive of newlines)
                    // For the j antinode, we need to account for if the antinode position would be pushed into the newline
                    // which we do by adding one before the division.
                    let ai_row = std::intrinsics::unchecked_div(ai, row_byte_count);
                    let pi_row = std::intrinsics::unchecked_div(pi, row_byte_count);
                    let pj_row = std::intrinsics::unchecked_div(pj, row_byte_count);
                    let aj_row = std::intrinsics::unchecked_div(ai + 1, row_byte_count);
                    let gap = pj_row - pi_row;
                    let ai_gap = pi_row - ai_row;
                    let aj_gap = aj_row - pj_row;

                    if ai >= 0 && ai_gap == gap {
                        let was_set = antinodes.set_unchecked(ai as usize);
                        antinode_count += was_set as u64;
                    }

                    if aj < size && aj_gap == gap {
                        let was_set = antinodes.set_unchecked(aj as usize);
                        antinode_count += was_set as u64;
                    }
                }
            }
        }
    }

    antinode_count
}

// There's possibly a speedup here where we use an array of bitsets rather than a packed bitset for the antennas mask.
// But nothing statistically significant.
// So whatever makes this slower than the _rc version is a mystery to me.
pub fn part1_solve_enumerated2(input: &str, size: usize) -> u64 {
    debug_assert!(size <= MAP_SIZE);
    debug_assert!(size > 0);

    let mut antennas: [ArrayVec<i64, 4>; ANTENNA_TYPES] = [ArrayVec::new(); ANTENNA_TYPES];

    // Loop over the input.
    // We only want the radio antennas, and their offset into the input.
    for (pos, c) in input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, &c)| c >= b'0')
    {
        let pos = pos as i64;
        debug_assert!(
            (b'0'..=b'9').contains(c) || (b'A'..=b'Z').contains(&c) || (b'a'..=b'z').contains(&c)
        );

        // First thing is to convert the antenna letters into an index 0..(10+26+26)
        let antenna_index = antenna_to_index_usize_early(*c);
        unsafe {
            antennas
                .get_unchecked_mut(antenna_index)
                .push_unchecked(pos);
        }
    }

    let mut antinode_count = 0;
    let mut antinodes: [U64Bitset; MAP_SIZE] = [U64Bitset::empty(); MAP_SIZE];

    unsafe {
        let size = size as i64;
        let row_byte_count = size + 1;
        for ans in antennas {
            for i in 0..ans.len() {
                let an_i = ans.get_unchecked(i);
                for j in i + 1..ans.len() {
                    let an_j = ans.get_unchecked(j);

                    // do the arithmetic to find the antinodes
                    let pi = *an_i;
                    let pj = *an_j;
                    let dist = pj - pi;
                    let ai = pi - dist;
                    let aj = pj + dist;

                    // however, we want to get rid of those that are outside the arena
                    // there are two cases:
                    // * top/bottom: ai < 0, aj >= size
                    // * left/right: the number of rows between antinode and node isn't the same as between nodes
                    //
                    // We can mod the positions with the row length (inclusive of newlines)
                    // For the j antinode, we need to account for if the antinode position would be pushed into the newline
                    // which we do by adding one before the modulus.
                    let ai_row = std::intrinsics::unchecked_div(ai, row_byte_count);
                    let pi_row = std::intrinsics::unchecked_div(pi, row_byte_count);
                    let pj_row = std::intrinsics::unchecked_div(pj, row_byte_count);
                    let aj_row = std::intrinsics::unchecked_div(ai + 1, row_byte_count);
                    let gap = pj_row - pi_row;
                    let ai_gap = pi_row - ai_row;
                    let aj_gap = aj_row - pj_row;

                    if ai >= 0 && ai_gap == gap {
                        let ai_col = std::intrinsics::unchecked_rem(ai, row_byte_count);
                        let was_set = antinodes
                            .get_unchecked_mut(ai_row as usize)
                            .set_unchecked(ai_col as usize);
                        antinode_count += was_set as u64;
                    }

                    if aj < size && aj_gap == gap {
                        let aj_col = std::intrinsics::unchecked_rem(aj, row_byte_count);
                        let was_set = antinodes
                            .get_unchecked_mut(aj_row as usize)
                            .set_unchecked(aj_col as usize);
                        antinode_count += was_set as u64;
                    }
                }
            }
        }
    }

    antinode_count
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u64 {
    part1_solve_rc(input, MAP_SIZE)
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u64 {
    part2_solve_rc(input, MAP_SIZE)
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {
       "............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
        "
    };

    const EXAMPLE_ANTINODES: &str = indoc! {
        "......#....#
        ...#........
        ....#.....#.
        ..#.........
        .........#..
        .#....#.....
        ...#........
        #......#....
        ............
        ............
        ..........#.
        ..........#.
        "
    };

    const DAY8_INPUT: &str = include_str!("../input/2024/day8.txt");

    #[test]
    fn test_skip_parser_nowhitespace() {
        let skip_list_with = parse_skip(EXAMPLE).collect::<Vec<_>>();
        let skip_list_without = parse_skip(
            EXAMPLE
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .as_str(),
        )
        .collect::<Vec<_>>();

        assert_eq!(skip_list_with, skip_list_without);
    }

    #[test]
    fn test_skip_parser() {
        let skip_list = parse_skip(EXAMPLE).collect::<Vec<_>>();
        let expected_skip_list = vec![
            PackedSkip {
                skip: 20,
                item: b'0',
            },
            PackedSkip {
                skip: 9,
                item: b'0',
            },
            PackedSkip {
                skip: 14,
                item: b'0',
            },
            PackedSkip {
                skip: 9,
                item: b'0',
            },
            PackedSkip {
                skip: 14,
                item: b'A',
            },
            PackedSkip {
                skip: 6 + 12 + 12 + 8,
                item: b'A',
            },
            PackedSkip {
                skip: 13,
                item: b'A',
            },
        ];

        assert_eq!(skip_list, expected_skip_list);
    }

    #[test]
    fn test_antenna_to_index_usize_early() {
        assert_eq!(antenna_to_index_usize_early(b'0'), 0);
        assert_eq!(antenna_to_index_usize_early(b'1'), 1);
        assert_eq!(antenna_to_index_usize_early(b'2'), 2);
        assert_eq!(antenna_to_index_usize_early(b'3'), 3);
        assert_eq!(antenna_to_index_usize_early(b'4'), 4);
        assert_eq!(antenna_to_index_usize_early(b'5'), 5);
        assert_eq!(antenna_to_index_usize_early(b'6'), 6);
        assert_eq!(antenna_to_index_usize_early(b'7'), 7);
        assert_eq!(antenna_to_index_usize_early(b'8'), 8);
        assert_eq!(antenna_to_index_usize_early(b'9'), 9);
        assert_eq!(antenna_to_index_usize_early(b'A'), 10);
        assert_eq!(antenna_to_index_usize_early(b'B'), 11);
        assert_eq!(antenna_to_index_usize_early(b'C'), 12);
        assert_eq!(antenna_to_index_usize_early(b'D'), 13);
        assert_eq!(antenna_to_index_usize_early(b'E'), 14);
        assert_eq!(antenna_to_index_usize_early(b'F'), 15);
        assert_eq!(antenna_to_index_usize_early(b'G'), 16);
        assert_eq!(antenna_to_index_usize_early(b'H'), 17);
        assert_eq!(antenna_to_index_usize_early(b'I'), 18);
        assert_eq!(antenna_to_index_usize_early(b'J'), 19);
        assert_eq!(antenna_to_index_usize_early(b'K'), 20);
        assert_eq!(antenna_to_index_usize_early(b'L'), 21);
        assert_eq!(antenna_to_index_usize_early(b'M'), 22);
        assert_eq!(antenna_to_index_usize_early(b'N'), 23);
        assert_eq!(antenna_to_index_usize_early(b'O'), 24);
        assert_eq!(antenna_to_index_usize_early(b'P'), 25);
        assert_eq!(antenna_to_index_usize_early(b'Q'), 26);
        assert_eq!(antenna_to_index_usize_early(b'R'), 27);
        assert_eq!(antenna_to_index_usize_early(b'S'), 28);
        assert_eq!(antenna_to_index_usize_early(b'T'), 29);
        assert_eq!(antenna_to_index_usize_early(b'U'), 30);
        assert_eq!(antenna_to_index_usize_early(b'V'), 31);
        assert_eq!(antenna_to_index_usize_early(b'W'), 32);
        assert_eq!(antenna_to_index_usize_early(b'X'), 33);
        assert_eq!(antenna_to_index_usize_early(b'Y'), 34);
        assert_eq!(antenna_to_index_usize_early(b'Z'), 35);
    }

    #[test]
    fn test_rc_parser() {
        let rc_list = parse_rc(EXAMPLE).collect::<Vec<_>>();
        let expected_rc_list = vec![
            PackedRC {
                rc: RC { row: 1, col: 8 },
                antenna: b'0',
            },
            PackedRC {
                rc: RC { row: 2, col: 5 },
                antenna: b'0',
            },
            PackedRC {
                rc: RC { row: 3, col: 7 },
                antenna: b'0',
            },
            PackedRC {
                rc: RC { row: 4, col: 4 },
                antenna: b'0',
            },
            PackedRC {
                rc: RC { row: 5, col: 6 },
                antenna: b'A',
            },
            PackedRC {
                rc: RC { row: 8, col: 8 },
                antenna: b'A',
            },
            PackedRC {
                rc: RC { row: 9, col: 9 },
                antenna: b'A',
            },
        ];

        assert_eq!(rc_list, expected_rc_list);
    }

    #[test]
    fn test_part1_rc_example() {
        let count = part1_solve_rc(EXAMPLE, 12);
        assert_eq!(count, 14);
    }

    #[test]
    fn test_part1_rc() {
        assert_eq!(part1(DAY8_INPUT), 323);
    }

    fn test_part1_enumerated() {
        assert_eq!(part1_solve_enumerated(DAY8_INPUT, MAP_SIZE), 323);
    }

    #[test]
    fn test_part2_rc() {
        assert_eq!(part2(DAY8_INPUT), 1077);
    }
}

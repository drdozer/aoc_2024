use aoc_runner_derive::aoc;
use packed::PackedBitset;
use primitives::PrimitiveBitset;

use crate::bitset::*;

// In day 6, there's a map of a lab.
//
// ```
// ....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...
// ```
//
// The `^` is where a guard is starting, facing upwards.
// Each '#' is an obstacle that the guard can not pass through.
// She will turn 90 degrees clockwise and then continue walking.
// Eventually, she will walk off the edge of the map.
// The task is to calculate how many unique positions she visits before leaving the map.
// The guard is considered to have visited their starting position.
// Visiting the same position multiple times only counts once.
//
// We assume she does indeed leave the map, and isn't stuck in a loop.
// The real data is 130 positions square, which sadly does not fit into a u128.
//
// Ideally, we would like to store a bit field of the map.
// We can mark each visited position with a 1.
// Then we can either count as we go, or use bitwise operations to count at the end.
//
// Horizontal movements are easy to count efficiently with bitwise operations,
// by a combination of `count_ones` and `trailing_zeros`.
// Vertical movemements only apply for columns containing obstacles that she hits.
// This means that the cost of calculating the bitfield for all columns is higher than
// simply looping over the small number that have a collision.
//
// To optimize the representation, we can pad out each row(column) of the map to a multiple of 8 bytes.
// Any overhang doesn't matter for collision detection.
// We need to subtract the padding only for the case where the guard exits to the right or bottom.

// A position on the map, for example, where the guard is.

/// A map row needs to cover 130 columns.

// Vital statistics
const MAP_SIZE: usize = 130;
type BitsetRep = u16;
const COLUMN_BYTES: usize = 9;
type RowBitset = PackedBitset<PrimitiveBitset<BitsetRep>, COLUMN_BYTES>;
const UNUSED_BITS: usize = std::mem::size_of::<u16>() * COLUMN_BYTES * 8 - MAP_SIZE;

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Right => write!(f, ">"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct LabMapRow(
    // I've benchmarked performance for various bitset implementations, and on my machine, this is an optimal trade-off.
    // For operations within the bitset, u16 to u64 behave almost equivalently.
    // However, for arrays of bitsets 130 elements long, u8 and u16 are 10x faster than u32 or larger.
    RowBitset,
);

impl Default for LabMapRow {
    fn default() -> Self {
        LabMapRow(RowBitset::default())
    }
}

impl LabMapRow {
    fn set_obstacle(&mut self, index: usize) {
        self.0.set(index);
    }

    fn is_obstacle(&self, index: usize) -> bool {
        self.0.get(index)
    }
}

pub struct LabMap {
    rows: [LabMapRow; MAP_SIZE],
}

impl Default for LabMap {
    fn default() -> Self {
        LabMap {
            rows: [LabMapRow::default(); MAP_SIZE],
        }
    }
}

impl LabMap {
    fn obstacle_at(&self, row: usize, col: usize) -> bool {
        unsafe { self.rows.get_unchecked(row).is_obstacle(col) }
    }

    fn next_obstacle(
        &self,
        row: usize,
        col: usize,
        direction: Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => (0..row)
                .rev()
                .take_while(|&r| !self.obstacle_at(r, col))
                .map(|r| (r, col))
                .next(),
            Direction::Right => unsafe {
                self.rows
                    .get_unchecked(row)
                    .0
                    .into_iter()
                    .filter(|&c| c > col)
                    .map(|c| (row, c))
                    .next()
            },
            Direction::Down => (row + 1..MAP_SIZE)
                .take_while(|&r| !self.obstacle_at(r, col))
                .map(|r| (r, col))
                .next(),
            Direction::Left => unsafe {
                self.rows
                    .get_unchecked(row)
                    .0
                    .into_iter()
                    .rev()
                    .filter(|&c| c < col)
                    .map(|c| (row, c))
                    .next()
            },
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct VisitedRow(RowBitset);

impl VisitedRow {
    fn visit(&mut self, col: usize) -> bool {
        unsafe {
            let unvisited = !self.0.get_unchecked(col);
            self.0.set_unchecked(col);
            unvisited
        }
    }
}

pub struct Visited {
    rows: [VisitedRow; MAP_SIZE],
}

impl Default for Visited {
    fn default() -> Self {
        Visited {
            rows: [VisitedRow::default(); MAP_SIZE],
        }
    }
}

impl Visited {
    fn visit(&mut self, row: usize, col: usize) -> bool {
        unsafe { self.rows.get_unchecked_mut(row).visit(col) }
    }
}

pub struct Guard {
    pos: (usize, usize),
    direction: Direction,
}

pub struct WalkState {
    map: LabMap,
    visited: Visited,
    guard: Guard,
}

pub fn parse_lab_map(input: &str) -> (LabMap, Guard) {
    let input = input.as_bytes();

    // let mut rows = unsafe { std::mem::MaybeUninit::<[LabMapRow; 130]>::uninit().assume_init() };
    let mut rows = [LabMapRow::default(); MAP_SIZE];

    let mut i = 0;
    let mut pos = (0, 0);
    let mut direction = Direction::Right;
    for row in 0..MAP_SIZE {
        let map_row = unsafe { rows.get_unchecked_mut(row) };
        for col in 0..MAP_SIZE {
            match unsafe { input.get_unchecked(i) } {
                b'.' => {}
                b'#' => map_row.set_obstacle(col),
                b'\n' => {
                    break;
                }
                b'^' => {
                    pos = (row, col);
                    direction = Direction::Up
                }
                b'v' => {
                    pos = (row, col);
                    direction = Direction::Down
                }
                b'<' => {
                    pos = (row, col);
                    direction = Direction::Left
                }
                b'>' => {
                    pos = (row, col);
                    direction = Direction::Right
                }
                c => unreachable!("Unexpected character: {:?}", *c as char),
            }
            i += 1;
        }
        i += 1; // skip the newline
        if i >= input.len() {
            break;
        }
    }

    let map = LabMap { rows };
    let guard = Guard { pos, direction };

    (map, guard)
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    part1_with_size(input, MAP_SIZE)
}

// This is solved by simply walking around the map.
// I can't see any obvious optimizations.
// It is possible to use bitwise operations to zoom horizontally,
// but we still need to fill in all the visited bits, unless I'm missing someting obvious.
// This is becuase paths intersect, so we need to not double-count where our path crosses itself.
pub fn part1_with_size(input: &str, map_size: usize) -> usize {
    let (lab_map, mut guard) = parse_lab_map(input);

    // We've visited the staring position.
    let mut visited = Visited::default();
    let mut visit_count = 0;
    loop {
        // We always mark the current position as visited.
        let (row, col) = guard.pos;
        visit_count += visited.visit(row, col) as usize;

        // Then we move the guard in the direction she is facing.
        match guard.direction {
            Direction::Up => {
                if row == 0 {
                    break;
                }
                let new_row = row - 1;
                if lab_map.obstacle_at(new_row, col) {
                    guard.direction.turn_right();
                } else {
                    guard.pos.0 = new_row;
                }
            }
            Direction::Right => {
                if col == map_size - 1 {
                    break;
                }
                let new_col = col + 1;
                if lab_map.obstacle_at(row, new_col) {
                    guard.direction.turn_right();
                } else {
                    guard.pos.1 = new_col;
                }
            }
            Direction::Down => {
                if row == map_size - 1 {
                    break;
                }
                let new_row = row + 1;
                if lab_map.obstacle_at(new_row, col) {
                    guard.direction.turn_right();
                } else {
                    guard.pos.0 = new_row;
                }
            }
            Direction::Left => {
                if col == 0 {
                    break;
                }
                let new_col = col - 1;
                if lab_map.obstacle_at(row, new_col) {
                    guard.direction.turn_right();
                } else {
                    guard.pos.1 = new_col;
                }
            }
        }
    }

    visit_count
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u64 {
    todo!()
}

// #[aoc(day6, part2)]
// pub fn part2(input: &str) -> usize {
//     part2_with_size(input, MAP_SIZE)
// }

// // We perform the same walk.
// // However, instead of counting where we've visited, we track candidate positions that would loop the path.
// // We then have to actually walk the path to see if we hit these candidates.
// // Potentially the horizontal movement could be optimized using bitwise operations.
// pub fn part2_with_size(input: &str, map_size: usize) -> usize {
//     let mut walk_state = parse_lab_map(input);

//     // We've visited the staring position.
//     let mut obstruction_count = 0;

//     // The next candidate obstruction that we're going to check if we visit.
//     let mut candidate_obstruction = (map_size, map_size);

//     // Array of past 4 obstructions.
//     // Because we always hit an obstruction from a defined direction, we can index this array by direction.
//     let mut obstructions = [(map_size + 1, map_size + 1); 4]; // initialized to be unreachable

//     loop {
//         // if we've hit an obstruction, we found an obstruction
//         obstruction_count += (guard.pos == candidate_obstruction) as usize;

//         if guard.pos == candidate_obstruction {
//             println!(
//                 "Hit candidate {} at {:?}",
//                 obstruction_count, candidate_obstruction
//             );
//         }

//         let (row, col) = walk_state.guard.pos;
//         match walk_state.guard.direction {
//             Direction::Up => {
//                 if row == 0 {
//                     break;
//                 }
//                 let new_row = row - 1;
//                 if walk_state.map.obstacle_at(new_row, col) {
//                     obstructions[0] = (new_row, col);
//                     candidate_obstruction = (row, obstructions[2].1 + 1);
//                     println!(
//                         "Obstructions: {:?} {:?}",
//                         obstructions, walk_state.guard.direction
//                     );
//                     println!("Cew candidate: {:?}", candidate_obstruction);
//                     walk_state.guard.direction.turn_right();
//                 } else {
//                     walk_state.guard.pos.0 = new_row;
//                 }
//             }
//             Direction::Right => {
//                 if col == map_size - 1 {
//                     break;
//                 }
//                 let new_col = col + 1;
//                 if walk_state.map.obstacle_at(row, new_col) {
//                     obstructions[1] = (row, new_col);
//                     candidate_obstruction = (obstructions[3].0 + 1, col);
//                     println!(
//                         "Obstructions: {:?} {:?}",
//                         obstructions, walk_state.guard.direction
//                     );
//                     println!("Cew candidate: {:?}", candidate_obstruction);
//                     walk_state.guard.direction.turn_right();
//                 } else {
//                     walk_state.guard.pos.1 = new_col;
//                 }
//             }
//             Direction::Down => {
//                 if row == map_size - 1 {
//                     break;
//                 }
//                 let new_row = row + 1;
//                 if walk_state.map.obstacle_at(new_row, col) {
//                     obstructions[2] = (new_row, col);
//                     candidate_obstruction = (row, obstructions[0].1 - 1);
//                     println!(
//                         "Obstructions: {:?} {:?}",
//                         obstructions, walk_state.guard.direction
//                     );
//                     println!("Cew candidate: {:?}", candidate_obstruction);
//                     walk_state.guard.direction.turn_right();
//                 } else {
//                     walk_state.guard.pos.0 = new_row;
//                 }
//             }
//             Direction::Left => {
//                 if col == 0 {
//                     break;
//                 }
//                 let new_col = col - 1;
//                 if walk_state.map.obstacle_at(row, new_col) {
//                     obstructions[3] = (row, new_col);
//                     candidate_obstruction = (obstructions[1].0 - 1, col);
//                     println!(
//                         "Obstructions: {:?} {:?}",
//                         obstructions, walk_state.guard.direction
//                     );
//                     println!("Cew candidate: {:?}", candidate_obstruction);
//                     walk_state.guard.direction.turn_right();
//                 } else {
//                     walk_state.guard.pos.1 = new_col;
//                 }
//             }
//         }
//     }

//     obstruction_count
// }

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_parse_map() {
        let input = indoc! {
            "....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
            "
        };

        let (lab_map, guard) = parse_lab_map(input);
        assert!(lab_map.obstacle_at(0, 4));
        assert!(lab_map.obstacle_at(1, 9));
        assert_eq!(guard.pos, (6, 4));
        assert_eq!(guard.direction, Direction::Up);
    }

    #[test]
    fn test_part1_example() {
        let input = indoc! {
            "....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
            "
        };

        let visited = part1_with_size(input, 10);
        assert_eq!(visited, 41);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/2024/day6.txt");
        let answer = part1(input);
        assert_eq!(answer, 5162);
    }

    // #[test]
    // fn test_part2_example() {
    //     let input = indoc! {
    //         "....#.....
    //         .........#
    //         ..........
    //         ..#.......
    //         .......#..
    //         ..........
    //         .#..^.....
    //         ........#.
    //         #.........
    //         ......#...
    //         "
    //     };

    //     let obstructions = part2_with_size(input, 10);
    //     assert_eq!(obstructions, 6);
    // }
}

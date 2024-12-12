use aoc_runner_derive::aoc;
use num::{BigUint, FromPrimitive};

use crate::stack_vec::ArrayVec;

static FILE_COUNT: u64 = 10_000;

pub fn checksum_disk_diagram(input: &str) -> u64 {
    // The example inputs look like this:
    //  00...111...2...333.44.5555.6666.777.888899
    // We need sum the product of the block number (0..) with the file ID at that block.
    // This iterator expression does this.
    input
        .as_bytes()
        .iter()
        .map(|b| b.saturating_sub(b'0') as u64) // convert digits to numbers, and `.` to `0` due to underflow
        .enumerate()
        .map(|(i, b)| b * i as u64) // the block/file ID product
        .sum()
}

pub fn sum_range(start: u64, len: u64) -> u64 {
    len * (len + 2 * start - 1) / 2
}

// Sum up the difference between two ranges.
// For ... reasons ... we express this as
// - the start of the lower range
// - the end of the upper range
// - the length.
pub fn sum_range_diff(start1: u64, start2: u64, len: u64) -> u64 {
    len * (start2 - start1)
}

pub fn sum_checksum_diff(start1: u64, start2: u64, len: u64, id: u64) -> u64 {
    let block_sum = sum_range_diff(start1, start2, len);

    block_sum * id
}

// this benchmarks as the faster version
pub fn sum_checksum_range(start: u64, len: u64, id: u64) -> u64 {
    let block_sum = sum_range(start, len);

    block_sum * id
}

pub fn sum_checksum_range_loop(start: u64, len: u64, id: u64) -> u64 {
    let mut checksum = 0;
    for i in start..start + len {
        checksum += i * id;
    }

    checksum
}

#[derive(Debug)]
pub struct FileDetails {
    used: u64,
    free: u64,
}

pub fn get_file_details(input: &[u8], id: u64) -> FileDetails {
    unsafe {
        let p = id as usize * 2;
        let used = input.get_unchecked(p).unchecked_sub(b'0') as u64;
        let free = input.get_unchecked(p + 1).wrapping_sub(b'0') as u64;

        FileDetails { used, free }
    }
}

#[aoc(day9, part1)]
fn part1(input: &str) -> u64 {
    // We don't need to actually construct the file system.
    // The trick is to consume from the beginning, and back-fill from the end as we go.
    // * fetch the next value from the beginning
    // * add it's contribution to the checksum
    // * back-fill the gap with the next value from the end
    // * if the gap was not completely filled, pull another value from the end, and so on
    // * if the gap was filled, save what is left over from the end, and use that in the next iteration
    //
    // The checksum for a rangecan be calculated in one go, using the arithmetic series formula.
    let input = input.as_bytes();
    debug_assert!(
        input.len() % 2 == 0,
        "input length must be even but was {}",
        input.len()
    );
    let file_count = input.len() / 2;

    #[cfg(debug_assertions)]
    let total_blocks: u64 = (0..file_count)
        .map(|i| get_file_details(input, i as u64).used)
        .sum();
    // println!("total_blocks: {}", total_blocks);

    let mut checksum = 0;

    let mut left_id = 0;
    let mut right_id = file_count as u64; // must point *past* the last file
    let mut right_remaining = 0;
    let mut block = 0;

    // we loop until the ID's collide
    while right_id > left_id {
        // Process the left-hand file.
        let mut left = get_file_details(input, left_id);

        // Calculate the sum of the blocks occupied by this.
        let left_checksum = sum_checksum_range(block, left.used, left_id as u64);
        // println!(
        //     "block: {block}\tleft: {left_id}\t{left:?}\tright: {right_id}\tbackfill: {right_remaining}\tchecksum: {checksum}\tAdvancing\t{}x{left_id} ({block}..{})= {left_checksum}",left.used, block + left.used
        // );
        checksum += left_checksum;

        // increment left and block to point to the next empty space
        left_id += 1;
        block += left.used;

        // Back-fill the gap.
        //
        // There are several case:
        // * we have drained the right file, so we pull a new one
        // * we consume the entire right file
        // * we consume all the space after the left file
        //
        // We can loop these until the free space after the left file is exhausted.
        while left.free > 0 && right_id > left_id {
            // If there's nothing remaining in the right file, we pull from the end.
            if right_remaining == 0 {
                // println!(
                //     "block: {block}\tleft: {left_id}\t{left:?}\tright: {right_id}\tbackfill: {right_remaining}\tchecksum: {checksum}\tPulling from the end"
                // );
                right_id -= 1;
                let right = get_file_details(input, right_id);
                right_remaining = right.used;
            }

            // We can pull can_fill values from the remaining right file.
            let can_fill = left.free.min(right_remaining);
            let right_fragment_checksum = sum_checksum_range(block, can_fill, right_id);
            // println!(
            //     "block: {block}\tleft: {left_id}\t{left:?}\tright: {right_id}\tbackfill: {right_remaining}\tchecksum: {checksum}\tfilling\t{can_fill}x{right_id} ({block}..{}) = {right_fragment_checksum}", block+can_fill
            // );
            checksum += right_fragment_checksum;
            right_remaining -= can_fill;
            left.free -= can_fill;
            block += can_fill;
        }
    }

    // It is possible that the last right file was not fully consumed.
    // println!(
    //     "Last right file: {} with {} remaining",
    //     right_id, right_remaining
    // );

    let last_remaining_checksum = sum_checksum_range(block, right_remaining, right_id);
    // println!(
    //         "block: {block}\tleft: {left_id}\t\t\t\t\tright: {right_id}\tbackfill: {right_remaining}\tchecksum: {checksum}\tEnd\t{right_remaining}x{right_id} ({block}..{}) = {last_remaining_checksum}", block + right_remaining
    //     );
    checksum += last_remaining_checksum;
    block += right_remaining;

    // debug_assert_eq!(
    //     block, total_blocks,
    //     "The last block does not match the total blocks."
    // );

    checksum
}

// Note - this didn't give the answer needed by AOC, but did for te example input.
#[aoc(day9, part2)]
fn part2(input: &str) -> u64 {
    // This time we would need to move entire files around, not individual blocks from those files.
    // We want to avoid building a datastructure for the entire file system, as much as we can avoid it.
    // The differnece between the checksum of the system before and after a file is moved is:
    // * checksum_before - file_checksum_before_move + file_checksum_after_move
    // Luckilly, the difference in file checksums is easy to calculate.
    // This is done in: `sum_checksum_diff`.

    let input = input.as_bytes();
    let file_count = (input.len() / 2) as u64;
    debug_assert!(file_count <= FILE_COUNT);

    // We start by calcualting the checksum of the filesystem before we make any edits.
    // At the same time, we're going to build an array of gaps and their starting block.

    // TODO: try different word sizes - mut be at least u16 to fit the full block range
    #[derive(Default, Debug, Clone, Copy)]
    struct GapRecord {
        file_block: u32,
        padding_block: u32,
        length: u8,
    }
    // TODO: we may be able to populate this gaps datastructure lazily
    let mut gaps: ArrayVec<GapRecord, { FILE_COUNT as usize }> = ArrayVec::new();
    let mut checksum = unsafe {
        let mut block = 0;
        let mut sum = 0;
        for p in 0..file_count {
            let f = get_file_details(input, p);
            let delta = sum_checksum_range(block, f.used, p);
            sum += delta;
            println!(
                "p: {}\tblock: {}\t{:?}\tdelta: {}\tchecksum: {}",
                p, block, f, delta, sum
            );
            gaps.push_unchecked(GapRecord {
                file_block: block as u32,
                padding_block: block as u32 + f.used as u32,
                length: f.free as u8,
            });
            block += f.used;
            block += f.free;
        }
        sum
    };

    // We'd like to keep indexes that are guaranteed not before a valid insertion index.
    // * $\not \exists i \in [0, gap_indexes_{f.size})
    // We'll do this with an array, one entry per gap size.
    // When a file is inserted, it makes a smaller gap.
    // This may need to be updated in the gap_index entry for that smaller gap.
    let mut gap_indexes = [0; 10];

    for p in (0..file_count).rev() {
        // the end file to move if we can
        let to_move = get_file_details(input, p);

        // Its block offset. We have the position of the gap, so need to subtract the file size.
        let move_start_block = unsafe { gaps.get_unchecked(p as usize).file_block as u64 };

        println!(
            "p: {}\t{:?}\t{:?}\t{:?}\tBlock to move",
            p, gap_indexes, to_move, move_start_block
        );

        // Find the first gap, if it exists, that will take f.
        loop {
            let &i = unsafe { gap_indexes.get_unchecked(to_move.used as usize) };
            let gap = unsafe { gaps.get_unchecked_mut(i) };
            println!(
                "p: {}\t{:?}\t{:?}\t{:?}\t{}\t{:?}",
                p, gap_indexes, to_move, move_start_block, i, gap
            );
            if i >= (p as usize) {
                println!("No more gaps");
                // No big-enough gaps left
                break;
            }
            if (gap.length as u64) >= to_move.used {
                // We found a gap
                let checksum_diff =
                    sum_checksum_diff(gap.padding_block as u64, move_start_block, to_move.used, p);
                checksum -= checksum_diff;
                println!(
                    "Moving into gap start_block: {} move start_block: {} length: {} checksum_diff: {} checksum: {}",
                    gap.padding_block, move_start_block, to_move.used, checksum_diff, checksum
                );
                gap.padding_block += to_move.used as u32;
                gap.length -= to_move.used as u8;
                let shorter_i = unsafe { gap_indexes.get_unchecked_mut(gap.length as usize) };
                *shorter_i = (*shorter_i).min(i);
                unsafe {
                    // i += 1; // but we can't double-borrow from gap_indexes, because the borrow-checker is silly
                    *gap_indexes.get_unchecked_mut(to_move.used as usize) += 1;
                }

                break;
            }
            unsafe {
                // i += 1; // but we can't double-borrow from gap_indexes, because the borrow-checker is silly
                *gap_indexes.get_unchecked_mut(to_move.used as usize) += 1;
            }
        }
    }

    checksum
    // TODO: It should be possible to implement this without the gaps array.
    // In principle, we can calculate  the block offsets on the fly.
    // But I can't work it out in the timescale we have.
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = include_str!("../input/2024/day9.txt");
    const EXAMPLE: &str = "2333133121414131402\n";
    const EXAMPLE_CHECKSUM_1: u64 = 1928;
    const EXAMPLE_CHECKSUM_2: u64 = 2858;

    const EXAMPLE1_DIAGRAM: &str = indoc! {"
        00...111...2...333.44.5555.6666.777.888899
        009..111...2...333.44.5555.6666.777.88889.
        0099.111...2...333.44.5555.6666.777.8888..
        00998111...2...333.44.5555.6666.777.888...
        009981118..2...333.44.5555.6666.777.88....
        0099811188.2...333.44.5555.6666.777.8.....
        009981118882...333.44.5555.6666.777.......
        0099811188827..333.44.5555.6666.77........
        00998111888277.333.44.5555.6666.7.........
        009981118882777333.44.5555.6666...........
        009981118882777333644.5555.666............
        00998111888277733364465555.66.............
        0099811188827773336446555566.............."
    };

    const EXAMPLE2_DIAGRAM: &str = indoc! {"
        00...111...2...333.44.5555.6666.777.888899
        0099.111...2...333.44.5555.6666.777.8888..
        0099.1117772...333.44.5555.6666.....8888..
        0099.111777244.333....5555.6666.....8888..
        00992111777.44.333....5555.6666.....8888.."
    };

    #[test]
    fn test_checksum_disk_diagram_example1() {
        let mut cs = 0;
        for line in EXAMPLE1_DIAGRAM.lines().filter(|l| !l.is_empty()) {
            let checksum = checksum_disk_diagram(line);
            cs = checksum;
            println!("{line}\t{checksum}");
        }
        assert_eq!(cs, EXAMPLE_CHECKSUM_1);
    }

    #[test]
    fn test_checksum_disk_diagram_exaple2() {
        let mut cs: i64 = 0;
        for line in EXAMPLE2_DIAGRAM.lines().filter(|l| !l.is_empty()) {
            let checksum = checksum_disk_diagram(line);
            println!("{line}\t{checksum}\t{}", cs - checksum as i64);
            cs = checksum as i64;
        }
        assert_eq!(cs as u64, EXAMPLE_CHECKSUM_2);
        // test case for the example 2 that was failing with 18 instead of 14
        assert_eq!(sum_checksum_diff(4, 11, 1, 2), 14);
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE), EXAMPLE_CHECKSUM_1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6332189866718);
    }

    #[test]
    fn test_part1_alt() {
        assert_eq!(
            part1(include_str!("../input/2024/day9_backup.txt")),
            6386640365805
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE), EXAMPLE_CHECKSUM_2);
    }

    #[ignore] // we failed to get the right answer
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1);
    }

    #[test]
    fn test_sum_range() {
        for i in 0..100 {
            for j in 1..100 {
                assert_eq!(
                    sum_range(i, j),
                    (i..i + j).sum::<u64>(),
                    "Failed for {}, {}",
                    i,
                    j
                );
            }
        }
    }

    #[test]
    fn test_sum_checksum_range() {
        for i in 0..100 {
            for j in 1..10 {
                for k in 0..100 {
                    assert_eq!(
                        sum_checksum_range(i, j, k),
                        sum_checksum_range_loop(i, j, k),
                        "Failed for {}, {}, {}",
                        i,
                        j,
                        k
                    );
                }
            }
        }
    }

    #[test]
    fn test_sum_checksum_range_values() {
        // A particular checksum example
        let s = (49989..(49989 + 8)).sum::<u64>() * 5218;
        assert_eq!(sum_checksum_range(49989, 8, 5218), s);
        assert_eq!(sum_checksum_range_loop(49989, 8, 5218), s);
        assert_eq!(s, 2086886920);
    }

    #[test]
    fn test_sum_range_diff() {
        for s1 in 0..100 {
            for s2 in (s1 + 1)..100 {
                for len in 1..10 {
                    let diff = sum_range(s2, len) - sum_range(s1, len);
                    let calc = sum_range_diff(s1, s2, len);

                    assert_eq!(diff, calc);
                }
            }
        }
    }
}

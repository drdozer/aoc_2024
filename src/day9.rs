use aoc_runner_derive::aoc;
use num::{BigUint, FromPrimitive};

pub fn sum_range(start: u64, len: u64) -> u64 {
    len * (len + 2 * start - 1) / 2
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

    debug_assert_eq!(
        block, total_blocks,
        "The last block does not match the total blocks."
    );

    checksum
}

#[aoc(day9, part2)]
fn part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day9.txt");
    const EXAMPLE: &str = "2333133121414131402\n";
    const EXAMPLE_CHECKSUM: u64 = 1928;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE), EXAMPLE_CHECKSUM);
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
    fn test_part2() {
        assert_eq!(part2(INPUT), 1);
    }

    #[test]
    fn test_sum_range() {
        for i in 0..100 {
            for j in 1..100 {
                assert_eq!(sum_range(i, j), (i..i + j).sum(), "Failed for {}, {}", i, j);
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
}

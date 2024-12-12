use std::{collections::HashMap, marker::PhantomData};

use aoc_runner_derive::aoc;

use crate::stack_vec::ArrayVec;

const MAX_BLINKS_PART1: usize = 25;
const MAX_BLINKS_PART2: usize = 75;

const SUMMARISE: bool = false;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

// This is faster.
pub fn count_digits_loop(n: u64) -> u64 {
    let mut n = n;
    let mut d = 0;
    loop {
        d += 1;
        if n < 10 {
            break;
        }
        n /= 10;
    }
    d
}

// This is slower.
pub fn count_digits_table(n: u64) -> u64 {
    let d_10 = (n >= 10) as u64;
    let d_100 = (n >= 100) as u64;
    let d_1000 = (n >= 1000) as u64;
    let d_10000 = (n >= 10000) as u64;
    let d_100000 = (n >= 100000) as u64;
    let d_1000000 = (n >= 1000000) as u64;
    let d_10000000 = (n >= 10000000) as u64;
    let d_100000000 = (n >= 100000000) as u64;

    d_10 + d_100 + d_1000 + d_10000 + d_100000 + d_1000000 + d_10000000 + d_100000000
}

// This is slowest.
pub fn count_digits_ilog10(n: u64) -> u64 {
    1 + n.abs_diff(0).checked_ilog10().unwrap_or_default() as u64
}

pub fn stone_rule(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }

    let mut n = stone;
    let mut p = 1;
    let mut digits = 0;

    while n > 0 {
        n /= 10;
        if digits % 2 == 0 {
            p *= 10;
        }
        digits += 1;
    }

    if digits % 2 == 0 {
        let (first, second) = (stone / p, stone % p);
        return (first, Some(second));
    }

    ((stone * 2024), None)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StackFrame {
    stone: u64,
    remaining_blinks: usize,
}

pub trait StoneMemo {
    fn empty() -> Self;
    fn memo_get(&self, key: &StackFrame) -> Option<&usize>;
    fn memo_insert(&mut self, key: StackFrame, value: usize);
    fn summarise(&self);
}

pub trait StoneCounter<SM: StoneMemo> {
    fn count_stones(&self, stone: u64, remaining_blinks: usize) -> usize {
        self.count_stones_memo(stone, remaining_blinks, &mut SM::empty())
    }
    fn count_multiple_stones(&self, stones: &[u64], remaining_blinks: usize) -> usize {
        let mut memo = SM::empty();
        let stone_count = stones
            .iter()
            .map(|&stone| self.count_stones_memo(stone, remaining_blinks, &mut memo))
            .sum();

        if SUMMARISE {
            memo.summarise();
        }

        stone_count
    }
    fn count_stones_memo(&self, stone: u64, remaining_blinks: usize, memo: &mut SM) -> usize;
}

struct NaiveMemoisedRecursion<SM>(PhantomData<SM>);
impl<SM> Default for NaiveMemoisedRecursion<SM> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<SM: StoneMemo> StoneCounter<SM> for NaiveMemoisedRecursion<SM> {
    // The simplest version of the function. It always recurses.
    fn count_stones_memo(&self, stone: u64, remaining_blinks: usize, memo: &mut SM) -> usize {
        if remaining_blinks == 0 {
            // println!("{}", stone);
            return 1;
        }

        if let Some(&count) = memo.memo_get(&StackFrame {
            stone,
            remaining_blinks,
        }) {
            return count;
        }

        let smaller_blinks = remaining_blinks - 1;
        let (left, right) = stone_rule(stone);
        let left_count = self.count_stones_memo(left, smaller_blinks, memo);
        let right_count = right
            .map(|right| self.count_stones_memo(right, smaller_blinks, memo))
            .unwrap_or(0);

        let stone_count = left_count + right_count;
        memo.memo_insert(
            StackFrame {
                stone,
                remaining_blinks,
            },
            stone_count,
        );
        stone_count
    }
}

struct LeftLoopingMemoisedRecursion<SM>(PhantomData<SM>);
impl<SM> Default for LeftLoopingMemoisedRecursion<SM> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<SM: StoneMemo> StoneCounter<SM> for LeftLoopingMemoisedRecursion<SM> {
    // This version is about 35% faster than `NaiveMemoisedRecursion`.
    //
    // It loops in the non-branching cases and only recurses on branches.
    fn count_stones_memo(&self, stone: u64, remaining_blinks: usize, memo: &mut SM) -> usize {
        if remaining_blinks == 0 {
            return 1;
        }

        if let Some(&count) = memo.memo_get(&StackFrame {
            stone,
            remaining_blinks,
        }) {
            return count;
        }

        let mut current_stone = stone;
        let mut current_blinks = remaining_blinks;
        // let mut right_stones = ArrayVec::<(u64, usize), MAX_BLINKS>::new();

        // Follow the linear path until we either hit a split or run out of blinks
        loop {
            let (left, right) = stone_rule(current_stone);
            current_blinks -= 1;

            match right {
                // We found a split, recurse from here
                Some(right) => {
                    let left_count = self.count_stones_memo(left, current_blinks, memo);
                    let right_count = self.count_stones_memo(right, current_blinks, memo);
                    let stone_count = left_count + right_count;

                    // Memoize all intermediate results we calculated
                    memo.memo_insert(
                        StackFrame {
                            stone,
                            remaining_blinks,
                        },
                        stone_count,
                    );
                    return stone_count;
                }
                // No split, continue linear path
                None => {
                    if current_blinks == 0 {
                        // We've used all blinks, memoize and return
                        memo.memo_insert(
                            StackFrame {
                                stone,
                                remaining_blinks,
                            },
                            1,
                        );
                        return 1;
                    }
                    current_stone = left;
                }
            }
        }
    }
}

struct LoopingMemoisingNoRecursion<SM>(PhantomData<SM>);
impl<SM> Default for LoopingMemoisingNoRecursion<SM> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<SM: StoneMemo> StoneCounter<SM> for LoopingMemoisingNoRecursion<SM> {
    // Avoid all recursion by maintaining a local stack.
    //
    // This turns out to be really slow, as I think we do a lot more hashset lookups.

    fn count_stones_memo(&self, stone: u64, remaining_blinks: usize, memo: &mut SM) -> usize {
        unsafe {
            let mut stack = ArrayVec::<StackFrame, { MAX_BLINKS_PART2 * 2 + 1 }>::new();
            stack.push_unchecked(StackFrame {
                stone,
                remaining_blinks,
            });

            'stack_check: while let Some(&StackFrame {
                stone,
                remaining_blinks,
            }) = stack.get_last()
            {
                // With no more blinks left, we just have the one stone.
                if remaining_blinks == 0 {
                    memo.memo_insert(stack.pop_unsafe(), 1);
                    continue 'stack_check;
                }
                // Sanity check: early exit for when we already know the count.
                if let Some(_) = memo.memo_get(&StackFrame {
                    stone,
                    remaining_blinks,
                }) {
                    stack.pop_unsafe();
                    continue 'stack_check;
                }

                // OK, I guess we need to do some work then...
                let (left, right) = stone_rule(stone);
                let child_blinks = remaining_blinks - 1;

                // The cases are:
                // - left only
                //   - left in cache: set count for stone to left count, pop stack
                //   - left not in cache: push left onto stack
                // - left and right
                //   - left and right in cache: set stone count to sum of left and right count, pop stack
                //   - otherwise, push left and righ onto stack if they don't have a count

                match right {
                    None => {
                        let left_sf = StackFrame {
                            stone: left,
                            remaining_blinks: child_blinks,
                        };
                        let left_count = memo.memo_get(&left_sf);
                        if let Some(&left_count) = left_count {
                            memo.memo_insert(stack.pop_unsafe(), left_count);
                        } else {
                            stack.push_unchecked(StackFrame {
                                stone: left,
                                remaining_blinks: child_blinks,
                            });
                        }
                    }
                    Some(right) => {
                        let left_sf = StackFrame {
                            stone: left,
                            remaining_blinks: child_blinks,
                        };
                        let right_sf = StackFrame {
                            stone: right,
                            remaining_blinks: child_blinks,
                        };

                        let left_count = memo.memo_get(&left_sf);
                        let right_count = memo.memo_get(&right_sf);

                        if let (Some(left_count), Some(right_count)) = (left_count, right_count) {
                            memo.memo_insert(stack.pop_unsafe(), left_count + right_count);
                        } else {
                            if let None = right_count {
                                stack.push_unchecked(right_sf);
                            }
                            if let None = left_count {
                                stack.push_unchecked(left_sf);
                            }
                        }
                    }
                }
            }
        }

        // Assuming we don't have bugs, the count should be in the memo
        *memo
            .memo_get(&StackFrame {
                remaining_blinks,
                stone,
            })
            .expect("Count should have been calculated")
    }
}

// This is simple but slightly slower, using a key that combines the stone and blink count.
pub struct FlatHashMapMemo {
    memo: HashMap<StackFrame, usize>,
}

impl StoneMemo for FlatHashMapMemo {
    fn empty() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    fn memo_get(&self, key: &StackFrame) -> Option<&usize> {
        self.memo.get(key)
    }

    fn memo_insert(&mut self, key: StackFrame, value: usize) {
        self.memo.insert(key, value);
    }

    fn summarise(&self) {
        println!(
            "FlathashMapMemo summary. Memoised count: {}",
            self.memo.len()
        )
    }
}

// This is slightly faster. It stores a hashset per blink count.
struct IndexedHashMapsMemo {
    memo: [HashMap<u64, usize>; MAX_BLINKS_PART2 + 1],
}

impl StoneMemo for IndexedHashMapsMemo {
    fn empty() -> Self {
        Self {
            memo: std::array::from_fn(|_| HashMap::new()),
        }
    }

    fn memo_get(&self, key: &StackFrame) -> Option<&usize> {
        unsafe {
            self.memo
                .get_unchecked(key.remaining_blinks)
                .get(&key.stone)
        }
    }

    fn memo_insert(&mut self, key: StackFrame, value: usize) {
        unsafe {
            self.memo
                .get_unchecked_mut(key.remaining_blinks)
                .insert(key.stone, value)
        };
    }

    fn summarise(&self) {
        println!(
            "FlathashMapMemo summary. Memoised count: {}",
            self.memo
                .iter()
                .enumerate()
                .map(|(i, m)| format!("{i}: {}", m.len()))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let numbers = parse_input(input);
    let sc: LeftLoopingMemoisedRecursion<IndexedHashMapsMemo> = Default::default();

    sc.count_multiple_stones(&numbers, MAX_BLINKS_PART1)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let numbers = parse_input(input);
    let sc: LeftLoopingMemoisedRecursion<IndexedHashMapsMemo> = Default::default();

    sc.count_multiple_stones(&numbers, MAX_BLINKS_PART2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day11.txt");
    const INPUT_PARSED: [u64; 8] = [2, 54, 992917, 5270417, 2514, 28561, 0, 990];
    const PART1_ANSWER: usize = 222461;
    const PART2_ANSWER: usize = 264350935776416;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT), INPUT_PARSED);
    }

    #[test]
    fn test_count_digits_loop() {
        let counts = INPUT_PARSED
            .iter()
            .map(|&n| count_digits_loop(n))
            .collect::<Vec<_>>();
        let expected = [1, 2, 6, 7, 4, 5, 1, 3];

        assert_eq!(counts, expected);
    }

    #[test]
    fn test_stone_rule_0() {
        assert_eq!(stone_rule(0), (1, None));
    }

    #[test]
    fn test_stone_rule_1() {
        assert_eq!(stone_rule(1), (2024, None));
    }

    #[test]
    fn test_stone_rule_2() {
        assert_eq!(stone_rule(2), (4048, None));
    }

    #[test]
    fn test_stone_rule_7() {
        assert_eq!(stone_rule(7), (14168, None));
    }

    #[test]
    fn test_stone_rule_4() {
        assert_eq!(stone_rule(4), (8096, None));
    }

    #[test]
    fn test_stone_rule_11() {
        assert_eq!(stone_rule(11), (1, Some(1)));
    }

    #[test]
    fn test_stone_rule_111() {
        assert_eq!(stone_rule(111), (111 * 2024, None));
    }

    #[test]
    fn test_stone_rule_1111() {
        assert_eq!(stone_rule(1111), (11, Some(11)));
    }

    #[test]
    fn test_stone_rule_1110() {
        assert_eq!(stone_rule(1110), (11, Some(10)));
    }

    #[test]
    fn test_stone_rule_1011() {
        assert_eq!(stone_rule(1011), (10, Some(11)));
    }

    #[test]
    fn test_stone_rule_2024() {
        assert_eq!(stone_rule(2024), (20, Some(24)));
    }

    #[test]
    fn test_stone_rule() {
        assert_eq!(stone_rule(125), (253000, None));
        assert_eq!(stone_rule(253000), (253, Some(0)));
        assert_eq!(stone_rule(253), (512072, None));
        assert_eq!(stone_rule(512072), (512, Some(72)));
        assert_eq!(stone_rule(512), (1036288, None));
        assert_eq!(stone_rule(1036288), (2097446912, None));
    }

    fn test_count_stones<SC: StoneCounter<SM> + Default, SM: StoneMemo>() {
        let sc = SC::default();
        // 0 -> 1
        println!("1 steps from 0");
        assert_eq!(sc.count_stones(0, 1), 1);
        //  -> 2024
        println!("2 steps from 0");
        assert_eq!(sc.count_stones(0, 2), 1);
        //  -> 2 24 ->
        println!("3 steps from 0");
        assert_eq!(sc.count_stones(0, 3), 2);
        //  -> 4048 2 4
        println!("4 steps from 0");
        assert_eq!(sc.count_stones(0, 4), 4);
        println!("5 steps from 0");
        assert_eq!(sc.count_stones(0, 5), 4);
    }

    #[test]
    fn test_count_stones_flat_memo() {
        test_count_stones::<LeftLoopingMemoisedRecursion<FlatHashMapMemo>, _>();
    }

    #[test]
    fn test_count_stones_indexed_memo() {
        test_count_stones::<LeftLoopingMemoisedRecursion<IndexedHashMapsMemo>, _>();
    }

    fn test_example0<SC: StoneCounter<SM> + Default, SM: StoneMemo>() {
        let sc = SC::default();

        let input = [0, 1, 10, 99, 999];
        assert_eq!(sc.count_multiple_stones(&input, 1), 7);
    }

    #[test]
    fn test_example0_flat_memo() {
        test_example0::<LeftLoopingMemoisedRecursion<FlatHashMapMemo>, _>();
    }

    #[test]
    fn test_example0_indexed_memo() {
        test_example0::<LeftLoopingMemoisedRecursion<IndexedHashMapsMemo>, _>();
    }

    #[test]
    fn test_example1() {
        assert_eq!(part1("125 17"), 55312);
    }

    fn test_example1_steps<SC: StoneCounter<SM> + Default, SM: StoneMemo>() {
        let sc = SC::default();

        let input = [125, 17];
        println!("Step 1");
        assert_eq!(sc.count_multiple_stones(&input, 1), 3);
        println!("Step 2");
        assert_eq!(sc.count_multiple_stones(&input, 2), 4);
        println!("Step 3");
        assert_eq!(sc.count_multiple_stones(&input, 3), 5);
        println!("Step 4");
        assert_eq!(sc.count_multiple_stones(&input, 4), 9);
        println!("Step 5");
        assert_eq!(sc.count_multiple_stones(&input, 5), 13);
        println!("Step 6");
        assert_eq!(sc.count_multiple_stones(&input, 6), 22);
    }

    #[test]
    fn test_example1_steps_llmr_flat_memo() {
        test_example1_steps::<LeftLoopingMemoisedRecursion<FlatHashMapMemo>, _>();
    }

    #[test]
    fn test_example1_steps_llmr_indexed_memo() {
        test_example1_steps::<LeftLoopingMemoisedRecursion<IndexedHashMapsMemo>, _>();
    }

    #[test]
    fn test_example1_steps_lmnr_flat_memo() {
        test_example1_steps::<LoopingMemoisingNoRecursion<FlatHashMapMemo>, _>();
    }

    #[test]
    fn test_example1_steps_lmnr_indexed_memo() {
        test_example1_steps::<LoopingMemoisingNoRecursion<IndexedHashMapsMemo>, _>();
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), PART1_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), PART2_ANSWER);
    }
}

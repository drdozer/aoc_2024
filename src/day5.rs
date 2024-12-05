use std::{cmp::Ordering, str::Lines};

use aoc_runner_derive::aoc;

// Page numbers in the day 5 problem are 2-digit numbers.
// This fits into the lower 7 bits of a u8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageNumber(u8);

impl From<u8> for PageNumber {
    fn from(value: u8) -> Self {
        // We're going to assume that the input is valid outside of debug
        #[cfg(debug_assertions)]
        {
            assert!(value <= 127);
            assert!(value >= 10);
        }

        PageNumber(value)
    }
}

// A set of pages.
#[derive(Debug, Clone, Copy)]
struct PageSet(u128);

impl PageSet {
    fn empty() -> Self {
        Self(0)
    }

    fn insert(&mut self, page: PageNumber) {
        self.0 |= 1 << page.0;
    }

    fn contains(&self, page: PageNumber) -> bool {
        (self.0 & (1 << page.0)) != 0
    }
}

// The ordering rules can be represented as a map from page to the set of pages that must be after it.
// As we're limited to pages in the range 10-99, we can use a 100 element array, and ignore the lower 10 pages.
#[derive(Debug)]
struct OrderingRules([PageSet; 100]);

impl Default for OrderingRules {
    fn default() -> Self {
        Self([PageSet::empty(); 100])
    }
}

impl OrderingRules {
    fn add_rule(&mut self, before: PageNumber, after: PageNumber) {
        unsafe {
            self.0.get_unchecked_mut(before.0 as usize).insert(after);
        }
    }

    fn is_in_order(&self, before: PageNumber, after: PageNumber) -> bool {
        let before_set = unsafe { &self.0.get_unchecked(before.0 as usize) };
        before_set.contains(after)
    }
}

// Parses out a page number from the two bytes starting at the given offsset.
fn parse_page(bytes: &[u8], at: usize) -> PageNumber {
    let tens = unsafe { bytes.get_unchecked(at) } - b'0';
    let ones = unsafe { bytes.get_unchecked(at + 1) } - b'0';

    PageNumber(tens * 10 + ones)
}

fn parse_rules(lines: &mut Lines<'_>) -> OrderingRules {
    // Parse out the ordering rules.
    // We are assuming that they are well-formed.
    // In real production code, we'd take the speed hit and validate the input.
    let mut rules = OrderingRules::default();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            // The ordering rules are terminated by an empty line.
            break;
        }
        let line = line.as_bytes();
        #[cfg(debug_assertions)]
        {
            assert!(line.len() == 5);
            assert!(line[2] == b'|');
        }
        let before = parse_page(line, 0);
        let after = parse_page(line, 3);

        rules.add_rule(before, after);
    }
    rules
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let rules = parse_rules(&mut lines);

    let mut sum = 0;

    // Parse out the page lists, and check and sum them on the fly.
    // Again, we are assuming the input is well-formed:
    // * a comma-separated list
    // * two-digit pages
    // * no empty lines
    // * always an odd number of pages
    let mut pages = vec![];
    for line in lines {
        let line = line.as_bytes();
        pages.clear();
        for i in (0..line.len()).step_by(3) {
            pages.push(parse_page(line, i));
        }

        let mut well_ordered = true;
        for i in 0..pages.len() - 1 {
            well_ordered &=
                unsafe { rules.is_in_order(*pages.get_unchecked(i), *pages.get_unchecked(i + 1)) };
        }

        let middle_page = unsafe { pages.get_unchecked(pages.len() / 2) };

        sum += (middle_page.0 as usize) * well_ordered as usize;
    }

    sum
}

fn quickselect_median(pages: &mut [PageNumber], rules: &OrderingRules) -> PageNumber {
    let target_idx = pages.len() / 2;
    quickselect(pages, 0, pages.len() - 1, target_idx, rules)
}

fn quickselect(
    pages: &mut [PageNumber],
    left: usize,
    right: usize,
    k: usize,
    rules: &OrderingRules,
) -> PageNumber {
    if left == right {
        return unsafe { *pages.get_unchecked(left) };
    }

    let pivot_idx = partition(pages, left, right, rules);

    match k.cmp(&pivot_idx) {
        std::cmp::Ordering::Equal => unsafe { *pages.get_unchecked(k) },
        std::cmp::Ordering::Less => quickselect(pages, left, pivot_idx - 1, k, rules),
        std::cmp::Ordering::Greater => quickselect(pages, pivot_idx + 1, right, k, rules),
    }
}

fn partition(pages: &mut [PageNumber], left: usize, right: usize, rules: &OrderingRules) -> usize {
    // Use the rightmost element as pivot
    let pivot = unsafe { *pages.get_unchecked(right) };
    let mut i = left;

    for j in left..right {
        let in_order = unsafe { rules.is_in_order(*pages.get_unchecked(j), pivot) };

        // branchless swap using xor
        unsafe {
            let mask = -(in_order as i8) as u8;
            let a = pages.get_unchecked(i);
            let b = pages.get_unchecked(j);
            let xor = (a.0 ^ b.0) & mask;
            pages.get_unchecked_mut(i).0 ^= xor;
            pages.get_unchecked_mut(j).0 ^= xor;
        }

        i += in_order as usize;
    }
    //pages.swap(i, right);
    unsafe {
        let tmp = *pages.get_unchecked(i);
        *pages.get_unchecked_mut(i) = *pages.get_unchecked(right);
        *pages.get_unchecked_mut(right) = tmp;
    }

    i
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    // Most of this is the same code as part 1.

    let mut lines = input.lines();
    let rules = parse_rules(&mut lines);

    let mut sum = 0;

    let mut pages = vec![];
    for line in lines {
        let line = line.as_bytes();
        pages.clear();
        for i in (0..line.len()).step_by(3) {
            pages.push(parse_page(line, i));
        }

        let mut well_ordered = true;
        for i in 0..pages.len() - 1 {
            well_ordered =
                unsafe { rules.is_in_order(*pages.get_unchecked(i), *pages.get_unchecked(i + 1)) };
            // Not sure about the wisdom of the early exit here.
            if !well_ordered {
                break;
            }
        }

        // Not sure if we should use an early continue to skip the cost of sorting.
        if well_ordered {
            continue;
        }

        // We have a badly ordered page list. The task is to sort it by the ordering rules.
        //

        let middle_page = quickselect_median(&mut pages, &rules);
        sum += middle_page.0 as usize;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        let example = indoc! {
        "47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47"
            };
        assert_eq!(part1(example), 143);
    }

    #[test]
    fn part2_example() {
        let example = indoc! {
        "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47"
            };
        assert_eq!(part2(example), 123);
    }

    // #[test]
    fn ordering_rules_transitive() {
        // This test fails.
        // It is testing if the ordering rules are a full transtivie closure.
        let example = include_str!("../input/2024/day5.txt");
        let rules = parse_rules(&mut example.lines());

        for a in 10..=99 {
            for b in 10..=99 {
                for c in 10..=99 {
                    let a = PageNumber(a);
                    let b = PageNumber(b);
                    let c = PageNumber(c);
                    if rules.is_in_order(a, b) && rules.is_in_order(b, c) {
                        assert!(
                            rules.is_in_order(a, c),
                            "Rules not transitive: {a:?} < {b:?} < {c:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn ordering_rules_transitive_for_examples() {
        // This passes.
        //
        // It tests if the rules are a transitive closure for all example updates in the input data.
        let example = include_str!("../input/2024/day5.txt");
        let mut lines = example.lines();
        let rules = parse_rules(&mut example.lines());

        let mut pages = vec![];
        for line in lines {
            let line = line.as_bytes();
            pages.clear();
            for i in (0..line.len()).step_by(3) {
                pages.push(parse_page(line, i));
            }

            for a in pages.iter() {
                for b in pages.iter() {
                    for c in pages.iter() {
                        if rules.is_in_order(*a, *b) && rules.is_in_order(*b, *c) {
                            assert!(
                                rules.is_in_order(*a, *c),
                                "Rules not transitive: {a:?} < {b:?} < {c:?}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn ordering_rules_transitive_for_unsorted_examples() {
        // This passes.
        //
        // It tests if the rules are a transitive closure for all example updates in the input data that are badly ordered.
        let example = include_str!("../input/2024/day5.txt");
        let mut lines = example.lines();
        let rules = parse_rules(&mut example.lines());

        let mut pages = vec![];
        for line in lines {
            if line.is_empty() {
                continue;
            }
            let line = line.as_bytes();
            pages.clear();
            for i in (0..line.len()).step_by(3) {
                pages.push(parse_page(line, i));
            }

            let mut well_ordered = true;
            for i in 0..pages.len() - 1 {
                well_ordered &= unsafe {
                    rules.is_in_order(*pages.get_unchecked(i), *pages.get_unchecked(i + 1))
                };
            }

            if !well_ordered {
                for a in pages.iter() {
                    for b in pages.iter() {
                        for c in pages.iter() {
                            if rules.is_in_order(*a, *b) && rules.is_in_order(*b, *c) {
                                assert!(
                                    rules.is_in_order(*a, *c),
                                    "Rules not transitive: {a:?} < {b:?} < {c:?}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_pt2_correct_answer() {
        let example = include_str!("../input/2024/day5.txt");
        let answer = part2(example);
        assert_eq!(answer, 5180);
    }
}

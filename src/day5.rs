use aoc_runner_derive::aoc;

use crate::stack_vec::ArrayVec;

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

impl Default for PageNumber {
    fn default() -> Self {
        Self(0)
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

    fn intersect(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    fn size(&self) -> usize {
        self.0.count_ones() as usize
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

fn parse_rules(input: &[u8]) -> (OrderingRules, usize) {
    // Parse out the ordering rules.
    // We are assuming that they are well-formed.
    // In real production code, we'd take the speed hit and validate the input.
    let mut rules = OrderingRules::default();
    let mut pos = 0;

    loop {
        if unsafe { *input.get_unchecked(pos) } == b'\n' {
            break;
        }
        // Parse first number (2 digits)
        let tens = unsafe { *input.get_unchecked(pos) } - b'0';
        let ones = unsafe { *input.get_unchecked(pos + 1) } - b'0';
        let before = PageNumber(tens * 10 + ones);

        // Skip the pipe
        debug_assert_eq!(unsafe { *input.get_unchecked(pos + 2) }, b'|');

        // Parse second number (2 digits)
        let tens = unsafe { *input.get_unchecked(pos + 3) } - b'0';
        let ones = unsafe { *input.get_unchecked(pos + 4) } - b'0';
        let after = PageNumber(tens * 10 + ones);

        rules.add_rule(before, after);

        // Skip newline and move to next line
        pos += 6;
    }

    // we need to skip the separating newline, so return pos + 1
    (rules, pos + 1)
}

type Vec32<T> = ArrayVec<T, 32>;

fn parse_page_list(input: &[u8], at: usize) -> (Vec32<PageNumber>, PageSet, usize) {
    let mut pages = Vec32::new();
    let mut page_set = PageSet::empty();
    let mut pos = at;

    while pos < input.len() - 2 {
        // println!("pos: {}", pos);
        // println!("Parsing {:?}", std::str::from_utf8(&input[pos..pos + 3]));
        let tens = unsafe { *input.get_unchecked(pos) } - b'0';
        let ones = unsafe { *input.get_unchecked(pos + 1) } - b'0';
        let sep = unsafe { *input.get_unchecked(pos + 2) };
        let after = PageNumber(tens * 10 + ones);
        page_set.insert(after);

        unsafe {
            pages.push_unchecked(after);
        }

        pos += 3;

        if sep == b'\n' {
            break;
        }
    }
    // handle the special case of an input with a final line that's not newline-terminated
    if pos == input.len() - 2 {
        let tens = unsafe { *input.get_unchecked(pos) } - b'0';
        let ones = unsafe { *input.get_unchecked(pos + 1) } - b'0';
        let after = PageNumber(tens * 10 + ones);

        unsafe {
            pages.push_unchecked(after);
        }
        pos += 3;
    }

    (pages, page_set, pos)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let (rules, start) = parse_rules(input);

    // println!("input length: {}", input.len());
    // println!("parsed up to {}", start);

    let mut sum = 0;

    // Parse out the page lists, and check and sum them on the fly.
    // Again, we are assuming the input is well-formed:
    // * a comma-separated list
    // * two-digit pages
    // * no empty lines
    // * always an odd number of pages
    // * ends with a newline
    let mut pos = start;
    while pos < input.len() {
        // println!("pos: {}", pos);
        let (pages, _, new_pos) = parse_page_list(input, pos);
        pos = new_pos;

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

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let (rules, start) = parse_rules(input);

    let mut sum = 0;
    let mut pos = start;

    while pos < input.len() {
        let (pages, all_pages, new_pos) = parse_page_list(input, pos);
        pos = new_pos;

        let mut well_ordered = true;
        for i in 0..pages.len() - 1 {
            well_ordered &=
                unsafe { rules.is_in_order(*pages.get_unchecked(i), *pages.get_unchecked(i + 1)) };
        }

        // Not sure if we should use an early continue to skip the cost of sorting.
        if well_ordered {
            continue;
        }

        // If we intersect the pages in this update with the rules, we get a total order.
        // This means that when we count the number of pages that are after a page,
        // this is exactly its position from the end of the list.
        // That is, the last page has zero following pages, the second-to-last page has one following page, etc.
        // So to find the median, we just need to loop over pages, and find the one that has the median number of pages following it.
        // This avoids an off-by-one error due to that last page having zero followers.
        // In effect, the rules table is a constant-time lookup of the page update position.
        let mid = pages.len() / 2;
        for i in 0..pages.len() {
            let p = unsafe { *pages.get_unchecked(i) };
            let gt_p = unsafe { rules.0.get_unchecked(p.0 as usize) };
            let gt_count = gt_p.intersect(&all_pages).size();
            if gt_count == mid {
                sum += p.0 as usize;
                break;
            }
        }
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
    97,13,75,29,47
    "
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
        97,13,75,29,47
        "
            };
        assert_eq!(part2(example), 123);
    }

    #[ignore]
    #[test]
    fn ordering_rules_transitive() {
        // This test fails.
        // It is testing if the ordering rules are a full transtivie closure.
        let example = include_str!("../input/2024/day5.txt");
        let (rules, _) = parse_rules(example.as_bytes());

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
        let (rules, start) = parse_rules(example.as_bytes());

        let mut pages = vec![];
        for line in example[start..].lines() {
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
        let (rules, start) = parse_rules(example.as_bytes());

        let mut pages = vec![];
        for line in example[start..].lines() {
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

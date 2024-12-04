use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Lists(Vec<u32>, Vec<u32>);

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Lists {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    // Scruffy parsing of the input file, with no real error handling.
    for line in input.lines() {
        let mut pair = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
        if let (Some(id_left), Some(id_right), None) = (pair.next(), pair.next(), pair.next()) {
            left_list.push(id_left);
            right_list.push(id_right);
        }
    }

    Lists(left_list, right_list)
}

#[aoc(day1, part1)]
pub fn part1(input: &Lists) -> u32 {
    let Lists(mut left_list, mut right_list) = input.clone();
    // We need to sum the absolte difference between items from the two lists,
    // smallest in each list to largest in each list.
    // A simple solution is to sort them, and then loop over the pairs.
    left_list.sort();
    right_list.sort();

    // The sum of pairs can be expressed as a comprehension like this.
    // Rust requires a type for `sum` to guide the `sum()` function.
    // I wish the type inference could do better.
    let sum: u32 = left_list
        .iter()
        .zip(right_list)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &Lists) -> u32 {
    let Lists(left_list, right_list) = input;

    // We need the sum of each item in list 1 by its frequency in list 2.
    // A simple solution is to counstruct a histogram of list 2 first,
    // which we can do using a HashMap.
    let mut hist = HashMap::new();
    for r in right_list {
        // The rust `entry` API is very powerful,
        // allowing us to modify data *inside* the HashMap.
        let c = hist.entry(r).or_insert(0);
        *c += 1;
    }

    let dist: u32 = left_list
        .iter()
        .map(|l| hist.get(l).unwrap_or(&0) * l)
        .sum();

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}

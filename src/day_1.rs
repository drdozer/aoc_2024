use std::collections::HashMap;

fn main() {
    println!("Hello AOC 2024 Day 1!");

    let input_file_name = "./input/day_1_puzzle_1.txt";

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    // Scruffy parsing of the input file, with no real error handling.
    for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
        let mut pair = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
        if let (Some(id_left), Some(id_right), None) = (pair.next(), pair.next(), pair.next()) {
            left_list.push(id_left);
            right_list.push(id_right);
        }
    }

    task_1(&mut left_list, &mut right_list);
    task_2(&left_list, &right_list);
}

fn task_1(left_list: &mut Vec<u32>, right_list: &mut Vec<u32>) {
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
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    println!("sum of differences: {:?}", sum);
}

fn task_2(left_list: &Vec<u32>, right_list: &Vec<u32>) {
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

    println!("distance: {:?}", dist);
}

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Record(Vec<u32>);
pub struct Records(Vec<Record>);

// Scruffy parsing into a a vector of vectors of integers,
// using a helper function.
fn parse_line(line: &str) -> Record {
    Record(
        line.split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>(),
    )
}

pub fn parse(input: &str) -> Records {
    let records = input.lines().map(parse_line).collect::<Vec<_>>();
    Records(records)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let records = parse(input);
    records.0.iter().filter(|r| is_safe(r)).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let records = parse(input);
    records.0.iter().filter(|r| is_nearly_safe(r)).count()
}

fn is_safe(record: &Record) -> bool {
    let Record(record) = record;

    // Short records are always safe, although I haven't seen any in the input.
    if record.len() < 2 {
        return true;
    }

    // Edge case: first pair are equal, so they violate the change rule,
    // but also because they are the same we can't infer the record ordering,
    // so we return early.
    if record[0] == record[1] {
        return false;
    }

    // Now that we know the first pair are different, we can infer the ordering.
    let record_is_ascending = is_ascending(record[0], record[1]);

    // For all pairs, validate them.
    for i in 1..record.len() {
        let (l, r) = (record[i - 1], record[i]);

        // They go the wrong way
        if is_ascending(l, r) != record_is_ascending {
            return false;
        }

        // They have the wrong change
        match l.abs_diff(r) {
            1 | 2 | 3 => (),
            _ => return false,
        }
    }

    // All test passed, so the record is safe.
    return true;
}

fn is_nearly_safe(record: &Record) -> bool {
    // Safe records are safe.
    if is_safe(record) {
        return true;
    }

    let Record(record) = record;

    // The record wasn't safe. Check if we can make it safe by removing one item.
    for i in 0..record.len() {
        // We need a new copy of the record as we're removing an item.
        let mut leave_i_out = record.clone();
        leave_i_out.remove(i);
        if is_safe(&Record(leave_i_out)) {
            return true;
        }
    }

    // There was no way to make it safe by removing a single item.
    false
}

// Helper to check if a pair of numbers are in ascending order.
// I personally get `<` and `>` confused, and it is used in more than one place,
// so I pulled it out here behind something with a descriptive name.
fn is_ascending(l: u32, r: u32) -> bool {
    l < r
}

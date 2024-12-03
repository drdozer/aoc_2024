fn main() {
    println!("Hello AOC 2024 Day 2!");
    let input_file_name = "./input/day_2_puzzle_1.txt";

    // Scruffy parsing into a a vector of vectors of integers,
    // using a helper function.
    fn parse_line(line: &str) -> Vec<u32> {
        line.split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    }

    let records = std::fs::read_to_string(input_file_name)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    // Task 1: Count the number of safe records.
    let safe_count = records.iter().filter(|r| is_safe(r)).count();
    println!("safe count: {:?}", safe_count);

    // Task 2: Count the number of nearly safe records.
    let neary_safe_count = records.iter().filter(|r| is_nearly_safe(r)).count();
    println!("nearly safe count: {:?}", neary_safe_count);
}

fn is_safe(record: &[u32]) -> bool {
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

fn is_nearly_safe(record: &[u32]) -> bool {
    // Safe records are safe.
    if is_safe(record) {
        return true;
    }

    // The record wasn't safe. Check if we can make it safe by removing one item.
    for i in 0..record.len() {
        // We need a new copy of the record as we're removing an item.
        let mut leave_i_out = Vec::from(record);
        leave_i_out.remove(i);
        if is_safe(&leave_i_out) {
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

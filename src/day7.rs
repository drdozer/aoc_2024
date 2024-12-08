use aoc_runner_derive::aoc;

use crate::stack_vec::ArrayVec;

const MAX_NUMBERS: usize = 12;
type NumberVec = ArrayVec<u64, MAX_NUMBERS>;

#[derive(Debug)]
pub struct CalibrationData {
    pub test_value: u64,
    pub numbers: NumberVec,
}

pub struct CalibrationDataIterator<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for CalibrationDataIterator<'a> {
    type Item = CalibrationData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        // Parse in the test value.
        // It is terminated by a colon.
        let mut test_value = 0;
        loop {
            let &c = unsafe { self.input.get_unchecked(self.pos) };
            if c == b':' {
                // skip the colon and following space
                self.pos += 2;
                break;
            }
            test_value *= 10;
            test_value += (c - b'0') as u64;
            self.pos += 1;
        }

        // Parse in the numbers.
        let mut numbers = ArrayVec::new();
        'numbers: loop {
            let mut n = 0;
            'number: loop {
                if self.pos >= self.input.len() {
                    unsafe { numbers.push_unchecked(n) };
                    break 'numbers;
                }
                let &c = unsafe { self.input.get_unchecked(self.pos) };
                if c == b' ' {
                    unsafe { numbers.push_unchecked(n) };
                    self.pos += 1;
                    break 'number;
                } else if c == b'\n' {
                    unsafe { numbers.push_unchecked(n) };
                    self.pos += 1;
                    break 'numbers;
                } else {
                    n *= 10;
                    n += (c - b'0') as u64;
                    self.pos += 1;
                }
            }
        }

        Some(CalibrationData {
            test_value,
            numbers,
        })
    }
}

pub fn parse_calibration_data(input: &str) -> CalibrationDataIterator {
    CalibrationDataIterator {
        input: input.as_bytes(),
        pos: 0,
    }
}

pub fn find_solution_1(data: &CalibrationData) -> bool {
    // The objective is to find any way to combine the calibration data to produce the test value.
    // We can brute-force this by tring all combinations of sums and products.
    // Terminate when we are at the end of the numbers and have got the test value.
    // However, in the general case, and quite likely in the cases where there is no solution, this is O(p^2).
    //
    // Alternatively ...
    //
    // Whatever the expression is, we know that it must evaluate to test_value if this has solutions.
    // If we take the last number, then the expression evaluated for all numbers prior to the last number must be either:
    // - test_value - last_number
    // - test_value / last_number
    //
    // However, in the case of division, it must exactly divide.
    // If there is a remainder, then there is no expression for the prefix which can be multiplied with the last number to get test_value.
    //
    // We should be able to then recurse from the end to the beginning.
    // Underflow during the recursion means we do not have a solution.
    // Reaching the beginning, and the running value being equal to the first value means we have a solution.
    //
    //

    #[derive(Debug, Clone, Copy, Default)]
    enum State {
        #[default]
        Multiply,
        Sum,
        Dead,
    }
    #[derive(Debug, Clone, Copy, Default)]
    struct StackFrame {
        current_target: u64,
        state: State,
    }

    debug_assert!(data.numbers.len() <= MAX_NUMBERS);
    let mut stack = [StackFrame::default(); MAX_NUMBERS];
    let mut stack_pos = data.numbers.len() - 1;
    unsafe { stack.get_unchecked_mut(stack_pos).current_target = data.test_value };

    loop {
        debug_assert!(stack_pos < data.numbers.len());
        unsafe {
            let mut pop_stack = false;
            let current_target = stack.get_unchecked(stack_pos).current_target;
            let current_number = *data.numbers.get_unchecked(stack_pos);
            let state = stack.get_unchecked(stack_pos).state;

            if stack_pos == 0 {
                if current_target == current_number {
                    // println!("Found solution!");
                    return true;
                }

                // not a solution
                pop_stack = true;
            } else {
                match state {
                    State::Multiply => {
                        // We will try multiplication.
                        // We should expect this to typically fail, as most numbers don't divide cleanaly.

                        // Update the state immeiately. We always try sum after multiplication.
                        stack.get_unchecked_mut(stack_pos).state = State::Sum;

                        // This is horrific -- we're working around a%b doing a check for b=0.
                        let divides = current_target
                            .checked_rem(current_number)
                            .map(|r| r == 0)
                            .unwrap_or(false);
                        if divides {
                            // It divided cleanly!
                            // We can now decrement the stack position, and recurse.
                            stack_pos -= 1;

                            let div = current_target / current_number;
                            let next_frame = stack.get_unchecked_mut(stack_pos);
                            next_frame.current_target = div;
                            next_frame.state = State::Multiply;
                        }
                    }
                    State::Sum => {
                        // Update the state immediately. We always are dead after addition.
                        stack.get_unchecked_mut(stack_pos).state = State::Dead;
                        // We will try addition.
                        if current_target < current_number {
                            // It would underflow, so this can't be a solution.
                            pop_stack = true;
                        } else {
                            // We can subtract!
                            // We can now decrement the stack position, and recurse.
                            stack_pos -= 1;
                            let next_frame = stack.get_unchecked_mut(stack_pos);
                            next_frame.current_target = current_target - current_number;
                            next_frame.state = State::Multiply;
                        }
                    }
                    State::Dead => {
                        // We've processed all the options for this stack level, so will pop.
                        pop_stack = true;
                    }
                }
            }

            if pop_stack {
                // We've processed all the options for this stack level, and need to return.
                stack_pos += 1;
                if stack_pos == data.numbers.len() {
                    // We've processed all the possibilities for all numbers.
                    // There is no solution.
                    return false;
                }
            }
        }
    }
}

fn num_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn concat_digits(a: u64, b: u64) -> u64 {
    a * 10u64.pow(num_digits(b)) + b
}

pub fn find_solution_2(data: &CalibrationData) -> bool {
    // This is essentially the same as find_solution_1, except that we need to also handle digit concatenations
    //
    // Concatenation is a bit tricky to handle.
    // a || b is equivalent to a * 10^num_digits(b) + b.
    //
    // So if we have a current target of 123456, and the current numberis 56,
    // it is possible that we reached it by concatenation,
    // in which case the target for the next step would be 1234.
    // If the prefix of the target is anything else, then it could not be reached by concatenation.
    #[derive(Debug, Clone, Copy, Default)]
    enum State {
        #[default]
        Multiply,
        Sum,
        Concat,
        Dead,
    }
    #[derive(Debug, Clone, Copy, Default)]
    struct StackFrame {
        current_target: u64,
        state: State,
    }

    debug_assert!(data.numbers.len() <= MAX_NUMBERS);
    let mut stack = [StackFrame::default(); MAX_NUMBERS];
    let mut stack_pos = data.numbers.len() - 1;
    unsafe { stack.get_unchecked_mut(stack_pos).current_target = data.test_value };

    loop {
        debug_assert!(stack_pos < data.numbers.len());
        unsafe {
            let mut pop_stack = false;
            let current_target = stack.get_unchecked(stack_pos).current_target;
            let current_number = *data.numbers.get_unchecked(stack_pos);
            let state = stack.get_unchecked(stack_pos).state;

            if stack_pos == 0 {
                if current_target == current_number {
                    // println!("Found solution!");
                    return true;
                }

                // not a solution
                pop_stack = true;
            } else {
                match state {
                    State::Multiply => {
                        // We will try multiplication.
                        // We should expect this to typically fail, as most numbers don't divide cleanaly.

                        // Update the state immeiately. We always try sum after multiplication.
                        stack.get_unchecked_mut(stack_pos).state = State::Concat;

                        // This is horrific -- we're working around a%b doing a check for b=0.
                        let divides = current_target
                            .checked_rem(current_number)
                            .map(|r| r == 0)
                            .unwrap_or(false);
                        if divides {
                            // It divided cleanly!
                            // We can now decrement the stack position, and recurse.
                            stack_pos -= 1;

                            let div = current_target / current_number;
                            let next_frame = stack.get_unchecked_mut(stack_pos);
                            next_frame.current_target = div;
                            next_frame.state = State::Multiply;
                        }
                    }
                    State::Concat => {
                        // Update the state immediately. We are always dead after concatenation.
                        stack.get_unchecked_mut(stack_pos).state = State::Sum;
                        let d = num_digits(current_number);
                        let pow_10 = 10u64.pow(d);
                        let lower_digits_match = current_target
                            .checked_rem(pow_10)
                            .map(|ld| ld == current_number)
                            .unwrap_or(false);

                        if lower_digits_match {
                            // This could be a potential concatenation.
                            stack_pos -= 1;

                            let next_frame = stack.get_unchecked_mut(stack_pos);
                            next_frame.current_target = current_target / pow_10;
                            next_frame.state = State::Multiply;
                        }
                    }
                    State::Sum => {
                        // Update the state immediately. We always concatenate after addition.
                        stack.get_unchecked_mut(stack_pos).state = State::Dead;
                        // We will try addition.
                        if current_target < current_number {
                            // It would underflow, so this can't be a solution.
                            pop_stack = true;
                        } else {
                            // We can subtract!
                            // We can now decrement the stack position, and recurse.
                            stack_pos -= 1;
                            let next_frame = stack.get_unchecked_mut(stack_pos);
                            next_frame.current_target = current_target - current_number;
                            next_frame.state = State::Multiply;
                        }
                    }
                    State::Dead => {
                        // We've processed all the options for this stack level, so will pop.
                        pop_stack = true;
                    }
                }
            }

            if pop_stack {
                // We've processed all the options for this stack level, and need to return.
                stack_pos += 1;
                if stack_pos == data.numbers.len() {
                    // We've processed all the possibilities for all numbers.
                    // There is no solution.
                    return false;
                }
            }
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    parse_calibration_data(input)
        .filter(find_solution_1)
        .map(|c| c.test_value)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    parse_calibration_data(input)
        .filter(find_solution_2)
        .map(|c| c.test_value)
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    fn example_input() -> &'static str {
        indoc! {
            "190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
            "
        }
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(example_input()), 3749);
    }

    #[test]
    fn test_find_solution_1() {
        let data = parse_calibration_data(example_input()).next().unwrap();
        assert_eq!(find_solution_1(&data), true);
    }

    #[test]
    fn test_input() {
        let max_numbers = parse_calibration_data(include_str!("../input/2024/day7.txt"))
            .map(|x| x.numbers.len())
            .max()
            .unwrap();
        assert_eq!(MAX_NUMBERS, max_numbers);
    }

    #[test]
    fn test_part1() {
        let result = part1(include_str!("../input/2024/day7.txt"));
        assert_eq!(result, 945512582195);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(example_input()), 11387);
    }
    
    #[test]
    fn test_part2() {
        let result = part2(include_str!("../input/2024/day7.txt"));
        assert_eq!(result, 271691107779347);
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(99), 2);
        assert_eq!(num_digits(100), 3);
        assert_eq!(num_digits(999), 3);
        assert_eq!(num_digits(1000), 4);
        assert_eq!(num_digits(9999), 4);
        assert_eq!(num_digits(10000), 5);
        assert_eq!(num_digits(99999), 5);
        assert_eq!(num_digits(100000), 6);
        assert_eq!(num_digits(999999), 6);
        assert_eq!(num_digits(1000000), 7);
        assert_eq!(num_digits(9999999), 7);
        assert_eq!(num_digits(10000000), 8);
        assert_eq!(num_digits(99999999), 8);
        assert_eq!(num_digits(100000000), 9);
        assert_eq!(num_digits(999999999), 9);
        assert_eq!(num_digits(1000000000), 10);
    }

    #[test]
    fn test_concat_digits() {
        assert_eq!(concat_digits(1, 1), 11);
        assert_eq!(concat_digits(1, 10), 110);
        assert_eq!(concat_digits(1, 99), 199);
        assert_eq!(concat_digits(12, 10), 1210);
    }
}

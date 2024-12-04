use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::anychar,
    multi::many0,
    sequence::tuple,
    Parser,
};
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    // We're looking for things like `mul(123,456)`.
    // This can be matched with a simple regex.
    // We have to escape elipses, which makes it a bit difficult to read.
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // We can now sum all the matches.
    mul_re
        .captures_iter(&input)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .sum()
}

// Task 2 is a bit more complicated.
// There are now three different commands:
//   - do: all subsequent mul commands should be applied
//   - dont: all subsequent mul commands should be ignored
//   - mul: multiply the current sum by the two numbers, but only apply if do is active
// Also, there is junk in the file, which we need to ignore.
//
// My solution is to create a simple machine.
//   - `EvalState`: The machine's evaluation state
//   - `Command`: A command that can be executed to update the state
//   - `eval`: Execute a command, updating the state
// The default eval state captures the starting rules.

#[derive(Debug)]
enum Command {
    Do,
    Dont,
    Mul(u32, u32),
    Noop,
}

#[derive(Debug)]
struct EvalState {
    sum: u32,
    apply_mul: bool,
}

impl Default for EvalState {
    fn default() -> Self {
        Self {
            sum: 0,
            apply_mul: true,
        }
    }
}

impl EvalState {
    fn eval(&mut self, c: &Command) {
        match c {
            Command::Do => self.apply_mul = true,
            Command::Dont => self.apply_mul = false,
            Command::Mul(l, r) => {
                if self.apply_mul {
                    self.sum += l * r;
                }
            }
            Command::Noop => (),
        }
    }
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    // There are now three different commands, as well as junk.
    // I considered using a complicated regex, but found it impossible to read.
    // So instead, I've built a small nom parser.
    // The API is a little ideosyncratic, but for tasks like this it works well.

    let p_don_t = tag::<&str, &str, ()>("don't()");
    let p_do = tag("do()");
    let p_mul = tag("mul(");
    let p_rparens = tag(")");
    let p_comma = tag(",");

    let p_digits = || {
        take_while_m_n(1, 3, |c: char| c.is_ascii_digit()).map(|s: &str| s.parse::<u32>().unwrap())
    };

    let p_don_t_expr = p_don_t.map(|_| Command::Dont);
    let p_do_expr = p_do.map(|_| Command::Do);
    let p_mul_expr = tuple((p_mul, p_digits(), p_comma, p_digits(), p_rparens))
        .map(|(_, a, _, b, _)| Command::Mul(a, b));
    let p_noop_expr = anychar.map(|_| Command::Noop);

    let p_expr = p_don_t_expr.or(p_do_expr).or(p_mul_expr).or(p_noop_expr);
    let mut parser = many0(p_expr);

    // Now that we've built up the parser, we can parse the input.
    match parser.parse(input) {
        Ok((remaining, commands)) => {
            debug_assert!(remaining.is_empty());

            // This evaluates the commands, updating the state.
            let mut state = EvalState::default();
            commands.iter().for_each(|c| state.eval(c));

            return state.sum;
        }
        Err(e) => {
            panic!("Error parsing: {:?}", e);
        }
    }
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

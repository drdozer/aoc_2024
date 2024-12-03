use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::anychar,
    multi::many0,
    sequence::tuple,
    Parser,
};
use regex::Regex;

fn main() {
    println!("Hello AOC 2024 Day 3!");
    let input_file_name = "./input/day_3_puzzle_1.txt";

    // This task doesn't require any particular input parsing,
    // so we can just slurp it into a string.
    let input = std::fs::read_to_string(input_file_name).unwrap();

    task_1(&input);
    task_2(&input);
}

fn task_1(input: &str) {
    // We're looking for things like `mul(123,456)`.
    // This can be matched with a simple regex.
    // We have to escape elipses, which makes it a bit difficult to read.
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // We can now sum all the matches.
    let sum: i32 = mul_re
        .captures_iter(&input)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .sum();

    println!("Sum: {}", sum);
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
    Mul(i32, i32),
    Noop,
}

#[derive(Debug)]
struct EvalState {
    sum: i32,
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

fn task_2(input: &str) {
    // There are now three different commands, as well as junk.
    // I considered using a complicated regex, but found it impossible to read.
    // So instead, I've built a small nom parser.
    // The API is a little ideosyncratic, but for tasks like this it works well.

    let p_don_t = tag::<&str, &str, ()>("don't");
    let p_do = tag("do");
    let p_mul = tag("mul");
    let p_parens = tag("()");
    let p_lparens = tag("(");
    let p_rparens = tag(")");
    let p_comma = tag(",");

    let p_digits = || {
        take_while_m_n(1, 3, |c: char| c.is_ascii_digit()).map(|s: &str| s.parse::<i32>().unwrap())
    };

    let p_don_t_expr = p_don_t.and(&p_parens).map(|_| Command::Dont);
    let p_do_expr = p_do.and(&p_parens).map(|_| Command::Do);
    let p_mul_expr = tuple((p_mul, p_lparens, p_digits(), p_comma, p_digits(), p_rparens))
        .map(|(_, _, a, _, b, _)| Command::Mul(a, b));
    let p_noop_expr = anychar.map(|_| Command::Noop);

    let p_expr = p_don_t_expr.or(p_do_expr).or(p_mul_expr).or(p_noop_expr);
    let mut parser = many0(p_expr);

    // Now that we've built up the parser, we can parse the input.
    match parser.parse(input) {
        Ok((remaining, commands)) => {
            println!("Commands: {:?}", commands);
            println!("Remaining: {:?}", remaining);

            assert!(remaining.is_empty());

            // This evaluates the commands, updating the state.
            let mut state = EvalState::default();
            commands.iter().for_each(|c| state.eval(c));

            println!("Final state: {:?}", state);
        }
        Err(e) => {
            println!("Error parsing: {:?}", e);
        }
    }
}

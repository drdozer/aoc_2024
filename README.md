# Advent of Code 2024

These are my solves of the Advent Of Code 2024 puzzles.
I'm mostly (exclusively?) solving these using rust.
This is fun, disposable code, not production grade stuff, so be prepared to embrace the scruffy!
It is obsessively over-commented, to describe both my thought process, 
and any quirks of rust that I think are notable.
It may be useful to learn from, if only to learn what not to do.

[https://adventofcode.com/2024](https://adventofcode.com/2024)
[https://codspeed.io/advent](https://codspeed.io/advent)

## Running

The project builds against Nightly.

To run the code, you need to have rust and cargo installed.
You also need to install the codspeed aoc cli, by following the codspeed advent instructions.
You can then run the code with the cli command:


```bash
cargo aoc -d 2
```

Substitute in the day number you want to run.
This will print out the task 1 and 2 solutions and their runtimes.

#![feature(impl_trait_in_assoc_type)]
#![feature(generic_const_exprs)]

use aoc_runner;
use aoc_runner_derive::aoc_lib;

pub mod bitset;
pub mod stack_vec;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

aoc_lib! { year = 2024 }

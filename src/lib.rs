#![feature(impl_trait_in_assoc_type)]
#![feature(hash_set_entry)]
#![feature(core_intrinsics)]
#![feature(strict_overflow_ops)]
#![feature(slice_internals)]
#![feature(portable_simd)]

use aoc_runner;
use aoc_runner_derive::aoc_lib;

pub mod bitset;
pub mod stack_vec;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

aoc_lib! { year = 2024 }

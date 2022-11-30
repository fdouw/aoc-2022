use crate::solutions::*;
use std::{env, fs, io};

mod lib_2017;
#[allow(dead_code)]
mod lib_aoc;
mod solutions;
mod tests;

fn read_input_file(folder: &str, day: u8) -> io::Result<String> {
    let cwd = env::current_dir()?;
    let filepath = cwd.join(folder).join(format!("{:02}", day));
    fs::read_to_string(filepath)
}

fn solve(day: u8, verbose: bool) {
    let solvers = [
        // Add day 1 twice, to make indexing nicer; should not be used
        day01::solve,
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
        day08::solve,
        day09::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
        day14::solve,
        day15::solve,
        day16::solve,
        day17::solve,
        day18::solve,
        day19::solve,
        day20::solve,
        day21::solve,
        day22::solve,
        day23::solve,
        day24::solve,
        day25::solve,
    ];
    match read_input_file("inputs", day) {
        Ok(input) => match day {
            1..=25 => {
                let (part1, part2) = solvers[usize::from(day)](input, verbose);
                println!("Day {day}\n Part 1: {part1}\n Part 2: {part2}\n");
            }
            _ => println!("Not a valid day: {}", day),
        },
        Err(e) => println!("Day {}: [ERROR] could not read input data ({})", day, e),
    }
}

fn main() {
    let mut day = 0;
    let mut verbose = false;
    for arg in env::args().skip(1) {
        if arg.starts_with('-') {
            if arg == "-v" || arg == "--verbose" {
                verbose = true;
            }
        } else if let Ok(x) = arg.parse() {
            day = x;
        }
    }

    if day == 0 {
        for d in 1..=25 {
            solve(d, verbose);
        }
    } else {
        solve(day, verbose);
    }
}

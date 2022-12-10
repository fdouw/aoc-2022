use crate::lib_aoc;
use std::collections::HashSet;

pub fn solve(input: String, verbose: bool) -> (String, String) {
    // 0th element is a dummy, to work 1-based, 1st is the initial state of x
    // We know from the problem statement (part 2) that we need 240 items (plus 2)
    let mut values = Vec::with_capacity(242);
    values.push(0);
    values.push(1);

    let mut x = 1;
    values.extend(input.trim().lines().flat_map(|l| {
        if l == "noop" {
            /* do nothing */
            vec![x]
        } else if l.starts_with("addx") {
            let val: i32 = l
                .split_whitespace()
                .nth(1)
                .expect("there should be 2 parts to this command")
                .parse()
                .expect("there should be an `int` after addx");
            let old_x = x;
            x += val;
            vec![old_x, x]
        } else {
            unreachable!("entries should be 'noop' or start with 'addx'")
        }
    }));

    let part1: i32 = values
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(cycle, x)| (cycle as i32) * x)
        .sum();

    // Part 2
    let points: HashSet<(u32, u32)> = HashSet::from_iter(
        values
            .iter()
            .skip(1)
            .enumerate()
            .filter_map(|(cycle, x_reg)| {
                let x = cycle as u32 % 40;
                if x_reg.abs_diff(x as i32) < 2 {
                    Some((x, cycle as u32 / 40))
                } else {
                    None
                }
            }),
    );

    if verbose {
        lib_aoc::print_block_output(6, 40, points.clone());
    }

    let part2 = lib_aoc::parse_block_output(&points, 40);

    (part1.to_string(), part2)
}

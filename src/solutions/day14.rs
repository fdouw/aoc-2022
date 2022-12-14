use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut depth = 0;
    let mut blocks = HashMap::new();

    // Read in the cave system from input
    for line in input.trim().lines() {
        let mut iter = line.split(" -> ");
        let mut prev: (u32, u32) = iter
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        if prev.1 > depth {
            depth = prev.1;
        }

        for (x, y) in iter.map(|l| {
            l.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        }) {
            if y > depth {
                depth = y;
            }

            // Lines of rock can go left-right, but also right-left (and down, but also up)
            for i in prev.0.min(x)..=x.max(prev.0) {
                for j in prev.1.min(y)..=y.max(prev.1) {
                    blocks.insert((i, j), Block::Rock);
                }
            }

            prev = (x, y);
        }
    }

    // println!("Cave before sand:");
    // print_cave(&blocks, (400, 0), (600, depth + 2));

    // Simulate sand dropping
    // If we reach y == depth, then the sand is falling into the abyss
    let mut sand_count = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    'outer: loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if part1 == 0 && y == depth {
                // If we haven't set `part1` yet, then the first time at depth means the sand is falling into the abyss
                part1 = sand_count;
            }

            if y < depth + 1 {
                // Still above the floor, therefor still falling
                if !blocks.contains_key(&(x, y + 1)) {
                    y += 1;
                    continue;
                }
                if !blocks.contains_key(&(x - 1, y + 1)) {
                    y += 1;
                    x -= 1;
                    continue;
                }
                if !blocks.contains_key(&(x + 1, y + 1)) {
                    y += 1;
                    x += 1;
                    continue;
                }

                if x == 500 && y == 0 {
                    // We haven't left the drop point, so the source is blocked
                    // Don't forget to count the unit blocking the source
                    part2 = sand_count + 1;
                    break 'outer;
                }
            }
            blocks.insert((x, y), Block::Sand);
            sand_count += 1;
            break;
        }
    }

    // println!("Cave with sand:");
    // print_cave(&blocks, (400, 0), (600, depth + 2));

    (part1.to_string(), part2.to_string())
}

enum Block {
    Air,
    Rock,
    Sand,
}

fn print_cave(blocks: &HashMap<(u32, u32), Block>, start: (u32, u32), end: (u32, u32)) {
    // Mark the drop column
    for _ in 0..(500 - start.0) {
        print!(" ");
    }
    println!("v");

    for y in start.1..=end.1 {
        for x in start.0..=end.0 {
            match blocks.get(&(x, y)).unwrap_or(&Block::Air) {
                Block::Air => print!("."),
                Block::Rock => print!("#"),
                Block::Sand => print!("0"),
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(
            ("24".to_string(), "93".to_string()),
            solve(input.to_string(), false)
        );
    }
}

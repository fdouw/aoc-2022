pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let part1: u32 = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            let b: Vec<char> = b.chars().collect();
            let mut val = 0;
            for c in a.chars() {
                if b.contains(&c) {
                    let d: u32 = c.into();
                    val = match d {
                        97..=122 => d - 96, // a-z => 1-26
                        65..=90 => d - 38,  // A-Z => 27-52
                        _ => panic!("Invalid character"),
                    };
                    break;
                }
            }
            val
        })
        .sum();

    // Part 2
    let mut part2 = 0;
    let rucksacks: Vec<&str> = input.lines().collect();
    for i in (0..rucksacks.len()).step_by(3) {
        let c: u32 = rucksacks[i]
            .chars()
            .filter(|item| rucksacks[i + 1].contains(*item))
            .filter(|item| rucksacks[i + 2].contains(*item))
            .next()
            .unwrap()
            .into();
        part2 += match c {
            97..=122 => c - 96, // a-z => 1-26
            65..=90 => c - 38,  // A-Z => 27-52
            _ => panic!("Invalid character"),
        }
    }

    (part1.to_string(), part2.to_string())
}

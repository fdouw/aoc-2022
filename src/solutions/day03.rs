use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let part1: u16 = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .filter_map(|(a, b)| {
            let b: Vec<char> = b.chars().collect(); // Vector seems faster than HashSet for these small sets
            a.chars().find(|c| b.contains(c)).map(priority)
        })
        .sum();

    let part2: u16 = input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            a.chars()
                .filter(|item| b.contains(*item))
                .find(|item| c.contains(*item))
                .expect("each triplet should have exactly 1 character in common")
        })
        .map(priority)
        .sum();

    (part1.to_string(), part2.to_string())
}

fn priority(item_type: char) -> u16 {
    match item_type {
        'a'..='z' => item_type as u16 - 96,
        'A'..='Z' => item_type as u16 - 38,
        _ => panic!("Invalid character: {item_type}"),
    }
}

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    //     let input = "A Y
    // B X
    // C Z";

    // Tried something clever at first, but there are only 9 cases...
    let (part1, part2) = input
        .lines()
        .map(|s| match s {
            "A X" => (3 + 1, 0 + 3),
            "A Y" => (6 + 2, 3 + 1),
            "A Z" => (0 + 3, 6 + 2),
            "B X" => (0 + 1, 0 + 1),
            "B Y" => (3 + 2, 3 + 2),
            "B Z" => (6 + 3, 6 + 3),
            "C X" => (6 + 1, 0 + 2),
            "C Y" => (0 + 2, 3 + 3),
            "C Z" => (3 + 3, 6 + 1),
            _ => panic!("Invalid round"),
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    (part1.to_string(), part2.to_string())
}

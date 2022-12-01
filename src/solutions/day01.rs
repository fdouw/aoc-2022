pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut calorie_counts: Vec<u32> = input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect();
    calorie_counts.sort_by(|a, b| a.cmp(b));

    let part1 = calorie_counts.iter().max().unwrap();
    let part2: u32 = calorie_counts.iter().rev().take(3).sum();

    (part1.to_string(), part2.to_string())
}

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    //     let input = "A Y
    // B X
    // C Z";

    const LOSE: u16 = 0;
    const DRAW: u16 = 3;
    const WIN: u16 = 6;

    const ROCK: u16 = 1;
    const PAPER: u16 = 2;
    const SCISSORS: u16 = 3;

    // Tried something clever at first, but there are only 9 cases...
    let (part1, part2) = input
        .lines()
        .map(|s| match s {
            "A X" => (DRAW + ROCK, LOSE + SCISSORS),
            "A Y" => (WIN + PAPER, DRAW + ROCK),
            "A Z" => (LOSE + SCISSORS, WIN + PAPER),
            "B X" => (LOSE + ROCK, LOSE + ROCK),
            "B Y" => (DRAW + PAPER, DRAW + PAPER),
            "B Z" => (WIN + SCISSORS, WIN + SCISSORS),
            "C X" => (WIN + ROCK, LOSE + PAPER),
            "C Y" => (LOSE + PAPER, DRAW + SCISSORS),
            "C Z" => (DRAW + SCISSORS, WIN + ROCK),
            _ => panic!("Invalid round"),
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    (part1.to_string(), part2.to_string())
}

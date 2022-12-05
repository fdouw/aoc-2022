use itertools::Itertools;

const REPLACEMENT_CHAR: char = '\u{FFFD}';

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let (data1, data2) = input
        .split_once("\n\n")
        .expect("inputs should be 2 parts, separated by an empty line");

    let stack_count = data1
        .lines()
        .last()
        .expect("data should not start with empty line")
        .split_ascii_whitespace()
        .last()
        .unwrap_or("0")
        .parse::<usize>()
        .expect("last line of first part should be integers");

    let mut stacks = Stacks::new(stack_count);

    // Read in the initial stacks; assumes the data is well-formed
    for l in data1.lines().rev().skip(1) {
        for i in 0..stack_count {
            if l.as_bytes()[4 * i + 1] != b' ' {
                stacks.stacks[i].push(l.as_bytes()[4 * i + 1] as char);
            }
        }
    }

    // Part1
    let mut stacks1 = stacks.clone();
    for line in data2.lines() {
        stacks1.process(line);
    }
    let part1 = stacks1
        .stacks
        .iter()
        .map(|v| v.last().unwrap_or(&REPLACEMENT_CHAR))
        .collect::<String>();

    // Part2
    for line in data2.lines() {
        stacks.process2(line);
    }
    let part2 = stacks
        .stacks
        .iter()
        .map(|v| v.last().unwrap_or(&REPLACEMENT_CHAR))
        .collect::<String>();

    (part1, part2)
}

#[derive(Clone)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn new(size: usize) -> Self {
        let mut stacks = Vec::with_capacity(size);
        for _ in 0..size {
            stacks.push(Vec::new());
        }
        Stacks { stacks }
    }

    fn process(&mut self, data: &str) {
        if let (_, Ok(a), _, Ok(b), _, Ok(c)) = data
            .split_ascii_whitespace()
            .map(|w| w.parse::<usize>())
            .collect_tuple()
            .expect("procedure should be of the form 'move <int> from <int> to <int>'")
        {
            // Input indices are 1-based
            let b = b - 1;
            let c = c - 1;

            for _ in 0..a {
                let val = self.stacks[b]
                    .pop()
                    .expect("there should be data on the stack");
                self.stacks[c].push(val);
            }
        }
    }

    fn process2(&mut self, data: &str) {
        if let (_, Ok(a), _, Ok(b), _, Ok(c)) = data
            .split_ascii_whitespace()
            .map(|w| w.parse::<usize>())
            .collect_tuple()
            .expect("procedure should be of the form 'move <int> from <int> to <int>'")
        {
            // Input indices are 1-based
            let b = b - 1;
            let c = c - 1;

            let at = self.stacks[b].len() - a;
            let mut val = self.stacks[b].split_off(at);
            self.stacks[c].append(&mut val);
        }
    }
}

use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    //     let input = "Monkey 0:
    // Starting items: 79, 98
    // Operation: new = old * 19
    // Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3

    // Monkey 1:
    // Starting items: 54, 65, 75, 74
    // Operation: new = old + 6
    // Test: divisible by 19
    //     If true: throw to monkey 2
    //     If false: throw to monkey 0

    // Monkey 2:
    // Starting items: 79, 60, 97
    // Operation: new = old * old
    // Test: divisible by 13
    //     If true: throw to monkey 1
    //     If false: throw to monkey 3

    // Monkey 3:
    // Starting items: 74
    // Operation: new = old + 3
    // Test: divisible by 17
    //     If true: throw to monkey 0
    //     If false: throw to monkey 1";

    let mut monkeys: Vec<Monkey> = input
        .split_terminator("\n\n")
        .map(|m| m.try_into().unwrap())
        .collect_vec();

    for _round in 0..20 {
        for idx in 0..monkeys.len() {
            let (next_if_true, next_if_false) = monkeys[idx].throw_to;
            let new = monkeys[idx]
                .items
                .iter()
                .map(|item| monkeys[idx].operation.apply(*item) / 3)
                .map(|x| (x % monkeys[idx].test == 0, x))
                .collect_vec();
            monkeys[next_if_true]
                .items
                .extend(
                    new.iter()
                        .filter_map(|x| if x.0 { Some(x.1) } else { None }),
                );
            monkeys[next_if_false]
                .items
                .extend(
                    new.iter()
                        .filter_map(|x| if !x.0 { Some(x.1) } else { None }),
                );
            monkeys[idx].inspections += monkeys[idx].items.len();
            monkeys[idx].items.clear();
        }
    }

    let (a, b) = monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .collect_tuple()
        .unwrap();
    let part1 = a * b;

    // Part 2
    let mut monkeys: Vec<Monkey> = input
        .split_terminator("\n\n")
        .map(|m| m.try_into().unwrap())
        .collect_vec();

    let filter = monkeys.iter().map(|m| m.test).product::<u64>();

    for _round in 0..10_000 {
        for idx in 0..monkeys.len() {
            let (next_if_true, next_if_false) = monkeys[idx].throw_to;
            let new = monkeys[idx]
                .items
                .iter()
                .map(|item| monkeys[idx].operation.apply(*item))
                .map(|x| (x % monkeys[idx].test == 0, x % filter))
                .collect_vec();
            monkeys[next_if_true]
                .items
                .extend(
                    new.iter()
                        .filter_map(|x| if x.0 { Some(x.1) } else { None }),
                );
            monkeys[next_if_false]
                .items
                .extend(
                    new.iter()
                        .filter_map(|x| if !x.0 { Some(x.1) } else { None }),
                );
            monkeys[idx].inspections += monkeys[idx].items.len();
            monkeys[idx].items.clear();
        }
    }

    let (a, b) = monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .collect_tuple()
        .unwrap();
    let part2 = a * b;

    (part1.to_string(), part2.to_string())
}

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl TryFrom<&str> for Operation {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: (&str, &str, &str) = s
            .rsplitn(3, " ")
            .collect_tuple()
            .ok_or("incorrect format for Operation")?;
        // NB: rsplitn splits in reverse!
        match parts.1 {
            "+" => {
                let val = parts.0.parse()?;
                Ok(Operation::Add(val))
            }
            "*" => match parts.0 {
                "old" => Ok(Operation::Square),
                x => {
                    let val = x.parse()?;
                    Ok(Operation::Multiply(val))
                }
            },
            // Need to implement some custom errors, I guess
            _ => panic!("unknown operation"),
        }
    }
}

impl Operation {
    fn apply(&self, val: u64) -> u64 {
        match self {
            Operation::Add(x) => val + x,
            Operation::Multiply(x) => val * x,
            Operation::Square => val * val,
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    throw_to: (usize, usize),
    inspections: usize,
}

impl TryFrom<&str> for Monkey {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut lines = s.trim().lines().skip(1);
        let items = lines
            .next()
            .ok_or("missing data (items) for Monkey")?
            .split(":")
            .last()
            .unwrap_or("")
            .split(", ")
            .map(|x| x.trim().parse().expect("items should be integers"))
            .collect_vec();
        let operation = lines
            .next()
            .ok_or("missing data (operation) for Monkey")?
            .try_into()?;
        let test: (u64, u64, u64) = lines
            .map(|l| l.rsplit_once(" ").unwrap().1.parse().unwrap())
            .collect_tuple()
            .ok_or("test definition should contain three parts")?;
        Ok(Monkey {
            items,
            operation,
            test: test.0,
            throw_to: (test.1 as usize, test.2 as usize),
            inspections: 0,
        })
    }
}

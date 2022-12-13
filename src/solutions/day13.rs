use std::{cmp::Ordering, fmt};

use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let part1: usize = input
        .split("\n\n")
        .map(|l| l.lines().map(|n| ListNum::new(n)).collect_tuple().unwrap())
        .enumerate()
        .map(|(i, (a, b))| {
            if a < b {
                i + 1 // Elves start counting at 1
            } else {
                0
            }
        })
        .sum();

    let mut s = input.trim().replace("\n\n", "\n");
    s.push_str("\n[[2]]\n[[6]]");
    let a = ListNum::new("[[2]]");
    let b = ListNum::new("[[6]]");
    let part2: usize = s
        .lines()
        .map(|l| ListNum::new(l))
        .sorted()
        .enumerate()
        .filter_map(|(idx, num)| {
            if num == a || num == b {
                println!("Found {:?} at {}", num, idx + 1);
                Some(idx + 1)
            } else {
                None
            }
        })
        .product();

    (part1.to_string(), part2.to_string())
}

#[derive(PartialEq, Eq)]
enum ListNum {
    Number(u8),
    List(Vec<Box<ListNum>>),
}

impl ListNum {
    fn new(data: &str) -> Self {
        // Expects 1 line of data, should contain a ListNum. Panics on invalid data
        let (num, _) = ListNum::parse_list(&data.chars().collect_vec(), 0);
        num
    }

    fn parse_list(data: &Vec<char>, start: usize) -> (ListNum, usize) {
        // reads a ListNum::List from `data`, starting at index `start`. Assumes `data` includes the leading '['.
        // returns a tuple containing the resulting ListNum::List and the index where this ListNum ended in `data`.
        let mut num = ListNum::default();
        let mut i = start + 1;
        let len = data.len();
        while i < len {
            match data[i] {
                ',' | ' ' => { /* Ignore commas and spaces */ }
                ']' => {
                    return (num, i);
                }
                '[' => {
                    let (n, end) = ListNum::parse_list(data, i);
                    num.push(n);
                    i = end;
                }
                c if c.is_ascii_digit() => {
                    let mut s = c.to_string();
                    while data[i + 1].is_ascii_digit() {
                        i += 1;
                        s.push(data[i]);
                    }
                    num.push(ListNum::Number(s.parse::<u8>().unwrap()))
                }
                c => panic!("unexpected character: {c}"),
            }
            i += 1;
        }
        panic!("We should have reached the end of the number before now")
    }

    fn push(&mut self, other: ListNum) {
        match self {
            ListNum::List(v) => v.push(Box::new(other)),
            ListNum::Number(_) => panic!("Cannot into numbers, only lists"),
        }
    }

    fn to_list(&self) -> ListNum {
        match self {
            ListNum::Number(n) => {
                let mut v = Vec::new();
                v.push(Box::new(ListNum::Number(*n)));
                ListNum::List(v)
            }
            _ => panic!("don't try to make a list a list"),
        }
    }
}

impl Default for ListNum {
    fn default() -> Self {
        ListNum::List(Vec::new())
    }
}

impl PartialOrd for ListNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNum {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            ListNum::Number(a) => match other {
                ListNum::Number(b) => a.cmp(b),
                ListNum::List(_) => self.to_list().cmp(other),
            },
            ListNum::List(a) => match other {
                ListNum::Number(_) => self.cmp(&other.to_list()),
                ListNum::List(b) => {
                    for i in 0..a.len().min(b.len()) {
                        if a[i] < b[i] {
                            return Ordering::Less;
                        } else if a[i] > b[i] {
                            return Ordering::Greater;
                        }
                    }
                    if a.len() < b.len() {
                        Ordering::Less
                    } else if a.len() > b.len() {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
            },
        }
    }
}

impl fmt::Debug for ListNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListNum::Number(n) => write!(f, "{}", n),
            ListNum::List(v) => write!(f, "{:?}", v),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day13() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            .to_string();
        assert_eq!(
            (String::from("13"), String::from("140")),
            solve(input, false)
        );
    }

    #[test]
    fn test_day13_to_list() {
        let a = ListNum::Number(4);
        assert_eq!(ListNum::new("[4]"), a.to_list())
    }

    #[test]
    fn test_day13_sizes() {
        assert!(ListNum::new("[[4],4]") == ListNum::new("[[4], 4]"))
    }
}

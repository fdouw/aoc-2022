use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    // Small example
    //     let input = "R 4
    // U 4
    // L 3
    // D 1
    // R 4
    // D 1
    // L 5
    // R 2";

    // Larger example (p2)
    //     let input = "R 5
    // U 8
    // L 8
    // D 3
    // R 17
    // D 10
    // L 25
    // U 20";

    let mut head = Knot::default();
    let mut tail = Knot::default();
    let mut visited = HashSet::new();

    let motions = input
        .trim_end()
        .lines()
        .map(|l| Motion::from_str(l).expect("each line should represent a motion"))
        .collect_vec();

    // Also count starting point
    visited.insert(tail);

    for motion in motions.iter() {
        match motion {
            Motion::Up(d) => {
                for _ in 0..*d {
                    head.y -= 1;
                    tail.follow(&head);
                    visited.insert(tail);
                }
            }
            Motion::Right(d) => {
                for _ in 0..*d {
                    head.x += 1;
                    tail.follow(&head);
                    visited.insert(tail);
                }
            }
            Motion::Down(d) => {
                for _ in 0..*d {
                    head.y += 1;
                    tail.follow(&head);
                    visited.insert(tail);
                }
            }
            Motion::Left(d) => {
                for _ in 0..*d {
                    head.x -= 1;
                    tail.follow(&head);
                    visited.insert(tail);
                }
            }
        }
    }
    let part1 = visited.len();

    let mut rope = Rope::new(10);
    for motion in motions {
        rope.step(motion);
    }
    let part2 = rope.visited.len();

    (part1.to_string(), part2.to_string())
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Knot {
    x: i16,
    y: i16,
}

impl Knot {
    fn follow(&mut self, other: &Knot) -> bool {
        // Make this point follow `other`; return true iff this point had to move
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);

        // Only follow if we're more than 1 away in at least 1 dimension
        if dx > 1 || dy > 1 {
            // Update dimension that differs, this way we move diagonally if both differ
            if dx > 0 {
                if other.x < self.x {
                    self.x -= 1;
                } else {
                    self.x += 1;
                }
            }
            if dy > 0 {
                if other.y < self.y {
                    self.y -= 1;
                } else {
                    self.y += 1;
                }
            }
            true
        } else {
            false
        }
    }
}

enum Motion {
    Up(u16),
    Right(u16),
    Down(u16),
    Left(u16),
}

impl FromStr for Motion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, size) = s
            .split_ascii_whitespace()
            .collect_tuple()
            .ok_or(format!("Not a valid motion: {s}"))?;
        let size = size.parse::<u16>().unwrap_or(0);

        match dir {
            "U" => Ok(Motion::Up(size)),
            "R" => Ok(Motion::Right(size)),
            "D" => Ok(Motion::Down(size)),
            "L" => Ok(Motion::Left(size)),
            _ => Err(format!("Invalid direction: {dir}")),
        }
    }
}

struct Rope {
    knots: Vec<Knot>,
    visited: HashSet<Knot>,
}

impl Rope {
    fn new(size: usize) -> Self {
        // Rope needs at least a head and a tail
        assert!(size >= 2);

        let mut knots = Vec::with_capacity(size);
        for _ in 0..size {
            knots.push(Knot::default());
        }
        let mut visited = HashSet::new();
        visited.insert(Knot::default());
        Self { knots, visited }
    }

    fn head(&mut self) -> &mut Knot {
        &mut self.knots[0]
    }

    fn tail(&mut self) -> &mut Knot {
        self.knots.last_mut().unwrap()
    }

    fn step(&mut self, motion: Motion) {
        match motion {
            Motion::Up(d) => {
                for _ in 0..d {
                    self.head().y -= 1;
                    self.follow();
                }
            }
            Motion::Right(d) => {
                for _ in 0..d {
                    self.head().x += 1;
                    self.follow();
                }
            }
            Motion::Down(d) => {
                for _ in 0..d {
                    self.head().y += 1;
                    self.follow();
                }
            }
            Motion::Left(d) => {
                for _ in 0..d {
                    self.head().x -= 1;
                    self.follow();
                }
            }
        }
    }

    fn follow(&mut self) {
        let len = self.knots.len();
        for i in 1..len {
            let prev = self.knots[i - 1];
            // Short-circuit: if the knot doesn't move, neither will subsequent knots
            if !self.knots[i].follow(&prev) {
                return;
            }
        }
        // If the tail moved, keep track of where it's been
        let tail = *self.tail();
        self.visited.insert(tail);
    }
}

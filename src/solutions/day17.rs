use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    // let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let limit1 = 2022;
    let limit2 = 1_000_000_000_000;
    let mut occupied = HashSet::with_capacity(4 * (limit1 + 1));
    let mut states: HashMap<(u32, usize, usize), (usize, usize)> = HashMap::with_capacity(limit1);

    // Hardcoded blocks, as given in problem statement
    // "Each rock appears so that its left edge is two units away from the left wall and its bottom edge is three units
    // above the highest rock in the room (or the floor, if there isn't one)."
    let shapes = vec![
        Shape {
            blocks: vec![Point2D(2, 3), Point2D(3, 3), Point2D(4, 3), Point2D(5, 3)],
        },
        Shape {
            blocks: vec![
                Point2D(3, 3),
                Point2D(2, 4),
                Point2D(3, 4),
                Point2D(4, 4),
                Point2D(3, 5),
            ],
        },
        Shape {
            blocks: vec![
                Point2D(2, 3),
                Point2D(3, 3),
                Point2D(4, 3),
                Point2D(4, 4),
                Point2D(4, 5),
            ],
        },
        Shape {
            blocks: vec![Point2D(2, 3), Point2D(2, 4), Point2D(2, 5), Point2D(2, 6)],
        },
        Shape {
            blocks: vec![Point2D(2, 3), Point2D(3, 3), Point2D(2, 4), Point2D(3, 4)],
        },
    ];

    // Start at the bottom
    let mut top_level = 0;

    // Jets keep cycling, regardless of which shape is falling down
    let mut jets = input
        .trim()
        .chars()
        .map(Direction::from)
        .enumerate()
        .cycle();

    // IDEA for part 2: keep track of states (last n levels, index in jets, index in shapes) and the height and current shape
    // If the state is a repeat, we can compute the final result from there

    let mut part1 = 0;
    let mut part2 = 0;
    for (count, (shape_idx, orig_shape)) in shapes.iter().enumerate().cycle().enumerate() {
        // Move shape up to starting position
        let mut shape = orig_shape.shift(Direction::UP, top_level);

        // Compensate 0-based counting
        let count = count + 1;

        let mut jet_idx;
        let mut dir;
        loop {
            // Pushed by jet
            (jet_idx, dir) = jets.next().unwrap();
            let new = shape.shift(dir, 1);
            if new.fits(&occupied) {
                shape = new;
            }

            // Falls down
            let new = shape.shift(Direction::DOWN, 1);
            if !new.fits(&occupied) {
                break;
            } else {
                shape = new;
            }
        }

        // Fix the shape in place
        top_level = top_level.max(shape.top() + 1);
        for p in shape.blocks {
            occupied.insert(p);
        }

        // Part 1
        if count == limit1 {
            part1 = top_level;
            if part2 > 0 {
                break;
            }
        }

        // Part 2
        // Remember this state
        if top_level >= 3 && part2 == 0 {
            let mut hash = 0;
            for y in (top_level - 3)..top_level {
                for x in 0..7 {
                    if occupied.contains(&Point2D(x as isize, y as isize)) {
                        hash += 1;
                    }
                    hash <<= 1;
                }
            }
            let state = (hash, jet_idx, shape_idx);
            if states.contains_key(&state) {
                let cycle_height = top_level - states[&state].0;
                let cycle_length = count - states[&state].1;
                let shapes_remaining = limit2 - count;
                let cycles_remaining = shapes_remaining / cycle_length;
                let left_over_shapes = shapes_remaining % cycle_length;
                let base_count = states[&state].1;
                let left_over_height = states
                    .iter()
                    .filter(|(_k, v)| v.1 >= base_count && v.1 - base_count == left_over_shapes)
                    .next()
                    .unwrap()
                    .1
                     .0
                    - states[&state].0;

                part2 = top_level + cycle_height * cycles_remaining + left_over_height;

                if part1 > 0 {
                    break;
                }
            } else {
                states.insert(state, (top_level, count));
            }
        }
    }

    (part1.to_string(), part2.to_string())
}

#[derive(Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            '<' => Direction::LEFT,
            _ => panic!("not a direction: '{c}'"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point2D(isize, isize);

impl Point2D {
    fn shift(&self, dir: Direction, n: usize) -> Self {
        let n = n as isize;
        match dir {
            Direction::UP => Point2D(self.0, self.1 + n),
            Direction::RIGHT => Point2D(self.0 + n, self.1),
            Direction::DOWN => Point2D(self.0, self.1 - n),
            Direction::LEFT => Point2D(self.0 - n, self.1),
        }
    }
}

#[derive(Clone)]
struct Shape {
    blocks: Vec<Point2D>,
}

impl Shape {
    fn top(&self) -> usize {
        self.blocks.iter().fold(0, |y, p| y.max(p.1)) as usize
    }
    fn shift(&self, dir: Direction, n: usize) -> Self {
        Shape {
            blocks: self.blocks.iter().map(|p| p.shift(dir, n)).collect_vec(),
        }
    }
    fn fits(&self, obstacles: &HashSet<Point2D>) -> bool {
        self.blocks
            .iter()
            .all(|p| p.0 >= 0 && p.0 < 7 && p.1 >= 0 && !obstacles.contains(&p))
    }
}

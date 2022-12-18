use std::collections::{hash_map::RandomState, HashSet, VecDeque};

use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    // Part 1
    let points: HashSet<Point3D, RandomState> =
        HashSet::from_iter(input.trim().lines().map(Point3D::from));

    let mut surface = 0;
    for point in points.iter() {
        for nb in point.neighbours() {
            if !points.contains(&nb) {
                surface += 1;
            }
        }
    }

    // Part 2
    // Input is only positive number, 1 or 2 digits. So we have an upper bound on how large a vec needs to be.
    // In fact, highest number in my input is 21, so I'm assuming all coords below 30.
    // +1 on all coordinates, so that the origin is definitely outside. Then do a flood fill to determine which units
    // are outside the droplet.
    let size = 30;
    let mut grid = (0..size)
        .map(|x| {
            (0..size)
                .map(|y| {
                    (0..size)
                        .map(|z| {
                            if points.contains(&Point3D(x - 1, y - 1, z - 1)) {
                                Cube::LAVA
                            } else {
                                Cube::EMPTY
                            }
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    while let Some((x, y, z)) = queue.pop_front() {
        if grid[x][y][z] == Cube::EMPTY {
            grid[x][y][z] = Cube::OUTSIDE;
            for p in Point3D(x as i32, y as i32, z as i32).neighbours() {
                if p.in_grid(0, size)
                    && grid[p.0 as usize][p.1 as usize][p.2 as usize] == Cube::EMPTY
                {
                    queue.push_back((p.0 as usize, p.1 as usize, p.2 as usize));
                }
            }
        }
    }

    let mut surface2 = 0;
    for point in points.iter() {
        for nb in point.shifted_up().neighbours() {
            if nb.in_grid(0, 30)
                && grid[nb.0 as usize][nb.1 as usize][nb.2 as usize] == Cube::OUTSIDE
            {
                surface2 += 1;
            }
        }
    }

    (surface.to_string(), surface2.to_string())
}

#[derive(PartialEq, Eq, Hash)]
struct Point3D(i32, i32, i32);

impl Point3D {
    fn neighbours(&self) -> [Point3D; 6] {
        [
            Point3D(self.0 - 1, self.1, self.2),
            Point3D(self.0 + 1, self.1, self.2),
            Point3D(self.0, self.1 - 1, self.2),
            Point3D(self.0, self.1 + 1, self.2),
            Point3D(self.0, self.1, self.2 - 1),
            Point3D(self.0, self.1, self.2 + 1),
        ]
    }
    fn shifted_up(&self) -> Point3D {
        // This point shifted 'upwards' by 1 along all axes, to create a margin at 0
        Point3D(self.0 + 1, self.1 + 1, self.2 + 1)
    }
    fn in_grid(&self, lo: i32, hi: i32) -> bool {
        self.0 >= lo && self.0 < hi && self.1 >= 0 && self.1 < hi && self.2 >= lo && self.2 < hi
    }
}

impl From<&str> for Point3D {
    fn from(s: &str) -> Self {
        let coords: (i32, i32, i32) = s
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Point3D(coords.0, coords.1, coords.2)
    }
}

#[derive(PartialEq, Eq)]
enum Cube {
    EMPTY,
    LAVA,
    OUTSIDE,
}

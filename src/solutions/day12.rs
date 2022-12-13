use std::collections::VecDeque;

use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    //     let input = "Sabqponm
    // abcryxxl
    // accszExk
    // acctuvwj
    // abdefghi"
    //         .to_string();

    let mut finder = PathFinder::from(input);
    // let part1 = finder.search().unwrap();
    // let part2 = finder.search2().unwrap();

    // Using dijkstra is faster
    finder.dijkstra();
    let part1 = finder.distances[finder.start.1][finder.start.0];
    let part2 = finder
        .map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, h)| {
                    if *h == b'a' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .map(|(x, y)| finder.distances[y][x])
        .min()
        .unwrap();

    (part1.to_string(), part2.to_string())
}

struct PathFinder {
    map: Vec<Vec<u8>>,
    distances: Vec<Vec<u16>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
}

impl PathFinder {
    fn search(&mut self) -> Result<usize, String> {
        let mut bfs_queue = VecDeque::new();
        bfs_queue.push_back(Scout {
            pos: self.start,
            len: 0,
        });
        self.bfs(bfs_queue)
    }

    fn search2(&mut self) -> Result<usize, String> {
        // Start bfs with all the levels 'a', this should give us the shortest path overall
        let bfs_queue = VecDeque::from_iter(self.map.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, h)| {
                if *h == b'a' {
                    Some(Scout {
                        pos: (x, y),
                        len: 0,
                    })
                } else {
                    None
                }
            })
        }));
        self.bfs(bfs_queue)
    }

    fn bfs(&mut self, mut bfs_queue: VecDeque<Scout>) -> Result<usize, String> {
        // Returns the length of the shortest path from `start` to `end`.
        // let mut visited = HashSet::with_capacity(self.map.len());
        let mut visited = (0..self.height)
            .map(|_| (0..self.width).map(|_| false).collect_vec())
            .collect_vec();

        while !bfs_queue.is_empty() {
            let current = bfs_queue.pop_front().ok_or(format!(
                "Did not find a path from start {:?} to end {:?}: only dead ends",
                self.start, self.end
            ))?;

            // If we're at the end, then we have found the shortest path
            if current.pos == self.end {
                return Ok(current.len);
            }

            // Check which neighbours are reachable, and add them to the search queue
            for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
                let x = current.pos.0 as isize + dx;
                let y = current.pos.1 as isize + dy;
                if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                    // Neighbour exists...
                    if self.map[y as usize][x as usize] - 1
                        <= self.map[current.pos.1][current.pos.0]
                    {
                        // ...and can be reached

                        // Don't check the same spot twice
                        if !visited[y as usize][x as usize] {
                            visited[y as usize][x as usize] = true;
                            bfs_queue.push_back(Scout {
                                pos: (x as usize, y as usize),
                                len: current.len + 1,
                            });
                        }
                    }
                }
            }
        }

        Err(format!(
            "Did not find a path from start {:?} to end {:?}",
            self.start, self.end
        ))
    }

    fn dijkstra(&mut self) {
        let mut search_queue = VecDeque::with_capacity(self.width * self.height);
        search_queue.push_back(self.end);

        while !search_queue.is_empty() {
            let current = search_queue.pop_front().unwrap();
            let new_dist = self.distances[current.1][current.0] + 1;

            // Check which neighbours can reach the current position.
            // If their distance needs updating, update their distance and add them for future checking
            for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
                let x = current.0 as isize + dx;
                let y = current.1 as isize + dy;

                if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                    // Neighbour exists...
                    if self.map[current.1][current.0] <= self.map[y as usize][x as usize] + 1 {
                        // ...and can reach the current position...
                        if self.distances[y as usize][x as usize] > new_dist {
                            // ...and this is a shorter path
                            self.distances[y as usize][x as usize] = new_dist;
                            search_queue.push_back((x as usize, y as usize));
                        }
                    }
                }
            }
        }
    }
}

impl From<String> for PathFinder {
    fn from(data: String) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map = data
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.char_indices()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = (x, y);
                            b'a'
                        }
                        'E' => {
                            end = (x, y);
                            b'z'
                        }
                        n if n.is_lowercase() => n as u8,
                        _ => panic!("invalid char {c}"),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let width = map[0].len();
        let height = map.len();
        let mut distances = map
            .iter()
            .map(|r| r.iter().map(|_| u16::MAX).collect_vec())
            .collect_vec();
        distances[end.1][end.0] = 0;
        PathFinder {
            map,
            distances,
            start,
            end,
            width,
            height,
        }
    }
}

#[derive(Debug)]
struct Scout {
    pos: (usize, usize),
    len: usize,
}

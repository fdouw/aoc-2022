use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    //     let input = "30373
    // 25512
    // 65332
    // 33549
    // 35390
    // ";

    // Read the forest from input
    let mut forest = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tree {
                    height: c.into(),
                    visible: false,
                    scenic_score: 1,
                })
                .collect_vec()
        })
        .collect_vec();

    // Find out which trees we can see, rows first
    for row in forest.iter_mut() {
        let mut max_height_l = 0;
        let mut max_height_r = 0;
        let len = row.len();
        for i in 0..len {
            if row[i].height > max_height_l {
                max_height_l = row[i].height;
                row[i].visible = true;
            }
            if row[len - i - 1].height > max_height_r {
                max_height_r = row[len - i - 1].height;
                row[len - i - 1].visible = true;
            }
        }
    }
    // Assume all the columns are the same height
    let w = forest[0].len();
    let h = forest.len();
    for i in 0..w {
        let mut max_height_t = 0;
        let mut max_height_b = 0;
        for r in 0..h {
            if forest[r][i].height > max_height_t {
                max_height_t = forest[r][i].height;
                forest[r][i].visible = true;
            }
            if forest[h - r - 1][i].height > max_height_b {
                max_height_b = forest[h - r - 1][i].height;
                forest[h - r - 1][i].visible = true;
            }
        }
    }

    let part1 = forest
        .iter()
        .flat_map(|row| row.iter())
        .filter(|t| t.visible)
        .count();

    // Part 2, quadratic for now
    let height = forest.len();
    for y in 0..height {
        for x in 0..forest[y].len() {
            // Horizontal
            let mut blocked = false;
            forest[y][x].scenic_score *= (0..x)
                .rev()
                .take_while(|&i| {
                    if blocked {
                        false
                    } else {
                        blocked |= forest[y][i].height >= forest[y][x].height;
                        true
                    }
                })
                .count() as u32;
            blocked = false;
            forest[y][x].scenic_score *= forest[y]
                .iter()
                .skip(x + 1)
                .take_while(|&t| {
                    if blocked {
                        false
                    } else {
                        blocked |= t.height >= forest[y][x].height;
                        true
                    }
                })
                .count() as u32;
            // Vertical
            blocked = false;
            forest[y][x].scenic_score *= (0..y)
                .rev()
                .take_while(|&i| {
                    if blocked {
                        false
                    } else {
                        blocked |= forest[i][x].height >= forest[y][x].height;
                        true
                    }
                })
                .count() as u32;
            blocked = false;
            forest[y][x].scenic_score *= (y + 1..height)
                .take_while(|&i| {
                    if blocked {
                        false
                    } else {
                        blocked |= forest[i][x].height >= forest[y][x].height;
                        true
                    }
                })
                .count() as u32;
        }
    }
    let part2 = forest
        .iter()
        .flatten()
        .map(|t| t.scenic_score)
        .max()
        .unwrap_or_default();

    (part1.to_string(), part2.to_string())
}

struct Tree {
    height: u32,
    visible: bool,
    scenic_score: u32,
}

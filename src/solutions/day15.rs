use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let row = 2_000_000;
    let limit = 4_000_000;
    // Testdata
    // let row = 10;
    // let limit = 20;

    let sensors = input.trim().lines().map(|l| l.into()).collect_vec();

    let part1 = sensors
        .iter()
        .flat_map(|s: &Sensor| s.line_cover(row))
        .sorted()
        .fold(Vec::new(), |mut v: Vec<Range>, r| {
            match v.last_mut() {
                Some(s) => {
                    if s.overlaps(&r) || s.borders(&r) {
                        s.absorb(&r)
                    } else {
                        v.push(r);
                    }
                }
                None => v.push(r),
            };
            v
        })
        .iter()
        .map(|r| r.len())
        .sum::<usize>();

    // walk around the perimeter of each sensor and see if the square is covered by other sensors
    let mut part2 = 0;
    'outer: for sensor in sensors.iter() {
        let x = sensor.x;
        let lo_y = sensor.y - (sensor.radius as isize) - 1;
        let hi_y = sensor.y + (sensor.radius as isize) + 1;

        for d in 0..sensor.radius as isize {
            if x + d >= 0 && x + d <= limit && lo_y + d >= 0 && lo_y + d <= limit {
                if !sensors.iter().any(|s| s.contains_point(x + d, lo_y + d)) {
                    part2 = (x + d) * 4_000_000 + lo_y + d;
                    break 'outer;
                }
            }

            if x - d >= 0 && x - d <= limit && lo_y + d >= 0 && lo_y + d <= limit {
                if !sensors.iter().any(|s| s.contains_point(x - d, lo_y + d)) {
                    part2 = (x - d) * 4_000_000 + lo_y + d;
                    break 'outer;
                }
            }

            if x + d >= 0 && x + d <= limit && hi_y + d >= 0 && hi_y + d <= limit {
                if !sensors.iter().any(|s| s.contains_point(x + d, hi_y + d)) {
                    part2 = (x + d) * 4_000_000 + hi_y + d;
                    break 'outer;
                }
            }

            if x - d >= 0 && x - d <= limit && hi_y + d >= 0 && hi_y + d <= limit {
                if !sensors.iter().any(|s| s.contains_point(x - d, hi_y + d)) {
                    part2 = (x - d) * 4_000_000 + hi_y + d;
                    break 'outer;
                }
            }
        }
    }

    (part1.to_string(), part2.to_string())
}

struct Sensor {
    // x,y are the sensor's coordinates; a,b are the nearest beacon's coordinates
    // radius is the manhattan radius between them: in this radius, there are no beacons other than the one at a,b
    x: isize,
    y: isize,
    radius: usize,
    a: isize,
    b: isize,
}

impl Sensor {
    fn line_cover(&self, y: isize) -> Vec<Range> {
        // Returns a vec containing the x-coordinates that this Sensor covers (ie, no Beacons)
        if y.abs_diff(self.y) > self.radius {
            // Sensor covers no points on this line
            return Vec::new();
        }

        let d = self.radius.abs_diff(y.abs_diff(self.y)) as isize;
        if self.b == y {
            // Do not count the beacon
            if self.a == self.x - d {
                vec![Range {
                    a: (self.a + 1),
                    b: (self.x + d + 1),
                }]
            } else if self.a == self.x + d {
                vec![Range {
                    a: (self.x - d),
                    b: (self.a),
                }]
            } else {
                vec![
                    Range {
                        a: (self.x - d),
                        b: (self.a),
                    },
                    Range {
                        a: (self.a + 1),
                        b: (self.x + d + 1),
                    },
                ]
            }
        } else {
            vec![Range {
                a: (self.x - d),
                b: (self.x + d + 1),
            }]
        }
    }

    fn contains_point(&self, x: isize, y: isize) -> bool {
        x.abs_diff(self.x) + y.abs_diff(self.y) <= self.radius
    }
}

impl From<&str> for Sensor {
    fn from(line: &str) -> Self {
        let (x, y, a, b) = line
            .split('=')
            .skip(1)
            .map(|w| w.trim_end_matches(|c: char| !c.is_numeric()))
            .map(|n| n.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        let radius = x.abs_diff(a) + y.abs_diff(b);
        Sensor { x, y, radius, a, b }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Range {
    // Half-open range: [a,b)
    a: isize,
    b: isize,
}

impl Range {
    fn len(&self) -> usize {
        if self.a == self.b {
            0
        } else {
            (self.b - self.a) as usize
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.a <= other.a && self.b >= other.b
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other)
            || (self.a >= other.a && self.a < other.b)
            || (self.b > other.a && self.b <= other.b)
    }

    fn borders(&self, other: &Self) -> bool {
        self.b == other.a || other.a == self.b
    }

    fn absorb(&mut self, other: &Range) {
        // Naive way to combine Ranges: assumes they overlap
        assert!(self.overlaps(other) || self.borders(other));

        self.a = self.a.min(other.a);
        self.b = self.b.max(other.b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
            .to_string();
        assert_eq!(
            ("26".to_string(), "56000011".to_string()),
            solve(input, false)
        );
    }
}

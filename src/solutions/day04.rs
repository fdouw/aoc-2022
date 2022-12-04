use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    //     let input = "2-4,6-8
    // 2-3,4-5
    // 5-7,7-9
    // 2-8,3-7
    // 6-6,4-6
    // 2-6,4-8
    // ";

    let pairs = input
        .trim()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|range| Range::from(range).expect("input range should be: <int>-<int>"))
                .collect_tuple()
                .expect("each pair should consist of exactly 2 ranges")
        })
        .collect::<Vec<_>>();

    let part1 = pairs
        .iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count();

    let part2 = pairs.iter().filter(|(a, b)| a.overlaps(b)).count();

    (part1.to_string(), part2.to_string())
}

struct Range {
    a: u16,
    b: u16,
}

impl Range {
    fn from(data: &str) -> Option<Self> {
        let (a, b) = data
            .split('-')
            .filter_map(|n| n.parse().ok())
            .collect_tuple()?;
        Some(Range { a, b })
    }

    fn contains(&self, other: &Self) -> bool {
        self.a <= other.a && self.b >= other.b
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other)
            || (self.a >= other.a && self.a <= other.b)
            || (self.b >= other.a && self.b <= other.b)
    }
}

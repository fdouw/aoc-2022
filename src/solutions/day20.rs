use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let size = input.trim().lines().count();
    let mut data = Vec::with_capacity(size);
    input.trim().lines().enumerate().for_each(|x| {
        data.push(Item {
            prev: (x.0 + size - 1) % size,
            next: (x.0 + 1) % size,
            value: x.1.parse().unwrap(),
            steps: x.1.parse::<isize>().unwrap() % (size as isize - 1),
        })
    });

    // Mix
    for i in 0..size {
        let mut nxt = data[i].next;
        let mut prv = data[i].prev;
        data[prv].next = nxt;
        data[nxt].prev = prv;
        for _ in 0..data[i].value {
            nxt = data[nxt].next;
        }
        prv = data[nxt].prev;
        for _ in 0..-data[i].value {
            prv = data[prv].prev;
        }
        nxt = data[prv].next;
        data[i].prev = prv;
        data[i].next = nxt;
        data[prv].next = i;
        data[nxt].prev = i;
    }

    // Find 0, and then the 1000th, 2000th, and 3000th value after that
    let mut idx = data.iter().find_position(|x| x.value == 0).unwrap().0;
    let mut part1 = 0;
    for i in 1..=3000 {
        idx = data[idx].next;
        if i % 1000 == 0 {
            part1 += data[idx].value;
        }
    }

    // Part 2
    let decrypt_key = 811589153;

    let mut data = Vec::with_capacity(size);
    input.trim().lines().enumerate().for_each(|x| {
        data.push(Item {
            prev: (x.0 + size - 1) % size,
            next: (x.0 + 1) % size,
            value: x.1.parse::<i64>().unwrap() * decrypt_key as i64,
            steps: (x.1.parse::<isize>().unwrap() * decrypt_key) % (size - 1) as isize,
        })
    });

    // Mix
    for _round in 0..10 {
        for i in 0..size {
            let mut nxt = data[i].next;
            let mut prv = data[i].prev;
            data[prv].next = nxt;
            data[nxt].prev = prv;
            for _ in 0..data[i].steps {
                nxt = data[nxt].next;
            }
            prv = data[nxt].prev;
            for _ in 0..-data[i].steps {
                prv = data[prv].prev;
            }
            nxt = data[prv].next;
            data[i].prev = prv;
            data[i].next = nxt;
            data[prv].next = i;
            data[nxt].prev = i;
        }
    }

    // Find 0, and then the 1000th, 2000th, and 3000th value after that
    let mut idx = data.iter().find_position(|x| x.value == 0).unwrap().0;
    let mut part2 = 0;
    for i in 1..=3000 {
        idx = data[idx].next;
        if i % 1000 == 0 {
            part2 += data[idx].value;
        }
    }

    (part1.to_string(), part2.to_string())
}

#[derive(Debug)]
struct Item {
    value: i64,
    steps: isize,
    prev: usize,
    next: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day20() {
        let input = "1\n\
        2\n\
        -3\n\
        3\n\
        -2\n\
        0\n\
        4"
        .to_owned();
        assert_eq!(
            ("3".to_owned(), "1623178306".to_owned()),
            solve(input, false)
        )
    }
}

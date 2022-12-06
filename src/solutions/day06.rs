use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let part1 =
        count_processed(&input, 4).expect("there should be a sequence of 4 non-repeating chars");
    let part2 =
        count_processed(&input, 14).expect("there should be a sequence of 14 non-repeating chars");

    (part1.to_string(), part2.to_string())
}

fn count_processed(input: &str, length: usize) -> Option<usize> {
    let data = input.chars().collect_vec();
    let mut pat_start = 0;
    for (i, val) in input.char_indices() {
        for j in pat_start..i {
            if data[j] == val {
                pat_start = j + 1;
                break; // won't be further matches
            }
        }
        // i and pat_start are both inclusive
        if i - pat_start == length - 1 {
            // compensate the fact that i counts 0-based
            return Some(i + 1);
        }
    }
    None
}

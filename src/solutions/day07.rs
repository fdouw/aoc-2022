use std::collections::HashMap;

const DISK_SIZE: usize = 70_000_000;
const PART1_LIMIT: usize = 100_000;
const REQUIRED_SPACE: usize = 30_000_000;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    // Testinput
    //     let input = "$ cd /
    // $ ls
    // dir a
    // 14848514 b.txt
    // 8504156 c.dat
    // dir d
    // $ cd a
    // $ ls
    // dir e
    // 29116 f
    // 2557 g
    // 62596 h.lst
    // $ cd e
    // $ ls
    // 584 i
    // $ cd ..
    // $ cd ..
    // $ cd d
    // $ ls
    // 4060174 j
    // 8033020 d.log
    // 5626152 d.ext
    // 7214296 k";

    // Keep track of the dirrs we see, full path inc. first /
    let mut dirs = HashMap::new();
    dirs.insert("/".to_owned(), 0); // Make sure we have an entry for the root dir

    // Trees are hard in Rust, so we're going to use a HashMap with files (full path) mapping to sizes
    let mut files = HashMap::new();

    // Execute the commands
    let mut cwd = Vec::new();
    cwd.push(""); // This way joining gives a leading /
    for line in input.trim().lines() {
        if line.starts_with("$ cd") {
            let path = &line[5..];
            if path == ".." {
                cwd.pop();
            } else if path == "/" {
                cwd.clear();
                cwd.push("");
            } else {
                // should be dirname
                cwd.push(path);
                dirs.insert(cwd.join("/"), 0);
            }
        } else if !line.starts_with('$') && !line.starts_with("dir ") {
            // file entry
            let (size, filename) = line
                .split_once(' ')
                .expect("file entry should be '<size> <name>'");
            let mut fullpath = cwd.join("/");
            fullpath = format!("{fullpath}/{filename}");
            let size = size
                .parse::<usize>()
                .expect("file size should be an integer");
            files.insert(fullpath, size);
        }
    }

    // Collect dir sizes
    for (dir, size) in dirs.iter_mut() {
        *size = files
            .iter()
            .map(|(name, size)| if name.starts_with(dir) { *size } else { 0 })
            .sum();
    }

    let part1: usize = dirs
        .iter()
        .map(|(_, &size)| if size <= PART1_LIMIT { size } else { 0 })
        .sum();

    let free_space = DISK_SIZE - dirs.get("/").expect("there should be a root dir");
    let size_to_delete = REQUIRED_SPACE - free_space;

    let part2 = dirs
        .iter()
        .filter(|(_name, size)| **size >= size_to_delete)
        .min_by_key(|(_name, size)| *size)
        .expect("there should be some dir large enough")
        .1;

    // first read in the directory tree
    (part1.to_string(), part2.to_string())
}

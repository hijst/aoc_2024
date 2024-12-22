use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    solve();
    let elapsed = now.elapsed();
    println!("took {:.2?}", elapsed);
}

fn solve() {
    let grid: Vec<Vec<String>> = lines_from_file("../input/16.txt")
        .into_iter()
        .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect();

    let start: (usize, usize) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|p| p == "S").map(|j| (i, j)))
        .unwrap();

    let end: (usize, usize) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|p| p == "E").map(|j| (i, j)))
        .unwrap();

    let mut mins: HashMap<(usize, usize, usize), i32> = HashMap::new();
    let cur_dir: usize = 0;
    mins.insert((start.0, start.1, cur_dir), 0);
    mins.insert((start.0, start.1, 1), 100);
    mins.insert((start.0, start.1, 3), 100);

    traverse(&grid, &mut mins, cur_dir, start.0, start.1);

    let (x, y) = end;

    let end_mins = vec![
        mins.get(&(x, y, 0 as usize)).unwrap_or(&69696969),
        mins.get(&(x, y, 1 as usize)).unwrap_or(&69696969),
        mins.get(&(x, y, 2 as usize)).unwrap_or(&69696969),
        mins.get(&(x, y, 3 as usize)).unwrap_or(&69696969),
    ];

    //part 2
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
    seen.insert((end.0, end.1));
    seen.insert((start.0, start.1));
    let mut queue: Vec<(usize, usize, usize)> = vec![(end.0, end.1, 3)];
    let dirs: Vec<(isize, isize)> = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];

    while !queue.is_empty() {
        let cur = queue.pop().unwrap();
        seen.insert((cur.0, cur.1));
        for d in 0..4 {
            let (x, y) = dirs[d];
            let (nx, ny) = ((cur.0 as isize + x) as usize, (cur.1 as isize + y) as usize);

            let is_reasonable = is_reasonable_diff((nx, ny, d), cur, &mins);

            for r in is_reasonable {
                if !visited.contains(&(nx, ny, r)) {
                    queue.push((nx, ny, r));
                    visited.insert((nx, ny, r));
                }
            }
        }
    }

    println!("answer 1: {}", end_mins.iter().min().unwrap()); // 143580
    println!("answer 2: {}", seen.len()); // 645
}

fn traverse(
    grid: &Vec<Vec<String>>,
    mins: &mut HashMap<(usize, usize, usize), i32>,
    cur_dir: usize,
    cx: usize,
    cy: usize,
) {
    let dirs: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for d in vec![(cur_dir + 3) % 4, cur_dir, (cur_dir + 1) % 4].into_iter() {
        let (x, y) = dirs[d];
        let (nx, ny) = ((cx as isize + x) as usize, (cy as isize + y) as usize);
        if grid[nx][ny] != "#".to_string() {
            let mut cm = *mins.get(&(cx, cy, cur_dir)).unwrap_or(&0);
            cm += 1;
            if d != cur_dir {
                cm += 1000;
            }
            if cm < *mins.get(&(nx, ny, d as usize)).unwrap_or(&1000000) {
                mins.insert((nx, ny, d as usize), cm);
                traverse(grid, mins, d, nx, ny);
            }
        }
    }
}

fn is_reasonable_diff(
    next: (usize, usize, usize),
    cur: (usize, usize, usize),
    mins: &HashMap<(usize, usize, usize), i32>,
) -> Vec<usize> {
    let mut is_reasonable: Vec<usize> = vec![];
    for d in 0..4 {
        let diff = *mins.get(&cur).unwrap_or(&-99000000)
            - *mins.get(&(next.0, next.1, d)).unwrap_or(&-1000000);
        if diff == 1 || diff == 1001 {
            is_reasonable.push(d);
        }
    }
    return is_reasonable;
}

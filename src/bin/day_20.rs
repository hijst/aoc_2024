use std::{
    collections::HashMap,
    env::JoinPathsError,
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
    let grid: Vec<Vec<String>> = lines_from_file("../input/20.txt")
        .into_iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect();

    let start: (usize, usize) = grid
        .clone()
        .into_iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|c| c == "S").map(|j| (i, j)))
        .unwrap();

    let _end: (usize, usize) = grid
        .clone()
        .into_iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|c| c == "E").map(|j| (i, j)))
        .unwrap();

    let mut scores: HashMap<(usize, usize), i32> = HashMap::new();
    scores.insert(start, 0);
    let mut cur = start;

    loop {
        let nbs = get_neighbours(cur, &grid);
        for nb in nbs {
            if !scores.contains_key(&nb) && grid[nb.0][nb.1] != "#" {
                scores.insert(nb, scores.get(&cur).unwrap() + 1);
                cur = nb;
            }
        }
        if grid[cur.0][cur.1] == "E" {
            break;
        }
    }

    let min_diff = 100;
    let mut ans1 = 0;

    for (i, row) in grid.clone().into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            if col == "#" {}
            {
                let nbs: Vec<_> = get_neighbours((i, j), &grid)
                    .into_iter()
                    .filter(|nb| scores.contains_key(&nb))
                    .collect();
                if nbs.len() > 1 {
                    for x in 0..(nbs.len() - 1) {
                        for y in (x + 1)..(nbs.len()) {
                            let sx = scores.get(&nbs[x]).unwrap();
                            let sy = scores.get(&nbs[y]).unwrap();
                            if (sx - sy).abs() - 2 >= min_diff {
                                ans1 += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut ans2 = 0;
    let scores_vec: Vec<((usize, usize), i32)> = scores.into_iter().collect();
    for i in 0..(scores_vec.len() - 1) {
        for j in (i + 1)..(scores_vec.len()) {
            let p1 = scores_vec[i];
            let p2 = scores_vec[j];
            let md = (p1.0 .0 as isize - p2.0 .0 as isize).abs()
                + (p1.0 .1 as isize - p2.0 .1 as isize).abs();
            if md <= 20 {
                if (p1.1 - p2.1).abs() - md as i32 >= min_diff {
                    ans2 += 1;
                }
            }
        }
    }

    println!("answer 1: {}", ans1); // 1459
    println!("answer 2: {}", ans2); // 1016066
}

fn get_neighbours(cur: (usize, usize), grid: &Vec<Vec<String>>) -> Vec<(usize, usize)> {
    let mut nbs: Vec<(usize, usize)> = vec![];
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;
    let row = cur.0 as isize;
    let col = cur.1 as isize;
    let dirs: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];

    for dir in dirs {
        let nr = row + dir.0;
        let nc = col + dir.1;
        if nr >= 0 && nc >= 0 && nr <= h && nc <= w {
            nbs.push((nr as usize, nc as usize));
        }
    }
    return nbs;
}

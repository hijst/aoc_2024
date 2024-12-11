use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashSet,
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
    let grid: Vec<Vec<i64>> = lines_from_file("../input/10.txt").into_iter()
        .map(|line| line
            .chars()
            .map(|c| c
                .to_string()
                .parse::<i64>()
                .unwrap()
                ).collect()
            ).collect();

    let h = grid.len();
    let w = grid[0].len();
    let mut ans1 = 0;
    let mut ans2 = 0;

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 0 {
                let mut peaks: Vec<(usize, usize)> =  vec![];
                //println!("score for ({},{}) = {}",i,j,neighbour_scores(i,j,&grid));
                //ans1 += neighbour_scores(i,j,&grid)
                peaks.append(&mut neighbour_scores(i,j,&grid));
                ans1 +=  HashSet::<(usize, usize)>::from_iter(peaks.clone()).len();
                ans2 += peaks.len();
            }
        }
    }

    println!("answer 1: {}", ans1); // 5086
    println!("answer 2: {}", ans2); // 1770
}

fn neighbour_scores(i: usize, j: usize, grid: &Vec<Vec<i64>>) -> Vec<(usize,usize)> {
    // base case
    if grid[i][j] == 9 {
        return vec![(i,j)];
    }

    let mh = grid.len() - 1;
    let mw = grid[0].len() - 1;
    let val = grid[i][j];
    let mut neighbours: Vec<(usize,usize)> = vec![];
    let mut res: Vec<(usize, usize)> = vec![];

    if i > 0 { neighbours.push((i - 1, j)) }
    if j > 0 { neighbours.push((i, j - 1)) }
    if i < mh { neighbours.push((i + 1, j)) }
    if j < mw { neighbours.push((i, j + 1)) }
    for n in neighbours {
        if grid[n.0][n.1] == val + 1 {
            //res += neighbour_scores(n.0, n.1, grid);
            res.append(&mut neighbour_scores(n.0, n.1, grid))
        }
    }
    return res;
}

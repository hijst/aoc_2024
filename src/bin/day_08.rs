use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashSet,
    collections::HashMap,
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
    let grid: Vec<Vec<char>> = lines_from_file("../input/08.txt").into_iter().map(|line| line.chars().collect()).collect();

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let mut anti_nodes: HashSet<(i32,i32)> = HashSet::new();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    // first pass -> get pairs of same type

    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val != '.' {
                if antennas.contains_key(val) {
                    antennas.entry(*val).and_modify(|it| it.push((i as i32, j as i32)));
                } else {
                    antennas.insert(*val, vec![(i as i32, j as i32)] );
                }
            }
        }
    }

    // second pass -> add anti_nodes within bounds to set
    for (_, v) in &antennas {
        for i in 0..(v.len() - 1) {
            for j in (i+1)..v.len() {
                let pos1 = (v[i].0 + (v[i].0 - v[j].0), v[i].1 + (v[i].1 - v[j].1));
                let pos2 = (v[j].0 - (v[i].0 - v[j].0), v[j].1 - (v[i].1 - v[j].1));
                if !out_of_bounds(pos1, h, w) {
                    anti_nodes.insert(pos1);
                }
                if !out_of_bounds(pos2, h, w) {
                    anti_nodes.insert(pos2);
                }
            }
        }
    }

    // part 2
    let mut anti_nodes2: HashSet<(i32,i32)> = HashSet::new();

    for (_, v) in &antennas {
        for i in 0..(v.len() - 1) {
            for j in (i+1)..v.len() {
                if v[i].0 == v[j].0 {
                    for k in 0..w {
                        anti_nodes2.insert((v[i].0, k));
                    }
                } else if v[i].1 == v[j].1 {
                    for l in 0..h {
                        anti_nodes2.insert((l, v[i].1));
                    }
                } else {
                    anti_nodes2.insert((v[i].0, v[i].1));
                    anti_nodes2.insert((v[j].0, v[j].1));

                    let bsx = v[i].0 - v[j].0;
                    let bsy = v[i].1 - v[j].1;
                    let gcd = gcd(bsx.abs(), bsy.abs());
                    let sx = bsx / gcd;
                    let sy = bsy / gcd;
                    let mut next_pos = (v[i].0 - sx, v[i].1 - sy);

                    while !out_of_bounds(next_pos, h, w) {
                        anti_nodes2.insert(next_pos);
                        next_pos = (next_pos.0 - sx, next_pos.1 - sy);
                    }

                    next_pos = (v[i].0 + sx, v[i].1 + sy);

                    while !out_of_bounds(next_pos, h, w) {
                        anti_nodes2.insert(next_pos);
                        next_pos = (next_pos.0 + sx, next_pos.1 + sy);
                    }
                }
            }
        }
    }

    println!("answer 1: {}", anti_nodes.len()); // 261
    println!("answer 2: {}", anti_nodes2.len()); // 898
}

fn out_of_bounds(pos: (i32, i32), h: i32, w: i32) -> bool {
    return pos.0 < 0 || pos.0 >= h || pos.1 < 0 || pos.1 >= w
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { return b }
    return gcd(b % a, a)
}

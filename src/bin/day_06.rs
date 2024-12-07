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
    let grid: Vec<Vec<char>> = lines_from_file("../input/06.txt").into_iter().map(|line| line.chars().collect()).collect();

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let dirs = vec![(-1,0), (0,1), (1,0), (0,-1)];
    let mut seen: HashSet<(i32,i32)> = HashSet::new();

    let mut cur_dir: usize = 0;
    let start_pos = grid.iter()
        .enumerate()
        .filter_map(|(row, values)| match values.iter().position(|value| *value == '^') {
            Some(col) => Some((row as i32, col as i32)),
            None => None
        })
        .next()
        .unwrap();

    let mut cur_pos: (i32, i32) = start_pos;

    loop {
        seen.insert(cur_pos);
        let mut next_pos = get_next_pos(&cur_pos, &dirs, cur_dir);
        if out_of_bounds(next_pos, h, w) { break; }
        while grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            cur_dir = (cur_dir + 1) % 4;
            next_pos = get_next_pos(&cur_pos, &dirs, cur_dir);
            if out_of_bounds(next_pos, h, w) { break; }
        }
        if out_of_bounds(next_pos, h, w) { break; }
        cur_pos = next_pos;
    }

    //part 2
    let mut ans2 = 0;

    for (i, j) in seen.clone() {
        let mut new_grid = grid.clone();
        new_grid[i as usize][j as usize] = '#';
        cur_dir = 0;
        cur_pos = start_pos;
        let mut seen_with_dir: HashSet<(i32,i32,usize)> = HashSet::new();

        loop {
            seen_with_dir.insert((cur_pos.0,cur_pos.1, cur_dir));
            let mut next_pos = get_next_pos(&cur_pos, &dirs, cur_dir);
            if out_of_bounds(next_pos, h, w) { break; }
            while new_grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                cur_dir = (cur_dir + 1) % 4;
                next_pos = get_next_pos(&cur_pos, &dirs, cur_dir);
                if out_of_bounds(next_pos, h, w) { break; }
            }
            if out_of_bounds(next_pos, h, w) { break; }
            if seen_with_dir.contains(&(next_pos.0, next_pos.1, cur_dir)) {
                ans2 = ans2 + 1;
                break;
            }
            cur_pos = next_pos;
        }
    }

    println!("{}, {}", cur_pos.0, cur_pos.1);

    println!("answer 1: {}", seen.len()); // 5086
    println!("answer 2: {}", ans2); // 1770
}

fn get_next_pos(cur_pos: &(i32,i32), dirs: &Vec<(i32,i32)>, cur_dir: usize) -> (i32,i32) {
        return (cur_pos.0 + dirs[cur_dir].0, cur_pos.1 + dirs[cur_dir].1);
}

fn out_of_bounds(pos: (i32, i32), h: i32, w: i32) -> bool {
    return pos.0 < 0 || pos.0 >= h || pos.1 < 0 || pos.1 >= w
}


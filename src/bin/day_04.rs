use std::{
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
    let lines = lines_from_file("../input/04.txt");
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut ans = 0;
    for (row, data) in grid.iter().enumerate() {
        for (col, _) in data.iter().enumerate() {
            ans = ans + check_pos(row, col, &grid);
        }
    }

    let mut ans2 = 0;
    
    for (row, data) in grid.iter().enumerate() {
        for (col, _) in data.iter().enumerate() {
            ans2 = ans2 + check_x(row, col, &grid);
        }
    }

    println!("answer 1: {}", ans); // 2507
    println!("answer 2: {}", ans2); // 1969
}

fn check_pos(row: usize, col: usize, grid: &Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    let directions: Vec<(i32, i32)> = vec![(-1,-1), (-1,0), (0,-1), (1,1), (1,0), (0,1), (-1,1), (1,-1)];
    let xmas: &str = "XMAS";
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    if grid[row][col] != xmas.chars().nth(0).unwrap() { return 0 };
    for direction in directions {
        let mut next_pos = (row as i32 + direction.0, col as i32 + direction.1);
        for letter in xmas[1..4].chars() {
            if !within_bounds(next_pos, h, w) { break; }
            if letter != grid[next_pos.0 as usize][next_pos.1 as usize] { break; }
            if letter == 'S' { res = res + 1; }
            next_pos = (next_pos.0 + direction.0, next_pos.1 + direction.1);
        }
    }
    return res;
}

fn check_x(row: usize, col: usize, grid: &Vec<Vec<char>>) -> i32 {
    if grid[row][col] != 'A' { return 0 };
    if is_on_edge(row, col, grid) { return 0 };

    if ((grid[row+1][col+1] == 'M' && grid[row-1][col-1] == 'S') || (grid[row+1][col+1] == 'S' && grid[row-1][col-1] == 'M')) && ((grid[row+1][col-1] == 'M' && grid[row-1][col+1] == 'S') || (grid[row+1][col-1] == 'S' && grid[row-1][col+1] == 'M')) {
        return 1
    }

    return 0;

}

fn within_bounds(pos: (i32, i32), h: i32, w: i32) -> bool {
    let (r,c) = pos;
    r >= 0 && r < h && c >= 0 && c < w
}

fn is_on_edge(row: usize, col: usize, grid: &Vec<Vec<char>>) -> bool {
    row < 1 || col < 1 || row == grid.len() - 1 || col == grid[0].len() - 1
}


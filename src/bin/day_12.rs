use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashSet,
    collections::VecDeque,
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
    let grid: Vec<Vec<String>> = lines_from_file("../input/12.txt").into_iter()
        .map(|line| line
            .chars()
            .map(|c| c
                .to_string()
                ).collect()
            ).collect();

    let h = grid.len();
    let w = grid[0].len();
    let (mut ans1, mut ans2) = (0,0);
    let mut components: Vec<HashSet<(isize,isize)>> = vec![];
    let mut seen: HashSet<(usize,usize)> = HashSet::new();

    for i in 0..h {
        for j in 0..w {
            if seen.contains(&(i,j)) { continue }
            let mut fences: i32 = 0;
            let mut queue = VecDeque::from([(i,j)]);
            let mut in_component: HashSet<(isize,isize)> = HashSet::new();
            let dirs: Vec<(isize, isize)> = vec![(0,1), (1,0), (-1,0), (0,-1)];

            while !queue.is_empty() {
                let (x,y) = queue.pop_front().unwrap();
                if seen.contains(&(x,y)) { continue }
                in_component.insert((x as isize,y as isize));
                seen.insert((x,y));

                for dir in &dirs {
                    let pos_i: (isize, isize) = (x as isize + dir.0, y as isize + dir.1);

                    if out_of_bounds(pos_i, h, w) {
                        fences += 1;
                    } else { 
                        let pos_u = (pos_i.0 as usize, pos_i.1 as usize);
                        if grid[i][j] != grid[pos_u.0][pos_u.1] {
                        fences += 1;
                        } else if !seen.contains(&pos_u) {
                            queue.push_back(pos_u);
                        }
                    }
                }
            }
            ans1 += in_component.len() as i32 * fences;
            ans2 += count_corners(in_component.clone()) * &in_component.len();
            components.push(in_component);
        }
    }

    println!("answer 1: {}", ans1); // 1375476
    println!("answer 2: {}", ans2); // 821372
}

fn out_of_bounds(pos: (isize, isize), h: usize, w: usize) -> bool {
    return pos.0 < 0 || pos.0 >= h as isize || pos.1 < 0 || pos.1 >= w as isize
}

fn count_corners(component: HashSet<(isize,isize)>) -> usize {
    let mut corners: HashSet<(isize, isize, &str)> = HashSet::new();
    for cell in &component {
        let left = component.contains(&(cell.0, cell.1 - 1));
        let top_left = component.contains(&(cell.0 - 1, cell.1 - 1));
        let top = component.contains(&(cell.0 - 1, cell.1));
        let top_right = component.contains(&(cell.0 - 1 , cell.1 + 1));
        let right = component.contains(&(cell.0, cell.1 + 1));
        let bottom_right = component.contains(&(cell.0 + 1, cell.1 + 1));
        let bottom = component.contains(&(cell.0 + 1, cell.1));
        let bottom_left = component.contains(&(cell.0 + 1, cell.1 - 1));

        if !top_left && top == left { corners.insert((cell.0, cell.1, "S")); };
        if !top_right && top == right { corners.insert((cell.0, cell.1 + 1, "S")); };
        if !bottom_right && bottom == right { corners.insert((cell.0 + 1, cell.1 + 1, "S")); };
        if !bottom_left && bottom == left { corners.insert((cell.0 + 1, cell.1, "S")); };

        if top_left && !top && !left { corners.insert((cell.0, cell.1, "D")); };
        if top_right && !top && !right { corners.insert((cell.0, cell.1 + 1, "D")); };
        if bottom_right && !bottom && !right { corners.insert((cell.0 + 1, cell.1 + 1, "D")); };
        if bottom_left && !bottom && !left { corners.insert((cell.0 + 1, cell.1, "D")); };
    }
    return corners.into_iter().fold(0, |acc,(_,_,t)| if t == "S" { acc + 1 } else { acc + 2 });
}

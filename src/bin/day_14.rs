use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    thread::sleep,
    time::Duration,
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
let robots: Vec<Vec<i64>> = lines_from_file("../input/14.txt")
    .into_iter()
    .map(|l| l.split("p=").map(|s| s.to_string()).collect::<Vec<String>>())
    .map(|line| line[1].split(" v=").map(|s| s.to_string()).collect::<Vec<String>>())
    .map(|sline| sline.into_iter()
        .flat_map(|part| part.split(",")
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()).collect()
    )
    .collect();

    println!("{:?}", robots);

    let (mut q1, mut q2, mut q3, mut q4) = (0,0,0,0);

    for robot in &robots {
        let new_pos = calculate_new_pos(robot, 100);
        println!("{:?}", new_pos);
        let mx = 50; //50
        let my = 51; //51
        if new_pos.0 < mx {
            if new_pos.1 < my {
                q1 += 1;
            }
            if new_pos.1 > my {
                q2 += 1;
            }
        }
        if new_pos.0 > mx {
            if new_pos.1 < my {
                q3 += 1;
            }
            if new_pos.1 > my {
                q4 += 1;
            }
        }

    }

    let ans1 = q1 * q2 * q3 * q4;

    println!("answer 1: {}", ans1); // 1375476

    let mut sec = 0;
    let mut grid = robots.clone();
    loop {
        sec += 1;

        for (ix,robot) in grid.clone().into_iter().enumerate() {
            let mut new_robot: Vec<i64> = robot.clone();
            let new_pos = calculate_new_pos(&robot, 1);
            new_robot[0] = new_pos.0;
            new_robot[1] = new_pos.1;
            grid[ix] = new_robot;
        }

        let mut picture: Vec<Vec<&str>> = vec![vec!["."; 101]; 103];

        for i in 0..103 {
            for j in 0..101 {
                if find_robot(&grid, i as i64, j as i64) {
                    picture[i][j] = "@";
                } else {
                    picture[i][j] = " ";
                }
            }
        }
        //sleep(Duration::from_millis(33));

        println!("---SEC {} ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------", sec);
        print_grid(picture);
        println!("---SEC {} ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------", sec);
        
    }
}

fn calculate_new_pos(robot: &Vec<i64>, step_size: i64) -> (i64,i64) {
    let dx = if robot[2] < 0 {  robot[2] + 101} else {robot[2]};
    let dy = if robot[3] < 0 {  robot[3] + 103} else {robot[3]};
    return ((robot[0] + step_size * dx) % 101, (robot[1] + step_size * dy) % 103);
}

fn find_robot(robots: &Vec<Vec<i64>>, row: i64, col: i64) -> bool {
    for robot in robots {
        if robot[1] == row && robot[0] == col { return true}
    }
    return false
}

fn print_grid(grid: Vec<Vec<&str>>) {
    println!("{}", grid.into_iter().map(|l| l.join(" ")).collect::<Vec<_>>().join("|\n|"));
}

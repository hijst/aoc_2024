use std::fs;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    solve();
    let elapsed = now.elapsed();
    println!("took {:.2?}", elapsed);

}

fn solve() {
    let contents = fs::read_to_string("../input/01.txt")
        .expect("Should have been able to read the file");

    let split: Vec<&str> = contents.split_whitespace().collect::<Vec<&str>>();
    let mut left: Vec<i32> = Vec::<i32>::new();
    let mut right: Vec<i32> = Vec::<i32>::new();

    for (i, el) in split.iter().enumerate() {
        if i%2 == 0 {
            left.push(el.parse::<i32>().unwrap());
        }
        else {
            right.push(el.parse::<i32>().unwrap());
        }
    }

    left.sort();
    right.sort();

    let mut ans = 0;
    let mut ans2 = 0;
    for i in 0..1000 {
        ans = ans + (left[i] - right[i]).abs();
        ans2 = ans2 + (left[i] * right.clone().into_iter().filter(|n| *n == left[i]).count() as i32);
    }

    println!("answer 1: {}", ans);
    println!("answer 2: {}", ans2);
    
}

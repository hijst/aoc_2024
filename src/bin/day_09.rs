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
    let nums: Vec<i64> = lines_from_file("../input/09.txt")[0].chars().map(|d| d.to_string().parse::<i64>().unwrap()).collect();

    //part 1
    let mut nums1: Vec<i64> = nums.clone();

    let mut l: usize = 0;
    let mut r: usize = nums.len() - 1;
    let mut pos = 0;
    let mut checksum = 0;

    while l <= r {
        // free space
        if l % 2 != 0 {
            for _p in 0..nums1[l] {
                if nums1[r] == 0 {
                    r = r - 2;
                }
                if l > r {
                    break;
                }
                checksum += pos * (r/2);
                nums1[r] = nums1[r] - 1;
                pos += 1;
            }
            if l > r {
                break;
            }
        // number
        } else {
            for _p in 0..nums1[l] {
                checksum += pos * (l/2);
                pos += 1;
            }
        }
        l = l + 1;
    }

    //part 2 just DO THE THING
    // pass 1 initialize playing field
    let mut nums2: Vec<(i64, usize, &str)> = nums
        .clone()
        .iter()
        .enumerate()
        .map(|(i,num)| 
            if i % 2 == 0 { (*num, i/2, "T") } 
            else { (*num, 0, "F") }
            )
        .collect();

    // pass 2 try to move each one once
    for (c, v, t) in nums2.clone().iter().rev() {
        if *t == "T" {
            let ix = nums2.iter().position(|&r| r == (*c,*v,*t)).unwrap();
            for (idx, (c2, v2, t2)) in nums2.clone().iter().enumerate() {
                if *t2 == "F" && *c2 >= *c {
                    if ix <= idx { break; }
                    nums2[ix] = (*c, 0, "F");
                    if *c2 == *c {
                        nums2.splice(idx..(idx+1), [(*c, *v, *t)] );
                    } else {
                        nums2.splice(idx..(idx+1), [(*c, *v, *t), (*c2 - *c, *v2, *t2)]);
                    }
                    break;
                }
            }
        }
    }
    // pass 3 calculate checksum 2 
    let mut checksum2 = 0;
    let mut pos = 0;

    for (count, val, _) in nums2.iter() {
        for _ in 0..(*count as usize) {
            checksum2 += val * pos;
            pos += 1;
        }
    }

    println!("answer 1: {}", checksum); // 6607511583593
    println!("answer 2: {}", checksum2); // 663660878123
}


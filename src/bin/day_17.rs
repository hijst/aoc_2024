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

#[derive(Debug)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

fn solve() {
    let path = "../input/17.txt";
    let nums: Vec<String> = lines_from_file(path)
        .into_iter()
        .map(|line| line.chars()
            .filter(|c| c.is_digit(10))
            .collect()
        ).collect();

    let register_values: Vec<i64> = nums[0..3].into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut registers = Registers {
        a: register_values[0],
        b: register_values[1],
        c: register_values[2],
    };

    println!("{:?}", registers);

    let instructions: Vec<Vec<i64>> = nums[4].chars().map(|c|
        c.to_digit(10).unwrap() as i64)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| c.to_vec())
        .collect();

    let target: &str = &lines_from_file(path)[4].clone()[9..];
    println!("TARGET {}", target);

    let mut t: i64 = 1732324875100000;
    t=0;
    while t < i64::MAX {
        registers = Registers {
            a: t,
            b: register_values[1],
            c: register_values[2],
        };
        let mut ix: usize = 0;
        let mut prev_ix = 0;
        let mut output: String = "".to_string();

        while ix < instructions.len() {
            let i = &instructions[ix];

            match i[0] {
                0 => adv(i[1], &mut registers),
                1 => bxl(i[1], &mut registers),
                2 => bst(i[1], &mut registers),
                3 => jnz(i[1], &mut registers, &mut ix),
                4 => bxc(i[1], &mut registers),
                5 => out(i[1], &mut registers, &mut output),
                6 => bdv(i[1], &mut registers),
                7 => cdv(i[1], &mut registers),
                _ => panic!("invalid operator {}", i[0])
            }

            if i[0] != 3 {
                ix = ix + 1;
            }

            if prev_ix == ix { break; }
            prev_ix = ix;
        }
        if t % 10 == 0 {
            println!("{}", t);
            let o = &output[1..];
            println!("output {} target {}", o, target);
        }
        if output[1..] == *target { break; }
        t += 1;
    }

    let ans1: i64 = 0;
    let ans2: i64 = 0;

    println!("answer 1: {}", ans1); // 1,6,7,4,3,0,5,0,6
    println!("answer 2: {}", ans2); // 1521635
}

fn adv(operand: i64, registers: &mut Registers) {
    let combo_operand = get_combo_operand(operand, registers);
    let num = registers.a;
    let base: i64 = 2;
    let den: i64 = base.pow(combo_operand.try_into().unwrap());
    registers.a = num/den;
}

fn bxl(operand: i64, registers: &mut Registers) {
    let bxor = registers.b ^ operand;
    registers.b = bxor;
}

fn bst(operand: i64, registers: &mut Registers) {
    let combo_operand = get_combo_operand(operand, registers);
    registers.b = combo_operand % 8;
}

fn jnz(operand: i64, registers: &mut Registers, i: &mut usize) {
    if registers.a == 0 { return }
    *i = operand as usize;
}

fn bxc(_operand: i64, registers: &mut Registers) {
    let bxor = registers.b ^ registers.c;
    registers.b = bxor;
}

fn out(operand: i64, registers: &mut Registers, output: &mut String) {
    let combo_operand = get_combo_operand(operand, registers) % 8;
    output.push_str(&format!(",{}", combo_operand)[..]);
}

fn bdv(operand: i64, registers: &mut Registers) {
    let combo_operand = get_combo_operand(operand, registers);
    let num = registers.a;
    let base: i64 = 2;
    let den = base.pow(combo_operand.try_into().unwrap());
    registers.b = num/den;
}

fn cdv(operand: i64, registers: &mut Registers) {
    let combo_operand: i64 = get_combo_operand(operand, registers);
    let num: i64 = registers.a;
    let base: i64 = 2;
    let den = base.pow(combo_operand.try_into().unwrap());
    registers.c = num/den;
}

fn get_combo_operand(operand: i64, registers: &Registers) -> i64 {
    let res: i64 = match operand {
        0..4 => operand,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!("no valid operand {}", operand),
    };
    return res
}

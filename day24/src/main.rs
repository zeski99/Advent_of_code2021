use std::fs;

fn main() {
    let mut divz: Vec<i64> = vec![];
    let mut addx: Vec<i64> = vec![];
    let mut addy: Vec<i64> = vec![];
    let input = fs::read_to_string("in.txt").expect("h");
    for (i,line) in input.lines().enumerate() {
        let tmp = line.split(" ").last().unwrap().parse::<i64>();
        if i % 18 == 4 { divz.push(tmp.unwrap()) }
        else if i % 18 == 5 { addx.push(tmp.unwrap()) }
        else if i % 18 == 15 { addy.push(tmp.unwrap()) }
    }
    let mut pairs: Vec<((usize,i64),(usize,i64))> = vec![];
    let mut stack: Vec<(usize,i64)> = vec![];
    for i in 0..14 {
        if divz[i] == 1 {
            stack.push((i,addy[i]));
        } else {
            pairs.push(((i,addx[i]), stack.pop().unwrap()));
        }
    }

    let mut num = vec![0;14];
    for p in &pairs {
        for w in (1..10).rev() {
            let t1 = w + p.1.1;
            let t0 = t1 + p.0.1;
            if t0 < 10 {
                num[p.0.0] = t0;
                num[p.1.0] = w;
                break;
            }
        }
    }
    println!("Part One: {}", num.iter().fold(0, |a, x| a*10+x));

    let mut num = vec![0;14];
    for p in &pairs {
        for w in 1..10 {
            let t1 = w + p.1.1;
            let t0 = t1 + p.0.1;
            if t0 > 0 {
                num[p.0.0] = t0;
                num[p.1.0] = w;
                break;
            }
        }
    }
    println!("Part Two: {}", num.iter().fold(0, |a, x| a*10+x));
}
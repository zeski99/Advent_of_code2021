use std::fs;

fn part_one(input: &Vec<i64>){
    let median: i64 = input[input.len() / 2];
    let fuel: i64 = input.iter().map(|n| (n-median).abs()).sum();
    println!("Part One: {}", fuel);
}

fn part_two(input: &Vec<i64>){
    let avg = input.iter().sum::<i64>() as f64 / input.len() as f64;
    let sign_avg = 0.5 - input.iter().filter(|&&n| n as f64 > avg).count() as f64 / input.len() as f64;
    let num = (avg - sign_avg).round() as i64;
    let fuel: i64 = input.iter().map(|n| ((num-n).pow(2) + (num-n).abs()) / 2).sum();
    println!("Part Two: {}", fuel);
}

fn main() {
    let mut input:Vec<i64> = fs::read_to_string("in.txt").expect("h").split(",").map(|n| n.parse().unwrap()).collect();
    input.sort();
    part_one(&input);
    part_two(&input);
}

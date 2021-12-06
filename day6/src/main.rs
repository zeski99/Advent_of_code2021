use std::fs;

fn calc(d: i64) -> Vec<i64> {
    let mut vals = vec![1; 9];
    for _i in 0..d{
        vals.push(vals[0] + vals[2]);
        vals.remove(0);
    }
    vals.drain(0..4);
    return vals;
}

fn part_one(input: &Vec<i64>){
    let mapping: Vec<i64> = calc(79);
    let num: i64 = input.iter().map(|&n| mapping[5-n as usize]).sum::<i64>();
    println!("Part One: {}", num);
}

fn part_two(input: &Vec<i64>){
    let mapping: Vec<i64> = calc(255);
    let num: i64 = input.iter().map(|&n| mapping[5-n as usize]).sum::<i64>();
    println!("Part One: {}", num);
}

fn main() {
    let input:Vec<i64> = fs::read_to_string("in.txt").expect("h").split(",").map(|n| n.parse().unwrap()).collect();
    part_one(&input);
    part_two(&input);
}

use std::fs;

fn part_one(nums: &Vec<i32>){
    let mut ctr = 0;

    for i in 1..nums.len(){
        if nums[i-1] < nums[i]{
            ctr += 1;
        }
    }

    println!("Part One: {}", ctr);
}

fn part_two(nums: &Vec<i32>){
    let mut ctr = 0;
    for i in 3..nums.len(){
        if nums[i-3] < nums[i]{
            ctr += 1;
        }
    }
    println!("Part Two: {}", ctr);
}

fn main() {
    let input = fs::read_to_string("in.txt")
        .expect("Something went wrong reading the file");

    let nums: Vec<i32> = input.split_whitespace().map(|s| s.parse().expect("parse error")).collect();
    part_one(&nums);
    part_two(&nums);
}

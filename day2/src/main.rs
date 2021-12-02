use std::fs;

fn part_one(input: &String) {
    let mut x = 0;
    let mut y = 0;

    let lines = input.lines();

    for line in lines{
        let vec = line.split(" ").collect::<Vec<&str>>();
        let op = vec[0];
        let n = vec[1].parse::<i32>().unwrap();

        if op == "forward" {
            x += n;
        }
        else if op == "up" {
            y -= n;
        }
        else{
            y += n;
        }
    }

    println!("Part One: {}", x*y);
}

fn part_two(input: &String) {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    let lines = input.lines();

    for line in lines{
        let vec = line.split(" ").collect::<Vec<&str>>();
        let op = vec[0];
        let n = vec[1].parse::<i32>().unwrap();

        if op == "forward" {
            x += n;
            y += aim * n;
        }
        else if op == "up" {
            aim -= n;
        }
        else{
            aim += n;
        }
    }

    println!("Part Two: {}", x*y);
}

fn main() {
    let input = fs::read_to_string("in.txt")
        .expect("Something went wrong reading the file");
    part_one(&input);
    part_two(&input);
}

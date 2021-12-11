use std::fs;

fn part_one(data: &Vec<Vec<i32>>, field: &Vec<Vec<i32>>){
    let mut field = field.clone();
    let mut a: Vec<i32>;
    let mut b: Vec<i32>;
    for d in data{
        if d[0] == d[2]{
            if d[1] > d[3]{
                a = (d[3]..=d[1]).rev().collect();
            }else{
                a = (d[1]..=d[3]).collect();
            }
            for y in &a{
                field[d[0] as usize][*y as usize] += 1;
            }
        }
        else if d[1] == d[3]{
            if d[0] > d[2]{
                b = (d[2]..=d[0]).rev().collect();
            }else{
                b = (d[0]..=d[2]).collect();
            }
            for x in &b{
                field[*x as usize][d[1] as usize] += 1;
            }
        }
    }
    let count: usize = field.iter().map(|r| r.iter().filter(|&&e| e > 1).count()).sum();
    println!("Part One: {}", count);
}

fn part_two(data: &Vec<Vec<i32>>, field: &Vec<Vec<i32>>){
    let mut field = field.clone();
    let mut a: Vec<i32>;
    let mut b: Vec<i32>;
    for d in data{
        if d[0] == d[2]{
            if d[1] > d[3]{
                a = (d[3]..=d[1]).rev().collect();
            }else{
                a = (d[1]..=d[3]).collect();
            }
            for y in &a{
                field[d[0] as usize][*y as usize] += 1;
            }
        }
        else if d[1] == d[3]{
            if d[0] > d[2]{
                b = (d[2]..=d[0]).rev().collect();
            }else{
                b = (d[0]..=d[2]).collect();
            }
            for x in &b{
                field[*x as usize][d[1] as usize] += 1;
            }
        }
        else{
            if d[1] > d[3]{
                a = (d[3]..=d[1]).rev().collect();
            }else{
                a = (d[1]..=d[3]).collect();
            }
            if d[0] > d[2]{
                b = (d[2]..=d[0]).rev().collect();
            }else{
                b = (d[0]..=d[2]).collect();
            }
            for (x,y) in b.iter().zip(a.iter()){
                field[*x as usize][*y as usize] += 1;
            }
        }
    }
    let count: usize = field.iter().map(|r| r.iter().filter(|&&e| e > 1).count()).sum();
    println!("Part One: {}", count);
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let data: Vec<Vec<i32>> = input.lines().map(|line| line.split(&[',', ' '][..]).filter_map(|n| n.parse().ok()).collect()).collect();
    let xmax: i32 = data.iter().map(|l| l[0].max(l[2])).max().unwrap();
    let ymax: i32 = data.iter().map(|l| l[1].max(l[3])).max().unwrap();
    let field: Vec<Vec<i32>> = (0..=xmax).map(|_i| vec![0; (ymax+1) as usize]).collect();
    part_one(&data, &field);
    part_two(&data, &field);
}

use std::fs;
use std::char;

fn part_one(arr: &Vec<Vec<u32>>){
    let bin = (0..arr[0].len())
        .map(|i| arr.iter()
            .map(|c| c[i])
            .sum::<u32>() * 2 / arr.len() as u32);

    let gamma = u32::from_str_radix(&bin.clone()
        .map(|c| char::from_digit(c,10).unwrap())
        .collect::<String>(), 2).unwrap();

    let epsilon = u32::from_str_radix(&bin.clone()
        .map(|c| char::from_digit(c^1,10).unwrap())
        .collect::<String>(), 2).unwrap();

    println!("{}", gamma*epsilon);
}

fn part_two(arr: &Vec<Vec<u32>>){
    let mut oxy = arr.clone();
    let mut co2 = arr.clone();

    for i in 0..arr[0].len(){
        if oxy.len() > 1{
            let bit = oxy.iter().map(|c| c[i]).sum::<u32>() * 2 / oxy.len() as u32;
            oxy = oxy.into_iter().filter(|c| c[i] == bit).collect();
        }
        if co2.len() > 1{
            let bit = (co2.iter().map(|c| c[i]).sum::<u32>() * 2 / co2.len() as u32) ^ 1;
            co2 = co2.into_iter().filter(|c| c[i] == bit).collect();
        }
    }
    let oxy = u32::from_str_radix(&oxy[0].iter()
        .map(|c| char::from_digit(*c,10).unwrap())
        .collect::<String>(), 2).unwrap();

    let co2 = u32::from_str_radix(&co2[0].iter()
        .map(|c| char::from_digit(*c,10).unwrap())
        .collect::<String>(), 2).unwrap();
        
    println!("{}", oxy*co2)
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let arr: Vec<Vec<u32>> = input.split_whitespace().map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    part_one(&arr);
    part_two(&arr);
}

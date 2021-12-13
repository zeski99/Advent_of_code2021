use std::fs;
use std::collections::HashSet;

fn main() {
    let mut first = true;
    let input = fs::read_to_string("in.txt").expect("h");
    let input = input.split_once("\n\n").unwrap();
    let mut points: HashSet<(u32, u32)> = input.0.lines().map(|l| l.split_once(",").unwrap())
        .map(|(a,b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())).collect();
    let folds = input.1.lines().map(|l| l.rsplit_once(" ").unwrap().1.split_once("=").unwrap())
        .map(|(a,b)| (a=="x", b.parse::<u32>().unwrap()));

    for fold in folds{
        if fold.0{
            points = points.iter().map(|(x,y)| (*x.min(&(fold.1 * 2 - x)), *y)).collect();
        }
        else{
            points = points.iter().map(|(x,y)| (*x, *y.min(&(fold.1 * 2 - y)))).collect();
        }
        if first{
            println!("Part One: {}", points.len());
            first = false;
        }
    }
    let mut part_two = String::new();
    for y in 0..=points.iter().max_by_key(|(_,y)| y).unwrap().1{
        part_two.push('\n');
        for x in 0..=points.iter().max_by_key(|(x,_)| x).unwrap().0{
            if points.contains(&(x,y)){
                part_two.push('#');
            }
            else{
                part_two.push('.');
            }
        }
    }
    println!("Part Two:{}", part_two);
}

use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::char;

fn part_one(dig: &Vec<Vec<&str>>){
    println!("Part One: {}", dig.iter().map(|l| l.iter().filter(|n| (n.len()-1) ^ 4 > 1).count()).sum::<usize>());
}

fn part_two(map: &Vec<Vec<&str>>, dig: &Vec<Vec<&str>>){
    let mut sum = 0;
    for (m, d) in map.iter().zip(dig.iter()){
        let sets: Vec<HashSet<char>> = m.iter().map(|&v| HashSet::from_iter(v.chars())).collect();
        let null = HashSet::new();
        let mut mapping: Vec<&HashSet<char>> = vec![&null;10];
        mapping[1] = sets.iter().find(|s| s.len() == 2).unwrap();
        mapping[4] = sets.iter().find(|s| s.len() == 4).unwrap();
        mapping[7] = sets.iter().find(|s| s.len() == 3).unwrap();
        mapping[8] = sets.iter().find(|s| s.len() == 7).unwrap();
        mapping[6] = sets.iter().find(|s| s.len() == 6 && s.union(mapping[7]).count() == 7).unwrap();
        mapping[9] = sets.iter().find(|s| s.len() == 6 && s.intersection(mapping[4]).count() == 4).unwrap();
        mapping[2] = sets.iter().find(|s| s.len() == 5 && s.intersection(mapping[4]).count() == 2).unwrap();
        mapping[5] = sets.iter().find(|s| s.len() == 5 && s.union(mapping[6]).count() == 6).unwrap();
        mapping[3] = sets.iter().find(|s| s.len() == 5 && s.union(mapping[1]).count() == 5).unwrap();
        mapping[0] = sets.iter().find(|s| s.len() == 6 && s.union(mapping[5]).count() == 7).unwrap();
        let num: i32 = d.iter().map(|n| mapping.iter().position(|&s| s.eq(&HashSet::from_iter(n.chars()))).unwrap()).map(|h| char::from_digit(h as u32, 10).unwrap()).collect::<String>().parse().unwrap();
        sum += num;
    }
    println!("Part Two: {}", sum);
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let mut map: Vec<Vec<&str>> = vec![];
    let mut dig: Vec<Vec<&str>> = vec![];
    for l in input.lines(){
        let tmp: Vec<&str> = l.split(" | ").collect();
        map.push(tmp[0].split(" ").collect());
        dig.push(tmp[1].split(" ").collect());
    }
    part_one(&dig);
    part_two(&map, &dig);
}

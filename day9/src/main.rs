use std::fs;
use std::collections::VecDeque;

fn part_one(map: &Vec<Vec<u32>>) -> Vec<(usize,usize)>{
    let mut sum = 0;
    let mut points: Vec<(usize,usize)> = vec![];
    for i in 0..map.len(){
        for j in 0..map[0].len(){
            if i < map.len() - 1 && map[i+1][j] <= map[i][j] { continue; }
            if j < map[0].len() - 1 && map[i][j+1] <= map[i][j] { continue; }
            if i > 0 && map[i-1][j] <= map[i][j] { continue; }
            if j > 0 && map[i][j-1] <= map[i][j] { continue; }
            sum += map[i][j] + 1;
            points.push((i,j));
        }
    }
    println!("Part One: {}", sum);
    return points;
}

fn basin(map: &Vec<Vec<u32>>, i: usize, j: usize) -> i32{
    let mut count = 0;
    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
    let mut visited: Vec<(usize, usize)> = vec![];
    queue.push_back((i, j, -1));
    while queue.len() > 0 {
        let point = queue.pop_front().unwrap();
        let val = map[point.0][point.1] as i32;
        if val > point.2 && val != 9 && !visited.contains(&(point.0, point.1)) {
            if point.0 > 0 { queue.push_back((point.0 - 1, point.1, val)) }
            if point.1 > 0 { queue.push_back((point.0, point.1 - 1, val)) }
            if point.0 < map.len() - 1 { queue.push_back((point.0 + 1, point.1, val)) }
            if point.1 < map[0].len() - 1 { queue.push_back((point.0, point.1 + 1, val)) }
            visited.push((point.0, point.1));
            count += 1;
        }
    }
    return count;
}

fn part_two(map: &Vec<Vec<u32>>, points: &Vec<(usize,usize)>){
    let mut sizes: Vec<i32> = points.iter().map(|p| basin(map, p.0, p.1)).collect();
    sizes.sort_by(|a,b| b.cmp(a));
    sizes.truncate(3);
    println!("Part Two: {}", sizes.iter().product::<i32>());
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let map: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let points = part_one(&map);
    part_two(&map, &points);
}

use std::fs;

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let mut map: Vec<Vec<i32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();
    for i in 0..map.len(){
        map[i].push(i32::MIN);
        map[i].insert(0, i32::MIN);
    }
    map.push(vec![i32::MIN; map[0].len()]);
    map.insert(0, vec![i32::MIN; map[0].len()]);
    
    let mut count = 0;
    let d: Vec<(i32,i32)>= (-1..=1).map(|dx| (-1..=1).map(|dy| (dx, dy)).filter(|&x| x != (0,0)).collect::<Vec<(i32,i32)>>()).flatten().collect();
    let mut change:bool;
    for i in 0..u32::MAX{
        change = true;
        for x in 1..map[0].len()-1 {
            for y in 1..map.len()-1{
                map[x][y] += 1;
            }
        }
        while change{
            change = false;
            for x in 1..map[0].len()-1 {
                for y in 1..map.len()-1{
                    if map[x][y] > 9 {
                        map[x][y] = 0;
                        for (dx, dy) in &d{
                            if map[(x as i32 + dx) as usize][(y as i32 + dy) as usize] != 0 {
                                map[(x as i32 + dx) as usize][(y as i32 + dy) as usize] += 1;
                            }
                        }
                        change = true;
                        count += 1;
                    }
                }
            }
        }
        if i == 99 { println!("Part One: {}", count) }
        if (1..map[0].len()-1).map(|x| (1..map.len()-1).map(|y| map[x][y]).sum::<i32>()).sum::<i32>() == 0 {
            println!("Part Two: {}", i+1);
            break;
        }
    }
}

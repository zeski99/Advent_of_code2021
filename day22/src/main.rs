use std::fs;

fn exec(instructions: Vec<(bool, Vec<i64>)>) -> i64 {
    let mut xs: Vec<i64> = instructions.iter().map(|i| i.1[0..2].to_vec()).flatten().collect::<Vec<i64>>();
    xs.sort(); xs.dedup();
    let mut ys: Vec<i64> = instructions.iter().map(|i| i.1[2..4].to_vec()).flatten().collect::<Vec<i64>>();
    ys.sort(); ys.dedup();
    let mut zs: Vec<i64> = instructions.iter().map(|i| i.1[4..6].to_vec()).flatten().collect::<Vec<i64>>();
    zs.sort(); zs.dedup();
    let mut map: Vec<Vec<Vec<bool>>> = vec![vec![vec![false;zs.len()];ys.len()];xs.len()];
    
    for ins in instructions {
        for x in xs.iter().position(|&e| e == ins.1[0]).unwrap()..xs.iter().position(|&e| e == ins.1[1]).unwrap(){
            for y in ys.iter().position(|&e| e == ins.1[2]).unwrap()..ys.iter().position(|&e| e == ins.1[3]).unwrap(){
                for z in zs.iter().position(|&e| e == ins.1[4]).unwrap()..zs.iter().position(|&e| e == ins.1[5]).unwrap(){
                    map[x][y][z] = ins.0;
                }
            }
        }
    }
    let mut total = 0;
    for x in 1..xs.len() {
        for y in 1..ys.len() {
            for z in 1..zs.len() {
                if map[x-1][y-1][z-1] {
                    total += (xs[x] - xs[x-1]) * (ys[y] - ys[y-1]) * (zs[z] - zs[z-1]);
                }
            }
        }
    }
    total
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let instructions: Vec<(bool, Vec<i64>)> = input.lines().map(|l| l.split(&[' ', '.', '=', ','][..]))
        .map(|mut spl| (spl.next().unwrap() == "on", spl.filter_map(|s| s.parse::<i64>().ok()).collect::<Vec<i64>>()))
        .map(|(b,v)| (b, vec![v[0], v[1]+1, v[2], v[3]+1, v[4], v[5]+1])).collect();

    println!("Part One: {}", exec(instructions.iter()
        .map(|(b,v)| (*b, vec![v[0].max(-50), v[1].min(51), v[2].max(-50), v[3].min(51), v[4].max(-50), v[5].min(51)])).collect()));
    println!("Part Two: {}", exec(instructions));
}

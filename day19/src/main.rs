use std::fs;
use std::collections::{HashSet,HashMap};

//this is just horrible, please dont look
fn rot(cube: &Vec<Vec<i64>>) -> Vec<Vec<Vec<i64>>> {
    let mut rots: Vec<Vec<Vec<i64>>> = vec![];
    let opts = [(1,1,1,0,1,2), (-1,1,1,1,0,2), (-1,-1,1,0,1,2), (1,-1,1,1,0,2),
                (-1,1,-1,0,1,2), (1,1,-1,1,0,2), (1,-1,-1,0,1,2), (-1,-1,-1,1,0,2),
                (-1,1,1,2,1,0), (-1,-1,1,1,2,0), (1,-1,1,2,1,0), (1,1,1,1,2,0),
                (1,1,-1,2,1,0), (1,-1,-1,1,2,0), (-1,-1,-1,2,1,0), (-1,1,-1,1,2,0),
                (1,-1,1,0,2,1), (1,1,1,2,0,1), (-1,1,1,0,2,1), (-1,-1,1,2,0,1),
                (-1,-1,-1,0,2,1), (-1,1,-1,2,0,1), (1,1,-1,0,2,1), (1,-1,-1,2,0,1) ];
    for (a,b,c,x,y,z) in opts{
        rots.push(cube.iter().map(|v| vec![a*v[x], b*v[y], c*v[z]]).collect());
    }
    rots
}

fn matching(fixed: &mut HashMap<usize, Vec<Vec<i64>>>, cand: &Vec<Vec<Vec<i64>>>, idx: usize) -> Option<(i64,i64,i64)> {
    if fixed.contains_key(&idx) { return None }
    for rot in cand {
        for point in rot {
            for f in fixed.values() {
                for l in f {
                    let base = (l[0] - point[0], l[1] - point[1], l[2] - point[2]);
                    let mapped: Vec<Vec<i64>> = rot.iter().map(|l| vec![l[0] + base.0, l[1] + base.1, l[2] + base.2]).collect();
                    if mapped.iter().filter(|l| f.contains(l)).count() >= 12 {
                        fixed.insert(idx, mapped);
                        return Some(base);
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let input: Vec<Vec<Vec<Vec<i64>>>> = fs::read_to_string("in.txt").expect("h").split("\n\n")
        .map(|sc| sc.lines().skip(1)
            .map(|l| l.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect())
            .collect())
        .collect::<Vec<Vec<Vec<i64>>>>().iter().map(rot).collect();
    let mut fixed: HashMap<usize, Vec<Vec<i64>>> = HashMap::new();
    fixed.insert(0, input.get(0).unwrap().get(0).unwrap().to_vec());
    let mut scanners: Vec<(i64,i64,i64)> = vec![(0,0,0)];

    while input.len() > fixed.len(){
        for (i,e) in input.iter().enumerate(){
            match matching(&mut fixed, e, i) {
                Some(s) => scanners.push(s),
                None => continue
            }
        }
    }
    let mut set: HashSet<Vec<i64>> = HashSet::new();
    for v in fixed.values(){
        for p in v {
            set.insert(p.clone());
        }
    }
    println!("Part One: {}", set.len());
    let mut max = 0;
    for a in &scanners {
        for b in &scanners {
            if a == b { continue }
            let dist = (a.0-b.0).abs() + (a.1-b.1).abs() + (a.2-b.2).abs();
            if dist > max {
                max = dist;
            }
        }
    }
    println!("Part Two: {}", max);
}

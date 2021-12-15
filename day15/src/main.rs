use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(map: &Vec<Vec<i32>>) -> i32{
    let mut memo: Vec<Vec<i32>> = (0..map.len()).map(|_| vec![i32::MAX; map[0].len()]).collect();
    let mut pqueue: BinaryHeap<State> = BinaryHeap::new();
    pqueue.push(State {x:1, y:1, cost:0});
    while let Some(State { x, y, cost }) = pqueue.pop() {
        if (x,y) == (map[0].len() - 2, map.len() - 2){
            return cost;
        }
        if memo[x][y] <= cost{
            continue;
        }
        memo[x][y] = cost;
        pqueue.push(State {x:x+1, y:y, cost:cost+map[x+1][y]});
        pqueue.push(State {x:x-1, y:y, cost:cost+map[x-1][y]});
        pqueue.push(State {x:x, y:y+1, cost:cost+map[x][y+1]});
        pqueue.push(State {x:x, y:y-1, cost:cost+map[x][y-1]});
    }
    return -1;
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let map: Vec<Vec<i32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();
    let mut map2 = map.clone();
    for i in 0..map2.len(){
        map2[i].push(i32::MAX/2);
        map2[i].insert(0, i32::MAX/2);
    }
    map2.push(vec![i32::MAX/2; map2[0].len()]);
    map2.insert(0, vec![i32::MAX/2; map2[0].len()]);
    println!("Part One: {}", dijkstra(&map2));
    let mut tmp_map: Vec<Vec<i32>> = vec![];
    let mut map2: Vec<Vec<i32>> = vec![];
    for m in map{
        let mut tmp: Vec<i32> = m.clone();
        for i in 0..4{
            tmp.extend(m.iter().map(|e| (e+i) % 9 + 1));
        }
        tmp_map.push(tmp);
    }
    for i in 0..5{
        for m in tmp_map.iter(){
            map2.push(m.iter().map(|e| (e+i-1) % 9 + 1).collect());
        }
    }
    for i in 0..map2.len(){
        map2[i].push(i32::MAX/2);
        map2[i].insert(0, i32::MAX/2);
    }
    map2.push(vec![i32::MAX/2; map2[0].len()]);
    map2.insert(0, vec![i32::MAX/2; map2[0].len()]);
    println!("Part Two: {}", dijkstra(&map2));
}

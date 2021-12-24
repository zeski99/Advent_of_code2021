use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

static COSTS: [usize;5] = [0,1,10,100,1000];
static DIST: [[usize;4];7] = [[2, 4, 6, 8], [1, 3, 5, 7], [1, 1, 3, 5], [3, 1, 1, 3], [5, 3, 1, 1], [7, 5, 3, 1], [8, 6, 4, 2]];

const SIZE: usize = 4; //part one: 2, part two: 4

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    hall: [usize;7],
    rooms: [[usize;SIZE];4]
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

fn done(st: &State) -> bool {
    for (i,room) in st.rooms.iter().enumerate() {
        for r in room{
            if *r != i + 1 {
                return false;
            }
        }
    }
    true
}

fn make_move(state: &State, x:usize, y:usize, h:usize) -> State {
    let mut hall = state.hall.clone();
    let mut rooms = state.rooms.clone();
    let move_cost =  COSTS[cmp::max(hall[h], rooms[x][y])] * (y+1 + DIST[h][x]);
    hall[h] = state.rooms[x][y];
    rooms[x][y] = state.hall[h];
    State {cost: state.cost + move_cost, hall:hall, rooms:rooms}
}

fn move_in(state: &State) -> Option<State> {
    for (i,room) in state.rooms.iter().enumerate() {
        for (j,e) in room.iter().enumerate().rev() {
            if *e == 0 {
                for h in (0..i+2).rev() {
                    if state.hall[h] != 0 {
                        if state.hall[h] == i+1 { return Some(make_move(&state, i, j, h)) }
                        break
                    }
                }
                for h in i+2..7 {
                    if state.hall[h] != 0 {
                        if state.hall[h] == i+1 { return Some(make_move(&state, i, j, h)) }
                        break
                    }
                }
            }
            if *e != i+1 { break }
        }
    }
    None
}

fn move_out(state: &State) -> Vec<State> {
    let mut out: Vec<State> = vec![];
    for (i,room) in state.rooms.iter().enumerate() {
        for (j,e) in room.iter().enumerate() {
            let mut remove = false;
            if *e == i+1 { 
                for k in j+1..room.len() {
                    if room[k] != i+1 {
                        remove = true;
                        break
                    }
                }
                if !remove { break }
            }
            else if *e != i+1 {
                remove = *e != 0
            }
            if remove {
                for h in (0..i+2).rev() {
                    if state.hall[h] != 0 { break }
                    out.push(make_move(&state, i, j, h));
                }
                for h in i+2..7 {
                    if state.hall[h] != 0 { break }
                    out.push(make_move(&state, i, j, h));
                }
                break
            }
        }
    }
    out
}

fn solve(initial: State) -> usize {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(initial);
    while let Some(state) = heap.pop() {
        if done(&state) {
            return state.cost
        }
        if let Some(st) = move_in(&state) {
            heap.push(st)
        }
        for st in move_out(&state) {
            heap.push(st)
        }
    }
    0
}

fn main() {
    // let state = State { cost: 0, hall: [0;7], rooms: [[1,3],[4,4],[3,2],[1,2]]};
    // println!("Part One: {}", solve(state));
    let state = State { cost: 0, hall: [0;7], rooms: [[1,4,4,3],[4,3,2,4],[3,2,1,2],[1,1,3,2]]};
    println!("Part Two: {}", solve(state));
}

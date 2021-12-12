use std::fs;
use std::collections::{HashSet, HashMap};

fn part_one(graph: &HashMap<&str, HashSet<&str>>, visited: &HashSet<&str>, current: &str) -> u32{
    if current == "end"{
        return 1;
    }
    if visited.contains(current) { 
        return 0
    }

    let mut vis_next = visited.clone();
    if current.chars().all(|x| x.is_lowercase()){
        vis_next.insert(current);
    }
    let mut sum = 0;
    for next in graph.get(current).unwrap().iter(){
        sum += part_one(graph, &vis_next, next);
    }
    return sum;
}

fn part_two(graph: &HashMap<&str, HashSet<&str>>, visited: &HashSet<&str>, current: &str, mut twice: bool) -> u32{
    if current == "end"{
        return 1;
    }
    if visited.contains(current) { 
        if twice || current == "start" {
            return 0
        }
        else{
            twice = true;
        }
    }
    let mut vis_next = visited.clone();
    if current.chars().all(|x| x.is_lowercase()){
        vis_next.insert(current);
    }
    let mut sum = 0;
    for next in graph.get(current).unwrap().iter(){
        sum += part_two(graph, &vis_next, next, twice);
    }
    return sum;
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines(){
        let (a,b) = line.split_once("-").unwrap();
        if graph.contains_key(a){
            graph.get_mut(a).unwrap().insert(b);
        }
        else{
            graph.insert(a, HashSet::from([b]));
        }
        if graph.contains_key(b){
            graph.get_mut(b).unwrap().insert(a);
        }
        else{
            graph.insert(b, HashSet::from([a]));
        }
    }
    println!("Part One: {}", part_one(&graph, &HashSet::new(), "start"));
    println!("Part Two: {}", part_two(&graph, &HashSet::new(), "start", false));
}

use std::fs;

#[derive(Clone)]
enum Body{
    Val(u64),
    Child(usize,usize),
    No
}

#[derive(Clone)]
struct Node{
    body: Body,
    parent: Option<usize>
}

use Body::*;

fn parse_num(line: &str, tree: &mut Vec<Node>, p: Option<usize>) -> usize {
    let mut next = Node {body: No, parent: p};
    match line.parse::<u64>(){
        Ok(n) => { 
            next = Node {body: Val(n), parent: p}
        },
        Err(_) => ()
    }
    let idx = tree.len();
    tree.push(next);
    let mut ctr = 0;
    for (i, c) in line.chars().enumerate(){
        if c == '[' { ctr += 1 }
        if c == ']' { ctr -= 1 }
        if ctr == 1 && c == ','{
            let l = parse_num(&line[1..i], tree, Some(idx));
            let r = parse_num(&line[i+1..line.len()-1], tree, Some(idx));
            let mut curr = tree.get_mut(idx).unwrap();
            curr.body = Child(l,r);
            break;
        }
    }
    return idx;
}

fn add_left(tree: &mut Vec<Node>, i: usize, val: u64) {
    if let Some(idx) = tree.get(i).unwrap().parent {
        let p = tree.get(idx).unwrap();
        match p.body {
            Child(l,r) => {
                if r == i {
                    let mut tmp = tree.get_mut(l).unwrap();
                    loop {
                        match tmp.body {
                            Val(v) => {
                                tmp.body = Val(v+val);
                                break;
                            },
                            Child(_ll,rr) => tmp = tree.get_mut(rr).unwrap(),
                            No => ()
                        }
                    }
                } else {
                    add_left(tree, idx, val);
                }
            },
            _ => ()
        }
    }
}

fn add_right(tree: &mut Vec<Node>, i: usize, val: u64) {
    if let Some(idx) = tree.get(i).unwrap().parent {
        let p = tree.get(idx).unwrap();
        match p.body {
            Child(l,r) => {
                if l == i {
                    let mut tmp = tree.get_mut(r).unwrap();
                    loop {
                        match tmp.body {
                            Val(v) => {
                                tmp.body = Val(v+val);
                                break;
                            },
                            Child(ll,_rr) => tmp = tree.get_mut(ll).unwrap(),
                            No => ()
                        }
                    }
                } else {
                    add_right(tree, idx, val);
                }
            },
            _ => ()
        }
    }
}

fn explode(tree: &mut Vec<Node>, i: usize, depth: u32) -> Option<u64>{
    let node = tree.get_mut(i).unwrap();
    match node.body {
        Val(n) => if depth == 5 { Some(n) } else { None }
        Child(l,r) => {
            if depth == 4 {
                match explode(tree, l, depth+1) {
                    Some(n) => add_left(tree, i, n),
                    None => ()
                };
                match explode(tree, r, depth+1) {
                    Some(n) => add_right(tree, i, n),
                    None => ()
                };
                tree.get_mut(i).unwrap().body = Val(0);
                Some(0)
            } else {
                match explode(tree, l, depth+1) {
                    Some(n) => return Some(n),
                    None => ()
                };
                match explode(tree, r, depth+1) {
                    Some(n) => return Some(n),
                    None => None
                }
            }
        },
        No => None
    }
}

fn split(tree: &mut Vec<Node>, i: usize) -> bool {
    let len = tree.len();
    let mut node = tree.get_mut(i).unwrap();
    match node.body {
        Val(n) => {
            if n >= 10 {
                node.body = Child(len, len + 1);
                tree.push(Node {body: Val(n / 2), parent: Some(i)});
                tree.push(Node {body: Val(n / 2 + n % 2), parent: Some(i)});
                return true;
            } 
            false
        }
        Child(l,r) => {
            if split(tree, l) { return true; }
            if split(tree, r) { return true; }
            false
        },
        No => false
    }
}

fn reduce(tree: &mut Vec<Node>, i: usize){
    loop{
        if explode(tree, i, 0) != None {
            continue;
        }
        if split(tree, i) {
            continue;
        }
        break;
    }
}

fn magnitude(tree: &Vec<Node>, i: usize) -> u64 {
    match tree.get(i).unwrap().body {
        Val(v) => v,
        Child(l,r) => 3 * magnitude(tree, l) + 2 * magnitude(tree, r),
        No => 0
    }
}

fn part_one(tree: &mut Vec<Node>, nodes: &Vec<usize>){
    let mut sum = *nodes.get(0).unwrap();
    reduce(tree, sum);
    for node in nodes.iter().skip(1) {
        let tmp = Node {body: Child(sum, *node), parent: None};
        let len = tree.len();
        tree.get_mut(sum).unwrap().parent = Some(len);
        tree.get_mut(*node).unwrap().parent = Some(len);
        sum = len;
        tree.push(tmp);
        reduce(tree, sum);
    }
    println!("Part One: {}", magnitude(&tree, sum));
}

fn sum(tree: &mut Vec<Node>, i: usize, j: usize) -> u64{
    let tmp = Node {body: Child(i, j), parent: None};
    let len = tree.len();
    tree.get_mut(i).unwrap().parent = Some(len);
    tree.get_mut(j).unwrap().parent = Some(len);
    let out = len;
    tree.push(tmp);
    reduce(tree, out);
    magnitude(tree, out)
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let mut tree_original: Vec<Node> = vec![];
    let nodes: Vec<usize> = input.lines().map(|n| parse_num(n, &mut tree_original, None)).collect();
    part_one(&mut tree_original.clone(), &nodes);
    let mut max = 0;
    for i in &nodes {
        for j in &nodes {
            if i == j { continue }
            let tmp = sum(&mut tree_original.clone(), *i, *j);
            if tmp > max { max = tmp }
        }
    }
    println!("Part Two: {}", max);
}

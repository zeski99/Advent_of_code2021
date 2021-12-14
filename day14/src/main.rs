use std::fs;
use std::collections::HashMap;

fn count(state: &HashMap<(char,char), u64>, last: &char) -> u64{
    let mut counts: HashMap<char,u64> = HashMap::new();
    for ((a,_),v) in state{
        if counts.contains_key(&a){
            *counts.get_mut(&a).unwrap() += v;
        }else{
            counts.insert(*a, *v);
        }
    }
    *counts.get_mut(last).unwrap() += 1;
    return counts.values().max().unwrap() - counts.values().min().unwrap();
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let mut lines = input.lines();
    let poly = lines.next().unwrap();
    let last = &poly.chars().last().unwrap();
    lines.next();
    let rules = lines.map(|l| l.chars().collect::<Vec<char>>()).map(|v| ((v[0],v[1]),v[6])).collect::<HashMap<(char, char), char>>();
    let mut state = HashMap::new();

    for (a,b) in poly.chars().zip(poly.chars().skip(1)){
        if state.contains_key(&(a,b)){
            *state.get_mut(&(a,b)).unwrap() += 1;
        }else{
            state.insert((a,b), 1);
        }
    }
    for i in 0..40{
        let mut new_state = HashMap::new();
        for (k, v) in state.iter(){
            match rules.get(k){
                Some(&c) => {
                    let k1 = (k.0, c);
                    let k2 = (c, k.1);
                    if new_state.contains_key(&k1){
                        *new_state.get_mut(&k1).unwrap() += v;
                    }else{
                        new_state.insert(k1, *v);
                    }
                    if new_state.contains_key(&k2){
                        *new_state.get_mut(&k2).unwrap() += v;
                    }else{
                        new_state.insert(k2, *v);
                    }
                },
                _ => ()
            }
        }
        state = new_state;
        if i == 9{
            println!("Part One: {}", count(&state, &last));
        }
    }
    println!("Part Two: {}", count(&state, &last));
}
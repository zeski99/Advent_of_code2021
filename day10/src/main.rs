use std::fs;

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let mut stack: Vec<char> = vec![];
    let mut sum = 0;
    let mut arr: Vec<u64> = vec![];

    for line in input.lines() {
        for c in line.chars(){
            match c {
                '(' | '{' | '[' | '<' => stack.push(c),
                ')' | '}' | ']' | '>' => {
                    let closing = stack.pop().unwrap();
                    if (closing as i32 - c as i32).abs() > 2 { 
                        sum += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0
                        };
                        stack.clear();
                        break;
                    }
                },
                _ => ()
            }
        }

        let mut tmp: u64 = 0;
        while !stack.is_empty(){
            tmp *= 5;
            tmp += match stack.pop().unwrap() {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0
            };
        }
        if tmp > 0 { arr.push(tmp) }
    }
    println!("Part One: {}", sum);
    arr.sort();
    println!("Part Two: {}", arr[arr.len() / 2]);
}

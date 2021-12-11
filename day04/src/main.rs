use std::fs;

fn when_bingo(board: &Vec<Vec<i32>>) -> i32{
    let mut vals: Vec<i32> = vec![];
    for row in board.iter(){
        let mut max = 0;
        let tmp = *row.iter().max().unwrap();
        if max < tmp{
            max = tmp;
        }
        vals.push(max);
    }

    for col in 0..board.len(){
        let mut max = 0;
        let tmp = board.iter().map(|r| r[col]).max().unwrap();
        if max < tmp{
            max = tmp;
        }
        vals.push(max);
    }
    let min = vals.iter().min().unwrap();
    return *min;
}

fn calc(index: &Vec<Vec<i32>>, board: &Vec<Vec<i32>>, round: &i32) -> i32{
    let sum: i32 = board.iter().zip(index.iter())
        .map(|(a,b)| a.iter().zip(b.iter())
            .filter(|(_a,b)| *b > round).map(|(a,_b)| a).sum::<i32>())
        .sum::<i32>();
    let last: i32 = board.iter().zip(index.iter())
        .map(|(a,b)| a.iter().zip(b.iter())
            .filter(|(_a,b)| *b == round).map(|(a,_b)| a).sum::<i32>())
        .sum::<i32>();
    return sum*last;
}

fn part_one(indexes: &Vec<Vec<Vec<i32>>>, boards: &Vec<Vec<Vec<i32>>>, vals: &Vec<i32>){
    let min = vals.iter().min().unwrap();
    let idx = vals.iter().position(|m| m == min).unwrap();
    println!("Part One: {}", calc(&indexes[idx], &boards[idx], &min));
}

fn part_two(indexes: &Vec<Vec<Vec<i32>>>, boards: &Vec<Vec<Vec<i32>>>, vals: &Vec<i32>){
    let max = vals.iter().max().unwrap();
    let idx = vals.iter().position(|m| m == max).unwrap();
    println!("Part Two: {}", calc(&indexes[idx], &boards[idx], &max));
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let lines = input.lines();
    let mut seq: Vec<i32> = vec![]; 
    let mut boards: Vec<Vec<Vec<i32>>> = vec![];
    let mut tmp: Vec<Vec<i32>> = vec![];

    for line in lines{
        if seq.len() == 0{
            seq = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        }
        else if line.trim() == ""{
            if tmp.len() != 0{
                boards.push(tmp);
            }
            tmp = vec![];
        }
        else{
            tmp.push(line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect());
        }
    }

    let indexes: Vec<Vec<Vec<i32>>> = boards.iter().map(|b| b.iter().map(|r| r.iter().map(|e| seq.iter().position(|x| x == e).unwrap() as i32).collect()).collect()).collect();
    let vals: Vec<i32> = indexes.iter().map(|b| when_bingo(b)).collect();
    part_one(&indexes, &boards, &vals);
    part_two(&indexes, &boards, &vals);
}

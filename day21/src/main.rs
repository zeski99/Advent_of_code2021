use std::collections::HashMap;

fn part_one(pos: [i8;2]) {
    let mut pos = pos;
    let mut score = [0,0];
    let mut step = 6;
    for i in 0.. {
        pos[i&1] = (pos[i&1] + step - 1) % 10 + 1;
        score[i&1] += pos[i&1] as u32;
        if score[i&1] >= 1000 {
            println!("Part One: {}", (i+1)*3*score[(i&1)^1] as usize);
            break;
        }
        step = (step + 9) % 10;
    }
}

fn part_two(pos: [i8;2], score: [u32;2], turn: usize, memo: &mut HashMap<([i8;2],[u32;2],usize), [usize;2]>) -> [usize;2] {
    if score[turn^1] >= 21 {
        return [turn, turn^1];
    }
    match memo.get(&(pos, score, turn)) {
        Some(&ret) => return ret,
        None => ()
    }
    let mut out = [0,0];
    for d1 in [1,2,3] {
        for d2 in [1,2,3] {
            for d3 in [1,2,3] {
                let mut new_pos = pos;
                let mut new_score = score;
                new_pos[turn] = (pos[turn] + d1 + d2 + d3 - 1) % 10 + 1;
                new_score[turn] += new_pos[turn] as u32;
                let tmp = part_two(new_pos, new_score, turn^1, memo);
                out[0] += tmp[0];
                out[1] += tmp[1];
            }
        }
    }
    memo.insert((pos, score, turn), out);
    out
}

fn main() {
    let pos = [7,1];
    part_one(pos);
    println!("Part Two: {}", part_two(pos, [0,0], 0, &mut HashMap::new()).iter().max().unwrap());
}

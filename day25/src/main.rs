use std::fs;

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let h = map.len();
    let w = map[0].len();
    let mut moved;
    for n in 1.. {
        moved = false;
        let tmp = map.clone();
        for y in 0..h {
            for x in (0..w).rev() {
                if tmp[y][x] == '>' && tmp[y][(x+1)%w] == '.' {
                    map[y][(x+1)%w] = '>';
                    map[y][x] = '.';
                    moved = true;
                }
            }
        }
        let tmp = map.clone();
        for y in (0..h).rev() {
            for x in 0..w {
                if tmp[y][x] == 'v' && tmp[(y+1)%h][x] == '.' {
                    map[(y+1)%h][x] = 'v';
                    map[y][x] = '.';
                    moved = true;
                }
            }
        }
        if !moved {
            println!("Part One: {}", n);
            break;
        }
    }
}

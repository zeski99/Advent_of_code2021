use std::fs;

fn step(map: &Vec<Vec<u8>>, algo: &Vec<u8>) -> Vec<Vec<u8>> {
    let x = map[0].len();
    let y = map.len();
    let mut out: Vec<Vec<u8>> = vec![vec![map[0][0] ^ 1;x];y];
    for i in 1..x-1{
        for j in 1..y-1{
            let num : usize = (i-1..=i+1).map(|di| (j-1..=j+1)
                                        .map(|dj| map[di][dj]).collect::<Vec<u8>>())
                                        .flatten()
                                        .fold(0, |acc, x| 2*acc + x as usize);
            out[i][j] = algo[num];
        }
    }
    out
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let mut lines = input.lines();
    let algo: Vec<u8> = lines.next().unwrap().chars().map(|c| if c == '#' {1} else {0}).collect();
    lines.next();
    let map: Vec<Vec<u8>> = lines.map(|l| l.chars().map(|c| if c == '#' {1} else {0}).collect()).collect();
    let d = 51;
    let mut map: Vec<Vec<u8>> = [vec![vec![0; map[0].len()+2*d]; d], 
                            map.iter().map(|l| [vec![0;d], l.to_vec(), vec![0;d]].concat()).collect(), 
                            vec![vec![0; map[0].len()+2*d]; d]].concat();
    
    for i in 1..d {
        map = step(&map, &algo);
        if i == 2 {
            println!("Part One: {}", map.iter().map(|l| l.iter().sum::<u8>() as usize).sum::<usize>());
        }
    }
    println!("Part Two: {}", map.iter().map(|l| l.iter().sum::<u8>() as usize).sum::<usize>());
}
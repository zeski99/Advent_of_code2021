use std::fs;

struct Packet{
    ver: u64,
    typ: u64,
    val: u64,
    sub: Vec<Packet>,
}

fn hex_to_bin(hex: &str) -> String {
    hex.chars().map(to_bin).collect()
}

fn to_bin(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn read_packets(input: &str, len: usize, ltype: char) -> (Vec<Packet>, usize){
    let mut idx = 0;
    let mut pcount = 0;
    let mut out: Vec<Packet> = vec![];
    while idx < input.len() - 8 && (ltype == '0' || pcount < len){
        let ver = u64::from_str_radix(&input[idx..idx+3], 2).unwrap();
        let typ = u64::from_str_radix(&input[idx+3..idx+6], 2).unwrap();
        idx += 6;
        let mut val:u64 = 0;
        let mut sub = vec![];
        if typ == 4{
            for i in 0..{
                let tmp = u64::from_str_radix(&input[idx+5*i..idx+5*(i+1)], 2).unwrap();
                val = (val << 4) + (tmp & 0xf);
                if tmp & 0x10 == 0 {
                    idx += 5*(i+1);
                    break
                }
            }
        }
        else{
            if &input[idx..idx+1] == "0"{
                let len = usize::from_str_radix(&input[idx+1..idx+16],2).unwrap();
                let ret = read_packets(&input[idx+16..idx+16+len], len, '0');
                idx += ret.1 + 16;
                sub = ret.0;
            }
            else{
                let len = usize::from_str_radix(&input[idx+1..idx+12],2).unwrap();
                let ret = read_packets(&input[idx+12..], len, '1');
                idx += ret.1 + 12;
                sub = ret.0;
            }
        }
        out.push(Packet {ver: ver, typ: typ, val: val, sub: sub});
        pcount += 1;
    }
    return (out, idx);
}

fn part_one(packet: &Packet) -> u64 {
    packet.ver + packet.sub.iter().map(part_one).sum::<u64>()
}

fn part_two(packet: &Packet) -> u64{
    match packet.typ{
        0 => packet.sub.iter().map(part_two).sum(),
        1 => packet.sub.iter().map(part_two).product(),
        2 => packet.sub.iter().map(part_two).min().unwrap(),
        3 => packet.sub.iter().map(part_two).max().unwrap(),
        4 => packet.val,
        5 => (part_two(&packet.sub[0]) > part_two(&packet.sub[1])) as u64,
        6 => (part_two(&packet.sub[0]) < part_two(&packet.sub[1])) as u64,
        7 => (part_two(&packet.sub[0]) == part_two(&packet.sub[1])) as u64,
        _ => 0
    }
}

fn main() {
    let input = fs::read_to_string("in.txt").expect("h");
    let input = hex_to_bin(&input);
    let p = read_packets(&input, 1, '0').0;
    println!("Part One: {}", part_one(&p[0]));
    println!("Part Two: {}", part_two(&p[0]));
}

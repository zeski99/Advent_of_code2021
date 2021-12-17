fn main() {
    let xmin = 111;
    let xmax = 161;
    let ymin = -154;
    let ymax = -101;
    println!("Part One: {}", ((-ymin-1)*(-ymin))/2);
    let mut count = 0;
    for sx in 1..=xmax{
        for sy in ymin..-ymin{
            let mut x = 0;
            let mut dx = sx;
            let mut y = 0;
            let mut dy = sy;
            while y > ymin && x < xmax{
                x += dx;
                y += dy;
                dy -= 1;
                if dx > 0 { dx -= 1 }
                if xmin <= x && xmax >= x && ymin <= y && ymax >= y {
                    count += 1;
                    break;
                }
            }
        }
    }
    println!("Part Two: {}", count);
}

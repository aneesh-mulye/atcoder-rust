use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i32; n],
    }

    let mut coord: f64 = 0.5;
    let mut zero_crossings = 0;
    for li in l {
        let next_coord = if coord > 0.0 {
            coord - (li as f64)
        } else {
            coord + (li as f64)
        };
        if coord.signum() * next_coord.signum() < 0.0 {
            zero_crossings += 1;
        }
        coord = next_coord;
    }

    println!("{zero_crossings}");
}

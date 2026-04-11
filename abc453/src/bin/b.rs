use num::abs;
use proconio::input;

fn main() {
    input! {
        t: i32,
        x: i32,
        a: [i32; t+1],
    }

    let mut saved = a[0];
    println!("0 {}", saved);
    for i in 1..=t {
        if abs(a[i as usize] - saved) >= x {
            saved = a[i as usize];
            println!("{i} {}", saved);
        }
    }
}

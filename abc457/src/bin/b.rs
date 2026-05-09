use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [[u16]; n],
        x: Usize1,
        y: Usize1,
    }

    println!("{}", a[x][y]);
}

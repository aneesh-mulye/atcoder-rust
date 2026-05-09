use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u8; n],
        x: usize,
    }

    println!("{}", a[x - 1]);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[u32]; n],
        c: [usize; n],
    }

    let mut remaining = k;
    for i in 0..n {
        if remaining <= a[i].len() * c[i] {
            println!("{}", a[i][(remaining - 1) % a[i].len()]);
            break;
        } else {
            remaining -= a[i].len() * c[i];
        }
    }
}

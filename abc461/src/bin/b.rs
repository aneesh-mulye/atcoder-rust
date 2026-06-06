use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
    }

    for i in 0..n {
        if b[a[i]] != i {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

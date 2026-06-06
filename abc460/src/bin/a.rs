use proconio::input;

fn main() {
    input! {
        mut n: i32,
        mut m: i32,
    }

    let mut i = 0;

    while m != 0 {
        m = n % m;
        i += 1;
    }

    println!("{}", i);
}

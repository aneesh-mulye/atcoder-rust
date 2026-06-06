use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        x: Usize1,
    }

    let mut s = String::from("HelloWorld");
    s.remove(x);
    println!("{}", s);
}

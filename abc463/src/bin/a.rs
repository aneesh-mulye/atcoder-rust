use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    };

    println!("{}", if 9 * x == 16 * y { "Yes" } else { "No" });
}

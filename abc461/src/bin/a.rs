use proconio::input;

fn main() {
    input! {
        a: u8,
        d: u8,
    }

    println!("{}", if a <= d { "Yes" } else { "No" });
}

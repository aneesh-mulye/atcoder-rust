use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!(
        "{}",
        if s.matches('E').count() > s.matches('W').count() {
            "East"
        } else {
            "West"
        }
    );
}

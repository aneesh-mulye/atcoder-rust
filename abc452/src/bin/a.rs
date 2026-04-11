use proconio::input;

fn main() {
    input! {
        m: u8,
        d: u8,
    }

    println!(
        "{}",
        if (m == 1 && d == 7)
            || (m == 3 && d == 3)
            || (m == 5 && d == 5)
            || (m == 7 && d == 7)
            || (m == 9 && d == 9)
        {
            "Yes"
        } else {
            "No"
        }
    );
}

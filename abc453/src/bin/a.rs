use proconio::input;

fn main() {
    input! {
        _: usize,
        mut s: String,
    }

    s = String::from(s.trim_start_matches("o"));
    if !s.is_empty() {
        println!("{}", s);
    }
}

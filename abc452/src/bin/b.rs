use proconio::input;

fn main() {
    input! {
        h: u8,
        w: u8,
    }

    println!("{}", "#".repeat(w as usize));
    for _ in 0..(h - 2) {
        print!("#");
        print!("{}", ".".repeat((w - 2) as usize));
        println!("#");
    }
    println!("{}", "#".repeat(w as usize));
}

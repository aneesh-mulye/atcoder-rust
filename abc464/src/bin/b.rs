use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        _: usize,
        mut c: [Chars; h],
    }

    while c[0].iter().all(|&ch| ch == '.') {
        c.drain(..1);
    }
    while c[c.len() - 1].iter().all(|&ch| ch == '.') {
        c.drain(c.len() - 1..);
    }
    while c.iter().all(|r| r[0] == '.') {
        for row in &mut c {
            row.drain(..1);
        }
    }
    while c.iter().all(|r| r[r.len() - 1] == '.') {
        for row in &mut c {
            row.drain(row.len() - 1..);
        }
    }

    for row in c {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

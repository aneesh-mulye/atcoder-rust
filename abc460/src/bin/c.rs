use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i32; n],
        mut b: [i32; m],
    }

    a.sort();
    b.sort();

    let mut sushi = 0;
    let mut i = 0;
    let mut j = 0;

    while i < n && j < m {
        if 2 * a[i] >= b[j] {
            i += 1;
            j += 1;
            sushi += 1;
        } else {
            i += 1;
        }
    }

    println!("{}", sushi);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: char,
        sn: [String; n],
    }

    let index = match x {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        _ => 100,
    } as usize;

    for s in sn {
        if s.chars().nth(index).unwrap() == 'o' {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

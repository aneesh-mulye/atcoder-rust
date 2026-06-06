use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let mut op = String::new();

    for s in ss {
        op.push(strmap(&s) as char)
    }

    println!("{}", op);
}

fn strmap(s: &String) -> u8 {
    let mut c = s.as_bytes()[0];
    if (c - 'a' as u8) / 3 <= 5 {
        return '0' as u8 + (c - 'a' as u8) / 3 + 2;
    }
    if c == 's' as u8 {
        return '0' as u8 + 7;
    }
    if c >= 't' as u8 && c <= 'v' as u8 {
        return '0' as u8 + 8;
    }
    '0' as u8 + 9
}

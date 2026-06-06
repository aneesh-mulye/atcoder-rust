use proconio::input;

fn main() {
    input! {
        t: usize,
        cases: [[i64; 6]; t],
    }

    for case in cases {
        println!("{}", intersectp(&case));
    }
}

fn intersectp(case: &Vec<i64>) -> &'static str {
    match case.as_slice() {
        [x1, y1, r1, x2, y2, r2] => {
            let cdist = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
            if cdist <= (r1 + r2) * (r1 + r2) && cdist >= (r1 - r2) * (r1 - r2) {
                "Yes"
            } else {
                "No"
            }
        }
        _ => "",
    }
}

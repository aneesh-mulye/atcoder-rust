use proconio::input;

fn main() {
    input! { n: usize }

    // Initialize N×N with zeros (or whatever default)
    let mut distances = vec![vec![0u64; n]; n];

    for i in 0..n - 1 {
        input! { row: [u64; n - 1 - i] }
        for (j, val) in row.into_iter().enumerate() {
            distances[i][i + 1 + j] = val;
        }
    }

    println!("{}", triangle_break(&distances));
}

fn triangle_break(distances: &Vec<Vec<u64>>) -> &str {
    let n = distances.len();

    for a in 0..n {
        for b in (a + 1)..n {
            for c in (b + 1)..n {
                if distances[a][b] + distances[b][c] < distances[a][c] {
                    return "Yes";
                }
            }
        }
    }

    "No"
}

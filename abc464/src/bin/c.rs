use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut changes: [(Usize1, Usize1, Usize1); n],
    }

    changes.sort_by_key(|k| k.1);
    let mut frequencies: Vec<usize> = vec![0; n];
    let mut total_colours: usize = 0;
    for (initial, day, fnal) in &changes {
        if *day == 0 {
            if frequencies[*fnal] == 0 {
                total_colours += 1;
            }
            frequencies[*fnal] += 1;
            continue;
        } else {
            if frequencies[*initial] == 0 {
                total_colours += 1;
            }
            frequencies[*initial] += 1;
        }
    }

    let mut current_index: usize = 0;
    while changes[current_index].1 == 0 {
        current_index += 1;
    }
    println!("{}", total_colours);
    for j in 1..m {
        if current_index >= changes.len() || changes[current_index].1 > j {
            println!("{}", total_colours);
            continue;
        }

        while current_index < changes.len() && changes[current_index].1 == j {
            let (initial, _, fnal) = changes[current_index];
            if initial != fnal {
                if frequencies[initial] == 1 {
                    total_colours -= 1;
                }
                if frequencies[fnal] == 0 {
                    total_colours += 1;
                }
                frequencies[initial] -= 1;
                frequencies[fnal] += 1;
            }

            current_index += 1;
        }

        println!("{}", total_colours);
    }
}

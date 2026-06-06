use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        gems_target: i64,
        colours_target: i64,
        mut gems: [(i64, i64); n],
    }

    gems.sort_by_key(|x| x.1);
    gems.reverse();
    let mut colours_in: HashSet<i64> = HashSet::new();
    let mut gems_value: i64 = 0;
    let mut gems_selected: i64 = 0;
    let mut i: usize = 0;

    while gems_selected != gems_target {
        let (colour, value) = gems[i];
        if !(colours_target - (colours_in.len() as i64) == gems_target - gems_selected
            && colours_in.contains(&colour))
        {
            gems_selected += 1;
            gems_value += value;
            colours_in.insert(colour);
        }
        i += 1;
    }
    println!("{}", gems_value);
}

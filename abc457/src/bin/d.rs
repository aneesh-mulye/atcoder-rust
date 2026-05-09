// use itertools::Itertools;
use proconio::input;
// use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        ops_budget: u64,
        a: [u64; n],
    }

    let mut delta: u64 = 1;
    let mut ceiling: u64 = 0;

    while delta != 0 {
        if ops_cost(&a, ceiling + delta) <= ops_budget {
            ceiling += delta;
            delta *= 2;
        } else {
            delta /= 2;
        }
    }

    println!("{}", ceiling);
}

fn ops_cost(a: &[u64], ceiling: u64) -> u64 {
    let mut curr_cost: u64 = 0;

    for i in 0..a.len() {
        if a[i] >= ceiling {
            continue;
        }

        curr_cost += (ceiling - a[i] + i as u64) / (i as u64 + 1);
    }

    curr_cost
}

/*
 * An approach: a function which computes the cost in
 * terms of operations (our given K) for raising the floor to some given
 * number, by iterating over the array.
 *
 * Call this function with some initial guess, and perform binary search on
 * possible such ceilings using this until you find the highest that fits.
 *
 * ADD: Holy crap, that worked! In 119 milliseconds, WTF.
 *
 * Modern machines *STRONK*!
 */

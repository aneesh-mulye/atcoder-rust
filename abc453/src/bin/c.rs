use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [u32; n],
    }

    let zero_crossings = max_zero_crossings(0.5, &l);

    println!("{zero_crossings}");
}

/* Right, the greedy doesn't work because imagine you have two gigantic jumps,
* followed by a string of say 1s, but separated such that the key is to in fact
* focus on what leads to a global optimum.
*
* Yeah, this is DP. Oh well.
*
* Or, you know, I could literally brute force it? Call stack depth 20 is def
* doable, as is 2^20 ops in 2 seconds, I think.
 */

fn max_zero_crossings(coord: f64, l: &[u32]) -> u32 {
    if l.is_empty() {
        return 0;
    }

    let move_dist = l[0] as f64;
    let left_coord = coord - move_dist;
    let right_coord = coord + move_dist;
    (if coord.signum() * left_coord.signum() < 0.0 {
        1
    } else {
        0
    } + max_zero_crossings(left_coord, &l[1..]))
    .max(
        if coord.signum() * right_coord.signum() < 0.0 {
            1
        } else {
            0
        } + max_zero_crossings(right_coord, &l[1..]),
    )
}

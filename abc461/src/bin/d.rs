use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        height: usize,
        width: usize,
        k: usize,
        s: [Chars; height],
    }

    let mut line_totals: Vec<Vec<u16>> = Vec::new();

    for line in s {
        let mut total: u16 = 0;
        let mut row: Vec<u16> = Vec::new();
        for c in line {
            if c == '1' {
                total += 1;
            }
            row.push(total);
        }
        line_totals.push(row);
    }

    // OK, not enough time. But the approach is:
    // Create an additional table of cumulative totals for columns; this plus
    // the one for lines/rows will provide the baseline 1xwhatever or whateverx1
    // totals for calculation.
    // Then DP a 4-d cube of each possible size, starting with 1x1,
    // and building upwards; so a 2x2 cube, for instance, could easily be
    // composed of the 1x1 in its upper left plus 1x2 1x1, etc; or 5x5 is 4x4 +
    // 1x5 + 4x1, etc, whichever.
    // Now you don't have to necessarily *allocate* all these, right? As in, I
    // imagine you could do stuff whereby you're using the precomputed totals
    // in such a way that at one time, you're only keeping one such 'layer' for
    // each mxn sized region, etc?
    // Anyway, this is as far as I've reached. I think this should work, maybe
    // with some refinement, IDK, but the basic idea or approach seems OK to me.
}

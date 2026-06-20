use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        heights_times: [(i64, i64); n],
        num_queries: usize,
        queries: [i64; num_queries],
    }

    // Put them all in a binary heap; extract as you go.
    let mut heights: Vec<i64> = Vec::new();
    let mut leaving_times: Vec<i64> = Vec::new();
    for (height, leaving_time) in heights_times {
        heights.push(height);
        leaving_times.push(leaving_time);
    }

    let mut heights_heap: BinaryHeap<i64> = BinaryHeap::new();
    let mut top_heights_before: Vec<i64> = Vec::new();
    for h in heights.iter().rev() {
        heights_heap.push(*h);
        top_heights_before.push(*heights_heap.peek().unwrap());
    }

    top_heights_before.reverse();

    for query_time in queries {
        let i = leaving_times.partition_point(|&t| t <= query_time);
        println!("{}", top_heights_before[i]);
    }

    /*
    println!("Heights:\n{:?}", heights);
    println!("Leaving times:\n{:?}", leaving_times);
    println!("heights_heap:\n{:?}", heights_heap);
    println!("top_heights_before:\n{:?}", top_heights_before);
    */
}

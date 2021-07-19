use std::{
    cmp::max,
    collections::HashMap,
};

use proconio::input;

fn most_colorful_candy(candy_color: &Vec<i32>, k: usize) -> usize {
    let n = candy_color.len();
    let mut max_color = 1;
    let mut segment_unique_colors = HashMap::new();
    for i in 0..k {
        let count = segment_unique_colors.entry(candy_color[i]).or_insert(1);
        *count += 1;
    }
    for i in (k + 1)..n {
        let count = segment_unique_colors.entry(candy_color[i]).or_insert(1);
        *count += 1;
        let c2 = segment_unique_colors.get_mut(&candy_color[i - k]).unwrap();
        *c2 -= 1;

        if *c2 == 0 {
            segment_unique_colors.remove(&candy_color[i - k]);
        }

        max_color = max(max_color, segment_unique_colors.len());
    }
    max_color
}

fn main() {
    input! {
        n: i32,
        k: usize,
        candy_color: [i32; n]
    }

    let most_colorful_candy = most_colorful_candy(&candy_color, k);
    println!("{}", &most_colorful_candy);
}

use std::{collections::BTreeMap, fs};

use itertools::Itertools;

fn sum(input: &str) -> u32 {
    let nums = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().expect("valid int"))
        .collect_vec();
    // just keep a map of the number and the rightmost index, since all else will be discarded
    let idx_map: BTreeMap<u32, usize> =
        nums.iter()
            .enumerate()
            .fold(BTreeMap::new(), |mut acc, (idx, n)| {
                acc.entry(*n).and_modify(|v| *v = idx).or_insert(idx);
                acc
            });

    let mut n = 0;
    let mut solution = vec![];
    while n < nums.len() {
        let val = &nums[n];
        if idx_map.contains_key(val) && idx_map[val] > n {
            n = idx_map[val];
        }
        solution.push(*val);
        n += 1;
    }
    solution.iter().sum()
}

pub fn solve() -> u32 {
    let input = fs::read_to_string("./inputs/2.txt").expect("file should exist");
    sum(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum("1 4 3 2 4 7 2 6 3 6"), 20)
    }
}

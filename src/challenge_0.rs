use std::{collections::BTreeMap, fs};

// 1   2   3
//    abc def

// 4   5   6
// ghi jkl mno

// 7   8   9
// pqrs tuv wxyz

//  0
//  _

pub fn solve() -> String {
    let input = fs::read_to_string("inputs/0.txt").expect("should have read input");
    let mut buttons = BTreeMap::new();
    buttons.insert("2", vec!['a', 'b', 'c']);
    buttons.insert("3", vec!['d', 'e', 'f']);
    buttons.insert("4", vec!['g', 'h', 'i']);
    buttons.insert("5", vec!['j', 'k', 'l']);
    buttons.insert("6", vec!['m', 'n', 'o']);
    buttons.insert("7", vec!['p', 'q', 'r', 's']);
    buttons.insert("8", vec!['t', 'u', 'v']);
    buttons.insert("9", vec!['w', 'x', 'y', 'z']);
    buttons.insert("0", vec![' ']);
    input
        .lines()
        .map(|line| {
            let (button, times) = line.split_once(" ").expect("should have parsed");
            (button, times.parse::<usize>().expect("valid number"))
        })
        .map(|(button, times)| buttons.get(&button).expect("valid input")[times - 1])
        .collect()
}

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
};

type BingoCard = BTreeMap<String, BTreeSet<u8>>;

fn win_on_turn(card: &mut BingoCard, nums: &Vec<u8>) -> u32 {
    nums.iter()
        .take_while(|n| {
            card.iter_mut().for_each(|(_, v)| {
                v.remove(&n);
            });
            card.values().all(|v| !v.is_empty())
        })
        .count() as u32
        + 1
}

fn bingo_card(grid: &str) -> BingoCard {
    let mut m: BingoCard = BTreeMap::new();
    grid.lines().enumerate().for_each(|(row, line)| {
        line.split_ascii_whitespace()
            .enumerate()
            .for_each(|(col, digits)| {
                let d = digits.parse::<u8>().unwrap();
                m.entry(format!("r{row}")).or_default().insert(d);
                m.entry(format!("c{col}")).or_default().insert(d);
                if row == col {
                    m.entry(format!("d1")).or_default().insert(d);
                }
                if col == 4 - row {
                    m.entry(format!("d2")).or_default().insert(d);
                }
            })
    });
    m
}

pub fn solve() -> u32 {
    let grid = r"6  17 34 50 68
    10 21 45 53 66
    5  25 36 52 69
    14 30 33 54 63
    15 23 41 51 62";
    let card = bingo_card(&grid);

    let input = read_to_string("./inputs/14.txt").expect("data file");

    input
        .lines()
        .map(|line| {
            let called: Vec<u8> = line
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            win_on_turn(&mut card.clone(), &called)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_conditions() {
        let grid = r"6  17 34 50 68
        10 21 45 53 66
        5  25 36 52 69
        14 30 33 54 63
        15 23 41 51 62";
        let card = bingo_card(&grid);
        assert_eq!(12, card.len());
        assert!(card.values().all(|v| v.len() == 5))
    }

    #[test]
    fn test_example() {
        let grid = r"6  17 34 50 68
10 21 45 53 66
5  25 36 52 69
14 30 33 54 63
15 23 41 51 62";

        let mut card = bingo_card(&grid);

        let input: Vec<u8> = r"10 5 21 45 53 70 66 4"
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        assert_eq!(7, win_on_turn(&mut card, &input));
    }

    #[test]
    fn test_solve() {
        assert_eq!(4327, solve());
    }
}

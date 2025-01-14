use std::{collections::HashSet, fs};

use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_grid() -> HashSet<(i8, i8)> {
    r"  ##  
 #### 
######
######
 #### 
  ##   "
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, c)| match c {
                    '#' => Some((col as i8, row as i8)),
                    _ => None,
                })
                .collect_vec()
        })
        .flatten()
        .collect()
}

fn execute_moves(moves: Vec<Direction>, valid_squares: HashSet<(i8, i8)>) -> Vec<(i8, i8)> {
    let mut pos = (2, 0); // 0,2 in indices
    let mut positions = vec![]; // don't count starting position in the solution
    for m in moves.iter() {
        let new_pos = match m {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        };
        // don't need to check bounds, since we only have valid positions in the set
        if valid_squares.contains(&new_pos) {
            pos = new_pos;
        }
        positions.push(pos);
    }
    positions
}

fn walk_grid(input: &str) -> u32 {
    let moves = input
        .chars()
        .map(|c| match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect_vec();
    let valid_squares = parse_grid();
    let result = execute_moves(moves, valid_squares);

    result
        .iter()
        .fold(0, |acc, (x, y)| acc + *x as u32 + *y as u32)
}

pub fn solve() -> u32 {
    let input = fs::read_to_string("inputs/3.txt").expect("file should exist");
    walk_grid(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(walk_grid("UDRR"), 14)
    }
}

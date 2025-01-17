//     3
//  2  1  5
//     4
//     6

use std::fs::read_to_string;

struct Die {
    front: u8,
    left: u8,
    top: u8,
}

impl Die {
    fn new(front: u8, left: u8, top: u8) -> Self {
        Self { front, left, top }
    }
    fn roll_left(&mut self) -> u8 {
        let new_front = 7 - self.left;
        self.left = self.front;
        self.front = new_front;
        self.front
    }
    fn roll_up(&mut self) -> u8 {
        let new_front = 7 - self.top;
        self.top = self.front;
        self.front = new_front;
        self.front
    }
    fn roll_down(&mut self) -> u8 {
        let new_front = self.top;
        self.top = 7 - self.front;
        self.front = new_front;
        self.front
    }
    fn roll_right(&mut self) -> u8 {
        let new_front = self.left;
        self.left = 7 - self.front;
        self.front = new_front;
        self.front
    }
}

fn rotate_dice(input: &str) -> u32 {
    let mut die1 = Die::new(1, 2, 3);
    let mut die2 = Die::new(1, 3, 2);
    input
        .chars()
        .map(|d| match d {
            'U' => (die1.roll_up(), die2.roll_up()),
            'D' => (die1.roll_down(), die2.roll_down()),
            'L' => (die1.roll_left(), die2.roll_left()),
            'R' => (die1.roll_right(), die2.roll_right()),
            _ => unreachable!(),
        })
        .enumerate()
        // .inspect(|x| println!("about to filter: {x:?}"))
        .filter(|(_, (a, b))| a == b)
        .map(|(n, _)| n as u32)
        .sum()
}

pub fn solve() -> u32 {
    let input = read_to_string("./inputs/5.txt").expect("file should have been there");
    rotate_dice(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(rotate_dice("LRDLU"), 5)
    }
}

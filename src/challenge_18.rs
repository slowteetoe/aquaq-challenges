use std::fs;

use chrono::{DateTime, NaiveDateTime, NaiveTime};

pub fn solve() -> u32 {
    let input = fs::read_to_string("inputs/18.txt").expect("input should have existed");
    input.lines().map(|line| nearest(line)).sum()
}

fn nearest(time: &str) -> u32 {
    let tmp = time.replace(":", "");
    let (lhs, rhs) = tmp.split_at(time.len() / 2 - 1);
    dbg!(lhs, rhs);
    // there's some bad ranges where the first 2 or 3 digits can't be mirrored, e.g.
    // 160 will turn into 61 seconds, which is invalid
    // a 6 or higher in position 3 will result in invalid minutes
    // 075 -> 06 55 60
    // 090 -> 05 00 50
    // 090 -> 10 00 01
    let val = lhs.parse::<u32>().unwrap();
    dbg!(&val);
    let fix = match val {
        60..=69 | 70..73 => 55,
        73..=79 | 80..=99 => 15,
        160..169 => 155,
        _ => val,
    };
    let first_two = match &lhs[0..2] {
        "06" | "07" | "08" | "09" => "05",
        "16" | "17" | "18" | "19" => "15",
        s => s,
    };
    let three = match &lhs[2..3] {
        "6" | "7" | "8" | "9" => "5",
        s => s,
    };
    dbg!(&first_two, &three);
    let lhs = first_two.to_owned() + three;

    let mut mirrored: Vec<_> = lhs.chars().collect();
    lhs.chars().rev().for_each(|c| {
        mirrored.push(c);
    });

    let result = mirrored
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (idx, c)| {
            if idx > 0 && idx % 2 == 0 {
                acc.push(':');
            }
            acc.push(*c);
            acc
        });
    dbg!(&result);
    let prev = NaiveTime::parse_from_str(&time, "%H:%M:%S").expect("should have parsed orig");
    let now =
        NaiveTime::parse_from_str(&result, "%H:%M:%S").expect("should have parsed palindrome");
    let diff = now - prev;
    diff.num_seconds().abs() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        // 13:44:31
        assert_eq!(211, nearest("13:41:00"))
    }

    #[test]
    fn test_harder_example() {
        // 15:55:51
        assert_eq!(6969, nearest("17:52:00"))
    }

    #[test]
    fn test_solve() {
        // 1337956 is wrong
        assert_eq!(0, solve())
    }
}

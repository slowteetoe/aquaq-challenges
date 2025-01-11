use std::fs;

use itertools::Itertools;

// Set the string's non-hexadecimal characters to 0.
// Pad the string length to the next multiple of 3.
// Split the result into 3 equal sections.
// The first two digits of each remaining section are the hex components.
fn apply_rules(input: &str) -> String {
    let v = input
        .chars()
        .map(|c| if c.is_ascii_hexdigit() { c } else { '0' })
        .collect_vec();
    dbg!(&v);
    let new_len = v.len() + 3 - (v.len() % 3);
    let segment = new_len / 3;
    let mut solution = vec![];
    for n in 0..=2 {
        solution.push(v[n * segment]);
        solution.push(v[n * segment + 1]);
    }
    solution.iter().join("")
}

pub fn solve() -> String {
    let input = fs::read_to_string("inputs/1.txt").expect("should have read input");
    apply_rules(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!("0d40fe", apply_rules("kdb4life"))
    }

    #[test]
    fn test_challenge() {
        assert_eq!(
            "",
            apply_rules(&fs::read_to_string("inputs/1.txt").expect("should have read input"))
        )
    }
}

use std::fs;

use gcd::Gcd;

fn is_coprime(a: u64, b: u64) -> bool {
    a.gcd(b) == 1
}

fn find_sum_coprimes(val: u64) -> u64 {
    let coprimes = (1..val).fold(vec![], |mut acc, n| {
        if is_coprime(val, n) {
            acc.push(n);
        }
        acc
    });
    coprimes.iter().sum()
}

pub fn solve() -> u64 {
    let input = fs::read_to_string("inputs/4.txt").expect("file should exist");

    find_sum_coprimes(input.parse().expect("should have been valid num in input"))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(60, find_sum_coprimes(15))
    }
}

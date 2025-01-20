use std::{collections::vec_deque, fs};

fn balanced(input: &str) -> bool {
    let mut q = vec_deque::VecDeque::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' => q.push_front(c),
            ')' => {
                if let Some(next) = q.pop_front() {
                    if next == '(' {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            ']' => {
                if let Some(next) = q.pop_front() {
                    if next == '[' {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            '}' => {
                if let Some(next) = q.pop_front() {
                    if next == '{' {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    if q.is_empty() {
        true
    } else {
        false
    }
}

pub fn solve() -> u32 {
    let input = fs::read_to_string("inputs/32.txt").expect("data");
    input
        .lines()
        .map(|line| balanced(line))
        .filter(|b| *b == true)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_examples() {
        let input = r"()
([]{})
(a[b[]]c){}";
        assert_eq!(
            3,
            input
                .lines()
                .map(|line| balanced(line))
                .filter(|b| *b == true)
                .count()
        )
    }

    #[test]
    fn test_unbalanced_examples() {
        let input = r")()
([a)]
]{}[
((a)){]";
        assert_eq!(
            0,
            input
                .lines()
                .map(|line| balanced(line))
                .filter(|b| *b == true)
                .count()
        )
    }
}

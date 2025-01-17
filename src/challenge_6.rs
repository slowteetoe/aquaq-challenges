fn ones_in_permutations(goal: u32) -> u32 {
    let mut perms = vec![];
    for a in 0..=goal {
        for b in 0..=goal {
            let c: i32 = goal as i32 - (a + b) as i32;
            if c >= 0 {
                perms.push((a, b, c as u32));
            }
        }
    }
    perms.iter().fold(0, |acc, (a, b, c)| {
        let val = format!("{a}{b}{c}");
        // dbg!(&val);
        acc + val.chars().filter(|c| *c == '1').count() as u32
    })
}

pub fn solve() -> u32 {
    // 3 numbers which sum to 123
    ones_in_permutations(123)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(9, ones_in_permutations(3))
    }
}

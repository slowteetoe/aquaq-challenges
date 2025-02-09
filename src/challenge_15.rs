use std::fs;

fn shortest_distance(dict: &str, start: &str, goal: &str) -> Vec<String> {
    // change exactly on char at a time to get to the goal
    vec![]
}

pub fn solve() -> u32 {
    let dict = fs::read_to_string("inputs/dict.txt").expect("dictionary");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let dict = fs::read_to_string("inputs/dict.txt").expect("dictionary");
        let actual = shortest_distance(&dict, "fly", "try");
        assert_eq!(vec!["fly", "fry", "try"], actual)
    }
}

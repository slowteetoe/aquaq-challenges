use std::{collections::HashMap, fs::read_to_string};

use itertools::{Itertools, MinMaxResult};

fn parse_input(input: &str) -> HashMap<&str, f32> {
    input.lines().skip(1).fold(HashMap::new(), |mut acc, line| {
        let v: Vec<&str> = line.splitn(3, ",").collect();
        let [a, b, score] = v[..] else { unreachable!() };
        let (a_score, b_score) = score.split_once("-").expect("should have parsed score");
        let ra = *acc.entry(a).or_insert(1200.0);
        let rb = *acc.entry(b).or_insert(1200.0);

        // Ea = 1 / (1 + 10^((Rb-Ra)/400))
        let expected_win_rate_a = expected_win_rate(ra, rb);
        let expected_win_rate_b = 1.0 - expected_win_rate_a;
        let points_up_for_grab = ri_prime(expected_win_rate_a);

        dbg!(&a, &ra, &b, &rb, &expected_win_rate_a, &points_up_for_grab);
        if a_score.parse::<u8>().expect("a") > b_score.parse::<u8>().expect("b") {
            // a won
            let ri_prime = 20.0 * (1.0 - expected_win_rate_a);
            println!("{} wins {} points", &a, &ri_prime);
            println!("{} loses {} points", &b, &ri_prime);
            acc.entry(a).and_modify(|v| *v += ri_prime);
            acc.entry(b).and_modify(|v| *v -= ri_prime);
        } else {
            // b won
            let ri_prime = 20.0 * (1.0 - expected_win_rate_b);
            println!("{} loses {} points in an upset", &a, &ri_prime);
            println!("{} wins {} points in an upset", &b, &ri_prime);
            acc.entry(a).and_modify(|v| *v -= ri_prime);
            acc.entry(b).and_modify(|v| *v += ri_prime);
        };
        acc
    })
}

fn expected_win_rate(ranking_a: f32, ranking_b: f32) -> f32 {
    1.0 / (1.0 + 10_f32.powf((ranking_b - ranking_a) / 400.0))
}

fn ri_prime(ei: f32) -> f32 {
    20.0 * (1.0 - ei)
}

pub fn solve() -> String {
    let input = read_to_string("inputs/7.txt").expect("should have read data");
    let m = parse_input(&input);
    dbg!(&m);
    if let MinMaxResult::MinMax(min, max) = m.iter().map(|(_, v)| *v).minmax() {
        format!("{}-{}", max as u32, min as u32)
    } else {
        panic!("something went wrong")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_example() {
        let ei = expected_win_rate(1400.0, 1200.0);
        assert_approx_eq!(0.75, ei, 0.01);
        let points = ri_prime(ei);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve(), "dunno")
    }
}

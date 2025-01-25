use std::fs;

fn parse_input(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            let n: Vec<_> = line.splitn(4, ",").collect();
            (
                (n[0].parse().unwrap(), n[1].parse().unwrap()),
                (n[2].parse().unwrap(), n[3].parse().unwrap()),
            )
        })
        .collect()
}

type Grid = [[u8; 25]; 25];

pub fn solve() -> u32 {
    let input = fs::read_to_string("./inputs/11.txt").expect("input file");
    let mut grid: Grid = [[0u8; 25]; 25];
    let data = parse_input(&input);
    data.iter().for_each(|(a, b)| {
        for x in a.0..b.0 {
            for y in a.1..b.1 {
                grid[y][x] += 1;
            }
        }
    });

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = r"lx,ly,ux,uy
0,0,3,3
2,2,4,5
6,3,8,7";
        let result = parse_input(input);
        assert_eq!(
            result,
            vec![((0, 0), (3, 3)), ((2, 2), (4, 5)), ((6, 3), (8, 7))]
        )
    }

    #[test]
    fn test_solution() {
        solve();
    }
}

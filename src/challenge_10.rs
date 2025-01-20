use std::fs::read_to_string;

use pathfinding::prelude::dijkstra;

pub fn cheapest<'a>(nodes: &'a Vec<(&str, (&str, u32))>) -> u32 {
    let result = dijkstra(
        &"TUPAC",
        |&x| nodes.iter().filter(move |&p| p.0 == x).map(|p| p.1),
        |&p| p == "DIDDY",
    );
    if let Some(solution) = result {
        solution.1
    } else {
        0
    }
}

pub fn solve() -> u32 {
    let input = read_to_string("inputs/10.txt").expect("data");
    let nodes: Vec<_> = input
        .lines()
        .skip(1) // skip header
        .map(|line| {
            // source, dest, amount
            let data: Vec<_> = line.splitn(3, ",").collect();
            (
                data[0],
                (data[1], data[2].parse::<u32>().expect("valid amount")),
            )
        })
        .collect();
    cheapest(&nodes)
}

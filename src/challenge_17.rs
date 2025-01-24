use std::{collections::HashMap, fs, hash::Hash};

use chrono::NaiveDate;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct ScoringDrought {
    start: NaiveDate,
    end: Option<NaiveDate>,
}

impl ScoringDrought {
    fn new(start_date: &NaiveDate) -> Self {
        Self {
            start: start_date.clone(),
            end: None,
        }
    }
}

pub fn solve() -> String {
    let input = fs::read_to_string("inputs/17.txt").expect("data");
    let mut map: HashMap<&str, (Option<ScoringDrought>, Option<ScoringDrought>)> = HashMap::new();
    input.lines().skip(1).for_each(|line| {
        // date,home_team,away_team,home_score,away_score,tournament,city,country,neutral
        let data: Vec<_> = line.splitn(9, ",").collect();
        let date = NaiveDate::parse_from_str(data[0], "%Y-%m-%d").expect("parseable date");
        let home = data[1];
        let away = data[2];
        let home_score = data[3];
        let away_score = data[4];
        // keep track of ranges
        if home_score == "0" {
            // start a new drought if needed, otherwise don't do anything as the drought continues
            map.entry(home)
                // .and_modify(|_| ())
                .or_insert((None, Some(ScoringDrought::new(&date))));
            // println!("added new record for {home} (or continued drought)");
        } else {
            map.entry(home).and_modify(|d| {
                if let Some(mut drought) = d.1 {
                    // println!("ending current drought for {home}, max was {:?}", d.0);
                    drought.end = Some(date);
                    if let Some(max) = d.0 {
                        // println!(
                        //     "There was an existing max, comparing {:?} to {:?}",
                        //     max, drought
                        // );
                        if max.end.unwrap() - max.start < drought.end.unwrap() - drought.start {
                            d.0 = Some(drought);
                        }
                    } else {
                        // println!("first drought, added as max");
                        d.0 = Some(drought);
                    }
                    d.1 = None;
                } else {
                    // println!("nothing to do, no drought and score was non-zero for {home}");
                }
            });
        }
        if away_score == "0" {
            // start a new drought if needed, otherwise don't do anything as the drought continues
            map.entry(away)
                .and_modify(|_| ())
                .or_insert((None, Some(ScoringDrought::new(&date))));
            // println!("added new record for {away} (or continued drought)");
        } else {
            map.entry(away).and_modify(|d| {
                if let Some(mut drought) = d.1 {
                    // println!("ending current drought for {away}, max was {:?}", d.0);
                    drought.end = Some(date);
                    if let Some(max) = d.0 {
                        // println!(
                        //     "There was an existing max, comparing {:?} to {:?}",
                        //     max, drought
                        // );
                        if max.end.unwrap() - max.start < drought.end.unwrap() - drought.start {
                            d.0 = Some(drought);
                        }
                    } else {
                        // println!("first drought, added as max");
                        d.0 = Some(drought);
                    }
                    d.1 = None;
                } else {
                    // println!("nothing to do, no drought and score was non-zero for {away}");
                }
            });
        }
    });
    // dbg!(&map);
    let days: Vec<_> = map
        .iter()
        .map(|(name, (max, _))| {
            if let Some(max) = max {
                let diff = max.end.unwrap().signed_duration_since(max.start);
                let days = diff.num_days();
                (name, days)
            } else {
                (name, -1)
            }
        })
        .collect();
    // dbg!(&days);
    let biggest_loser = days.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let data = map.get(biggest_loser.0).unwrap();
    //Somaliland 19000103 19020101
    let dates = data.0.unwrap();
    format!(
        "{} {} {}",
        biggest_loser.0,
        dates.start.format("%Y%m%d"),
        dates.end.unwrap().format("%Y%m%d")
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!("blah".to_owned(), solve())
    }

    #[test]
    fn test_parsing_date() {
        let result =
            NaiveDate::parse_from_str("1872-11-30", "%Y-%m-%d").expect("should have parsed");
        assert_eq!(NaiveDate::from_ymd_opt(1872, 11, 30).unwrap(), result)
    }
}

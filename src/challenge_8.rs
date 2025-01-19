use std::fs::read_to_string;

struct Pantry {
    milk: [u32; 5],
    cereal: u32,
}

impl Pantry {
    fn milk(&self) -> u32 {
        self.milk.iter().sum()
    }
    fn cereal(&self) -> u32 {
        self.cereal
    }

    fn new() -> Self {
        Self {
            milk: [0; 5],
            cereal: 0,
        }
    }
    fn tick(&mut self, milk: u32, cereal: u32) {
        if milk % 100 != 0 || cereal % 100 != 0 {
            panic!("was being lazy since boundaries align");
        }
        if self.milk() >= 100 && self.cereal() >= 100 {
            // use 100 of each, consuming oldest milk first
            let mut milk_to_consume = 100;
            let mut oldest = 0;
            while milk_to_consume != 0 && oldest < 5 {
                if self.milk[oldest] >= 100 {
                    self.milk[oldest] -= 100;
                    milk_to_consume = 0;
                } else {
                    oldest += 1;
                }
            }
            self.cereal -= 100;
        }
        // age any remaining milk
        self.milk[0] = self.milk[1];
        self.milk[1] = self.milk[2];
        self.milk[2] = self.milk[3];
        self.milk[3] = self.milk[4];
        self.milk[4] = milk;

        self.cereal += cereal;
    }
}

pub fn solve() -> u32 {
    let mut p = Pantry::new();
    let input = read_to_string("inputs/8.txt").expect("data");
    //date,milk,cereal
    //2016-01-26,1000,1000
    input.lines().skip(1).for_each(|line| {
        let day: Vec<&str> = line.splitn(3, ",").collect();
        p.tick(
            day[1].parse::<u32>().expect("milk"),
            day[2].parse::<u32>().expect("cereal"),
        );
    });
    p.milk() + p.cereal()
}

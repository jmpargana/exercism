use std::cmp::Ordering;
use std::collections::HashMap;

struct Results {
    MP: u8,
    W: u8,
    D: u8,
    L: u8,
    P: u8,
}

impl Results {
    fn new() -> Self {
        Self {
            MP: 0,
            W: 0,
            D: 0,
            L: 0,
            P: 0,
        }
    }

    fn add_victory(&mut self) {
        self.MP += 1;
        self.W += 1;
        self.P += 3;
    }

    fn add_defeat(&mut self) {
        self.MP += 1;
        self.L += 1;
    }

    fn add_tie(&mut self) {
        self.MP += 1;
        self.D += 1;
        self.P += 1;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table = HashMap::new();

    for game in match_results.split('\n') {
        let words: Vec<&str> = game.split(';').collect();

        match words[2] {
            "win" => {
                table
                    .entry(words[0].to_string())
                    .or_insert(Results::new())
                    .add_victory();
                table
                    .entry(words[1].to_string())
                    .or_insert(Results::new())
                    .add_defeat();
            }
            "loss" => {
                table
                    .entry(words[1].to_string())
                    .or_insert(Results::new())
                    .add_victory();
                table
                    .entry(words[0].to_string())
                    .or_insert(Results::new())
                    .add_defeat();
            }
            _ => {
                table
                    .entry(words[0].to_string())
                    .or_insert(Results::new())
                    .add_tie();
                table
                    .entry(words[1].to_string())
                    .or_insert(Results::new())
                    .add_tie();
            }
        }
    }

    stringify(&table)
}

fn stringify(table: &HashMap<String, Results>) -> String {
    let mut vec: Vec<_> = table.into_iter().collect();

    vec.sort_by(|&a, &b| {
        let ord = a.1.P.partial_cmp(&b.1.P).unwrap();
        if ord == Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            ord.reverse()
        }
    });

    vec.into_iter().fold(
        String::from(format!(
            "{:<31}| {:>2} | {:>2} | {:>2} | {:>2} |{:>3}",
            "Team", "MP", "W", "D", "L", "P"
        )),
        |mut acc, r| {
            acc = format!(
                "{}\n{:<31}| {:>2} | {:>2} | {:>2} | {:>2} |{:>3}",
                acc, r.0, r.1.MP, r.1.W, r.1.D, r.1.L, r.1.P
            );
            acc
        },
    )
}

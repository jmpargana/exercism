use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), |mut acc, (score, chars)| {
        chars
            .iter()
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<char>>()
            .iter()
            .for_each(|c| {
                let c = *c;
                let score = *score;
                acc.insert(c, score);
            });
        
        acc
    })
}

use std::char;
use std::convert::TryInto;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| (0..row.len()).map(|j| trans(i, j, minefield)).collect())
        .collect()
}

fn trans(i: usize, j: usize, minefield: &[&str]) -> char {
    match minefield[i].chars().nth(j) {
        Some('*') => '*',
        _ => {
            let count = (i.saturating_sub(1)..=i + 1)
                .flat_map(|k| (j.saturating_sub(1)..=j + 1).map(move |l| (k, l)))
                .filter(|&(k, l)| (k, l) != (i, j))
                .filter_map(|(k, l)| minefield.get(k).and_then(|r| r.chars().nth(l)))
                .filter(|&c| c == '*')
                .count();

            match count {
                0 => ' ',
                x @ 1..=9 => char::from_digit(x.try_into().unwrap(), 10).unwrap(),
                _ => unreachable!(),
            }
        }
    }
}

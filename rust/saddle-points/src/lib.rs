pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input.iter().enumerate().fold(Vec::new(), |mut acc, (i, row)| {
        row.iter().enumerate().for_each(|(j, &elem)| {
            if is_saddle(i, j, &input, elem) {
                acc.push((i, j))
            }
        });
        acc
    })
}

fn is_saddle(row: usize, col: usize, matrix: &[Vec<u64>], elem: u64) -> bool {
    elem == *matrix[row].iter().max().unwrap()
        && elem == matrix.iter().map(|r| r[col]).min().unwrap()
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts = books.iter().fold(vec![0; 5], |mut acc, &book| {
        acc[book as usize - 1] += 1;
        acc
    });

    counts.sort();

    let (sets, _) = (0..counts.len()).fold((vec![0; 5], 0), |(mut sets, mut acc), i| {
        sets[i] = counts[i] - acc;
        acc += sets[i];
        (sets, acc)
    });

    let numthreeintofours = if sets[0] < sets[2] { sets[0] } else { sets[2] };

    800 * sets[4]
        + 1520 * sets[3]
        + 2160 * (sets[2] - numthreeintofours)
        + 2560 * (sets[1] + numthreeintofours * 2)
        + 3000 * (sets[0] - numthreeintofours)
}

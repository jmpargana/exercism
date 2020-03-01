use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let checker = ['A', 'C', 'G', 'T'];
    let invalid: Vec<char> = dna.chars().filter(|c| !checker.contains(c)).collect();

    if !invalid.is_empty() { return Err(invalid[0]); };
    if !checker.contains(&nucleotide) { return Err(nucleotide); }

    Ok(dna.matches(nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::new();
    for elem in "ACGT".chars() {
        result.insert(elem, count(elem, dna)?);
    }
    Ok(result)
}

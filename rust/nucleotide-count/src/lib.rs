use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .get(&nucleotide)
        .copied()
        .ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotide_counts = VALID_NUCLEOTIDES
        .iter()
        .map(|n| (*n, 0))
        .collect::<HashMap<char, usize>>();

    for nucleotide in dna.chars() {
        match nucleotide_counts.entry(nucleotide) {
            Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            Entry::Vacant(_) => return Err(nucleotide),
        }
    }

    Ok(nucleotide_counts)
}

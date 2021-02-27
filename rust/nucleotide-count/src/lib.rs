use std::collections::hash_map::Entry;
use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }
    // TODO: Combine counting and early error return to combined iteration.
    if let Some(invalid) = dna.chars().find(|n| !VALID_NUCLEOTIDES.contains(n)) {
        return Err(invalid);
    }

    let count = dna.chars().filter(|n| *n == nucleotide).count();
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotide_counts = VALID_NUCLEOTIDES
        .iter()
        .map(|n| (*n, 0 as usize))
        .collect::<HashMap<char, usize>>();

    for nucleotide in dna.chars() {
        match nucleotide_counts.entry(nucleotide) {
            Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            Entry::Vacant(_) => return Err(nucleotide),
        }
    }

    Ok(nucleotide_counts)
}

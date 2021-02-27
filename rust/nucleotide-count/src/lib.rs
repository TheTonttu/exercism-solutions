use std::collections::{HashMap, HashSet};

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
    // TODO: Not very efficient as we do multiple iterations on dna + use count function to do another iteration

    let mut uniques = HashSet::new();

    let mut nucleotide_counts = HashMap::with_capacity(VALID_NUCLEOTIDES.len());
    dna.chars()
        .collect::<Vec<char>>()
        .retain(|n| uniques.insert(*n));

    for nucleotide in uniques {
        match count(nucleotide, dna) {
            Ok(count) => {
                nucleotide_counts.insert(nucleotide, count);
            }
            Err(invalid) => {
                return Err(invalid);
            }
        }
    }

    for nucleotide in &VALID_NUCLEOTIDES {
        if !nucleotide_counts.contains_key(nucleotide) {
            nucleotide_counts.insert(*nucleotide, 0);
        }
    }

    Ok(nucleotide_counts)
}

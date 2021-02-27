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
    unimplemented!(
        "How much of every nucleotide type is contained inside DNA string '{}'?",
        dna
    );
}

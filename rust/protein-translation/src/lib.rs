use std::collections::HashMap;
use std::str;

pub struct CodonsInfo<'a> {
    codon_name_map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn new(codon_name_map: HashMap<&'a str, &'a str>) -> Self {
        Self { codon_name_map }
    }

    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codon_name_map.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        const CODON_LENGTH: usize = 3;
        const STOP_MARK: &str = "stop codon";

        rna.as_bytes()
            .chunks(CODON_LENGTH)
            .filter_map(|chunk| str::from_utf8(chunk).ok())
            .map(|sequence| self.name_for(sequence))
            .take_while(|&maybe_codon| maybe_codon != Some(STOP_MARK))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo::new(pairs.into_iter().collect())
}

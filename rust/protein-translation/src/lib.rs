use std::collections::HashMap;

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
        const STOP_MARK: &str = "stop codon";

        let codons: Vec<Option<&str>> = rna
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|c| c.iter().collect::<String>())
            .map(|c| self.name_for(&c))
            .take_while(|c| c.is_none() || c.and_then(|c| Some(c != STOP_MARK)) == Some(true))
            .collect();

        (!codons.is_empty() && codons.iter().all(|c| c.is_some()))
            .then(|| codons.into_iter().flatten().collect())
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo::new(pairs.into_iter().collect())
}

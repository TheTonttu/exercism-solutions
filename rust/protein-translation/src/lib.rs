use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct CodonsInfo<'a> {
    codons_hash_map: HashMap<&'a str, u64>,
    hash_name_map: HashMap<u64, &'a str>,
}

impl<'a> CodonsInfo<'a> {

    pub fn new(codons: HashMap<&'a str, u64>, names: HashMap<u64, &'a str>) -> Self {
        Self {
            codons_hash_map: codons,
            hash_name_map: names
        }
    }

    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons_hash_map
            .get(codon)
            .and_then(|h| self.hash_name_map.get(h))
            .cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        unimplemented!("Return a list of protein names that correspond to the '{}' RNA string or None if the RNA string is invalid", rna);
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {

    let mut hasher = DefaultHasher::new();

    let mut codons = HashMap::new();
    let mut names = HashMap::new();
    for (codon, name) in pairs {
        name.hash(&mut hasher);
        let name_hash = hasher.finish();
        names.entry(name_hash).or_insert( name);
        codons.entry(codon).or_insert( name_hash);
    }

    CodonsInfo::new(codons, names)
}

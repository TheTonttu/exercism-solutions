const VALID_DNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];
const VALID_RNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'U'];

#[derive(Debug, PartialEq)]
pub struct Dna {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    strand: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut errors = dna
            .chars()
            .enumerate()
            .skip_while(|(i, c)| VALID_DNA_NUCLEOTIDES.contains(c));

        if let Some((index, _invalid)) = errors.next() {
            return Err(index);
        }

        Ok(Self {
            strand: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        unimplemented!("Transform Dna {:?} into corresponding Rna", self);
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut errors = rna
            .chars()
            .enumerate()
            .skip_while(|(i, c)| VALID_RNA_NUCLEOTIDES.contains(c));

        if let Some((index, _invalid)) = errors.next() {
            return Err(index);
        }

        Ok(Self {
            strand: rna.to_string(),
        })
    }
}

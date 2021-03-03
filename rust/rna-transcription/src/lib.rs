const VALID_DNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];
const VALID_RNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'U'];

const DNA_RNA_TRANSCRIPT: [(char, char); 4] = [('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')];

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
        validate_strand(dna, VALID_DNA_NUCLEOTIDES.as_ref())?;

        Ok(Self {
            strand: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let transcribed_strand = self
            .strand
            .chars()
            .map(|dn| {
                DNA_RNA_TRANSCRIPT
                    .iter()
                    .find(|(tdn, _trn)| *tdn == dn)
                    .unwrap()
                    .1
            })
            .collect::<String>();

        Rna::new(transcribed_strand.as_str()).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate_strand(rna, VALID_RNA_NUCLEOTIDES.as_ref())?;

        Ok(Self {
            strand: rna.to_string(),
        })
    }
}

fn validate_strand(strand: &str, valid_nucleotides: &[char]) -> Result<(), usize> {
    let mut errors = strand
        .chars()
        .enumerate()
        .skip_while(|(_i, c)| valid_nucleotides.contains(c));

    match errors.next() {
        Some((index, _invalid)) => Err(index),
        None => Ok(()),
    }
}

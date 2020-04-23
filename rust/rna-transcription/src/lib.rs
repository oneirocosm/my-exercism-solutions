#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if let Some((index, _)) = dna
            .chars()
            .enumerate()
            .find(|(_, nuc)| !Self::valid_nucleotide(*nuc))
        {
            return Err(index);
        }

        Ok(DNA { strand: dna.into() })
    }

    pub fn into_rna(self) -> RNA {
        let rna_strand = self
            .strand
            .chars()
            .map(|nucleotide| match nucleotide {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!("impossible dna"),
            })
            .collect::<String>();

        RNA { strand: rna_strand }
    }

    fn valid_nucleotide(nucleotide: char) -> bool {
        const VALID: &str = "GCTA";
        VALID.contains(nucleotide)
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if let Some((index, _)) = rna
            .chars()
            .enumerate()
            .find(|(_, nuc)| !Self::valid_nucleotide(*nuc))
        {
            return Err(index);
        }

        Ok(RNA { strand: rna.into() })
    }

    fn valid_nucleotide(nucleotide: char) -> bool {
        const VALID: &str = "GCUA";
        VALID.contains(nucleotide)
    }
}

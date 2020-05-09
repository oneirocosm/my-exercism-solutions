use std::collections::HashMap;

#[derive(Default)]
pub struct CodonsInfo<'a> {
    codon_to_protein: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codon_to_protein.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let proteins = rna
            .as_bytes()
            .chunks(3)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .ok()?
            .into_iter()
            .map(|codon| self.name_for(codon))
            .collect::<Option<Vec<&str>>>()?
            .into_iter()
            .take_while(|&protein| protein != "stop codon")
            .collect();

        Some(proteins)
    }
}

pub fn parse<'b>(pairs: Vec<(&'b str, &'b str)>) -> CodonsInfo<'b> {
    let mut info = CodonsInfo::default();

    for (codon, protein) in pairs {
        info.codon_to_protein.insert(codon, protein);
    }
    info
}

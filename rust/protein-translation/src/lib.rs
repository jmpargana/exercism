use std::collections::HashMap;
use itertools::Itertools;

pub struct CodonsInfo<'a> {
    pub pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs.get(codon).map(|&c| c)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chars()
            .chunks(3)
            .into_iter()
            .map(|mut c| self.name_for(&c.join("")))
            .take_while(|&s| s != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: pairs.iter().cloned().collect()
    }
}

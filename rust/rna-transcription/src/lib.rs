#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleos: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleos: String,
}


impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match checker(dna, "ACGT") {
            None => Ok(DNA { nucleos: dna.to_string() }),
            Some(x) => Err(x)
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            nucleos: self.nucleos.chars().map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => panic!("Invalid string"),
                }).collect()
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match checker(rna, "ACGU") {
            None => Ok(RNA { nucleos: rna.to_string() }),
            Some(x) => Err(x)
        }
    }
}

fn checker(sequence: &str, allowed: &str) -> Option<usize> {
    for (i, c) in sequence.chars().enumerate() {
        if !allowed.contains(c) {
            return Some(i)
        }
    }
    None
}

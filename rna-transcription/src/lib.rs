#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: Vec<DnaNucleotide>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: Vec<RnaNucleotide>,
}

#[derive(Debug, PartialEq, Eq)]
enum DnaNucleotide {
    A,
    C,
    G,
    T,
}

#[derive(Debug, PartialEq, Eq)]
enum RnaNucleotide {
    A,
    C,
    G,
    U,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let dna_vec: Result<Vec<_>, _> = dna
            .char_indices()
            .map(|(i, c)| match c {
                'A' => Ok(DnaNucleotide::A),
                'C' => Ok(DnaNucleotide::C),
                'G' => Ok(DnaNucleotide::G),
                'T' => Ok(DnaNucleotide::T),
                _ => Err(i),
            })
            .collect();

        dna_vec.map(|dna| Dna { dna })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            rna: self
                .dna
                .into_iter()
                .map(|n| match n {
                    DnaNucleotide::A => RnaNucleotide::U,
                    DnaNucleotide::C => RnaNucleotide::G,
                    DnaNucleotide::G => RnaNucleotide::C,
                    DnaNucleotide::T => RnaNucleotide::A,
                })
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let rna_vec: Result<Vec<_>, _> = rna
            .char_indices()
            .map(|(i, c)| match c {
                'A' => Ok(RnaNucleotide::A),
                'C' => Ok(RnaNucleotide::C),
                'G' => Ok(RnaNucleotide::G),
                'U' => Ok(RnaNucleotide::U),
                _ => Err(i),
            })
            .collect();

        rna_vec.map(|rna| Rna { rna })
    }

    pub fn into_dna(self) -> Dna {
        Dna {
            dna: self
                .rna
                .into_iter()
                .map(|n| match n {
                    RnaNucleotide::A => DnaNucleotide::T,
                    RnaNucleotide::C => DnaNucleotide::G,
                    RnaNucleotide::G => DnaNucleotide::C,
                    RnaNucleotide::U => DnaNucleotide::A,
                })
                .collect(),
        }
    }
}

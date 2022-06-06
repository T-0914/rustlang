#[derive(Debug, PartialEq, Clone)]
pub struct Dna {
    seq: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    seq: String,
}

pub enum SeqType {
    DNA,
    RNA,
}

fn check_valid_seq(seq_type: SeqType, seq: &str) -> Result<bool, usize> {
    const DNA: [char; 4] = ['G', 'C', 'T', 'A'];
    const RNA: [char; 4] = ['G', 'C', 'U', 'A'];
    let data_check: [char; 4] = match seq_type {
        SeqType::DNA => DNA,
        SeqType::RNA => RNA,
    };

    for (index, nuc) in seq.chars().enumerate() {
        match data_check.contains(&nuc) {
            true => {
                continue;
            }
            false => return Err(index),
        }
    }
    return Ok(true);
}

impl<'a> Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        // unimplemented!("Construct new Dna from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", dna);
        let is_valid_seq: Result<bool, usize> = check_valid_seq(SeqType::DNA, dna);

        match is_valid_seq {
            #[allow(unused_variables)]
            Ok(is_valid) => Ok(Dna {
                seq: String::from(dna),
            }),
            Err(first_invalid_nuc) => Err(first_invalid_nuc),
        }
    }

    pub fn into_rna(&mut self) -> Rna {
        // unimplemented!("Transform Dna {:?} into corresponding Rna", self);
        let mut temp: String = "".to_owned();
        for nuc in self.seq.chars() {
            let _nuc = match nuc {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!("Invalid nuc"),
            };
            temp.push(_nuc)
        }
        Rna { seq: temp }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        // unimplemented!("Construct new Rna from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", rna);
        let is_valid_seq: Result<bool, usize> = check_valid_seq(SeqType::RNA, rna);

        match is_valid_seq {
            #[allow(unused_variables)]
            Ok(is_valid) => Ok(Rna {
                seq: String::from(rna),
            }),
            Err(first_invalid_nuc) => Err(first_invalid_nuc),
        }
    }
}

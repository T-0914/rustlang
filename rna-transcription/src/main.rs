use rna_transcription::Dna;

fn main() {
    let is_valid_dna = Dna::new("GCTA");

    match is_valid_dna {
        Ok(mut dna) => {
            println!("dna: {:#?}", dna);

            println!("dna.into_rna(): {:#?}", dna.into_rna());
        }
        Err(first_invalid_nuc) => println!("first_invalid_nuc: {:#?}", first_invalid_nuc),
    }
}

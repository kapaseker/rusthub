#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<char>);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<char>);

///  (A), cytosine (C), guanine (G) and thymine (T).
///    G -> C
///    C -> G
///    T -> A
///    A -> U
impl Dna {
    /// Construct new Dna from '{dna}' string. If string contains invalid nucleotides return index of first invalid nucleotide
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut dna_chars = vec![];
        for (i, c) in dna.chars().enumerate() {
            if matches!(c, 'A' | 'C' | 'G' | 'T') {
                dna_chars.push(c)
            } else {
                return Err(i);
            }
        }
        Ok(Dna(dna_chars))
    }

    /// Transform Dna {self:?} into corresponding Rna
    pub fn into_rna(self) -> Rna {
        let mut rna_chars = vec![];
        for c in self.0 {
            rna_chars.push(match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                _ => 'A',
            })
        }
        Rna(rna_chars)
    }
}

///  (A), cytosine (C), guanine (G) and uracil (U).
impl Rna {
    /// Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut rna_chars = vec![];
        for (i, c) in rna.chars().enumerate() {
            if matches!(c, 'A' | 'C' | 'G' | 'U') {
                rna_chars.push(c)
            } else {
                return Err(i);
            }
        }
        Ok(Rna(rna_chars))
    }
}

#[cfg(test)]
mod test {
    use crate::rna_transcription::*;

    #[test]

    fn empty_rna_sequence() {
        let input = "";


        let output = Dna::new(input).unwrap().into_rna();


        let expected = Rna::new("").unwrap();


        assert_eq!(output, expected);
    }


    #[test]

    fn rna_complement_of_cytosine_is_guanine() {
        let input = "C";


        let output = Dna::new(input).unwrap().into_rna();


        let expected = Rna::new("G").unwrap();


        assert_eq!(output, expected);
    }


    #[test]

    fn rna_complement_of_guanine_is_cytosine() {
        let input = "G";


        let output = Dna::new(input).unwrap().into_rna();


        let expected = Rna::new("C").unwrap();


        assert_eq!(output, expected);
    }


    #[test]

    fn rna_complement_of_thymine_is_adenine() {
        let input = "T";


        let output = Dna::new(input).unwrap().into_rna();


        let expected = Rna::new("A").unwrap();


        assert_eq!(output, expected);
    }


    #[test]

    fn rna_complement_of_adenine_is_uracil() {
        let input = "A";


        let output = Dna::new(input).unwrap().into_rna();


        let expected = Rna::new("U").unwrap();


        assert_eq!(output, expected);
    }


    #[test]

    fn rna_complement() {
        let input = "ACGTGGTCTTAA";


        let output = Dna::new(input).unwrap().into_rna();


        let expected = Rna::new("UGCACCAGAAUU").unwrap();


        assert_eq!(output, expected);
    }


    #[test]

    fn invalid_dna_input() {
        let input = "U";


        let output = Dna::new(input);


        let expected = Err(0);


        assert_eq!(output, expected);
    }


    #[test]

    fn invalid_dna_input_at_offset() {
        let input = "ACGTUXXCTTAA";


        let output = Dna::new(input);


        let expected = Err(4);


        assert_eq!(output, expected);
    }


    #[test]

    fn invalid_rna_input() {
        let input = "T";


        let output = Rna::new(input);


        let expected = Err(0);


        assert_eq!(output, expected);
    }


    #[test]

    fn invalid_rna_input_at_offset() {
        let input = "ACGTUXXCTTAA";


        let output = Rna::new(input);


        let expected = Err(3);


        assert_eq!(output, expected);
    }
}
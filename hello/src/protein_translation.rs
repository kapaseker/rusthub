use std::collections::HashMap;

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let protein_map = HashMap::from([
        ("AUG", "Methionine"),
        ("UUU", "Phenylalanine"),
        ("UUC", "Phenylalanine"),
        ("UUA", "Leucine"),
        ("UUG", "Leucine"),
        ("UCU", "Serine"),
        ("UCC", "Serine"),
        ("UCA", "Serine"),
        ("UCG", "Serine"),
        ("UAU", "Tyrosine"),
        ("UAC", "Tyrosine"),
        ("UGU", "Cysteine"),
        ("UGC", "Cysteine"),
        ("UGG", "Tryptophan"),
        ("UAA", ""),
        ("UAG", ""),
        ("UGA", ""),
    ]);
    let mut protein: Vec<&str> = vec![];
    if !rna.is_empty() {
        let chunked: Vec<String> = Vec::from_iter(rna.chars()).chunks(3).map(String::from_iter).collect();
        for x in chunked {
            if x.len() == 3 {
                let acid = protein_map.get(x.as_str());
                if acid.is_some() {
                    let acid = *acid.unwrap_or(&"");
                    if acid.is_empty() {
                        break;
                    } else {
                        protein.push(acid)
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }


    Some(protein)
}

mod test {
    use crate::protein_translation::translate;

    #[test]

    fn empty_rna_sequence_results_in_no_proteins() {
        assert_eq!(translate(""), Some(vec![]),);
    }


    #[test]

    fn methionine_rna_sequence() {
        assert_eq!(translate("AUG"), Some(vec!["Methionine"]),);
    }


    #[test]

    fn phenylalanine_rna_sequence_1() {
        assert_eq!(translate("UUU"), Some(vec!["Phenylalanine"]),);
    }


    #[test]

    fn phenylalanine_rna_sequence_2() {
        assert_eq!(translate("UUC"), Some(vec!["Phenylalanine"]),);
    }


    #[test]

    fn leucine_rna_sequence_1() {
        assert_eq!(translate("UUA"), Some(vec!["Leucine"]),);
    }


    #[test]

    fn leucine_rna_sequence_2() {
        assert_eq!(translate("UUG"), Some(vec!["Leucine"]),);
    }


    #[test]

    fn serine_rna_sequence_1() {
        assert_eq!(translate("UCU"), Some(vec!["Serine"]),);
    }


    #[test]

    fn serine_rna_sequence_2() {
        assert_eq!(translate("UCC"), Some(vec!["Serine"]),);
    }


    #[test]

    fn serine_rna_sequence_3() {
        assert_eq!(translate("UCA"), Some(vec!["Serine"]),);
    }


    #[test]

    fn serine_rna_sequence_4() {
        assert_eq!(translate("UCG"), Some(vec!["Serine"]),);
    }


    #[test]

    fn tyrosine_rna_sequence_1() {
        assert_eq!(translate("UAU"), Some(vec!["Tyrosine"]),);
    }


    #[test]

    fn tyrosine_rna_sequence_2() {
        assert_eq!(translate("UAC"), Some(vec!["Tyrosine"]),);
    }


    #[test]

    fn cysteine_rna_sequence_1() {
        assert_eq!(translate("UGU"), Some(vec!["Cysteine"]),);
    }


    #[test]

    fn cysteine_rna_sequence_2() {
        assert_eq!(translate("UGC"), Some(vec!["Cysteine"]),);
    }


    #[test]

    fn tryptophan_rna_sequence() {
        assert_eq!(translate("UGG"), Some(vec!["Tryptophan"]),);
    }


    #[test]

    fn stop_codon_rna_sequence_1() {
        assert_eq!(translate("UAA"), Some(vec![]),);
    }


    #[test]

    fn stop_codon_rna_sequence_2() {
        assert_eq!(translate("UAG"), Some(vec![]),);
    }


    #[test]

    fn stop_codon_rna_sequence_3() {
        assert_eq!(translate("UGA"), Some(vec![]),);
    }


    #[test]

    fn sequence_of_two_protein_codons_translates_into_proteins() {
        assert_eq!(

        

          

        translate("UUUUUU"),

        

          

        Some(vec!["Phenylalanine", "Phenylalanine"]),

        

          

    );
    }


    #[test]

    fn sequence_of_two_different_protein_codons_translates_into_proteins() {
        assert_eq!(translate("UUAUUG"), Some(vec!["Leucine", "Leucine"]),);
    }


    #[test]

    fn translate_rna_strand_into_correct_protein_list() {
        assert_eq!(

        

          

        translate("AUGUUUUGG"),

        

          

        Some(vec!["Methionine", "Phenylalanine", "Tryptophan"]),

        

          

    );
    }


    #[test]

    fn translation_stops_if_stop_codon_at_beginning_of_sequence() {
        assert_eq!(translate("UAGUGG"), Some(vec![]),);
    }


    #[test]

    fn translation_stops_if_stop_codon_at_end_of_two_codon_sequence() {
        assert_eq!(translate("UGGUAG"), Some(vec!["Tryptophan"]),);
    }


    #[test]

    fn translation_stops_if_stop_codon_at_end_of_three_codon_sequence() {
        assert_eq!(

        

          

        translate("AUGUUUUAA"),

        

          

        Some(vec!["Methionine", "Phenylalanine"]),

        

          

    );
    }


    #[test]

    fn translation_stops_if_stop_codon_in_middle_of_three_codon_sequence() {
        assert_eq!(translate("UGGUAGUGG"), Some(vec!["Tryptophan"]),);
    }


    #[test]

    fn translation_stops_if_stop_codon_in_middle_of_six_codon_sequence() {
        assert_eq!(

        

          

        translate("UGGUGUUAUUAAUGGUUU"),

        

          

        Some(vec!["Tryptophan", "Cysteine", "Tyrosine"]),

        

          

    );
    }


    #[test]

    fn sequence_of_two_non_stop_codons_does_not_translate_to_a_stop_codon() {
        assert_eq!(translate("AUGAUG"), Some(vec!["Methionine", "Methionine"]),);
    }


    #[test]

    fn unknown_amino_acids_not_part_of_a_codon_can_t_translate() {
        assert_eq!(translate("XYZ"), None,);
    }


    #[test]

    fn incomplete_rna_sequence_can_t_translate() {
        assert_eq!(translate("AUGU"), None,);
    }


    #[test]

    fn incomplete_rna_sequence_can_translate_if_valid_until_a_stop_codon() {
        assert_eq!(

        

          

        translate("UUCUUCUAAUGGU"),

        

          

        Some(vec!["Phenylalanine", "Phenylalanine"]),

        

          

    );
    }
}
//! https://www.codewars.com/kata/5556282156230d0e5e000089/train/rust

pub fn dna_to_rna(dna: &str) -> String {
    dna.chars().map(|c| if c == 'T' { 'U' } else { c }).collect()
}

#[cfg(test)]
mod tests {
    use super::dna_to_rna;

    #[test]
    fn returns_expected() {
      assert_eq!(dna_to_rna("TTTT"), "UUUU");
      assert_eq!(dna_to_rna("GCAT"), "GCAU");
    }
}

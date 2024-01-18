//! https://www.codewars.com/kata/59c5f4e9d751df43cf000035/train/rust

pub fn longest_vowel_chain(s: &str) -> usize {
    s.to_lowercase().chars().fold((0usize, 0usize), |(acc_max, acc_curr), c| {
        let new_acc_max;
        let new_acc_curr;
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                new_acc_curr = acc_curr + 1;
                new_acc_max = acc_max.max(new_acc_curr);
            },
            _ => {
                new_acc_curr = 0;
                new_acc_max = acc_max;
            },
        };
        (new_acc_max, new_acc_curr)
    }).0
}

#[cfg(test)]
mod tests {
    use super::longest_vowel_chain;
    
    #[test]
    fn basic_tests() {
        assert_eq!(longest_vowel_chain("codewarriors"), 2);
        assert_eq!(longest_vowel_chain("suoidea"), 3);
        assert_eq!(longest_vowel_chain("ultrarevolutionariees"), 3);
        assert_eq!(longest_vowel_chain("strengthlessnesses"), 1);
        assert_eq!(longest_vowel_chain("cuboideonavicuare"), 2);
        assert_eq!(longest_vowel_chain("chrononhotonthuooaos"), 5);
        assert_eq!(longest_vowel_chain("iiihoovaeaaaoougjyaw"), 8);
    }
}

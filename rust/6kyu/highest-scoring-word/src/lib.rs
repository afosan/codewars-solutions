//! https://www.codewars.com/kata/57eb8fcdf670e99d9b000272/train/rust

pub fn high(input: &str) -> &str {
    let words = input.split_whitespace().collect::<Vec<_>>();
    let scores = words.iter().map(|w| w.chars().map(|c| c as u32 - 'a' as u32 + 1).sum::<u32>()).collect::<Vec<u32>>();
    let argmax = scores.iter().enumerate().find(|(_, s)| *s == scores.iter().max().unwrap()).unwrap().0;
    words[argmax]
}

#[cfg(test)]
mod tests {
    use super::high;

    #[test]
    fn test_basic() {
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");               
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");                      
        assert_eq!(high("massage yes massage yes massage"), "massage");         
        assert_eq!(high("take two bintang and a dance please"), "bintang"); 
        assert_eq!(high("aa b"), "aa");         
        assert_eq!(high("b aa"), "b");     
        assert_eq!(high("bb d"), "bb");                            
        assert_eq!(high("d bb"), "d"); 
        assert_eq!(high("aaa b"), "aaa");                                     
    }
}

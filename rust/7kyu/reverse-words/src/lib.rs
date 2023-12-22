//! https://www.codewars.com/kata/5259b20d6021e9e14c0010d4/train/rust

pub fn reverse_words(s: &str) -> String {
    let rev_words = s.split_whitespace().map(|w| w.chars().rev().collect::<String>());
    let mut space_counts = vec![];
    let mut curr = 0;

    for c in s.chars() {
        if c == ' ' {
            curr += 1;
        } else {
            if curr != 0 {
                space_counts.push(curr);
                curr = 0;
            }
        }
    }
    space_counts.push(0);

    let spaces = space_counts.iter().map(|n| " ".repeat(*n));

    rev_words.zip(spaces).map(|(w, s)| format!("{w}{s}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
        assert_eq!(reverse_words("apple"), "elppa");
        assert_eq!(reverse_words("a b c d"),"a b c d");
        assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
    }
}

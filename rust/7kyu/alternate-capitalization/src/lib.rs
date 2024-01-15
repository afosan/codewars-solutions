//! https://www.codewars.com/kata/59cfc000aeb2844d16000075/train/rust

pub fn capitalize(s: &str) -> Vec<String> {
    let (u, l) = s.chars().enumerate().fold((String::new(), String::new()), |(mut acc_u, mut acc_l), (i, c)| {
        if i % 2 == 0 {
            acc_u.push(c.to_ascii_uppercase());
            acc_l.push(c.to_ascii_lowercase());
        } else {
            acc_l.push(c.to_ascii_uppercase());
            acc_u.push(c.to_ascii_lowercase());
        }

        (acc_u, acc_l)
    });

    vec![u, l]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(capitalize("abcdef"),["AbCdEf", "aBcDeF"]);
        assert_eq!(capitalize("codewars"),["CoDeWaRs", "cOdEwArS"]);
        assert_eq!(capitalize("abracadabra"),["AbRaCaDaBrA", "aBrAcAdAbRa"]);
        assert_eq!(capitalize("codewarriors"),["CoDeWaRrIoRs", "cOdEwArRiOrS"]);
    }
}

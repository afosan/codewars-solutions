//! https://www.codewars.com/kata/5848565e273af816fb000449/train/rust

fn switch_first_and_last(s: &str) -> String {
    let l = s.len();
    if l <= 1 {
        s.to_string()
    } else {
        format!("{}{}{}", &s[l-1..], &s[1..l-1], &s[..1])
    }
}

pub fn encrypt_this(text: &str) -> String {
    text.split_whitespace().map(|w| format!("{}{}", w.chars().nth(0).unwrap() as u32, switch_first_and_last(&w[1..]))).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(encrypt_this(&"A"), "65".to_string());
        assert_eq!(encrypt_this(&"A wise old owl lived in an oak"), "65 119esi 111dl 111lw 108dvei 105n 97n 111ka".to_string());
        assert_eq!(encrypt_this(&"The more he saw the less he spoke"), "84eh 109ero 104e 115wa 116eh 108sse 104e 115eokp".to_string());
        assert_eq!(encrypt_this(&"The less he spoke the more he heard"), "84eh 108sse 104e 115eokp 116eh 109ero 104e 104dare".to_string());
        assert_eq!(encrypt_this(&"Why can we not all be like that wise old bird"), "87yh 99na 119e 110to 97ll 98e 108eki 116tah 119esi 111dl 98dri".to_string());
        assert_eq!(encrypt_this(&"Thank you Piotr for all your help"), "84kanh 121uo 80roti 102ro 97ll 121ruo 104ple".to_string());
    }
}

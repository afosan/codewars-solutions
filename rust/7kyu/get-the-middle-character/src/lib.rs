//! https://www.codewars.com/kata/56747fd5cb988479af000028/train/rust

pub fn get_middle(s: &str) -> &str {
    let l = s.len();

    if l % 2 == 0 {
        &s[l/2-1..l/2+1]
    } else {
        &s[l/2..l/2+1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(get_middle("test"),"es");
        assert_eq!(get_middle("testing"),"t");
        assert_eq!(get_middle("middle"),"dd");
        assert_eq!(get_middle("A"),"A");
        assert_eq!(get_middle("of"),"of");
    }
}

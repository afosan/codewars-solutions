//! https://www.codewars.com/kata/568dc69683322417eb00002c/train/rust

pub fn triple_x(s: &str) -> bool {
    if let Some(l) = s.find("x") {
        if l + 3 > s.len() {
            false 
        } else {
            &s[l+1..l+3] == "xx"
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(triple_x("abraxxxas"), true);
        assert_eq!(triple_x("xoxotrololololololoxxx"), false);
        assert_eq!(triple_x("soft kitty, warm kitty, xxxxx"), true);
        assert_eq!(triple_x("softx kitty, warm kitty, xxxxx"), false);
    
        assert_eq!(triple_x("Xabraxxxas"), true);
        assert_eq!(triple_x("xoXotrololololololoxxx"), false);
        assert_eq!(triple_x("softXxxx kitty, warm kitty, xxxxx"), true);
        assert_eq!(triple_x("softxXxxx kitty, warm kitty, xxxxx"), false);
    }
}

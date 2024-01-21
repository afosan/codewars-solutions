//! https://www.codewars.com/kata/58223370aef9fc03fd000071/train/rust

pub fn dashatize(n: i64) -> String {
    if n == 0 {
        "0".to_string()
    } else if n < 0 {
        dashatize(-n)
    } else {
        let mut out = String::new();
        let mut num = n;
        let mut already_dashed = false;
        
        while num > 0 {
            let is_start = num == n;
            let is_end = num < 10;

            let d = num % 10;
            num /= 10;

            match (already_dashed, d % 2 == 0) {
                (_, true) => {
                    out.push_str(&format!("{d}"));
                    already_dashed = false;
                },
                (true, false) => {
                    out.push_str(&format!("{d}"));
                    if !is_end {
                        out.push_str("-");
                    }
                    already_dashed = true;
                },
                (false, false) => {
                    if !is_start {
                        out.push_str("-");
                    }
                    out.push_str(&format!("{d}"));
                    if !is_end {
                        out.push_str("-");
                    }
                    already_dashed = true;
                },
            }
        }

        out.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(dashatize(274), "2-7-4");
        assert_eq!(dashatize(5311), "5-3-1-1");
        assert_eq!(dashatize(86320), "86-3-20");
        assert_eq!(dashatize(974302), "9-7-4-3-02");
    }
    
    #[test]
    fn weird() {
        assert_eq!(dashatize(0), "0");
        assert_eq!(dashatize(-1), "1");
        assert_eq!(dashatize(-28369), "28-3-6-9");                
    }
}

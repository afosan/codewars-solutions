//! https://www.codewars.com/kata/57873ab5e55533a2890000c7/train/rust

pub fn time_correct(time_str: &str) -> Option<String> {
    let elems = time_str.split(":").map(|e| e.to_string()).collect::<Vec<String>>();
    
    if elems.len() != 3 {
        return None;
    }
    
    let mut h = match elems[0].parse::<u64>() {
        Ok(v) => v,
        Err(_) => return None,
    };
    
    let mut m = match elems[1].parse::<u64>() {
        Ok(v) => v,
        Err(_) => return None,
    };
    
    let mut s = match elems[2].parse::<u64>() {
        Ok(v) => v,
        Err(_) => return None,
    };
    
    m += s / 60;
    h += m / 60;
    s %= 60;
    m %= 60;
    h %= 24;

    Some(format!("{h:<02}:{m:<02}:{s:02}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(time_correct("").is_none());
    }
    
    #[test]
    fn invalid_format() {
        assert!(time_correct("001122").is_none());
        assert!(time_correct("00;11;22").is_none());
        assert!(time_correct("00:1c:22").is_none());
    }
    
    #[test]
    fn corrections() {
        assert_eq!(time_correct("09:10:01"), Some(String::from("09:10:01")));
        assert_eq!(time_correct("11:70:10"), Some(String::from("12:10:10")));
        assert_eq!(time_correct("19:99:09"), Some(String::from("20:39:09")));
        assert_eq!(time_correct("19:99:99"), Some(String::from("20:40:39")));
        assert_eq!(time_correct("24:01:01"), Some(String::from("00:01:01")));
        assert_eq!(time_correct("52:01:01"), Some(String::from("04:01:01")));        
    }
}

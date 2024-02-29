//! https://www.codewars.com/kata/544aed4c4a30184e960010f4/train/rust

pub fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let divs = (2..integer / 2 + 1).filter(|i| integer % *i == 0).collect::<Vec<_>>();
    
    if divs.len() == 0 {
        Err(format!("{integer} is prime"))
    } else {
        Ok(divs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(divisors(15), Ok(vec![3,5]));
        assert_eq!(divisors(12), Ok(vec![2,3,4,6]));
        assert_eq!(divisors(13), Err("13 is prime".to_string()));
    }
}

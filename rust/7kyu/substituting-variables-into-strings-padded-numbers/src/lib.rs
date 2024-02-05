//! https://www.codewars.com/kata/51c89385ee245d7ddf000001/train/rust

pub fn solution(n: u32) -> String {
    format!("Value is {n:0>5}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution(5), "Value is 00005");
        assert_eq!(solution(1204), "Value is 01204");
        assert_eq!(solution(109), "Value is 00109");
        assert_eq!(solution(0), "Value is 00000");
    }
}

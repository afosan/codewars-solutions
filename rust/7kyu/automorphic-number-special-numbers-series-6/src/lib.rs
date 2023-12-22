//! https://www.codewars.com/kata/5a58d889880385c2f40000aa/train/rust

pub fn automorphic(n: u64) -> String {
    let d = n.to_string().len();

    if n == ((n * n) % 10u64.pow(d as u32)) {
        "Automorphic"
    } else {
        "Not!!"
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(automorphic(1),"Automorphic");
        assert_eq!(automorphic(3),"Not!!");
        assert_eq!(automorphic(6),"Automorphic");
        assert_eq!(automorphic(9),"Not!!");
        assert_eq!(automorphic(25),"Automorphic");
        assert_eq!(automorphic(53),"Not!!");
        assert_eq!(automorphic(76),"Automorphic");
        assert_eq!(automorphic(95),"Not!!");
        assert_eq!(automorphic(625),"Automorphic");
        assert_eq!(automorphic(225),"Not!!");
    }
}

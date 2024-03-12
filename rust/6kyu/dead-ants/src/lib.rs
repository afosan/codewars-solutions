//! https://www.codewars.com/kata/57d5e850bfcdc545870000b7/train/rust

pub fn dead_ant_count(ants: &str) -> u32 {
    let (ca, cn, ct) = ants.replace("ant", "").chars().fold(
        (0_u32, 0_u32, 0_u32),
        |(ca, cn, ct), c| match c {
            'a' => (ca + 1, cn, ct),
            'n' => (ca, cn + 1, ct),
            't' => (ca, cn, ct + 1),
            _ => (ca, cn, ct),
        }
    );
    
    ca.max(cn).max(ct)
}

#[cfg(test)]
mod tests {
    use super::dead_ant_count;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: u32) {
        assert_eq!(dead_ant_count(s), expected, "{ERR_MSG} with ants = \"{s}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("ant ant ant ant", 0);
        dotest("", 0);
        dotest(" ", 0);
        dotest("ant anantt aantnt", 2);
        dotest("ant ant .... a nt", 1);
    }
}

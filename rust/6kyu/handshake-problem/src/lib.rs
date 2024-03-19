//! https://www.codewars.com/kata/5574835e3e404a0bed00001b/train/rust

pub fn get_participants(handshakes: u32) -> u32 {
    (0..).skip_while(|i| i * i - i < 2 * handshakes).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::get_participants;

    fn dotest(h: u32, expected: u32) {
        let actual = get_participants(h);
        assert!(actual == expected,
            "Test failed with handshakes = {h}\nExpected {expected} but got {actual}")
    }
    
    #[test]
    fn sample_tests() {
        dotest(0, 0);
        dotest(1, 2);
        dotest(3, 3);
        dotest(6, 4);
        dotest(7, 5);
    }
}

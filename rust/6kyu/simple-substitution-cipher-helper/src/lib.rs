use std::collections::HashMap;

struct Cipher {
    e: HashMap<char, char>,
    d: HashMap<char, char>,
}

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher {
        let e = HashMap::from_iter(map1.chars().zip(map2.chars()));
        let d = HashMap::from_iter(map2.chars().zip(map1.chars()));
        
        Cipher { e, d }
    }

    fn encode(&self, string: &str) -> String {
        string.chars().map(|c| self.e.get(&c).unwrap_or(&c).clone()).collect()
    }

    fn decode(&self, string: &str) -> String {
        string.chars().map(|c| self.d.get(&c).unwrap_or(&c).clone()).collect()
    }
}

#[test]
fn examples() {
    let map1 = "abcdefghijklmnopqrstuvwxyz";
    let map2 = "etaoinshrdlucmfwypvbgkjqxz";

    let cipher = Cipher::new(map1, map2);

    assert_eq!(cipher.encode("abc"), "eta");
    assert_eq!(cipher.encode("xyz"), "qxz");
    assert_eq!(cipher.decode("eirfg"), "aeiou");
    assert_eq!(cipher.decode("erlang"), "aikcfu");
}

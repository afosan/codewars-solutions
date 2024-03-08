//! https://www.codewars.com/kata/6584b7cac29ca91dd9124009/train/rust

pub fn convert_lojban(input: &str) -> u64 {
    let ins = input.chars().step_by(2).zip(input.chars().skip(1).step_by(2)).collect::<Vec<(char, char)>>();
    let nums = ins.iter().map(|(c1, c2)| match format!("{c1}{c2}").as_ref() {
        "no" => 0,
        "pa" => 1,
        "re" => 2,
        "ci" => 3,
        "vo" => 4,
        "mu" => 5,
        "xa" => 6,
        "ze" => 7,
        "bi" => 8,
        "so" => 9,
        _ => panic!("unexpected char sequence"),
    }).collect::<Vec<u64>>();
    
    nums.iter().fold(0_u64, |acc, n| 10 * acc + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_tests() {
        assert_eq!(convert_lojban("no"), 0);
        assert_eq!(convert_lojban("sobi"), 98);
        assert_eq!(convert_lojban("muxa"), 56);
        assert_eq!(convert_lojban("zexamu"), 765);
        assert_eq!(convert_lojban("vocirepa"), 4321);
        assert_eq!(convert_lojban("civozeno"), 3470);
        assert_eq!(convert_lojban("renonore"), 2002);
        assert_eq!(convert_lojban("panonononononono"), 10000000);
    }
}

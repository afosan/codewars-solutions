//! https://www.codewars.com/kata/592eaf848c91f248ca000012/train/rust

fn from_str_to_u64(s: &str) -> u64 {
    match s {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        "eleven" => 11,
        "twelve" => 12,
        "thirteen" => 13,
        "fourteen" => 14,
        "fifteen" => 15,
        "sixteen" => 16,
        "seventeen" => 17,
        "eighteen" => 18,
        "nineteen" => 19,
        "twenty" => 20,
        _ => unreachable!(),
    }
}

fn from_u64_to_str(i: u64) -> &'static str {
    match i {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        _ => unreachable!(),
    }
}

struct Arith {
	value : &'static str,
}

impl Arith {
    fn add(&self, s: &str) -> &'static str {
        from_u64_to_str(
            from_str_to_u64(self.value) + from_str_to_u64(s)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        let c = Arith{value: "three"};
        assert_eq!(c.add("seven"), "ten");
        assert_eq!(c.add("eight"), "eleven");
        assert_eq!(c.add("zero"), "three");
    }
}

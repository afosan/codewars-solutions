//! https://www.codewars.com/kata/5868b2de442e3fb2bb000119/train/rust

fn weight(n: u64) -> u64 {
    let mut s = 0;
    let mut n = n;
    
    while n > 0 {
        let d = n % 10;
        n /= 10;
        s += d;
    }

    s
}

pub fn closest(s: &str) -> String {
    let nums = s
        .split_whitespace()
        .map(|num_str| num_str.parse::<u64>().expect("expected word to be a number"))
        .collect::<Vec<_>>();

    let mut indexed_weights = nums
        .iter()
        .map(|num| weight(*num))
        .enumerate()
        .collect::<Vec<_>>();

    indexed_weights.sort_by_key(|iw| iw.1);

    let (_, (i1, i2)) = indexed_weights.windows(2).fold((u64::MAX, (0_usize, 0_usize)), |(acc_min, (acc_i1, acc_i2)), iws| {
        let diff = iws[1].1 - iws[0].1;
        if diff < acc_min {
            (diff, (iws[0].0, iws[1].0))
        } else {
            (acc_min, (acc_i1, acc_i2))
        }
    });
    
    let num1 = nums[i1];
    let num2 = nums[i2];
    let weight1 = weight(num1);
    let weight2 = weight(num2);
    
    format!("[({weight1},{i1},{num1})({weight2},{i2},{num2})]")
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn testing(s: &str, exp: String) -> () {
        let ans = closest(s);
        assert_eq!(ans, exp, "Testing: {}", s);
    }

    #[test]
    fn basic_tests() {
        let mut s = "456899 50 11992 176 272293 163 389128 96 290193 85 52"; // [(13, 9, "85"), (14, 3, "176")]
        testing(s, "[(13,9,85)(14,3,176)]".to_string());
        s = "239382 162 254765 182 485944 134 468751 62 49780 108 54"; // "[[8, 5, 134], [8, 7, 62]]";
        testing(s, "[(8,5,134)(8,7,62)]".to_string());
        s = "241259 154 155206 194 180502 147 300751 200 406683 37 57";
        let r = "[(10,1,154)(10,9,37)]";
        testing(s, r.to_string());
        
    }
}

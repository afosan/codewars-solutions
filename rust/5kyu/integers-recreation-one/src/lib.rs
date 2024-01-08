//! https://www.codewars.com/kata/55aa075506463dac6600010d/train/rust

pub fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..=n)
        .map(|num| {
            let div_square_sum = (1..=num)
                .take_while(|i| i * i <= num)
                .filter(|i| num % i == 0)
                .map(|i| 
                    if i * i == num {
                        num
                    } else {
                        let d2 = num / i;
                        i * i + d2 * d2
                    }
                )
                .sum::<u64>();
            (num, div_square_sum)
        })
        .filter(|(_, s)| {
            let sq = (*s as f64).sqrt();
            sq == sq.floor()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(list_squared(m, n), exp)
    }
    
    #[test]
    fn basics_list_squared() {
    
        testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
        testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
        testing(42, 250, vec![(42, 2500), (246, 84100)]);
        testing(300, 600, vec![]);
        
    }
}

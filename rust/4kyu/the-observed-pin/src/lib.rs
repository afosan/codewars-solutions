//! https://www.codewars.com/kata/5263c6999e0f40dee200059d/train/rust

fn get_possibilities(n: u8) -> Vec<String> {
    let mut v = vec![n];
    
    if n == 0 {
        return vec!["0".into(), "8".into()];
    }
    
    match n % 3 {
        0 => {
            v.push(n - 1);
        },
        1 => {
            v.push(n + 1);
        },
        2 => {
            v.push(n - 1);
            v.push(n + 1);
        },
        _ => unreachable!(),
    }
    
    if n <= 6 {
        v.push(n + 3);
    }
    
    if n >= 4 {
        v.push(n - 3);
    }
    
    if n == 8 {
        v.push(0);
    }

    v.iter().map(|d| d.to_string()).collect()
}

pub fn get_pins(observed: &str) -> Vec<String> {
    let v = observed.chars().map(|c| c.to_digit(10).expect("expected digit") as u8).map(|d| get_possibilities(d)).collect::<Vec<_>>();
    let mut vout = v.into_iter().reduce(|v1, v2| {
        let mut v = Vec::with_capacity(v1.len() * v2.len());
        for i1 in &v1 {
            for i2 in &v2 {
                v.push(format!("{i1}{i2}"));
            }
        }

        v
    }).unwrap();

    vout.sort();

    vout.iter().map(|n| n.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::get_pins;
    use itertools::Itertools;
    

    #[test]
    fn sample_tests() {
        assert_eq!(get_pins("8").iter().sorted().collect::<Vec<&String>>(), 
            vec!["0", "5", "7", "8", "9"]);
        assert_eq!(get_pins("11").iter().sorted().collect::<Vec<&String>>(), 
            vec!["11", "12", "14",  "21", "22", "24",  "41", "42", "44"]);
        assert_eq!(get_pins("369").iter().sorted().collect::<Vec<&String>>(), 
            vec!["236", "238", "239",  "256", "258", "259",  "266", "268", "269",  "296", "298", "299", 
                "336", "338", "339",  "356", "358", "359",  "366", "368", "369",  "396", "398", "399", 
                "636", "638", "639",  "656", "658", "659",  "666", "668", "669",  "696", "698", "699"]);
    }
}

//! https://www.codewars.com/kata/51ba717bb08c1cd60f00002f/train/rust

mod solution {
    pub fn update_string(s: &mut String, range: (i32, i32)) {
        let (start, last) = range;
        if last - start < 2 {
            (start..=last).for_each(|n| s.push_str(&format!("{n},")));
        } else {
            s.push_str(&format!("{start}-{last},"));
        };
    }
    
    pub fn range_extraction(a: &[i32]) -> String {
        let mut out: String = String::new();
        let mut range: Option<(i32, i32)> = None;
        
        for &n in a {
            match range {
                Some((start, last)) => {
                    if n == last + 1 {
                        range = Some((start, n));
                    } else {
                        update_string(&mut out, range.unwrap());
                        range = Some((n, n));
                    }
                },
                None => {
                    range = Some((n, n));
                },
            }
        }
        update_string(&mut out, range.unwrap());

        out[..out.len()-1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]), "-6,-3-1,3-5,7-11,14,15,17-20");	
        assert_eq!(solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]), "-3--1,2,10,15,16,18-20");
    }
}

//! https://www.codewars.com/kata/57d99f6bbfcdc5b3b0000286/train/rust

use std::collections::HashSet;

pub fn count_right_triangles(points: &[(i32, i32)]) -> u32 {
    let hs: HashSet<&(i32, i32)> = points.iter().collect();
    
    let mut c = 0;
    for p in hs.iter() {
        for p1 in hs.iter() {
            if p == p1 {
                continue;
            }
            
            for p2 in hs.iter() {
                if p == p2 {
                    continue;
                }
                
                if (p2.1 - p.1) * (p1.1 - p.1) + (p2.0 - p.0) * (p1.0 - p.0) == 0 {
                    c += 1;
                }
            }
        }  
    }
    
    c / 2
}

#[cfg(test)]
mod tests {
    use super::count_right_triangles;
        
    fn dotest(points: &[(i32, i32)], expected: u32) {
        let actual = count_right_triangles(points);
        assert!(actual == expected, 
            "With points = {points:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[(1, 2),(3, 3),(4, 1),(1, 1),(4, -1)], 3);
        dotest(&[(1, 2),(4, -1),(3, 3),(4, -1),(4, 1),(1, 1),(4, -1), (4, -1), (3, 3), (1, 2)], 3);
        dotest(&[(30, 26), (36, 6), (12, 27), (9, 8), (9, 22), (6, 35), (26, 40),
                 (35, 18), (27, 2), (19, 18), (2, 41), (18, 3), (4, 37), (13, 25), (21, 34),
                 (27, 45), (26, 12), (23, 16), (28, 1), (0, 25), (12, 25), (10, 41), (24, 18),
                 (31, 38), (28, 17), (9, 23), (29, 1), (21, 43), (20, 46), (50, 10)], 31);
    }
}

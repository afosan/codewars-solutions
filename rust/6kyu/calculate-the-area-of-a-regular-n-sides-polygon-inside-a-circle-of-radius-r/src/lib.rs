//! https://www.codewars.com/kata/5a58ca28e626c55ae000018a/train/rust

pub fn area_of_polygon_inside_circle(circle_radius: f64, number_of_sides: u32) -> f64 {
    let n = number_of_sides as f64;
    circle_radius * circle_radius * f64::sin(2_f64 * std::f64::consts::PI / n) * n / 2_f64
}

// mod preloaded;

// #[cfg(test)]
// mod tests {
//     use super::area_of_polygon_inside_circle;
//     use crate::preloaded::assert_approx_eq;
    
//     #[test]
//     fn sample_tests() {
//         for (radius, sides, expected) in [(3.0, 3, 11.691), (2.0, 4, 8.0), (2.5, 5, 14.86)] {
//             assert_approx_eq!(
//                 area_of_polygon_inside_circle(radius, sides),
//                 expected,
//                 1e-03,
//                 radius, sides
//             );
//         }
//     }
// }

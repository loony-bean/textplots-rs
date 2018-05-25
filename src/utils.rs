//! Helpers for passing the data into plots.
//!
//! Merely a bunch of functions hanging around while the library API is taking shape.

/// Transforms points into function using linear interpolation.
/// ```
/// # extern crate textplots;
/// # use textplots::utils;
/// assert_eq!(1.0, utils::interpolate( &[ (0.0, 0.0), (10.0, 10.0) ] )(1.0));
/// ```
pub fn interpolate<'a>(data: &'a [(f32, f32)]) -> impl Fn(f32) -> f32 + 'a {
    move |x| {
        for pair in data.windows(2) {
            let p1 = pair[0];
            let p2 = pair[1];
            if p1.0 <= x && x < p2.0 {
                let weight = (x - p1.0) / (p2.0 - p1.0);
                return p1.1 + weight * (p2.1 - p1.1);
            }
        }
        0.0
    }
}

/// Transforms points into step function.
/// ```
/// # extern crate textplots;
/// # use textplots::utils;
/// assert_eq!(5.0, utils::staircase( &[ (0.0, 0.0), (10.0, 10.0) ] )(1.0));
/// ```
pub fn staircase<'a>(data: &'a [(f32, f32)]) -> impl Fn(f32) -> f32 + 'a {
    move |x| {
        for pair in data.windows(2) {
            let p1 = pair[0];
            let p2 = pair[1];
            if p1.0 <= x && x < p2.0 {
                return (p1.1 + p2.1) / 2.0;
            }
        }
        0.0
    }
}

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

pub fn histogram(data: &[(f32, f32)], min: f32, max: f32, bins: usize) -> Vec<(f32, f32)> {
    let mut output = vec!(0; bins + 1);

    let step = (max - min) / (bins + 1) as f32;

    for &(_x, y) in data.iter() {
        if y < min || y > max {
            continue;
        }

        let bucket_id = ((y - min) / step) as usize;
        if bucket_id < output.len() {
            output[bucket_id as usize] += 1;
        }
    }

    output
        .into_iter()
        .enumerate()
        .map(|(x, y)| ((min + (x as f32) * step), y as f32) )
        .collect()
}

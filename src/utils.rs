//! Helpers for passing the data into plots.
//!
//! Merely a bunch of functions hanging around while the library API is taking shape.

/// Transforms points into frequency distribution (for using in histograms).
/// Values outside of [`min`, `max`] interval are ignored, and everything that
/// falls into the specified interval is grouped into `bins` number of buckets of equal width.
///
/// ```
/// # use textplots::utils::histogram;
/// assert_eq!(vec![(0.0, 1.0), (5.0, 1.0)], histogram( &[ (0.0, 0.0), (9.0, 9.0), (10.0, 10.0) ], 0.0, 10.0, 2 ));
/// ```
pub fn histogram(data: &[(f32, f32)], min: f32, max: f32, bins: usize) -> Vec<(f32, f32)> {
    let mut output = vec![0; bins];

    let step = (max - min) / bins as f32;

    for &(_x, y) in data.iter() {
        if y < min || y > max {
            continue;
        }

        let bucket_id = ((y - min) / step) as usize;
        if bucket_id < output.len() {
            output[bucket_id] += 1;
        }
    }

    output
        .into_iter()
        .enumerate()
        .map(|(x, y)| ((min + (x as f32) * step), y as f32))
        .collect()
}

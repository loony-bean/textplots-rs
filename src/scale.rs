//! Transformations between domain and range.

use std::ops::Range;

/// Holds mapping between domain and range of the function.
pub struct Scale {
    domain: Range<f32>,
    range: Range<f32>,
}

impl Scale {
    /// Translates value from domain to range scale.
    /// ```
    /// # extern crate textplots;
    /// # use textplots::scale::Scale;
    /// assert_eq!(-0.8, Scale::new(0_f32..10_f32, -1_f32..1_f32).linear(1.0));
    /// ```
    pub fn linear(&self, x: f32) -> f32 {
        let p = (x - self.domain.start) / (self.domain.end - self.domain.start);
        let r = self.range.start + p * (self.range.end - self.range.start);
        let r = r.max(self.range.start);
        let r = r.min(self.range.end);
        r
    }

    /// Translates value from range to domain scale.
    /// ```
    /// # extern crate textplots;
    /// # use textplots::scale::Scale;
    /// assert_eq!(5.5, Scale::new(0_f32..10_f32, -1_f32..1_f32).inv_linear(0.1));
    /// ```
    pub fn inv_linear(&self, i: f32) -> f32 {
        let p = (i - self.range.start) / (self.range.end - self.range.start);
        let d = self.domain.start + p * (self.domain.end - self.domain.start);

        let d = d.max(self.domain.start);
        let d = d.min(self.domain.end);
        d
    }

    pub fn new(domain: Range<f32>, range: Range<f32>) -> Self {
        Scale {
            domain,
            range,
        }
    }
}

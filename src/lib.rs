//! Terminal plotting library for using in CLI applications.
//! Should work well in any unicode terminal with monospaced font.
//!
//! It is inspired by [TextPlots.jl](https://github.com/sunetos/TextPlots.jl) which is inspired by [Drawille](https://github.com/asciimoo/drawille).
//! 
//! Currently it features only drawing line plots on Braille canvas, but could be extended
//! to support other canvas and chart types just like [UnicodePlots.jl](https://github.com/Evizero/UnicodePlots.jl)
//! or any other cool terminal plotting library.
//!
//! Contributions are very much welcome!
//!
//! # Usage
//! ```toml
//! [dependencies]
//! textplots = "0.2"
//! ```
//!
//! ```rust
//! extern crate textplots;
//!
//! use textplots::{Chart, Plot};
//!
//! fn main() {
//!     println!("y = sin(x) / x");
//!     Chart::default().lineplot(|x| x.sin() / x ).display();
//! }
//! ```
//! It will display something like this:
//!
//! <img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo.png?raw=true"/>
//!
//! Default viewport size is 120 x 60 points, with X values ranging from -10 to 10.
//! You can override the defaults calling `new`.
//!
//! ```rust
//! use textplots::{Chart, Plot};
//!
//! println!("y = cos(x), y = sin(x) / 2");
//! Chart::new(180, 60, -5.0, 5.0)
//!     .lineplot( |x| x.cos() )
//!     .lineplot( |x| x.sin() / 2.0 )
//!     .display();
//! ```
//! <img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo2.png?raw=true"/>
//!
extern crate drawille;

use drawille::{Canvas as BrailleCanvas};
use std::cmp;
use std::default::Default;

/// Controls the drawing.
pub struct Chart {
    /// Canvas width in points
    width: u32,
    /// Canvas height in points
    height: u32,
    /// X-axis start value
    xmin: f32,
    /// X-axis end value
    xmax: f32,
    /// Y-axis start value (calculated automatically to display all the domain values)
    ymin: f32,
    /// Y-axis end value (calculated automatically to display all the domain values)
    ymax: f32,
    /// Underlying canvas object
    canvas: BrailleCanvas,
}

/// Provides an interface for drawing plots.
pub trait Plot {
    /// Draws a [line chart](https://en.wikipedia.org/wiki/Line_chart) of points connected by straight line segments.
    fn lineplot(&mut self, func: impl Fn(f32) -> f32) -> &mut Chart;
}

impl Default for Chart {
    fn default() -> Self {
        Self::new(120, 60, -10.0, 10.0)
    }
}

impl Chart {
    /// Creates a new `Chart` object.
    ///
    /// # Panics
    ///
    /// Panics if `width` or `height` is less than 32.
    pub fn new(width: u32, height: u32, xmin: f32, xmax: f32) -> Self {
        if width < 32 {
            panic!("width should be more then 32, {} is provided", width);
        }

        if height < 32 {
            panic!("height should be more then 32, {} is provided", height);
        }

        Self {
            xmin,
            xmax,
            ymin: 0.0,
            ymax: 0.0,
            width,
            height,
            canvas: BrailleCanvas::new(width, height),
        }
    }

    /// Displays bounding rect,
    fn borders(&mut self) {
        let w = self.width;
        let h = self.height;

        self.canvas.line(0, 0, 0, h);
        self.canvas.line(0, 0, w, 0);
        self.canvas.line(0, h, w, h);
        self.canvas.line(w, 0, w, h);
    }

    /// Draws vertical line,
    fn vline(&mut self, i: u32) {
        if i > 0 && i < self.width {
            for j in 0..self.height {
                if j % 3 == 0 {
                    self.canvas.set(i, j);
                }
            }
        }
    }

    /// Draws horisontal line.
    fn hline(&mut self, j: u32) {
        if j > 0 && j < self.height {
            for i in 0..self.width {
                if i % 3 == 0 {
                    self.canvas.set(i, self.height - j);
                }
            }
        }
    }

    /// Prints canvas content.
    pub fn display(&self) {
        let frame = self.canvas.frame();
        let rows = frame.split('\n').into_iter().count();
        for (i, row) in frame.split('\n').into_iter().enumerate() {
            if i == 1 {
                println!("{0} {1:.1}", row, self.ymax);
            } else if i == (rows - 2) {
                println!("{0} {1:.1}", row, self.ymin);
            } else {
                println!("{}", row);
            }
        }

        println!("{0: <width$.1}{1:.1}", self.xmin, self.xmax, width=(self.width as usize) / 2 - 3);
    }
}

impl Plot for Chart {
    fn lineplot(&mut self, func: impl Fn(f32) -> f32) -> &mut Chart {
        self.borders();

        // calculation of x range
        let xrange = (self.xmax - self.xmin).abs();
        let xstep = xrange / self.width as f32;

        // auto calculation of y range
        let ys: Vec<_> = (0..self.width)
            .into_iter()
            .map(|i| func(self.xmin + (i as f32) * xstep) )
            .collect();

        let mut ymax = *ys.iter().max_by( |x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Equal) ).unwrap_or(&0.0);
        let mut ymin = *ys.iter().min_by( |x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Equal) ).unwrap_or(&0.0);

        self.ymin = f32::min(self.ymin, ymin);
        self.ymax = f32::max(self.ymax, ymax);

        let margin = (self.ymax - self.ymin) * 0.05;
        ymin = self.ymin - margin;
        ymax = self.ymax + margin;
        let yrange = ymax - ymin;

        // show axis
        let i_center = ((xrange - self.xmax) / xrange) * self.width as f32;
        self.vline(i_center as u32);

        let j_center = ((yrange - ymax) / yrange) * self.height as f32;
        self.hline(j_center as u32);

        // calculate func and translate (x, y) points into screen coordinates
        let points: Vec<_> = (0..self.width)
            .into_iter()
            .filter_map(|i| {
                let x = self.xmin + (i as f32) * xstep;
                let y = func(x);
                if y.is_normal() {
                    let j = (((y - ymin) / yrange) * self.height as f32) as i32;
                    let j = j.max(0) as u32;
                    let j = j.min(self.width);
                    Some((i, self.height - j))
                } else {
                    None
                }
            }).collect();

        for pair in points.windows(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            self.canvas.line(x1, y1, x2, y2);
        }

        self
    }
}

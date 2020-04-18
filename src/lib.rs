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
//! textplots = "0.3"
//! ```
//!
//! ```rust
//! extern crate textplots;
//!
//! use textplots::{Chart, Plot, Shape};
//!
//! fn main() {
//!     println!("y = sin(x) / x");
//!     Chart::default().lineplot( Shape::Continuous( |x| x.sin() / x )).display();
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
//! use textplots::{Chart, Plot, Shape};
//!
//! println!("y = cos(x), y = sin(x) / 2");
//! Chart::new(180, 60, -5.0, 5.0)
//!     .lineplot( Shape::Continuous( |x| x.cos() ))
//!     .lineplot( Shape::Continuous( |x| x.sin() / 2.0 ))
//!     .display();
//! ```
//! <img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo2.png?raw=true"/>
//!
//! You could also plot series of points. See [Shape](enum.Shape.html) and [examples](https://github.com/loony-bean/textplots-rs/tree/master/examples) for more details.
//!
//! <img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo3.png?raw=true"/>
//!

extern crate drawille;

pub mod utils;
pub mod scale;

use drawille::{Canvas as BrailleCanvas};
use scale::Scale;
use std::cmp;
use std::f32;
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

/// Specifies different kinds of plotted data.
pub enum Shape<'a> {
    /// Real value function
    Continuous(fn(f32) -> f32),
    /// Points of a scatter plot.
    Points(&'a [(f32, f32)]),
    /// Points connected with lines.
    Lines(&'a [(f32, f32)]),
    /// Points connected in step fashion.
    Steps(&'a [(f32, f32)]),
    /// Points represented with bars.
    Bars(&'a [(f32, f32)]),
}

/// Provides an interface for drawing plots.
pub trait Plot {
    /// Draws a [line chart](https://en.wikipedia.org/wiki/Line_chart) of points connected by straight line segments.
    fn lineplot(&mut self, shape: Shape) -> &mut Chart;
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
            ymin: f32::INFINITY,
            ymax: f32::NEG_INFINITY,
            width,
            height,
            canvas: BrailleCanvas::new(width, height)
        }
    }

    /// Displays bounding rect.
    fn borders(&mut self) {
        let w = self.width;
        let h = self.height;

        self.vline(0);
        self.vline(w);
        self.hline(0);
        self.hline(h);
    }

    /// Draws vertical line.
    fn vline(&mut self, i: u32) {
        if i <= self.width {
            for j in 0..=self.height {
                if j % 3 == 0 {
                    self.canvas.set(i, j);
                }
            }
        }
    }

    /// Draws horisontal line.
    fn hline(&mut self, j: u32) {
        if j <= self.height {
            for i in 0..=self.width {
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
            if i == 0 {
                println!("{0} {1:.1}", row, self.ymax);
            } else if i == (rows - 1) {
                println!("{0} {1:.1}", row, self.ymin);
            } else {
                println!("{}", row);
            }
        }

        println!("{0: <width$.1}{1:.1}", self.xmin, self.xmax, width=(self.width as usize) / 2 - 3);
    }

    /// Prints canvas content with some additional visual elements (like borders).
    pub fn nice(&mut self) {
        self.borders();
        // self.axis();
        self.display();
    }

    /// Return the frame
    pub fn frame(&self) -> String {
        self.canvas.frame()
    }
}

impl Plot for Chart {
    fn lineplot(&mut self, shape: Shape) -> &mut Chart {
        let x_scale = Scale::new(self.xmin..self.xmax, 0.0..self.width as f32);

        let ys: Vec<_> = match shape {
            Shape::Continuous(f) => {
                (0..self.width)
                .into_iter()
                .filter_map(|i| {
                    let x = x_scale.inv_linear(i as f32);
                    let y = f(x);
                    if y.is_normal() {
                        Some(y)
                    } else {
                        None
                    }
                })
                .collect()
            },
            | Shape::Points(dt)
            | Shape::Lines(dt)
            | Shape::Steps(dt)
            | Shape::Bars(dt) => {
                dt.iter()
                .filter_map(|(x, y)| {
                    if *x >= self.xmin && *x <= self.xmax {
                        Some(*y)
                    } else {
                        None
                    }
                }).collect()
            },
        };

        let ymax = *ys.iter().max_by( |x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Equal) ).unwrap_or(&0.0);
        let ymin = *ys.iter().min_by( |x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Equal) ).unwrap_or(&0.0);

        self.ymin = f32::min(self.ymin, ymin);
        self.ymax = f32::max(self.ymax, ymax);

        let y_scale = Scale::new(self.ymin..self.ymax, 0.0..self.height as f32);

        // show axis
        if self.xmin <= 0.0 && self.xmax >= 0.0 {
            self.vline(x_scale.linear(0.0) as u32);
        }
        if self.ymin <= 0.0 && self.ymax >= 0.0 {
            self.hline(y_scale.linear(0.0) as u32);
        }

        // translate (x, y) points into screen coordinates
        let points: Vec<_> = match shape {
            Shape::Continuous(f) => {
                (0..self.width)
                .into_iter()
                .filter_map(|i| {
                    let x = x_scale.inv_linear(i as f32);
                    let y = f(x);
                    if y.is_normal() {
                        let j = y_scale.linear(y).round();
                        Some((i, self.height - j as u32))
                    } else {
                        None
                    }
                })
                .collect()
            },
            | Shape::Points(dt)
            | Shape::Lines(dt)
            | Shape::Steps(dt)
            | Shape::Bars(dt) => {
                dt
                .into_iter()
                .filter_map(|(x, y)| {
                    let i = x_scale.linear(*x).round() as u32;
                    let j = y_scale.linear(*y).round() as u32;
                    if i <= self.width && j <= self.height {
                        Some( (i, self.height - j) )
                    } else {
                        None
                    }
                })
                .collect()
            },
        };

        // display segments
        match shape {
            Shape::Continuous(_) | Shape::Lines(_) => {
                for pair in points.windows(2) {
                    let (x1, y1) = pair[0];
                    let (x2, y2) = pair[1];

                    self.canvas.line(x1, y1, x2, y2);
                }
            },
            Shape::Points(_) => {
                for (x, y) in points {
                    self.canvas.set(x, y);
                }
            },
            Shape::Steps(_) => {
                for pair in points.windows(2) {
                    let (x1, y1) = pair[0];
                    let (x2, y2) = pair[1];

                    self.canvas.line(x1, y2, x2, y2);
                    self.canvas.line(x1, y1, x1, y2);
                }
            }
            Shape::Bars(_) => {
                for pair in points.windows(2) {
                    let (x1, y1) = pair[0];
                    let (x2, y2) = pair[1];

                    self.canvas.line(x1, y2, x2, y2);
                    self.canvas.line(x1, y1, x1, y2);
                    self.canvas.line(x1, self.height, x1, y1);
                    self.canvas.line(x2, self.height, x2, y2);
                }
            }
        }

        self
    }
}

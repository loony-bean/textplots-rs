# textplots [![Crates.io](https://img.shields.io/crates/v/textplots.svg)](https://crates.io/crates/textplots) [![Build Status](https://travis-ci.org/loony-bean/textplots-rs.svg?branch=master)](https://travis-ci.org/loony-bean/textplots-rs)

Terminal plotting library for using in Rust CLI applications.
Should work well in any unicode terminal with monospaced font.

It is insired by [TextPlots.jl](https://github.com/sunetos/TextPlots.jl) which is inspired by [Drawille](https://github.com/asciimoo/drawille).

Currently it features only drawing line charts on Braille canvas, but could be exterded
to support other canvas and chart type just like [UnicodePlots.jl](https://github.com/Evizero/UnicodePlots.jl)
or any other cool terminal plotting library.

Contributions are very much welcome!

```rust
extern crate textplots;

use textplots::{Chart, Plot};

fn main() {
    println!("y = sin(x) / x");
    Chart::default().lineplot(|x| x.sin() / x ).display();
}
```

<img src="https://raw.githubusercontent.com/loony-bean/textplots-rs/master/doc/demo.png">

# textplots [![Crates.io](https://img.shields.io/crates/v/textplots.svg)](https://crates.io/crates/textplots)

Terminal plotting library for using in Rust CLI applications.
Should work well in any unicode terminal with monospaced font.

It is inspired by [TextPlots.jl](https://github.com/sunetos/TextPlots.jl) which is inspired by [Drawille](https://github.com/asciimoo/drawille).

Currently it features only drawing line charts on Braille canvas, but could be extended
to support other canvas and chart types just like [UnicodePlots.jl](https://github.com/Evizero/UnicodePlots.jl)
or another cool terminal plotting library.

Contributions are very much welcome!

# Usage

## Using as a library

```rust
use textplots::{Chart, Plot, Shape};

fn main() {
    println!("y = sin(x) / x");

    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(|x| x.sin() / x)))
        .display();
}
```

<img src="https://raw.githubusercontent.com/loony-bean/textplots-rs/master/doc/demo.png">

## Using as a binary

```sh
textplots '10*x + x^2 + 10*sin(x)*abs(x)' --xmin=-20 --xmax=20
```

<img src="https://raw.githubusercontent.com/loony-bean/textplots-rs/master/doc/demo4.png">

## Bonus! Colored plots (see examples)

<img src="https://raw.githubusercontent.com/loony-bean/textplots-rs/master/doc/demo5.png">

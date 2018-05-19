# textplots [![Crates.io](https://img.shields.io/crates/v/textplots.svg)](https://crates.io/crates/textplots) [![Build Status](https://travis-ci.org/loony-bean/textplots-rs.svg?branch=master)](https://travis-ci.org/loony-bean/textplots-rs)

```
extern crate textplots;

use textplots::{Chart, Plot};

Chart::default()
    .lineplot(|x| x.sin() / x )
    .display();
```

<img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo.png" width="500">

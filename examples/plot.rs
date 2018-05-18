extern crate textplots;

use textplots::{Chart, Plot};

fn main() {
    println!("y = atan(x)");
    Chart::new(120, 60).lineplot(&(|x| x.atan() ));

    println!();
    println!("y = sin(x) / x");
    Chart::new(120, 60).lineplot(&(|x| x.sin() / x ));

    println!();
    println!("y = cos(x)");
    Chart::new(120, 60).lineplot(&(|x| x.cos()));
}

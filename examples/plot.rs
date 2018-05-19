extern crate textplots;

use textplots::{Chart, Plot};

fn main() {
    // You can pass any real value function
    println!("y = atan(x)");
    Chart::default().lineplot(|x| x.atan() ).display();

    // The plot try to display everything that is a `normal` float, skipping NaN's and friends
    println!("\ny = sin(x) / x");
    Chart::default().lineplot(|x| x.sin() / x ).display();

    // Default viewport size is 120 x 60 points, with X values ranging from -10 to 10.
    println!("\ny = ln(x)");
    Chart::default().lineplot(|x| x.ln() ).display();

    // You can plot several functions on the same chart.
    // However the resolution of text displays is low, and the result might not be great.
    // (note how you can use `new` to override some defaults)
    println!("\ny = cos(x), y = sin(x) / 2");
    Chart::new(180, 60, -5.0, 5.0)
        .lineplot( |x| x.cos() )
        .lineplot( |x| x.sin() / 2.0 )
        .display();
}

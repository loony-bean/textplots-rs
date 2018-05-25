extern crate textplots;

use textplots::{Chart, Plot, utils};

fn main() {
    // You can pass any real value function
    println!("y = atan(x)");
    Chart::default().lineplot(|x| x.atan() ).display();

    // The plot try to display everything that is a `normal` float, skipping NaN's and friends
    println!("\ny = sin(x) / x");
    Chart::default().lineplot(|x| x.sin() / x ).display();

    // Default viewport size is 120 x 60 points, with X values ranging from -10 to 10.
    println!("\ny = ln(x)");
    Chart::default().lineplot( f32::ln ).display();

    // You can plot several functions on the same chart.
    // However the resolution of text displays is low, and the result might not be great.
    println!("\ny = cos(x), y = sin(x) / 2");
    Chart::new(180, 60, -5.0, 5.0)
        .lineplot( |x| x.cos() )
        .lineplot( |x| x.sin() / 2.0 )
        .display();

    let points = [
        (-10.0, -1.0),
        (0.0, 0.0),
        (1.0, 1.0),
        (2.0, 0.0),
        (3.0, 3.0),
        (4.0, 4.0),
        (5.0, 3.0),
        (9.0, 1.0),
        (10.0, 0.0),
    ];

    println!("\ny = interpolated points");
    Chart::default()
        .lineplot(utils::interpolate(&points))
        .display();

    println!("\ny = staircase points");
    Chart::default()
        .lineplot(utils::staircase(&points))
        .display();
}

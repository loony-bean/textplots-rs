use textplots::{Chart, Plot, Shape};

fn main() {
    // You can pass any real value function.
    println!("y = atan(x)");
    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(|x| x.atan())))
        .display();

    // The plot try to display everything that is a `normal` float, skipping NaN's and friends.
    println!("\ny = sin(x) / x");
    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(|x| x.sin() / x)))
        .display();

    // Default viewport size is 120 x 60 points, with X values ranging from -10 to 10.
    println!("\ny = ln(x)");
    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(f32::ln)))
        .display();

    // You can plot several functions on the same chart.
    // However the resolution of text displays is low, and the result might not be great.
    println!("\ny = cos(x), y = sin(x) / 2");
    Chart::new(180, 60, -5.0, 5.0)
        .lineplot(&Shape::Continuous(Box::new(|x| x.cos())))
        .lineplot(&Shape::Continuous(Box::new(|x| x.sin() / 2.0)))
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
        (10.0, -1.0),
    ];

    println!("\ny = interpolated points");
    Chart::default().lineplot(&Shape::Lines(&points)).display();

    println!("\ny = staircase points");
    Chart::default().lineplot(&Shape::Steps(&points)).display();

    println!("\ny = scatter plot");
    Chart::default().lineplot(&Shape::Points(&points)).display();

    // You can instead get the raw string value
    println!("\nRender to string (and then print that string)");
    let chart = Chart::default()
        .lineplot(&Shape::Continuous(Box::new(|x| x.atan())))
        .to_string();

    println!("{}", chart);
}

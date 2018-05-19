extern crate textplots;

use textplots::{Chart, Plot};

fn main() {
    println!("y = atan(x)");
    Chart::default().lineplot(|x| x.atan() ).display();

    println!("\ny = sin(x) / x");
    Chart::default().lineplot(|x| x.sin() / x ).display();

    println!("\ny = ln(x)");
    Chart::default().lineplot(|x| x.ln() ).display();

    println!("\ny = cos(x), y = sin(x) / 2");
    Chart::new(180, 60, -5.0, 5.0)
        .lineplot( |x| x.cos() )
        .lineplot( |x| x.sin() / 2.0 )
        .display();
}

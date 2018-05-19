extern crate textplots;

use textplots::{Chart, Plot};

fn main() {
    println!("y = atan(x)");
    Chart::default().lineplot(&(|x: f32| x.atan() )).display();

    println!("\ny = sin(x) / x");
    Chart::default().lineplot(&(|x: f32| x.sin() / x )).display();

    println!("\ny = ln(x)");
    Chart::default().lineplot(&(|x: f32| x.ln() )).display();

    println!("\ny = cos(x), y = sin(x) / 2");
    Chart::default()
        .lineplot( &(|x| x.cos()) )
        .lineplot( &(|x| x.sin() / 2.0) )
        .display();
}

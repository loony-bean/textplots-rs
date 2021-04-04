use textplots::{Chart, Plot, Shape};
use clap::{Arg, App};
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_BIN_NAME"))
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .about(env!("CARGO_PKG_DESCRIPTION"))
      .arg(Arg::with_name("FORMULA")
           .help("Formula to plot")
           .required(true)
           .index(1))
      .get_matches();

    let formula = matches.value_of("FORMULA").unwrap(); // unreachable unwrap
    let res = formula.parse().and_then(|expr: meval::Expr| expr.bind("x"));
    let func = match res {
        Ok(func) => func,
        Err(err) => {
            // if there was an error with parsing
            // or binding "x", exit with error

            eprintln!("{}", err);
            exit(1);
        },
    };

    println!("y = {}", formula);
    Chart::default()
    	.lineplot(&Shape::Continuous(Box::new(|x| func(x.into()) as f32)))
    	.display();
}

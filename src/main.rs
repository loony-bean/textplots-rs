use std::process::exit;
use structopt::StructOpt;
use textplots::{Chart, Plot, Shape, AxisStyle, AxisBuilder};

#[derive(StructOpt)]
struct Opt {
    /// Formula to plot
    #[structopt(name = "FORMULA")]
    formula: String,
    /// X-axis start value.
    #[structopt(long, default_value = "-10.0")]
    xmin: f32,
    /// X-axis end value.
    #[structopt(long, default_value = "10.0")]
    xmax: f32,
    /// Y-axis start value.
    #[structopt(long)]
    ymin: Option<f32>,
    /// X-axis end value.
    #[structopt(long)]
    ymax: Option<f32>,
    /// Canvas width in points.
    #[structopt(short, long, default_value = "180")]
    width: u32,
    /// Canvas height in points.
    #[structopt(short, long, default_value = "60")]
    height: u32,
}

fn main() {
    let opt = Opt::from_args();

    let res = opt
        .formula
        .parse()
        .and_then(|expr: meval::Expr| expr.bind("x"));
    let func = match res {
        Ok(func) => func,
        Err(err) => {
            // if there was an error with parsing
            // or binding "x", exit with error

            eprintln!("{}", err);
            exit(1);
        }
    };

    // check for invalid ymin/ymax
    if (opt.ymax.is_none() && opt.ymin.is_some()) || (opt.ymax.is_some() && opt.ymin.is_none()) {
        eprintln!("both ymin and ymax must be specified");
        exit(2);
    }

    println!("y = {}", opt.formula);
    let mut chart = if opt.ymin.is_none() {
        Chart::new(opt.width, opt.height, opt.xmin, opt.xmax)
    } else {
        Chart::new_with_y_range(
            opt.width,
            opt.height,
            opt.xmin,
            opt.xmax,
            opt.ymin.unwrap(),
            opt.ymax.unwrap(),
        )
    };
    chart
        .lineplot(&Shape::Continuous(Box::new(|x| func(x.into()) as f32)))
        .x_style(AxisStyle::Solid)
        .y_style(AxisStyle::Solid)
        .display();
}

use chrono::{NaiveDate, Duration};
use textplots::{Chart, Shape, LabelBuilder, ColorPlot};

fn main() {
    // Specify how labels are displayed.
    let start = NaiveDate::from_ymd_opt(2023, 6, 1).unwrap();

    let end = NaiveDate::from_ymd_opt(2023, 9, 1).unwrap();

    println!("My step count over 3 months: ");
    Chart::new_with_y_range(200, 50, 0.0, (end - start).num_days() as f32, 0.0, 25_000.0)
        .linecolorplot(&Shape::Continuous(Box::new(|x| 1000.0 * (5.0 * (0.5 * x).sin() + 0.05 * x) + 9000.0)), rgb::RGB { r: 10, g: 100, b: 200 })
        .x_label_format(textplots::LabelFormat::Custom(Box::new(move |val| {format!("{}", start + Duration::days(val as i64))})))
        .y_label_format(textplots::LabelFormat::Value)
        .display();

}

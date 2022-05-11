use rgb::RGB8;
use textplots::{Chart, ColorPlot, Shape};

fn main() {
    // Plot a sparse set of points with colors

    let points = vec![
        (2.3, 2.1),
        (1.5, 2.1),
        (3.6, 0.20),
        (2.4, 0.22),
        (1.9, 2.9),
        (3.2, 3.0),
        (0.38, 1.2),
        (2.6, 0.45),
        (0.040, 2.9),
        (1.5, 1.8),
        (1.4, 0.41),
        (3.6, 3.3),
        (0.80, 0.83),
        (2.7, 1.5),
        (1.6, 1.8),
        (2.6, 0.70),
        (3.8, 0.64),
        (2.7, 2.7),
        (1.7, 2.3),
        (1.9, 0.87),
        (1.8, 2.4),
        (1.2, 2.1),
        (3.7, 1.6),
        (0.32, 3.7),
        (0.26, 3.2),
        (3.9, 2.4),
        (2.0, 1.6),
        (0.37, 1.6),
        (0.53, 0.71),
        (1.2, 3.4),
    ];
    let sparse_points = Shape::Points(points.as_slice());

    let mut plot = Chart::new(60, 40, 0., 3.);
    let point_color = RGB8 {
        // Red
        r: 255,
        g: 0,
        b: 0,
    };
    let dots = plot.linecolorplot(&sparse_points, point_color);
    dots.display();
}

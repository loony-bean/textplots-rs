use rgb::RGB8;
use textplots::{Chart, ColorPlot, Shape};

fn main() {
    // You can plot several functions on the same chart.
    // However the resolution of text displays is low, and the result might not be great.
    println!("\ny = cos(x), y = sin(x) / 2");
    Chart::new(180, 60, -5.0, 5.0)
        .linecolorplot(
            &Shape::Continuous(Box::new(|x| x.cos())),
            RGB8 {
                r: 255_u8,
                g: 0,
                b: 0,
            },
        )
        .linecolorplot(
            &Shape::Continuous(Box::new(|x| x.sin() / 2.0)),
            RGB8 { r: 0, g: 0, b: 255 },
        )
        .display();

    println!("\nRainbow");
    let min_radius = 5.0;
    let ring_gap = 0.5;
    let segment_count = 20;
    let num_rings = 6;

    let mut rings = vec![];
    for _ in 0..num_rings {
        rings.push(vec![]);
    }

    for i in 0..segment_count {
        let angle: f32 = ((std::f64::consts::PI as f32 / 2.0) / segment_count as f32) * i as f32;
        let angle_sin = angle.sin() as f32;
        let angle_cos = angle.cos() as f32;

        for j in 0..num_rings {
            rings[j].push((
                (min_radius + (ring_gap * j as f32)) * angle_cos,
                (min_radius + (ring_gap * j as f32)) * angle_sin,
            ));
        }
    }

    for i in 0..num_rings {
        let mut ring_copy = rings[i].clone();
        ring_copy.reverse();
        for coord in ring_copy.iter_mut() {
            coord.0 = 0.0 - coord.0;
        }
        rings[i].append(&mut ring_copy);
    }

    let max_radius = min_radius + ((num_rings as f32 + 1.0) * ring_gap);
    Chart::new(180, 60, 0.0 - max_radius, max_radius)
        .linecolorplot(
            &Shape::Lines(rings[5].as_ref()),
            RGB8 {
                // Red
                r: 255,
                g: 0,
                b: 0,
            },
        )
        .linecolorplot(
            &Shape::Lines(rings[4].as_ref()),
            RGB8 {
                // Orange
                r: 255,
                g: 165,
                b: 0,
            },
        )
        .linecolorplot(
            &Shape::Lines(rings[3].as_ref()),
            RGB8 {
                // Yellow
                r: 255,
                g: 255,
                b: 0,
            },
        )
        .linecolorplot(
            &Shape::Lines(rings[2].as_ref()),
            RGB8 {
                // Green
                r: 0,
                g: 255,
                b: 0,
            },
        )
        .linecolorplot(
            &Shape::Lines(rings[1].as_ref()),
            RGB8 {
                // Blue
                r: 0,
                g: 0,
                b: 255,
            },
        )
        .linecolorplot(
            &Shape::Lines(rings[0].as_ref()),
            RGB8 {
                // Violet
                r: 136,
                g: 43,
                b: 226,
            },
        )
        .display();
}

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use textplots::{Chart, ColorPlot, Shape};

const PRINT_LEN: usize = 100;
const PURPLE: rgb::RGB8 = rgb::RGB8::new(0xE0, 0x80, 0xFF);
const RED: rgb::RGB8 = rgb::RGB8::new(0xFF, 0x00, 0x00);
const GREEN: rgb::RGB8 = rgb::RGB8::new(0x00, 0xFF, 0x00);

const RUN_DURATION_S: u64 = 5;

fn main() {
    let should_run = Arc::new(AtomicBool::new(true));
    let should_run_ctrlc_ref = should_run.clone();

    let mut x: [(f32, f32); PRINT_LEN] = [(0., 0.); PRINT_LEN];
    let mut y: [(f32, f32); PRINT_LEN] = [(0., 0.); PRINT_LEN];
    let mut z: [(f32, f32); PRINT_LEN] = [(0., 0.); PRINT_LEN];

    // hide the cursor so we don't see it flying all over
    let term = console::Term::stdout();
    term.hide_cursor().unwrap();
    term.clear_screen().unwrap();

    // On ctrl+C, reset terminal settings and let the thread know to stop
    ctrlc::set_handler(move || {
        should_run_ctrlc_ref
            .as_ref()
            .store(false, Ordering::Relaxed);
    })
    .unwrap();

    // run until we get ctrl+C or timeout
    let mut time: f32 = 0.;
    let start_time = std::time::SystemTime::now();
    while should_run.as_ref().load(Ordering::Acquire)
        && start_time.elapsed().unwrap().as_secs() < RUN_DURATION_S
    {
        // update our plotting data
        let x_val = time.sin();
        let y_val = (time + std::f32::consts::FRAC_PI_3).sin();
        let z_val = (time + 2. * std::f32::consts::FRAC_PI_3).sin();
        x.copy_within(1..PRINT_LEN, 0);
        y.copy_within(1..PRINT_LEN, 0);
        z.copy_within(1..PRINT_LEN, 0);
        x[PRINT_LEN - 1] = (0., x_val as f32);
        y[PRINT_LEN - 1] = (0., y_val as f32);
        z[PRINT_LEN - 1] = (0., z_val as f32);
        for index in 0..PRINT_LEN {
            x[index].0 += 1.;
            y[index].0 += 1.;
            z[index].0 += 1.;
        }

        time += std::f32::consts::PI / 50.;

        // update our plot
        term.move_cursor_to(0, 0).unwrap();
        Chart::new_with_y_range(200, 100, 0., PRINT_LEN as f32, -1.5, 1.5)
            .linecolorplot(&Shape::Lines(&x), RED)
            .linecolorplot(&Shape::Lines(&y), GREEN)
            .linecolorplot(&Shape::Lines(&z), PURPLE)
            .display();

        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    // re-reveal the cursor
    let term = console::Term::stdout();
    term.show_cursor().unwrap();
}

//! Plot φ-encoded vs original signal using `plotters`
//! Run with: cargo run --example plot_signal

use hybrid_phi::signal::{generate_sine_wave, phi_encode_signal, phi_decode_signal};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 10;
    let len = 128;

    let original = generate_sine_wave(len, 3.0, 0.0);
    let encoded = phi_encode_signal(&original, n);
    let decoded = phi_decode_signal(&encoded, n);

    let root = BitMapBackend::new("phi_signal.png", (800, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = 1.1;
    let min_y = -1.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Original vs φ-decoded Signal", ("sans-serif", 20))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..len, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        original.iter().enumerate().map(|(i, y)| (i, *y)),
        &BLUE,
    ))?.label("Original").legend(|(x, y)| PathElement::new([(x, y), (x + 15, y)], &BLUE));

    chart.draw_series(LineSeries::new(
        decoded.iter().enumerate().map(|(i, y)| (i, *y)),
        &RED,
    ))?.label("φ-decoded").legend(|(x, y)| PathElement::new([(x, y), (x + 15, y)], &RED));

    chart.configure_series_labels().border_style(&BLACK).draw()?;

    println!("Saved plot to phi_signal.png");
    Ok(())
}

use plotters::prelude::*;
use crate::stellar::EvolutionSnapshot;

// HR Diagram: Effective Temperature vs. Luminosity
pub fn plot_hr_diagram(snapshots: &Vec<EvolutionSnapshot>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("hr_diagram.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Determine the ranges for effective temperature and luminosity.
    let min_teff = snapshots.iter().map(|s| s.effective_temp).fold(f64::INFINITY, |a, b| a.min(b));
    let max_teff = snapshots.iter().map(|s| s.effective_temp).fold(0.0_f64, |a, b| a.max(b));
    let min_lum  = snapshots.iter().map(|s| s.luminosity).fold(f64::INFINITY, |a, b| a.min(b));
    let max_lum  = snapshots.iter().map(|s| s.luminosity).fold(0.0_f64, |a, b| a.max(b));

    // To reverse the effective temperature axis, we swap the bounds.
    let x_range = (max_teff..min_teff);
    
    let mut chart = ChartBuilder::on(&root)
        .caption("HR Diagram", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_range, min_lum..max_lum)?;

    chart.configure_mesh()
        .x_desc("Effective Temperature (K)")
        .y_desc("Luminosity (W)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        snapshots.iter().map(|s| (s.effective_temp, s.luminosity)),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}

// Central Evolution: Central Temperature vs. Time
pub fn plot_central_evolution(snapshots: &Vec<EvolutionSnapshot>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("central_evolution.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_time = snapshots.first().unwrap().time;
    let max_time = snapshots.last().unwrap().time;
    let min_temp = snapshots.iter().map(|s| s.central_temp).fold(f64::INFINITY, |a, b| a.min(b));
    let max_temp = snapshots.iter().map(|s| s.central_temp).fold(0.0_f64, |a, b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("Central Temperature Evolution", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_time..max_time, min_temp..max_temp)?;

    chart.configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Central Temperature (K)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        snapshots.iter().map(|s| (s.time, s.central_temp)),
        &BLUE,
    ))?;

    root.present()?;
    Ok(())
}

// Radial Profiles: Plot Temperature vs. Radius from a given snapshot.
pub fn plot_radial_profiles(snapshot: &EvolutionSnapshot) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("radial_profile.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_radius = snapshot.radial_profile.iter().map(|z| z.radius).fold(f64::INFINITY, |a, b| a.min(b));
    let max_radius = snapshot.radial_profile.iter().map(|z| z.radius).fold(0.0_f64, |a, b| a.max(b));
    let min_temp = snapshot.radial_profile.iter().map(|z| z.temp).fold(f64::INFINITY, |a, b| a.min(b));
    let max_temp = snapshot.radial_profile.iter().map(|z| z.temp).fold(0.0_f64, |a, b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("Radial Temperature Profile", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_radius..max_radius, min_temp..max_temp)?;

    chart.configure_mesh()
        .x_desc("Radius (m)")
        .y_desc("Temperature (K)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        snapshot.radial_profile.iter().map(|z| (z.radius, z.temp)),
        &GREEN,
    ))?;

    root.present()?;
    Ok(())
}

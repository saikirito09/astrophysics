mod constants;
mod stellar;
mod eos;
mod nuclear;
mod solver;
mod evolution;
mod visualization;

use stellar::{Star, Zone, Composition};
use evolution::evolve_star;
use visualization::{plot_hr_diagram, plot_central_evolution, plot_radial_profiles};

fn main() {
    // Initialize the stellar model with a series of zones.
    let n_zones = 100;
    let total_radius = 6.957e8; // Solar radius (m)
    let dr = total_radius / n_zones as f64;
    
    let mut zones = Vec::new();
    for i in 0..n_zones {
        let r = i as f64 * dr;
        zones.push(Zone {
            r: r,
            mass: if i == 0 { 0.0 } else { 1.0e28 },  // Dummy mass (kg)
            density: 1.0e5,                           // Dummy density (kg/m³)
            pressure: 1.0e16,                         // Dummy pressure (Pa)
            temp: 1.5e7,                              // Dummy temperature (K)
            luminosity: if i == 0 { 0.0 } else { 1.0e26 }, // Dummy luminosity (W)
            composition: Composition {
                hydrogen: 0.70,
                helium: 0.28,
            },
        });
    }
    let mut star = Star { zones };

    // Evolve the star.
    let t_end = 1.0e7;    // End time (s)
    let dt_initial = 1.0e4; // Initial time step (s)
    let snapshots = evolve_star(&mut star, t_end, dt_initial);

    // Generate visualizations.
    if let Err(e) = plot_hr_diagram(&snapshots) {
        println!("Error plotting HR Diagram: {}", e);
    }
    if let Err(e) = plot_central_evolution(&snapshots) {
        println!("Error plotting Central Temperature Evolution: {}", e);
    }
    if let Some(last_snapshot) = snapshots.last() {
        if let Err(e) = plot_radial_profiles(last_snapshot) {
            println!("Error plotting Radial Profiles: {}", e);
        }
    }
    
    println!("Simulation and visualization complete.");
}

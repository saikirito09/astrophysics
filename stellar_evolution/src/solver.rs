use crate::constants::*;
use crate::stellar::Star;
use std::f64::consts::PI;

/// Finite difference relaxation method to update the stellar structure.
pub fn finite_diff_relaxation(star: &mut Star, tol: f64, max_iter: usize) -> Result<(), String> {
    for _iter in 0..max_iter {
        let mut max_diff: f64 = 0.0;

        for j in 0..star.zones.len() - 1 {
            let dr = star.zones[j + 1].r - star.zones[j].r;
            // Mass conservation: dM/dr = 4πr²ρ
            let new_mass = star.zones[j].mass + 4.0 * PI * star.zones[j].r.powi(2) * star.zones[j].density * dr;
            // Hydrostatic equilibrium: dP/dr = - G * M * ρ / r²
            let new_pressure = star.zones[j].pressure - (G * star.zones[j].mass * star.zones[j].density / star.zones[j].r.powi(2)) * dr;
            // Energy generation (dummy ε value)
            let epsilon = 1.0e-4;
            let new_luminosity = star.zones[j].luminosity + 4.0 * PI * star.zones[j].r.powi(2) * star.zones[j].density * epsilon * dr;
            // Energy transport (radiative diffusion)
            let kappa = 0.34;
            let a_rad = 4.0 * STEFAN_BOLTZMANN / SPEED_OF_LIGHT;
            let new_temp = star.zones[j].temp - (3.0 * kappa * star.zones[j].density * star.zones[j].luminosity)
                           / (16.0 * PI * a_rad * SPEED_OF_LIGHT * star.zones[j].temp.powi(3) * star.zones[j].r.powi(2)) * dr;

            // Compute differences for convergence.
            let mass_diff = (new_mass - star.zones[j + 1].mass).abs();
            let pressure_diff = (new_pressure - star.zones[j + 1].pressure).abs();
            let luminosity_diff = (new_luminosity - star.zones[j + 1].luminosity).abs();
            let temp_diff = (new_temp - star.zones[j + 1].temp).abs();

            max_diff = max_diff.max(mass_diff)
                               .max(pressure_diff)
                               .max(luminosity_diff)
                               .max(temp_diff);

            // Update the next zone.
            star.zones[j + 1].mass = new_mass;
            star.zones[j + 1].pressure = new_pressure;
            star.zones[j + 1].luminosity = new_luminosity;
            star.zones[j + 1].temp = new_temp;
        }

        if max_diff < tol {
            return Ok(());
        }
    }
    Err("Finite difference relaxation did not converge".to_string())
}

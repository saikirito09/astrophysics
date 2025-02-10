use crate::constants::*;
use crate::nuclear::{egr, update_composition};
use crate::solver::finite_diff_relaxation;
use crate::stellar::{Star, EvolutionSnapshot, ZoneData};
use std::f64::consts::PI;

/// Evolves the stellar model in time using adaptive time-stepping.
pub fn evolve_star(star: &mut Star, t_end: f64, dt_initial: f64) -> Vec<EvolutionSnapshot> {
    let mut snapshots = Vec::new();
    let mut time = 0.0;
    let mut dt = dt_initial;

    while time < t_end {
        // Update each zone using the nuclear reaction network.
        for zone in &mut star.zones {
            let epsilon = egr(zone.density, zone.temp, zone.composition.hydrogen);
            zone.luminosity += epsilon * dt;
            zone.composition = update_composition(&zone.composition, epsilon, dt);
        }

        // Update the stellar structure.
        match finite_diff_relaxation(star, 1.0e-6, 1000) {
            Ok(_) => (),
            Err(e) => {
                println!("Solver error at time {}: {}", time, e);
                break;
            }
        }

        // Record a snapshot of the current state.
        let radial_profile = star.zones.iter().map(|zone| ZoneData {
            radius: zone.r,
            temp: zone.temp,
            density: zone.density,
            pressure: zone.pressure,
        }).collect();

        let central_temp = if !star.zones.is_empty() { star.zones[0].temp } else { 0.0 };
        let luminosity = if !star.zones.is_empty() {
            star.zones[star.zones.len() - 1].luminosity
        } else {
            0.0
        };

        let effective_temp = if !star.zones.is_empty() {
            let radius = star.zones.last().unwrap().r;
            (luminosity / (4.0 * PI * radius.powi(2) * STEFAN_BOLTZMANN)).powf(0.25)
        } else {
            0.0
        };

        snapshots.push(EvolutionSnapshot {
            time,
            central_temp,
            luminosity,
            effective_temp,
            radial_profile,
        });

        // Adaptive time-stepping.
        let target_change = 1.0e-3;
        let central_temp_change = if snapshots.len() >= 2 {
            let prev_temp = snapshots[snapshots.len() - 2].central_temp;
            (star.zones[0].temp - prev_temp).abs() / prev_temp.max(1.0)
        } else {
            0.0
        };

        if central_temp_change > target_change {
            dt *= 0.5;
        } else if central_temp_change < target_change * 0.5 {
            dt *= 1.5;
        }

        time += dt;
    }

    snapshots
}

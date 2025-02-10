use crate::stellar::Composition;

/// Computes the energy generation rate (ε) for the p-p chain.
/// 
/// ε = ε₀ * ρ * X² * (T/1e7)⁴
pub fn egr(rho: f64, temp: f64, X: f64) -> f64 {
    let t7 = temp / 1.0e7;
    let epsilon_0 = 1.07e-7; 
    epsilon_0 * rho * X * X * t7.powi(4)
}

/// Updates the composition of a stellar zone based on the energy generation rate.
pub fn update_composition(composition: &Composition, epsilon: f64, dt: f64) -> Composition {
    let q = 6.3e18;
    let delta_x = epsilon * dt / q;
    let new_hydrogen = if composition.hydrogen > delta_x {
        composition.hydrogen - delta_x
    } else {
        0.0
    };
    let new_helium = composition.helium + (composition.hydrogen - new_hydrogen);

    Composition {
        hydrogen: new_hydrogen,
        helium: new_helium,
    }
}

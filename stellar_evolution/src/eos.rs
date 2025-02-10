use crate::constants::*;

// calculate the ideal gas pressure in a stellar zone
pub fn ideal_gas_pressure(rho: f64, temp: f64, mu:f64) -> f64 {
    (rho * BOLTZMANN * temp) / (mu * MASS_UNIT)
}

// calculate the radiation pressure
pub fn radiation_pressure(temp: f64) -> f64 {
    let a = 4.0 * STEFAN_BOLTZMANN / SPEED_OF_LIGHT;
    (1.0 / 3.0) * a * temp.powi(4)
}

// total pressure of the ideal gas pressure and radiation pressure
pub fn total_pressure(rho: f64, temp: f64, mu:f64) -> f64{
    ideal_gas_pressure(rho, temp, mu) + radiation_pressure(temp)
}

// specific internal energy
pub fn specific_internal_energy(temp: f64, mu:f64) -> f64{
    (3.0 / 2.0) * (BOLTZMANN * temp) / (mu * MASS_UNIT)
}
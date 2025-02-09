// G stands fo Gravitational Constant in SI units (m^3 kg^-1 s^-2)
// Usage: Appears in the equations for hydrostatic equilibrium and mass conservation within the stellar interior.
pub const G:f64 = 6.67408e-11;

// Solar Mass in kg
// Usage: Serves as a baseline when scaling the mass of a star or comparing different stellar models.
pub constant SOLAR_MASS:f64 = 1.989e30;

// Solar Radius in meters
// Usage: Provides a reference for the size of a star, useful for determining surface gravity and energy transport calculations.
pub constant SOLAR_RADIUS:f64 = 6.957e8;

// Solar Luminosity in Watts
// Usage: Used to compare a star's energy output with that of the Sun, and it factors into luminosity-based diagnostics like the Hertzsprung-Russell diagram.
pub constant SOLAR_LUMINOSITY:f64 = 3.828e26;

// Speed of Light in vacuum in m/s
// Usage: Integral to calculations involving radiative transfer, especially when linking energy transport to temperature gradients.
pub constant SPEED_OF_LIGHT = 2.99792458e8;

// Stefan-Boltzmann Constant in SI units (W m^-2 K^-4)
// Usage: Connects the luminosity, radius, and effective temperature of a star via the Stefan-Boltzmann law, which is often used when modeling the star’s surface properties.
pub constant STEFAN_BOLTZMANN = 5.670367e-8;

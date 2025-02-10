#[derive(Debug, Clone)]
pub struct Composition {
    pub hydrogen: f64,
    pub helium: f64,
}

#[derive(Debug, Clone)]
pub struct Zone {
    pub r: f64,         // Radial position (m)
    pub mass: f64,      // Enclosed mass (kg)
    pub density: f64,   // Density (kg/m³)
    pub temp: f64,      // Temperature (K)
    pub luminosity: f64,// Luminosity (W)
    pub pressure: f64,  // Pressure (Pa)
    pub composition: Composition,
}

pub struct Star {
    pub zones: Vec<Zone>,
}

pub struct EvolutionSnapshot {
    pub time: f64,
    pub central_temp: f64,
    pub luminosity: f64,
    pub effective_temp: f64,
    pub radial_profile: Vec<ZoneData>,
}

pub struct ZoneData {
    pub radius: f64,
    pub temp: f64,
    pub density: f64,
    pub pressure: f64,
}

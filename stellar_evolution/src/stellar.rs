#[derive(Debug, Clone)]
pub struct Composition {
    pub hydrogen: f64,
    pub helium: f64,
}

#[derive(Debug,Clone)]
pub struct Zone{
    pub r:f64,
    pub mass:f64,
    pub density:f64,
    pub temp:f64,
    pub luminosity: f64,
    pub pressure: f64,
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
    pub radial_profile: Vec<Zone>,
}

pub struct Zone{
    pub radius: f64,
    pub temp: f64,
    pub density: f64,
    pub pressure: f64,
}
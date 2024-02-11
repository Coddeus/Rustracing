use std::f64::consts::PI;

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.
}

pub fn rad_to_deg(radians: f64) -> f64 {
    radians / PI * 180.
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: nalgebra::Vector3<f64>,
    pub direction: nalgebra::Vector3<f64>,
}

impl Ray {
    pub fn new(origin: nalgebra::Vector3<f64>, direction: nalgebra::Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f64) -> nalgebra::Vector3<f64> {
        self.origin + self.direction * t
    }
}
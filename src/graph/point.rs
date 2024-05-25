pub struct GraphPoint {
    coordinates: (f64, f64, f64),
}

impl GraphPoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let coordinates = (x, y, z);
        GraphPoint { coordinates }
    }

    pub fn x(&self) -> f64 {
        self.coordinates.0
    }

    pub fn y(&self) -> f64 {
        self.coordinates.1
    }

    pub fn z(&self) -> f64 {
        self.coordinates.2
    }
}

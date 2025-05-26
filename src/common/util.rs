pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        // Default to empty Interval
        let min = f64::INFINITY;
        let max = f64::NEG_INFINITY;
        Self::new(min, max)
    }
}

impl Interval {

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    const EMPTY : Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    const UNIVERSE : Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
}

pub struct Interval {
    pub min: f32,
    pub max: f32
}

impl Interval {
    pub fn new() -> Self {
        Interval {
            min: -f32::INFINITY,
            max: f32::INFINITY,
        }
    }

    pub fn with_min_max(min: f32, max: f32) -> Self {
        Interval {
            min,
            max,
        }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        match x {
            x if x > self.max => self.max,
            x if x < self.min => self.min,
            x => x

        }
    }

    const EMPTY: Interval = Interval{ min: f32::INFINITY, max: -f32::INFINITY };
    const UNIVERSE: Interval = Interval{ min: -f32::INFINITY, max: f32::INFINITY };
}
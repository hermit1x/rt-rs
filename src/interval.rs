#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Interval {
    pub start: f64,
    pub end: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Interval {
    // Create an interval with explicit bounds
    pub const fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }

    pub fn size(&self) -> f64 {
        self.end - self.start
    }

    pub fn contains(&self, x: f64) -> bool {
        self.start <= x && x <= self.end
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.start < x && x < self.end
    }

    // Static-like constants analogous to C++ `static const` members
    pub const EMPTY: Interval = Interval { start: f64::INFINITY, end: f64::NEG_INFINITY };
    pub const UNIVERSE: Interval = Interval { start: f64::NEG_INFINITY, end: f64::INFINITY };
}


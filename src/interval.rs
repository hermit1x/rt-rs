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
    pub const fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }
    
    pub const fn merge(a: &Interval, b: &Interval) -> Self {
        let start = if a.start < b.start { a.start } else { b.start };
        let end = if a.end > b.end { a.end } else { b.end };
        Self { start, end }
    }

    pub const EMPTY: Interval = Interval {
        start: f64::INFINITY,
        end: f64::NEG_INFINITY,
    };
    pub const UNIVERSE: Interval = Interval {
        start: f64::NEG_INFINITY,
        end: f64::INFINITY,
    };

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.start {
            return self.start;
        }
        if x > self.end {
            return self.end;
        }
        x
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(
            self.start - padding,
            self.end + padding,
        )
    }
}

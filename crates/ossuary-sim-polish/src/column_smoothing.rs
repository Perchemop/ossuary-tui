//! Per-column inertia smoothing for rain trails.
//!
//! Provides a pure function that smooths a column's head movement to reduce
//! jitter while keeping the core engine untouched.

/// Simple one-pole smoother for column head positions.
pub struct ColumnSmoothing {
    alpha: f32,
}

impl ColumnSmoothing {
    pub fn new(alpha: f32) -> Self {
        Self { alpha: alpha.clamp(0.0, 1.0) }
    }

    /// Smooth `new_val` towards `prev` using exponential moving average.
    pub fn smooth(&self, prev: f32, new_val: f32) -> f32 {
        prev + self.alpha * (new_val - prev)
    }
}

#[cfg(test)]
mod tests {
    use super::ColumnSmoothing;
    #[test]
    fn smooth_converges() {
        let s = ColumnSmoothing::new(0.5);
        let a = s.smooth(0.0, 10.0);
        assert!((a - 5.0).abs() < 1e-6);
    }
}

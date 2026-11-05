//! A tiny, deterministic pseudo-random generator.
//!
//! Determinism matters: the rain is a simulation, and CI must be able to
//! reproduce a frame exactly from a seed (guide §M.5). The generator is
//! therefore small, dependency-free and identical on every platform.

/// A xorshift64* generator.
#[derive(Debug, Clone)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Create a generator from a seed. A zero seed is remapped so the state
    /// never collapses to the all-zero fixed point.
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
        }
    }

    /// The next 64-bit value.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// The next value as `f32` in `[0, 1)`.
    pub fn next_f32(&mut self) -> f32 {
        // The top 24 bits give a uniform value in a single rounding.
        let bits = (self.next_u64() >> 40) as u32;
        bits as f32 / (1u32 << 24) as f32
    }

    /// A uniform integer in `[0, n)`. Returns 0 when `n == 0`.
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next_u64() % n as u64) as usize
        }
    }

    /// A uniform value in `[low, high)`.
    pub fn range_f32(&mut self, low: f32, high: f32) -> f32 {
        low + (high - low) * self.next_f32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_gives_same_sequence() {
        let mut a = Rng::new(12345);
        let mut b = Rng::new(12345);
        for _ in 0..64 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn zero_seed_does_not_collapse() {
        let mut rng = Rng::new(0);
        let first = rng.next_u64();
        let second = rng.next_u64();
        assert_ne!(first, 0);
        assert_ne!(first, second);
    }

    #[test]
    fn floats_stay_in_unit_interval() {
        let mut rng = Rng::new(7);
        for _ in 0..1000 {
            let value = rng.next_f32();
            assert!((0.0..1.0).contains(&value));
        }
    }

    #[test]
    fn below_is_bounded_and_zero_safe() {
        let mut rng = Rng::new(99);
        for _ in 0..1000 {
            assert!(rng.below(7) < 7);
        }
        assert_eq!(rng.below(0), 0);
    }
}

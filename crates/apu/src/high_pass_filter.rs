#[derive(Debug, Default)]
pub struct HighPassFilter {
    prev_in_l: f32,
    prev_out_l: f32,
    prev_in_r: f32,
    prev_out_r: f32,
}

impl HighPassFilter {
    const A: f32 = 0.999715;

    pub fn new() -> Self {
        Self {
            prev_in_l: 0.0,
            prev_out_l: 0.0,
            prev_in_r: 0.0,
            prev_out_r: 0.0,
        }
    }

    /// Apply simple 1-pole high-pass filter to remove DC / low-freq hum.
    ///
    /// First-order (one-pole) high-pass filter on both channels.
    /// y[n] = a * (y[n-1] + x[n] ‑ x[n-1])
    /// where `a` is close to 1.0 and defines the cut-off frequency.
    pub fn apply(&mut self, left: f32, right: f32) -> (f32, f32) {
        let out_l = Self::A * (self.prev_out_l + left - self.prev_in_l);
        self.prev_in_l = left;
        self.prev_out_l = out_l;

        let out_r = Self::A * (self.prev_out_r + right - self.prev_in_r);
        self.prev_in_r = right;
        self.prev_out_r = out_r;

        (out_l, out_r)
    }
}

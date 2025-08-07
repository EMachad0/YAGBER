#[derive(Debug, Default)]
pub struct HighPassFilter {
    a: f32,
    prev_in_l: f32,
    prev_out_l: f32,
    prev_in_r: f32,
    prev_out_r: f32,
}

impl HighPassFilter {
    pub fn new(sample_rate_hz: u32) -> Self {
        let mut s = Self::default();
        s.set_sample_rate(sample_rate_hz);
        s
    }

    fn coefficient_for(sample_rate_hz: u32) -> f32 {
        // First-order HPF coefficient: a = exp(-2*pi*fc/Fs)
        // Use a gentle cutoff to remove DC and low hum without coloring audio
        const FC_HZ: f32 = 40.0;
        let fs = sample_rate_hz.max(1) as f32;
        (-2.0 * std::f32::consts::PI * FC_HZ / fs).exp()
    }

    pub fn set_sample_rate(&mut self, sample_rate_hz: u32) {
        self.a = Self::coefficient_for(sample_rate_hz);
    }

    /// Apply simple 1-pole high-pass filter to remove DC / low-freq hum.
    ///
    /// First-order (one-pole) high-pass filter on both channels.
    /// y[n] = a * (y[n-1] + x[n] ‑ x[n-1]) where `a` depends on sample rate
    pub fn apply(&mut self, left: f32, right: f32) -> (f32, f32) {
        let out_l = self.a * (self.prev_out_l + left - self.prev_in_l);
        self.prev_in_l = left;
        self.prev_out_l = out_l;

        let out_r = self.a * (self.prev_out_r + right - self.prev_in_r);
        self.prev_in_r = right;
        self.prev_out_r = out_r;

        (out_l, out_r)
    }
}

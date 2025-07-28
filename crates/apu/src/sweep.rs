use crate::{Apu, channels::PulseChannel};

#[derive(Debug, Default)]
pub struct Sweep {
    pub enabled: bool,
    pub sweep_timer: u8,
    pub shadow_register: u16,
}

impl Sweep {
    pub fn new() -> Self {
        Self {
            enabled: false,
            sweep_timer: 0,
            shadow_register: 0,
        }
    }

    fn trigger(&mut self, bus: &mut yagber_memory::Bus) {
        let aud_1_sweep = yagber_memory::Aud1Sweep::from_bus(bus);
        let period = PulseChannel::get_initial_period(bus, yagber_memory::AudioChannel::Ch1);
        self.enabled = aud_1_sweep.pace() > 0 || aud_1_sweep.step() > 0;
        self.sweep_timer = aud_1_sweep.pace();
        self.shadow_register = period;

        let new_period = self.frequency_calculation(&aud_1_sweep);
        if self.overflow_check(new_period) {
            self.disable(bus);
        }
    }

    pub(crate) fn tick(&mut self, bus: &mut yagber_memory::Bus) {
        if !self.enabled {
            return;
        }
        self.sweep_timer = self.sweep_timer.wrapping_sub(1);
        if self.sweep_timer != 0 {
            return;
        }
        let aud_1_sweep = yagber_memory::Aud1Sweep::from_bus(bus);
        self.sweep_timer = aud_1_sweep.pace();

        let new_period = self.frequency_calculation(&aud_1_sweep);
        if self.overflow_check(new_period) {
            self.disable(bus);
            return;
        }
        self.shadow_register = new_period;

        let period_low = new_period as u8;
        let period_high = (new_period >> 8) as u8;
        bus.write(yagber_memory::IOType::AUD1LOW.address(), period_low);
        let old_high = bus.read(yagber_memory::IOType::AUD1HIGH.address());
        let new_high = old_high & !0x07 | period_high;
        bus.write(yagber_memory::IOType::AUD1HIGH.address(), new_high);

        let new_period = self.frequency_calculation(&aud_1_sweep);
        if self.overflow_check(new_period) {
            self.disable(bus);
        }
    }

    fn frequency_calculation(&self, aud_1_sweep: &yagber_memory::Aud1Sweep) -> u16 {
        let period = self.shadow_register;
        let delta = period >> aud_1_sweep.step();
        let new_period = match aud_1_sweep.direction() {
            yagber_memory::SweepDirection::Increase => period.saturating_add(delta),
            yagber_memory::SweepDirection::Decrease => period.saturating_sub(delta),
        };
        new_period
    }

    fn overflow_check(&self, new_period: u16) -> bool {
        new_period > 0x7FF
    }

    fn disable(&mut self, bus: &mut yagber_memory::Bus) {
        self.enabled = false;
        let audena = bus.read(yagber_memory::IOType::AUDENA.address());
        let new_audena = audena & !yagber_memory::AudioChannel::Ch1.audena_bit();
        bus.write(yagber_memory::IOType::AUDENA.address(), new_audena);
    }

    pub(crate) fn on_aud_1_high_write(apu: &mut Apu, bus: &mut yagber_memory::Bus, value: u8) {
        let aud_1_high = yagber_memory::Aud1High::new(value);
        if aud_1_high.trigger_enabled() {
            apu.sweep.trigger(bus);
        }
    }

    pub(crate) fn on_aud_1_sweep_write(apu: &mut Apu, value: u8) {
        let aud_1_sweep = yagber_memory::Aud1Sweep::new(value);
        if !aud_1_sweep.enabled() {
            apu.sweep.enabled = false;
        } else {
            if !apu.sweep.enabled {
                apu.sweep.enabled = true;
                apu.sweep.sweep_timer = aud_1_sweep.pace();
            }
        }
    }
}

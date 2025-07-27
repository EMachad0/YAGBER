use crate::channels::Ch1;

#[derive(Debug, Default)]
pub struct Apu {
    cycles: u8,
    pub ch1: Ch1,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            ch1: Ch1::new(),
        }
    }

    pub(crate) fn on_tcycle(emulator: &mut yagber_app::Emulator) {
        let (apu, bus) = emulator
            .get_components_mut2::<Self, yagber_memory::Bus>()
            .expect("Apu and/or Bus components not found");

        apu.tick(bus);
    }

    /// Ticks the APU.
    /// Meant to be called every T-cycle.
    fn tick(&mut self, bus: &mut yagber_memory::Bus) {
        let audena = yagber_memory::Audena::from_bus(bus);
        if !audena.apu_enabled() {
            return;
        }

        self.cycles += 1;
        if self.cycles == 4 {
            self.cycles = 0;
        }
        if self.cycles != 0 {
            return;
        }

        if audena.ch_enabled(yagber_memory::AudioChannel::Ch1) {
            self.ch1.tick(bus);
        }
    }

    pub(crate) fn tick_sound_length(&mut self, bus: &mut yagber_memory::Bus) {
        let audena_value = bus
            .io_registers
            .read(yagber_memory::IOType::AUDENA.address());
        let audena = yagber_memory::Audena::new(audena_value);
        let mut new_audena = audena_value;

        if audena.ch_enabled(yagber_memory::AudioChannel::Ch1) {
            let length_counter = self.ch1.decrement_length_counter();
            if length_counter == 0 {
                new_audena &= !yagber_memory::AudioChannel::Ch1.audena_bit();
            }
        }

        bus.io_registers
            .write(yagber_memory::IOType::AUDENA.address(), new_audena);
    }

    pub(crate) fn tick_envelope(&mut self, bus: &mut yagber_memory::Bus) {
        let audena = yagber_memory::Audena::from_bus(bus);

        if audena.ch_enabled(yagber_memory::AudioChannel::Ch1) {
            let audenv = yagber_memory::Audenv::ch1(bus);
            let envelope_change = self.ch1.envelope.tick(&audenv);
            self.ch1.change_volume(envelope_change);
        }
    }
}

impl yagber_app::Component for Apu {}

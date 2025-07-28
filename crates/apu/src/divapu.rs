use yagber_app::EdgeDetector;
use yagber_memory::Bus;

use crate::Apu;

pub struct DivApu {
    ticks: u8,
    edge_detector: EdgeDetector,
}

impl DivApu {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            edge_detector: EdgeDetector::new(yagber_app::EdgeMode::Falling),
        }
    }

    pub(crate) fn on_mcycle(emulator: &mut yagber_app::Emulator) {
        let (div_apu, bus) = emulator
            .get_components_mut2::<DivApu, Bus>()
            .expect("DivApu and/or Bus component missing");
        if !div_apu.should_tick(bus) {
            return;
        }

        let ticks = div_apu.tick();

        let (apu, bus) = emulator
            .get_components_mut2::<Apu, Bus>()
            .expect("Apu and/or Bus component missing");

        if ticks == 7 {
            apu.tick_envelope(bus);
        }

        if ticks % 2 == 0 {
            apu.tick_sound_length(bus);
        }

        if ticks % 4 == 0 {
            apu.sweep.tick(bus);
        }
    }

    fn tick(&mut self) -> u8 {
        self.ticks += 1;
        if self.ticks >= 8 {
            self.ticks = 0;
        }
        self.ticks
    }

    fn should_tick(&mut self, bus: &Bus) -> bool {
        let speed_mode = yagber_memory::Spd::from_bus(bus);
        let div_mask = self.div_mask(speed_mode.speed_mode());
        let div = bus.io_registers.read(yagber_memory::IOType::DIV.address());
        self.edge_detector.tick((div & div_mask) != 0)
    }

    /// To make sure div-apu works on 512Hz.
    fn div_bit(&self, speed_mode: yagber_app::SpeedMode) -> u8 {
        match speed_mode {
            yagber_app::SpeedMode::Single => 4,
            yagber_app::SpeedMode::Double => 5,
        }
    }

    fn div_mask(&self, speed_mode: yagber_app::SpeedMode) -> u8 {
        1 << self.div_bit(speed_mode)
    }
}

impl yagber_app::Component for DivApu {}

use yagber_memory::{Bus, IOType};

pub struct Hdma {
    active: bool,
    paused: bool,
    blocks: u8,
    last_stat_mode: u8,
}

impl Hdma {
    pub fn new() -> Self {
        Self {
            active: false,
            paused: false,
            blocks: 0,
            last_stat_mode: 0xFF,
        }
    }

    fn transfer(bus: &mut Bus, len: u16) {
        let src = Self::get_hdma_src(bus);
        let dst = Self::get_hdma_dst(bus);

        for i in 0..len {
            let value = bus.read(src + i);
            bus.write(dst + i, value);
        }
    }

    fn get_hdma_src(bus: &Bus) -> u16 {
        let hdma_src_hi = bus.io_registers.read(IOType::HdmaSrcHi.address());
        let hdma_src_lo = bus.io_registers.read(IOType::HdmaSrcLo.address());
        let src = u16::from_be_bytes([hdma_src_hi, hdma_src_lo]);
        src & 0xFFF0
    }

    fn get_hdma_dst(bus: &Bus) -> u16 {
        let hdma_dst_hi = bus.io_registers.read(IOType::HdmaDstHi.address());
        let hdma_dst_lo = bus.io_registers.read(IOType::HdmaDstLo.address());
        let dst = u16::from_be_bytes([hdma_dst_hi, hdma_dst_lo]);
        let dst = dst & 0x1FF0;
        dst | 0x8000
    }

    pub(crate) fn on_hdma_len_write(&mut self, bus: &mut Bus, value: u8) {
        let mode = value & 0x80 != 0;
        let len = value & 0x7F;
        let blocks = len / 0x10 - 1;
        tracing::debug!("Hdma: mode={} len={} blocks={}", mode, len, blocks);
        if self.active {
            if !mode {
                self.paused = true;
            }
        } else {
            match mode {
                false => {
                    Self::transfer(bus, len as u16);
                }
                true => {
                    self.active = true;
                    self.paused = false;
                    self.blocks = blocks;
                }
            }
        }
    }

    pub(crate) fn hdma_len_reader(&mut self, _value: u8) -> u8 {
        match self.active {
            false => 0xFF,
            true => self.blocks | 0x80,
        }
    }

    pub(crate) fn on_stat_write(&mut self, bus: &mut Bus, value: u8) {
        let stat = yagber_memory::Stat::new(value);
        let just_entered_hblank = self.last_stat_mode != 0 && stat.mode() == 0;
        self.last_stat_mode = stat.mode();
        if !self.active || self.paused || !just_entered_hblank {
            return;
        }

        Self::transfer(bus, 0x10);
        if self.blocks == 0 {
            self.active = false;
        } else {
            self.blocks -= 1;
        }
    }
}

impl yagber_app::Component for Hdma {}

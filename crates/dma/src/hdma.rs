use yagber_memory::{Bus, IOType};

pub struct Hdma {
    active: bool,
    paused: bool,
    blocks: u8,
    last_stat_mode: u8,
    src: u16,
    dst: u16,
}

impl Hdma {
    pub fn new() -> Self {
        Self {
            active: false,
            paused: false,
            blocks: 0,
            last_stat_mode: 0xFF,
            src: 0,
            dst: 0,
        }
    }

    fn transfer(bus: &mut Bus, src: u16, dst: u16, len: u16) {
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
        let hblank_mode = value & 0x80 != 0;
        let blocks = (value & 0x7F) as u8;

        tracing::debug!(
            "HDMA write: hblank_mode={} blocks={} ({} bytes)",
            hblank_mode,
            blocks + 1,
            ((blocks as u16) + 1) * 0x10
        );

        if self.active {
            if !hblank_mode {
                self.paused = true;
            }
            return;
        }

        let src = Self::get_hdma_src(bus);
        let dst = Self::get_hdma_dst(bus);

        if hblank_mode {
            // HBlank DMA
            self.active = true;
            self.paused = false;
            self.blocks = blocks;
            self.src = src;
            self.dst = dst;
        } else {
            // Immediate GDMA
            let bytes = ((blocks as u16) + 1) * 0x10;
            Self::transfer(bus, src, dst, bytes);
            bus.io_registers
                .write_unhooked(IOType::HdmaLen.address(), 0xFF);
        }
    }

    pub(crate) fn on_stat_write(&mut self, bus: &mut Bus, value: u8) {
        let stat = yagber_memory::Stat::new(value);
        let just_entered_hblank = self.last_stat_mode != 0 && stat.mode() == 0;
        self.last_stat_mode = stat.mode();
        if !self.active || self.paused || !just_entered_hblank {
            return;
        }

        Self::transfer(bus, self.src, self.dst, 0x10);
        if self.blocks == 0 {
            self.active = false;
            bus.io_registers
                .write_unhooked(IOType::HdmaLen.address(), 0xFF);
        } else {
            self.src = self.src.wrapping_add(0x10);
            self.dst = self.dst.wrapping_add(0x10);
            self.blocks -= 1;
            let status = (self.blocks & 0x7F) | 0x80;
            bus.io_registers
                .write_unhooked(IOType::HdmaLen.address(), status);
        }
    }
}

impl yagber_app::Component for Hdma {}

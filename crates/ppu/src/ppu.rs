use yagber_memory::{Bus, IOType, LcdcRegister};

use crate::ppu_mode::PpuMode;

#[derive(Debug, Default, Clone, Copy)]
pub struct Ppu {
    x: u16, // 0-456
    y: u8,  // 0-153
}

impl Ppu {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn on_dot_cycle(emulator: &mut yagber_app::Emulator) {
        let (bus, ppu) = emulator
            .get_components_mut2::<Bus, Ppu>()
            .expect("Bus and/or PPU component missing");

        if !Ppu::enabled(bus) {
            return;
        }

        // Step the PPU even if there's no display, so that the scan line index is updated and interrupt is requested if necessary.
        ppu.step(bus);

        // If the ppu just finished to draw a frame, we need to render it.
        if ppu.y == 144 && ppu.x == 0 {
            Self::render_frame(emulator);
        }
    }

    fn render_frame(emulator: &mut yagber_app::Emulator) {
        // If the display component is not present, there's nowhere to render to.
        if !emulator.has_component::<yagber_display::Display>() {
            return;
        }
        let (bus, display) = emulator
            .get_components_mut2::<Bus, yagber_display::Display>()
            .expect("Bus and/or Display component missing");

        let lcdc = LcdcRegister::from_bus(bus);
        let tile_fetcher_mode = lcdc.tile_data_area();
        let bg_addr_mode = lcdc.bg_tile_map_area();
        let bg_tile_map = bus.vram.tile_map(bg_addr_mode);
        let gb_attr_map = bus.vram.attr_map(bg_addr_mode);

        let tile_addr = {
            if tile_fetcher_mode {
                |tile_index: u8| {
                    let tile_index = tile_index as u16;
                    0x8000u16 + tile_index * 16
                }
            } else {
                |tile_index: u8| {
                    let tile_index = tile_index as i8 as i32;
                    (0x9000i32 + tile_index * 16) as u16
                }
            }
        };

        let mut pixels = [[0; 4]; 256 * 256];

        for i in 0..32 {
            for j in 0..32 {
                let tile_index = bg_tile_map[i * 32 + j].expect("Tile index is missing");
                let tile_addr = tile_addr(tile_index);
                let tile_attr = gb_attr_map[i * 32 + j].expect("Tile attribute is missing");
                let tile = crate::models::Tile::from_memory(bus, tile_addr, tile_attr);

                for y in 0..8 {
                    for x in 0..8 {
                        let colour_index = tile.colour_index(x as u8, y as u8);
                        let palette_index = tile.attr.palette_index();
                        let colour_raw =
                            bus.background_cram.read_colour(palette_index, colour_index);
                        let colour = crate::models::Rgb555::from_u16(colour_raw);
                        let pixel = crate::models::Rgba::from(colour);

                        let pixel_index = (i * 8 + y) * 256 + (j * 8 + x);

                        pixels[pixel_index] = pixel.values();
                    }
                }
            }
        }

        let frame_buffer = display.frame_buffer();
        for (i, pixel) in frame_buffer.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&pixels[i]);
        }
        display.request_redraw();
    }

    pub fn step(&mut self, bus: &mut Bus) {
        self.x += 1;
        if self.x >= 456 {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= 154 {
            self.y = 0;
        }

        // Vblank interrupt at the start of the Vblank period.
        if self.y == 144 && self.x == 0 {
            bus.request_interrupt(yagber_memory::InterruptType::VBlank);
        }

        Self::set_scan_line_index(bus, self.y);
        Self::set_mode(bus, self.mode());
    }

    pub fn enabled(bus: &Bus) -> bool {
        let lcdc = LcdcRegister::from_bus(bus);
        lcdc.lcd_ppu_enabled()
    }

    fn set_scan_line_index(bus: &mut Bus, index: u8) {
        match index {
            0..=143 => Ppu::set_mode(bus, PpuMode::OamScan),
            144..=153 => Ppu::set_mode(bus, PpuMode::VBlank),
            _ => panic!("Invalid scan line index: {}", index),
        }
        bus.write(IOType::LY.address(), index);
    }

    fn mode(&self) -> PpuMode {
        match self.y {
            0..=143 => match self.x {
                0..=80 => PpuMode::OamScan,
                81..=252 => PpuMode::PixelTransfer,
                253..=456 => PpuMode::HBlank,
                _ => panic!("Invalid x index: {}", self.x),
            },
            144..=153 => PpuMode::VBlank,
            _ => panic!("Invalid y scan line index: {}", self.y),
        }
    }

    fn set_mode(bus: &mut Bus, mode: PpuMode) {
        let stat = bus.io_registers.read(IOType::STAT.address());
        let new_stat = (stat & !0x03) | mode.to_u8();
        bus.io_registers
            .write_unchecked(IOType::STAT.address(), new_stat);
    }
}

impl yagber_app::Component for Ppu {}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use yagber_memory::Memory;

    #[test]
    fn test_enabled() {
        let mut bus = Bus::new();
        bus.write(IOType::LCDC.address(), 0x80);

        assert!(Ppu::enabled(&bus));

        bus.write(IOType::LCDC.address(), 0x00);

        assert!(!Ppu::enabled(&bus));
    }

    #[test]
    fn step() {
        let mut bus = Bus::new();
        bus.write(IOType::LCDC.address(), 0x80);

        let mut ppu = Ppu::new();
        ppu.step(&mut bus);

        assert_eq!(ppu.y, 0);
        assert_eq!(ppu.x, 1);
    }

    #[test]
    fn ppu_timing_dots_per_scan_line() {
        let mut bus = Bus::new();
        bus.write(IOType::LCDC.address(), 0x80);

        let mut ppu = Ppu::new();
        assert_eq!(ppu.y, 0);
        assert_eq!(ppu.x, 0);

        for i in 0..154 {
            assert_eq!(ppu.y, i);
            assert_eq!(ppu.x, 0);
            for _ in 0..456 {
                ppu.step(&mut bus);
            }
        }
    }

    #[test]
    fn ppu_timing_dots_per_frame() {
        let mut bus = Bus::new();
        bus.write(IOType::LCDC.address(), 0x80);

        let mut ppu = Ppu::new();
        assert_eq!(ppu.y, 0);
        assert_eq!(ppu.x, 0);

        for _ in 0..70224 {
            ppu.step(&mut bus);
        }

        assert_eq!(ppu.y, 0);
        assert_eq!(ppu.x, 0);
    }

    #[test]
    fn ppu_vblank_interrupt() {
        let mut bus = Bus::new();
        bus.write(IOType::LCDC.address(), 0x80);

        let mut ppu = Ppu::new();

        for _ in 0..65664 {
            ppu.step(&mut bus);
        }

        assert_eq!(ppu.y, 144);
        assert_eq!(ppu.x, 0);

        assert!(bus.read_bit(
            IOType::IF.address(),
            yagber_memory::InterruptType::VBlank.to_u8()
        ));
    }
}

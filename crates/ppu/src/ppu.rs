use yagber_display::Display;
use yagber_memory::{Bus, IOType, LcdcRegister, TileFetcherMode};

use crate::ppu_mode::PpuMode;

#[derive(Debug)]
pub struct Ppu {
    x: u16, // 0-456
    y: u8,  // 0-153
    frame_buffer: [[u8; 4]; 256 * 256],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            frame_buffer: [[0; 4]; 256 * 256],
        }
    }

    pub fn on_dot_cycle(emulator: &mut yagber_app::Emulator) {
        let has_display = emulator.has_component::<yagber_display::Display>();
        let (bus, ppu) = emulator
            .get_components_mut2::<Bus, Ppu>()
            .expect("Bus and/or PPU component missing");

        if !Ppu::enabled(bus) {
            return;
        }

        // Step the PPU even if there's no display, so that the scan line index is updated and interrupt is requested if necessary.
        ppu.step(bus);

        let frame_just_finished = ppu.y == 144 && ppu.x == 0;

        // If the display component is not present, there's nowhere to render to.
        // If the ppu just finished to draw a frame
        if has_display && frame_just_finished {
            // Render the rest of the frame for debug purposes
            ppu.render_outer_frame(bus);
            // Render the frame to the display
            let (ppu, display) = emulator
                .get_components_mut2::<Ppu, Display>()
                .expect("PPU and/or Display component missing");
            ppu.render_frame(display);
        }
    }

    fn render_pixel(&mut self, x: u8, y: u8, bus: &Bus) {
        let lcdc = LcdcRegister::from_bus(bus);
        let tile_fetcher_mode = lcdc.tile_data_area();
        let bg_addr_mode = lcdc.bg_tile_map_area();
        let bg_tile_map = bus.vram.tile_map(bg_addr_mode);
        let bg_attr_map = bus.vram.attr_map(bg_addr_mode);

        let tile_map_index = (y as usize / 8) * 32 + (x as usize / 8);
        let tile_index = bg_tile_map[tile_map_index].expect("Tile index is missing");
        let tile_address = match tile_fetcher_mode {
            TileFetcherMode::TileDataArea1 => 0x8000u16 + (tile_index as u16) * 16,
            TileFetcherMode::TileDataArea0 => (0x9000i32 + (tile_index as i8 as i32) * 16) as u16,
        };
        let tile_attr = bg_attr_map[tile_map_index].expect("Tile attribute is missing");
        let tile = crate::models::Tile::from_memory(bus, tile_address, tile_attr);

        let colour_index = tile.colour_index(x % 8, y % 8);

        let palette_index = tile.attr.palette_index();
        let colour_raw = bus.background_cram.read_colour(palette_index, colour_index);
        let colour = crate::models::Rgb555::from_u16(colour_raw);
        let colour_rgba = crate::models::Rgba::from(colour);

        let pixel_index = y as usize * 256 + x as usize;
        self.frame_buffer[pixel_index] = colour_rgba.values();
    }

    fn render_outer_frame(&mut self, bus: &Bus) {
        let lcdc = LcdcRegister::from_bus(bus);
        let tile_fetcher_mode = lcdc.tile_data_area();
        let bg_addr_mode = lcdc.bg_tile_map_area();
        let bg_tile_map = bus.vram.tile_map(bg_addr_mode);
        let bg_attr_map = bus.vram.attr_map(bg_addr_mode);

        let get_tile_addr = match tile_fetcher_mode {
            TileFetcherMode::TileDataArea1 => |tile_index: u8| {
                let tile_index = tile_index as u16;
                0x8000u16 + tile_index * 16
            },
            TileFetcherMode::TileDataArea0 => |tile_index: u8| {
                let tile_index = tile_index as i8 as i32;
                (0x9000i32 + tile_index * 16) as u16
            },
        };

        let tile_addresses = {
            let mut addresses = [0; 32 * 32];
            for i in 0..32 {
                for j in 0..32 {
                    let tile_index = bg_tile_map[i * 32 + j].expect("Tile index is missing");
                    let tile_addr = get_tile_addr(tile_index);
                    addresses[i * 32 + j] = tile_addr;
                }
            }
            addresses
        };

        let tile_attrs = {
            let mut attrs = [0; 32 * 32];
            for i in 0..32 {
                for j in 0..32 {
                    attrs[i * 32 + j] = bg_attr_map[i * 32 + j].expect("Tile attribute is missing");
                }
            }
            attrs
        };

        let tiles = {
            (0..32 * 32)
                .map(|i| {
                    let tile_addr = tile_addresses[i];
                    let tile_attr = tile_attrs[i];
                    crate::models::Tile::from_memory(bus, tile_addr, tile_attr)
                })
                .collect::<Vec<_>>()
        };

        let mut pixels = Vec::with_capacity(256 * 256);

        for chunk in tiles.chunks_exact(32) {
            for y in 0..8 {
                for tile in chunk {
                    let palette_index = tile.attr.palette_index();
                    let row = tile.get_pixel_row(y);
                    for colour_index in row {
                        let colour_raw =
                            bus.background_cram.read_colour(palette_index, colour_index);
                        let colour = crate::models::Rgb555::from_u16(colour_raw);
                        let colour_rgba = crate::models::Rgba::from(colour);
                        pixels.push(colour_rgba.values());
                    }
                }
            }
        }

        let scy = bus.io_registers.read(IOType::SCY.address());
        let scx = bus.io_registers.read(IOType::SCX.address());
        let left_x = scx;
        let top_y = scy;
        let bottom_y = top_y.wrapping_add(143);
        let right_x = left_x.wrapping_add(159);

        let frame_buffer = &mut self.frame_buffer;
        for (i, pixel) in frame_buffer.iter_mut().enumerate() {
            let x = (i % 256) as u8;
            let y = (i / 256) as u8;
            if left_x <= x && x <= right_x && top_y <= y && y <= bottom_y {
                continue;
            }
            pixel.copy_from_slice(&pixels[i]);
        }
        Self::add_border(frame_buffer, scy, scx);
    }

    fn add_border(frame_buffer: &mut [[u8; 4]], scy: u8, scx: u8) {
        let mut paint_pixel = |i: usize, j: usize| {
            let y = (i + scy as usize) % 256;
            let x = (j + scx as usize) % 256;
            let pixel_index = y * 256 + x;
            frame_buffer[pixel_index] = [255, 0, 0, 255];
        };
        for i in [0, 143] {
            for j in 0..160 {
                paint_pixel(i, j);
            }
        }
        for j in [0, 159] {
            for i in 0..144 {
                paint_pixel(i, j);
            }
        }
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

        if self.mode() == PpuMode::PixelTransfer {
            let scy = bus.io_registers.read(IOType::SCY.address());
            let scx = bus.io_registers.read(IOType::SCX.address());
            let x = (self.x as u8 - 81).wrapping_add(scx);
            let y = self.y.wrapping_add(scy);
            self.render_pixel(x, y, bus);
        }

        // Vblank interrupt at the start of the Vblank period.
        if self.y == 144 && self.x == 0 {
            bus.request_interrupt(yagber_memory::InterruptType::VBlank);
        }

        Self::set_scan_line_index(bus, self.y);
        Self::set_mode(bus, self.mode());
    }

    fn render_frame(&self, display: &mut Display) {
        for (i, pixel) in display.frame_buffer().chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&self.frame_buffer[i]);
        }
        display.request_redraw();
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

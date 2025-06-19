use yagber_memory::{Bus, Memory};

use crate::{ppu_mode::PpuMode, scan_line::ScanLine};

#[derive(Debug, Default, Clone, Copy)]
pub struct Ppu {
    scan_line: ScanLine,
}

impl Ppu {
    pub const SCAN_LINE_ADDRESS: u16 = 0xFF44;
    pub const LCD_STATUS_ADDRESS: u16 = 0xFF41;
    pub const LCD_CONTROL_ADDRESS: u16 = 0xFF40;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_dot_cycle(emulator: &mut yagber_app::Emulator, _event: &yagber_app::DotCycleEvent) {
        let (bus, ppu) = emulator
            .get_components_mut2::<Bus, Ppu>()
            .expect("Bus and/or PPU component missing");

        if !Ppu::enabled(bus) {
            return;
        }

        // Step the PPU even if there's no display, so that the scan line index is updated and interrupt is requested if necessary.
        ppu.step(bus);

        // If the ppu just finished to draw a frame, we need to render it.
        if Ppu::scan_line_index(bus) != 144 || ppu.scan_line.dots() != 0 {
            return;
        }

        // If the display component is not present, there's nowhere to render to.
        if !emulator.has_component::<yagber_display::Display>() {
            return;
        }
        let (bus, display) = emulator
            .get_components_mut2::<Bus, yagber_display::Display>()
            .expect("Bus and/or Display component missing");

        let tile_fetcher_mode = bus.read_bit(Self::LCD_CONTROL_ADDRESS, 4);
        let bg_addr_mode = bus.read_bit(Self::LCD_CONTROL_ADDRESS, 3);
        let bg_addr = if bg_addr_mode { 0x9C00 } else { 0x9800 };

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

        let mut changed = false;
        let mut pixels = [[0; 4]; 256 * 256];

        for i in 0..32 {
            for j in 0..32 {
                let tile_index = bus.read(bg_addr + (i * 32 + j));
                let tile_addr = tile_addr(tile_index);
                let tile = crate::tile::Tile::from_memory(bus, tile_addr);

                for y in 0..8 {
                    for x in 0..8 {
                        let pixel = tile.get_pixel(x as u8, y as u8);
                        let pixel = match pixel {
                            0b00 => [0, 0, 0, 255],       // Black
                            0b01 => [255, 255, 255, 255], // White
                            0b10 => [255, 0, 0, 255],     // Red
                            0b11 => [0, 255, 0, 255],     // Green
                            _ => unreachable!("Invalid pixel colour: {}", pixel),
                        };
                        let pixel_index = (i * 8 + y) * 256 + (j * 8 + x);

                        pixels[pixel_index as usize] = pixel;
                    }
                }
            }
        }

        let frame_buffer = display.frame_buffer();
        for (i, pixel) in frame_buffer.chunks_exact_mut(4).enumerate() {
            if changed || pixels[i] != *pixel {
                changed = true;
                pixel.copy_from_slice(&pixels[i]);
            }
        }
        if changed {
            display.request_redraw();
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        self.scan_line.step(bus);
        if self.scan_line.finished() {
            let scan_line_index = Self::scan_line_index(bus);
            if scan_line_index >= 153 {
                Self::set_scan_line_index(bus, 0);
            } else {
                Self::set_scan_line_index(bus, scan_line_index + 1);
            }
            self.scan_line = ScanLine::new();
        }
    }

    pub fn enabled(bus: &Bus) -> bool {
        let lcd_control = bus.read(Self::LCD_CONTROL_ADDRESS);
        (lcd_control & 0x80) != 0
    }

    pub fn scan_line_index(bus: &mut Bus) -> u8 {
        bus.read(Self::SCAN_LINE_ADDRESS)
    }

    pub fn set_scan_line_index(bus: &mut Bus, index: u8) {
        match index {
            0..=143 => Ppu::set_mode(bus, PpuMode::OamScan),
            144..=153 => Ppu::set_mode(bus, PpuMode::VBlank),
            _ => panic!("Invalid scan line index: {}", index),
        }
        bus.write(Self::SCAN_LINE_ADDRESS, index);
    }

    pub fn get_mode(bus: &mut Bus) -> PpuMode {
        let mode = bus.read_masked(Ppu::LCD_STATUS_ADDRESS, 0x03);
        match mode {
            0 => PpuMode::OamScan,
            1 => PpuMode::PixelTransfer,
            2 => PpuMode::HBlank,
            3 => PpuMode::VBlank,
            _ => unreachable!(),
        }
    }

    pub fn set_mode(bus: &mut Bus, mode: PpuMode) {
        match mode {
            PpuMode::OamScan => {}
            PpuMode::PixelTransfer => {}
            PpuMode::HBlank => {}
            PpuMode::VBlank => {
                trace!("VBlank interrupt");
                bus.request_interrupt(yagber_memory::InterruptType::VBlank);
            }
        }
        bus.write_masked(Ppu::LCD_STATUS_ADDRESS, mode.to_u8(), 0x03);
    }
}

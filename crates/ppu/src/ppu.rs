use yagber_display::Display;
use yagber_memory::{
    Bus, IOType, LcdcRegister, OpriRegister, SysRegister, TileFetcherMode, TileSize,
};

use crate::{
    models::{FifoPixel, FifoPixelType, Object, Tile},
    ppu_mode::PpuMode,
};

const FRAME_BUFFER_WIDTH: usize = 160;
const FRAME_BUFFER_HEIGHT: usize = 144;
const FRAME_BUFFER_SIZE: usize = FRAME_BUFFER_WIDTH * FRAME_BUFFER_HEIGHT;

#[derive(Debug)]
pub struct Ppu {
    x: u16, // 0-456
    y: u8,  // 0-153
    frame_buffer: [[u8; 4]; FRAME_BUFFER_SIZE],
    objects: Vec<Object>,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            frame_buffer: [[0; 4]; FRAME_BUFFER_SIZE],
            objects: Vec::new(),
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

        let just_entered_oam_scan = ppu.y < 144 && ppu.x == 0;
        let just_entered_vblank = ppu.y == 144 && ppu.x == 0;

        if just_entered_oam_scan {
            ppu.objects = ppu.object_scan(bus, ppu.y);
        }

        // Step the PPU even if there's no display, so that the scan line index is updated and interrupt is requested if necessary.
        ppu.step(bus);

        // If the display component is not present, there's nowhere to render to.
        // If the ppu just finished to draw a frame
        if has_display && just_entered_vblank {
            // Render the frame to the display
            let (ppu, display) = emulator
                .get_components_mut2::<Ppu, Display>()
                .expect("PPU and/or Display component missing");
            ppu.render_frame(display);
        }
    }

    fn render_pixel(&mut self, x: u8, y: u8, bus: &Bus) {
        let bg_pixel = self.background_pixel(x, y, bus);
        let obj_pixel = self.object_pixel(x, y, bus);

        let fifo_pixel = match (bg_pixel, obj_pixel) {
            (Some(bg_pixel), None) => bg_pixel,
            (None, Some(obj_pixel)) => obj_pixel,
            (Some(bg_pixel), Some(obj_pixel)) => {
                Self::solve_bg_obj_priority(bus, bg_pixel, obj_pixel)
            }
            (None, None) => return,
        };

        self.render_fifo_pixel(fifo_pixel, bus, x, y);
    }

    fn solve_bg_obj_priority(bus: &Bus, bg_pixel: FifoPixel, obj_pixel: FifoPixel) -> FifoPixel {
        let lcdc = LcdcRegister::from_bus(bus);
        if bg_pixel.colour_index() == 0
            || !lcdc.bg_window_enabled_priority()
            || (!bg_pixel.priority() && !obj_pixel.priority())
        {
            obj_pixel
        } else {
            bg_pixel
        }
    }

    fn render_fifo_pixel(&mut self, pixel: FifoPixel, bus: &Bus, x: u8, y: u8) {
        let sys = SysRegister::from_bus(bus);
        let (pallet_index, colour_index) = match sys.mode() {
            yagber_memory::SysMode::Dmg => {
                let pallet_index = pixel.palette_index().value();
                let dmg_pallet_addr = match pixel.pixel_type() {
                    FifoPixelType::Background => IOType::BGP.address(),
                    FifoPixelType::Object => match pallet_index {
                        0 => IOType::OBP0.address(),
                        1 => IOType::OBP1.address(),
                        _ => panic!("Invalid palette index: {}", pallet_index),
                    },
                };
                let pallet = bus.io_registers.read(dmg_pallet_addr);
                let dmg_pallet = crate::models::DmgPallet::new(pallet);
                let colour_index = dmg_pallet.colour_index(pixel.colour_index());
                (pallet_index, colour_index)
            }
            yagber_memory::SysMode::Cgb => (pixel.palette_index().value(), pixel.colour_index()),
        };
        let cram = match pixel.pixel_type() {
            FifoPixelType::Background => &bus.background_cram,
            FifoPixelType::Object => &bus.object_cram,
        };
        let colour_raw = cram.read_colour(pallet_index, colour_index);
        let colour = crate::models::Rgb555::from_u16(colour_raw);
        let colour_rgba = crate::models::Rgba::from(colour);
        let pixel_index = y as usize * FRAME_BUFFER_WIDTH + x as usize;
        self.frame_buffer[pixel_index] = colour_rgba.values();
    }

    fn background_pixel(&mut self, x: u8, y: u8, bus: &Bus) -> Option<FifoPixel> {
        let sys = SysRegister::from_bus(bus);
        let lcdc = LcdcRegister::from_bus(bus);
        if sys.mode() == yagber_memory::SysMode::Dmg && !lcdc.bg_window_enabled_priority() {
            return None;
        }

        let scy = bus.io_registers.read(IOType::SCY.address());
        let scx = bus.io_registers.read(IOType::SCX.address());
        let x = x.wrapping_add(scx);
        let y = y.wrapping_add(scy);

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
        let palette_index = tile.attr.cgb_palette();
        let priority = tile.attr.priority();
        let pixel_type = FifoPixelType::Background;
        let fifo_pixel = FifoPixel::new(colour_index, palette_index, priority, pixel_type);
        Some(fifo_pixel)
    }

    fn object_pixel(&mut self, x: u8, y: u8, bus: &Bus) -> Option<FifoPixel> {
        let lcdc = LcdcRegister::from_bus(bus);
        if !lcdc.obj_enabled() {
            return None;
        }
        let sys = SysRegister::from_bus(bus);
        let object = self.find_priority_object(x, y, bus);
        object.map(|object| {
            let colour_index = self.object_colour_index(bus, &object, x, y).unwrap();
            let palette_index = match sys.mode() {
                yagber_memory::SysMode::Dmg => object.attr().dmg_palette(),
                yagber_memory::SysMode::Cgb => object.attr().cgb_palette(),
            };
            let priority = object.attr().priority();
            let pixel_type = FifoPixelType::Object;
            FifoPixel::new(colour_index, palette_index, priority, pixel_type)
        })
    }

    fn find_priority_object(&self, x: u8, y: u8, bus: &Bus) -> Option<Object> {
        let opri = OpriRegister::from_bus(bus);

        let mut priority_object = None;
        for object in &self.objects {
            let Some(colour_index) = self.object_colour_index(bus, object, x, y) else {
                continue;
            };

            // If the object is transparent, skip it.
            if colour_index == 0 {
                continue;
            }

            priority_object = match priority_object {
                None => Some(object),
                Some(other_object) => match opri.mode() {
                    yagber_memory::OpriMode::Cgb => Some(other_object),
                    yagber_memory::OpriMode::Dmg => {
                        if object.x() < other_object.x() {
                            Some(object)
                        } else {
                            Some(other_object)
                        }
                    }
                },
            }
        }
        priority_object.cloned()
    }

    /// Returns the colour index of the object at the given pixel.
    /// Returns None if the object does not intersect with the pixel.
    fn object_colour_index(&self, bus: &Bus, object: &Object, x: u8, y: u8) -> Option<u8> {
        let intersects = object.x() <= x + 8 && x < object.x();
        if !intersects {
            return None;
        }

        let lcdc = LcdcRegister::from_bus(bus);
        let tile_index = match lcdc.obj_size() {
            TileSize::TileSize8 => object.tile_index_8(),
            TileSize::TileSize16 => {
                let (tile_index_top, tile_index_bottom) = object.tile_index_16();
                if (object.y() as u16) < (y as u16 + 8) {
                    tile_index_top
                } else {
                    tile_index_bottom
                }
            }
        };

        let tile_y = (y + 16 - object.y()) % 8;
        let tile_x = x + 8 - object.x();

        let tile_address = 0x8000u16 + (tile_index as u16) * 16;
        let tile = Tile::from_memory(bus, tile_address, object.attr().value());
        let colour_index = tile.colour_index(tile_x, tile_y);

        Some(colour_index)
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
            let x = self.x as u8 - 80;
            if x < FRAME_BUFFER_WIDTH as u8 {
                let y = self.y;
                self.render_pixel(x, y, bus);
            }
        }

        // Vblank interrupt at the start of the Vblank period.
        if self.y == 144 && self.x == 0 {
            bus.request_interrupt(yagber_memory::InterruptType::VBlank);
        }

        if self.x == 0 {
            Self::set_scan_line_index(bus, self.y);
        }
        Self::set_mode(bus, self.mode());
    }

    fn render_frame(&self, display: &mut Display) {
        for (i, pixel) in display.frame_buffer().chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&self.frame_buffer[i]);
        }
        display.request_redraw();
    }

    fn object_scan(&self, bus: &Bus, scan_line: u8) -> Vec<Object> {
        let lcdc = LcdcRegister::from_bus(bus);
        let obj_size = lcdc.obj_size();

        let scan_line_filter = |object_y: u8| {
            let bottom_y = object_y;
            let top_y = bottom_y.wrapping_add(obj_size.as_u8());
            let scan_line_y = scan_line.wrapping_add(16);
            bottom_y <= scan_line_y && scan_line_y < top_y
        };

        let oam = bus.oam.data();
        oam.chunks_exact(4)
            .map(Object::from_bytes)
            .filter(|object| scan_line_filter(object.y()))
            .take(10)
            .collect::<Vec<_>>()
    }

    pub fn enabled(bus: &Bus) -> bool {
        let lcdc = LcdcRegister::from_bus(bus);
        lcdc.lcd_ppu_enabled()
    }

    fn set_scan_line_index(bus: &mut Bus, index: u8) {
        bus.write(IOType::LY.address(), index);
    }

    fn mode(&self) -> PpuMode {
        match self.y {
            0..=143 => match self.x {
                0..80 => PpuMode::OamScan,
                80..252 => PpuMode::PixelTransfer,
                252..456 => PpuMode::HBlank,
                _ => panic!("Invalid x index: {}", self.x),
            },
            144..=153 => PpuMode::VBlank,
            _ => panic!("Invalid y scan line index: {}", self.y),
        }
    }

    fn set_mode(bus: &mut Bus, mode: PpuMode) {
        let stat = bus.io_registers.read(IOType::STAT.address());
        let new_stat = (stat & !0x03) | mode.to_u8();
        if new_stat != stat {
            bus.io_registers
                .write_unchecked(IOType::STAT.address(), new_stat);
        }
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
            yagber_memory::InterruptType::VBlank.bit()
        ));
    }
}

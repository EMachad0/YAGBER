use yagber_memory::Bus;

use crate::ppu_mode::PpuMode;

#[derive(Debug)]
pub struct PpuModeObserver;

impl PpuModeObserver {
    pub fn on_memory_write(
        emulator: &mut yagber_app::Emulator,
        event: &yagber_memory::MemoryWriteEvent,
    ) {
        if event.address == yagber_memory::IOType::STAT.address() {
            let mode = PpuMode::from_u8(event.value);
            let bus = emulator.get_component_mut::<Bus>().unwrap();
            Self::update_accessibility(bus, mode);
        }
    }

    fn update_accessibility(bus: &mut Bus, mode: PpuMode) {
        Self::update_vram_accessibility(bus, mode);
        Self::update_oam_accessibility(bus, mode);
        Self::update_cram_accessibility(bus, mode);
    }

    fn update_vram_accessibility(bus: &mut Bus, mode: PpuMode) {
        bus.vram.set_accessible(mode != PpuMode::PixelTransfer);
    }

    fn update_oam_accessibility(bus: &mut Bus, mode: PpuMode) {
        let accessible = matches!(mode, PpuMode::OamScan | PpuMode::PixelTransfer);
        bus.oam.set_accessible(accessible);
    }

    fn update_cram_accessibility(bus: &mut Bus, mode: PpuMode) {
        let accessible = mode != PpuMode::PixelTransfer;
        bus.background_cram.set_accessible(accessible);
        bus.object_cram.set_accessible(accessible);
    }
}

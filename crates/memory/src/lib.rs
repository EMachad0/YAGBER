mod boot_rom;
mod bus;
mod cartridge;
mod cram;
mod interrupt;
mod io_registers;
mod mbc;
mod memory;
mod oam;
mod ram;
mod register;
mod save;
mod vram;
mod wram;

pub use bus::Bus;
pub use interrupt::InterruptType;
pub use io_registers::*;
pub use memory::Memory;
pub use register::{ByteRegister, Register};

pub struct MemoryPlugin {
    memory_bus: Option<Bus>,
}

impl MemoryPlugin {
    pub fn new() -> Self {
        Self {
            memory_bus: Some(Bus::new()),
        }
    }

    pub fn with_cartridge(mut self, data: &[u8]) -> Self {
        self.memory_bus.as_mut().unwrap().load_rom(data);
        self
    }
}

impl Default for MemoryPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl yagber_app::Plugin for MemoryPlugin {
    fn init(mut self, emulator: &mut yagber_app::Emulator) {
        let memory_bus = std::mem::take(&mut self.memory_bus).unwrap();
        let stat_interrupt_detector = io_registers::StatInterruptDetector::new();

        emulator
            .with_component(memory_bus)
            .with_component(stat_interrupt_detector);

        let stat_ly_hook = emulator.attach_component(io_registers::Stat::on_ly_write);
        let stat_lyc_hook = emulator.attach_component(io_registers::Stat::on_lyc_write);
        let stat_stat_hook =
            emulator.attach_components2(io_registers::StatInterruptDetector::on_stat_write);
        let bcps_bcpd_hook = emulator.attach_component(io_registers::BCPSRegister::on_bcpd_write);
        let ocps_ocpd_hook = emulator.attach_component(io_registers::OCPSRegister::on_ocpd_write);
        let bcpd_bcps_hook = emulator.attach_component(io_registers::BCPDRegister::on_bcps_write);
        let ocpd_ocps_hook = emulator.attach_component(io_registers::OCPDRegister::on_ocps_write);
        let bcpd_reader = emulator.attach_component(io_registers::BCPDRegister::bcpd_reader);
        let ocpd_reader = emulator.attach_component(io_registers::OCPDRegister::ocpd_reader);
        let vram_vbk_hook = emulator.attach_component(vram::Vram::on_vbk_write);
        let wram_svbk_hook = emulator.attach_component(wram::Wram::on_svbk_write);
        let oam_stat_hook = emulator.attach_component(oam::Oam::on_stat_write);
        let vram_stat_hook = emulator.attach_component(vram::Vram::on_stat_write);
        let aud_1_env_hook = emulator.attach_component(io_registers::Audena::on_aud_1_env_write);
        let aud_2_env_hook = emulator.attach_component(io_registers::Audena::on_aud_2_env_write);
        let aud_4_env_hook = emulator.attach_component(io_registers::Audena::on_aud_4_env_write);
        let aud_3_ena_hook = emulator.attach_component(io_registers::Audena::on_aud_3_ena_write);
        let emu_spd_hook = Spd::emu_spd_hook(emulator);
        let aud_1_high_hook = emulator.attach_component(io_registers::Audena::on_aud_1_high_write);
        let aud_2_high_hook = emulator.attach_component(io_registers::Audena::on_aud_2_high_write);
        let aud_3_high_hook = emulator.attach_component(io_registers::Audena::on_aud_3_high_write);
        let aud_4_go_hook = emulator.attach_component(io_registers::Audena::on_aud_4_go_write);

        emulator
            .get_component_mut::<Bus>()
            .expect("Bus component missing")
            .io_registers
            .with_hook(IOType::LY, stat_ly_hook)
            .with_hook(IOType::LYC, stat_lyc_hook)
            .with_transformer(IOType::STAT, io_registers::Stat::stat_transformer)
            .with_hook(IOType::STAT, stat_stat_hook)
            .with_hook(IOType::BCPD, bcps_bcpd_hook)
            .with_hook(IOType::OCPD, ocps_ocpd_hook)
            .with_hook(IOType::BCPS, bcpd_bcps_hook)
            .with_hook(IOType::OCPS, ocpd_ocps_hook)
            .with_reader(IOType::BCPS, bcpd_reader)
            .with_reader(IOType::OCPD, ocpd_reader)
            .with_hook(IOType::VBK, vram_vbk_hook)
            .with_hook(IOType::SVBK, wram_svbk_hook)
            .with_hook(IOType::STAT, oam_stat_hook)
            .with_hook(IOType::STAT, vram_stat_hook)
            .with_transformer(IOType::VBK, io_registers::Vbk::vbk_transformer)
            .with_transformer(IOType::AUDENA, io_registers::Audena::audena_transformer)
            .with_hook(IOType::AUD1ENV, aud_1_env_hook)
            .with_hook(IOType::AUD2ENV, aud_2_env_hook)
            .with_hook(IOType::AUD4ENV, aud_4_env_hook)
            .with_hook(IOType::AUD3ENA, aud_3_ena_hook)
            .with_transformer(IOType::SPD, io_registers::Spd::spd_transformer)
            .with_hook(IOType::SPD, emu_spd_hook)
            .with_hook(IOType::AUD1HIGH, aud_1_high_hook)
            .with_hook(IOType::AUD2HIGH, aud_2_high_hook)
            .with_hook(IOType::AUD3HIGH, aud_3_high_hook)
            .with_hook(IOType::AUD4GO, aud_4_go_hook);
    }
}

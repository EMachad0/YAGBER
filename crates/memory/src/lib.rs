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
mod vram;
mod wram;

pub use bus::Bus;
pub use interrupt::InterruptType;
pub use io_registers::{
    BCPDRegister, BCPSRegister, IOBus, IOType, LcdcRegister, OCPDRegister, OCPSRegister, Stat,
};
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

        let stat_interrupt_detector_ptr = emulator
            .get_component_mut::<io_registers::StatInterruptDetector>()
            .unwrap()
            as *mut io_registers::StatInterruptDetector;

        let memory_bus = emulator.get_component_mut::<Bus>().unwrap();

        // SAFETY: Memory bus stays alive inside the emulator
        let bus_ptr = memory_bus as *mut Bus;

        memory_bus
            .io_registers
            .with_transformer(IOType::JOYP, io_registers::JoypRegister::joyp_transformer)
            .with_hook(IOType::LY, move |value| {
                io_registers::Stat::on_ly_write(unsafe { &mut *bus_ptr }, value);
            })
            .with_hook(IOType::LYC, move |value| {
                io_registers::Stat::on_lyc_write(unsafe { &mut *bus_ptr }, value);
            })
            .with_transformer(IOType::STAT, io_registers::Stat::stat_transformer)
            .with_hook(IOType::STAT, move |value| unsafe {
                (*stat_interrupt_detector_ptr).on_stat_write(&mut *bus_ptr, value);
            })
            .with_hook(IOType::BCPD, move |value| {
                io_registers::BCPSRegister::on_bcpd_write(unsafe { &mut *bus_ptr }, value);
            })
            .with_hook(IOType::OCPD, move |value| {
                io_registers::OCPSRegister::on_ocpd_write(unsafe { &mut *bus_ptr }, value);
            })
            .with_hook(IOType::BCPS, move |value| {
                io_registers::BCPDRegister::on_bcps_write(unsafe { &mut *bus_ptr }, value);
            })
            .with_hook(IOType::OCPS, move |value| {
                io_registers::OCPDRegister::on_ocps_write(unsafe { &mut *bus_ptr }, value);
            })
            .with_reader(IOType::BCPS, move |value| {
                io_registers::BCPDRegister::bcpd_reader(unsafe { &mut *bus_ptr }, value)
            })
            .with_reader(IOType::OCPD, move |value| {
                io_registers::OCPDRegister::ocpd_reader(unsafe { &mut *bus_ptr }, value)
            })
            .with_hook(IOType::VBK, move |value| {
                crate::vram::Vram::on_vbk_write(unsafe { &mut *bus_ptr }, value)
            })
            .with_hook(IOType::SVBK, move |value| {
                crate::wram::Wram::on_svbk_write(unsafe { &mut *bus_ptr }, value)
            })
            .with_hook(IOType::STAT, move |value| {
                crate::oam::Oam::on_stat_write(unsafe { &mut *bus_ptr }, value)
            })
            .with_hook(IOType::STAT, move |value| {
                crate::vram::Vram::on_stat_write(unsafe { &mut *bus_ptr }, value)
            })
            .with_transformer(
                IOType::DIV,
                crate::io_registers::DivRegister::div_transformer,
            );
    }
}

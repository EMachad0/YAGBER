use crate::{Bus, IOType, cram::CramSpecification};

pub struct BCPSRegister {
    specification: CramSpecification,
}

impl BCPSRegister {
    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            specification: CramSpecification::from_bus(bus, IOType::BCPS.address()),
        }
    }

    pub(crate) fn on_bcpd_write(bus: &mut Bus, value: u8) {
        let bcps = BCPSRegister::from_bus(bus);

        bus.background_cram.write_data(&bcps, value);

        if bcps.auto_increment() {
            let new_bcps = bcps.value().wrapping_add(1);
            bus.io_registers.write(IOType::BCPS.address(), new_bcps);
        }
    }
}

impl std::ops::Deref for BCPSRegister {
    type Target = CramSpecification;
    fn deref(&self) -> &Self::Target {
        &self.specification
    }
}

pub struct OCPSRegister {
    specification: CramSpecification,
}

impl OCPSRegister {
    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            specification: CramSpecification::from_bus(bus, IOType::OCPS.address()),
        }
    }

    pub(crate) fn on_ocpd_write(bus: &mut Bus, value: u8) {
        let ocps = OCPSRegister::from_bus(bus);

        bus.object_cram.write_data(&ocps, value);

        if ocps.auto_increment() {
            let new_ocps = ocps.value().wrapping_add(1);
            bus.io_registers.write(IOType::OCPS.address(), new_ocps);
        }
    }
}

impl std::ops::Deref for OCPSRegister {
    type Target = CramSpecification;
    fn deref(&self) -> &Self::Target {
        &self.specification
    }
}

pub struct BCPDRegister;

impl BCPDRegister {
    pub(crate) fn on_bcps_write(bus: &mut Bus, value: u8) {
        let bcps = CramSpecification::new(value);
        let data = bus.background_cram.read_data(&bcps);
        bus.io_registers
            .write_unhooked(IOType::BCPD.address(), data);
    }

    pub(crate) fn bcpd_reader(bus: &mut Bus, value: u8) -> u8 {
        let stat = super::Stat::from_bus(bus);
        let mode = stat.mode();
        if mode == 3 { 0xFF } else { value }
    }
}

pub struct OCPDRegister;

impl OCPDRegister {
    pub(crate) fn on_ocps_write(bus: &mut Bus, value: u8) {
        let ocps = CramSpecification::new(value);
        let data = bus.object_cram.read_data(&ocps);
        bus.io_registers
            .write_unhooked(IOType::OCPD.address(), data);
    }

    pub(crate) fn ocpd_reader(bus: &mut Bus, value: u8) -> u8 {
        let stat = super::Stat::from_bus(bus);
        let mode = stat.mode();
        if mode == 3 { 0xFF } else { value }
    }
}

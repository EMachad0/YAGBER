mod cram_register;
mod div;
mod io_bus;
mod io_register;
mod io_type;
mod joyp;
mod lcdc;
mod opri;
mod stat;
mod sys;

pub use io_bus::IOBus;
pub use io_type::IOType;

pub use cram_register::{BCPDRegister, BCPSRegister, OCPDRegister, OCPSRegister};
pub use div::DivRegister;
pub use joyp::JoypRegister;
pub use lcdc::{LcdcRegister, TileFetcherMode, TileMapArea, TileSize};
pub use opri::{OpriMode, OpriRegister};
pub use stat::{Stat, StatInterruptDetector};
pub use sys::{SysMode, SysRegister};

mod mbc0;
mod mbc1;

pub use mbc0::Mbc0;
pub use mbc1::Mbc1;

pub trait Mbc {
    fn ram_enabled(&self) -> bool;
    fn rom_write(&mut self, address: u16, value: u8);
    fn rom_address(&self, address: u16) -> usize;
    fn ram_address(&self, address: u16) -> usize;
}

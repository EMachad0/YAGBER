pub trait SaveBackend {
    fn read(&self, address: usize) -> u8;
    fn write(&mut self, address: usize, value: u8);
    fn flush(&mut self) -> Result<usize, std::io::Error>;
}

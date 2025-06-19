use std::io::Write;

#[derive(Debug)]
pub enum Destination {
    Buffer(Vec<u8>),
    File(std::fs::File),
    Stdout(std::io::Stdout),
}

impl Destination {
    /// Append data to the destination.
    pub fn write(&mut self, data: u8) -> std::io::Result<()> {
        match self {
            Destination::Buffer(buffer) => {
                buffer.push(data);
                Ok(())
            }
            Destination::File(file) => {
                file.write_all(&[data])?;
                Ok(())
            }
            Destination::Stdout(stdout) => {
                stdout.write_all(&[data])?;
                stdout.write_all(&[b'\n'])?;
                stdout.flush()?;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct DestinationCollector {
    dests: Vec<Destination>,
}

impl DestinationCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_dest(&mut self, dest: Destination) {
        self.dests.push(dest);
    }

    pub fn write(&mut self, data: u8) -> std::io::Result<()> {
        for dest in self.dests.iter_mut() {
            dest.write(data)?;
        }
        Ok(())
    }

    pub fn get_buffer(&self) -> Option<&[u8]> {
        for dest in &self.dests {
            if let Destination::Buffer(buffer) = dest {
                return Some(buffer);
            }
        }
        None
    }
}

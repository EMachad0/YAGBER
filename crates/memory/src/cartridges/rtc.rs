#[derive(Debug, Clone, Copy)]
pub struct Rtc {
    pub registers: RtcRegisters,
    pub last_tick: std::time::Instant,
}

impl Rtc {
    pub fn from_registers(registers: RtcRegisters) -> Self {
        Self { registers, last_tick: std::time::Instant::now() }
    }

    pub fn tick(&mut self) {
        let seconds = self.last_tick.elapsed().as_secs();
        if seconds > 0 {
            if !self.registers.halted() {
                self.registers.advance_by(seconds);
            }
            self.last_tick += std::time::Duration::from_secs(seconds);
        }
    }
}

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct RtcRegisters {
    seconds: u8,
    minutes: u8,
    hours: u8,
    days_low: u8,
    days_high: u8,
}

impl RtcRegisters {
    pub fn advance_by(&mut self, seconds: u64) {
        let total_seconds = seconds + (self.seconds as u64);
        self.seconds = (total_seconds % 60) as u8;
        let total_minutes = (total_seconds / 60) + (self.minutes as u64);
        self.minutes = (total_minutes % 60) as u8;
        let total_hours = (total_minutes / 60) + (self.hours as u64);
        self.hours = (total_hours % 24) as u8;
        let total_days = (total_hours / 24) + (self.days() as u64);

        self.days_low = (total_days & 0xFF) as u8;
        self.days_high |= ((total_days >> 8) & 1) as u8;
        if total_days > 0x1FF {
            self.days_high |= 1 << 7;
        }
    }

    pub fn days(&self) -> u16 {
        let lo = self.days_low;
        let hi = self.days_high & 1;
        u16::from_be_bytes([lo, hi])
    }

    pub fn halted(&self) -> bool {
        (self.days_high & (1 << 6)) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcRegisterKind {
    /// Seconds register.
    Seconds,
    /// Minutes register.
    Minutes,
    /// Hours register.
    Hours,
    /// Days register.
    DaysLow,
    /// Days register (high bit) and Control.
    DaysHigh,
}

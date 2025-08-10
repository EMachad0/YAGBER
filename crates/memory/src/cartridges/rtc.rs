#[derive(Debug, Clone, Copy)]
pub struct Rtc {
    pub registers: RtcRegisters,
    pub last_tick: std::time::Instant,
    latched_registers: Option<RtcRegisters>,
    last_latch_value: u8,
}

impl Rtc {
    pub fn from_registers(registers: RtcRegisters) -> Self {
        Self {
            registers,
            last_tick: std::time::Instant::now(),
            latched_registers: None,
            last_latch_value: 0,
        }
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

    pub fn read_register(&self, kind: RtcRegisterKind) -> u8 {
        if let Some(latched) = self.latched_registers {
            latched.read(kind)
        } else {
            self.registers.read(kind)
        }
    }

    pub fn write_register(&mut self, kind: RtcRegisterKind, value: u8) {
        self.registers.write(kind, value)
    }

    /// Handle writes to the MBC3 latch register (0x6000..=0x7FFF).
    /// A transition from 0 to 1 latches the current time into readable registers.
    pub fn latch_write(&mut self, value: u8) {
        let value = value & 0x01;
        if self.last_latch_value == 0 && value == 1 {
            // Latch snapshot of current time
            self.tick();
            self.latched_registers = Some(self.registers);
        }
        self.last_latch_value = value;
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
        let day_bit_8 = ((total_days >> 8) & 0x01) as u8;

        let halt_bit = self.days_high & (1 << 6);
        let mut carry_bit = self.days_high & (1 << 7);
        if total_days > 0x1FF {
            carry_bit = 1 << 7;
        }
        self.days_high = (day_bit_8) | halt_bit | carry_bit;
    }

    pub fn days(&self) -> u16 {
        let lo = self.days_low;
        let hi = self.days_high & 1;
        u16::from_le_bytes([lo, hi])
    }

    pub fn halted(&self) -> bool {
        (self.days_high & (1 << 6)) != 0
    }

    pub fn read(&self, kind: RtcRegisterKind) -> u8 {
        match kind {
            RtcRegisterKind::Seconds => self.seconds,
            RtcRegisterKind::Minutes => self.minutes,
            RtcRegisterKind::Hours => self.hours,
            RtcRegisterKind::DaysLow => self.days_low,
            RtcRegisterKind::DaysHigh => self.days_high,
        }
    }

    pub fn write(&mut self, kind: RtcRegisterKind, value: u8) {
        match kind {
            RtcRegisterKind::Seconds => self.seconds = value,
            RtcRegisterKind::Minutes => self.minutes = value,
            RtcRegisterKind::Hours => self.hours = value,
            RtcRegisterKind::DaysLow => self.days_low = value,
            RtcRegisterKind::DaysHigh => {
                // Only bits 0 (day high), 6 (halt), 7 (carry) are meaningful
                self.days_high = value & 0b1100_0001;
            }
        }
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

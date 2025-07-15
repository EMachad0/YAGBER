/// 5 bits per channel. Little endian.
pub struct Rgb555 {
    value: u16,
}

impl Rgb555 {
    pub fn red(&self) -> u8 {
        (self.value & 0b11111) as u8
    }

    pub fn green(&self) -> u8 {
        ((self.value >> 5) & 0b11111) as u8
    }

    pub fn blue(&self) -> u8 {
        ((self.value >> 10) & 0b11111) as u8
    }

    pub fn from_u16(value: u16) -> Self {
        Self { value }
    }
}

impl From<Rgb555> for Rgba {
    fn from(rgb555: Rgb555) -> Self {
        let transform = |v: u8| (v << 3) | (v >> 2);
        let red = transform(rgb555.red());
        let green = transform(rgb555.green());
        let blue = transform(rgb555.blue());
        let alpha = 255;
        Self::new(red, green, blue, alpha)
    }
}

pub struct Rgba {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Rgba {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn values(&self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

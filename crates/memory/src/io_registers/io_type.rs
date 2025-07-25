use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, strum::EnumIter, PartialEq, Eq, Hash)]
pub enum IOType {
    JOYP,
    SB,
    SC,
    DIV,
    TIMA,
    TMA,
    TAC,
    IF,
    AUD1SWEEP, // NR10
    AUD1LEN, // NR11
    AUD1ENV, // NR12
    AUD1LOW, // NR13
    AUD1HIGH, // NR14
    AUD2LEN, // NR21
    AUD2ENV, // NR22
    AUD2LOW, // NR23
    AUD2HIGH, // NR24
    AUD3ENA, // NR30
    AUD3LEN, // NR31
    AUD3LEVEL, // NR32
    AUD3LOW, // NR33
    AUD3HIGH, // NR34
    AUD4LEN, // NR41
    AUD4ENV, // NR42
    AUD4POLY, // NR43
    AUD4GO, // NR44
    AUDVOL, // NR50
    AUDTERM, // NR51
    AUDENA, // NR52
    // TODO: Wave is actually 16 bytes long
    WAV,
    LCDC,
    STAT,
    SCY,
    SCX,
    LY,
    LYC,
    DMA,
    BGP,
    OBP0,
    OBP1,
    WY,
    WX,
    SYS,
    SPD,
    VBK,
    BANK,
    HdmaSrcHi,
    HdmaSrcLo,
    HdmaDstHi,
    HdmaDstLo,
    HdmaLen,
    RP,
    BCPS,
    BCPD,
    OCPS,
    OCPD,
    OPRI,
    SVBK,
    PCM12,
    PCM34,
    IE,
}

impl IOType {
    pub fn from_index(index: usize) -> Option<Self> {
        Self::iter().nth(index)
    }

    pub fn from_address(address: u16) -> Option<Self> {
        use IOType::*;
        match address {
            0xFF00 => Some(JOYP),
            0xFF01 => Some(SB),
            0xFF02 => Some(SC),
            0xFF04 => Some(DIV),
            0xFF05 => Some(TIMA),
            0xFF06 => Some(TMA),
            0xFF07 => Some(TAC),
            0xFF0F => Some(IF),
            0xFF10 => Some(AUD1SWEEP),
            0xFF11 => Some(AUD1LEN),
            0xFF12 => Some(AUD1ENV),
            0xFF13 => Some(AUD1LOW),
            0xFF14 => Some(AUD1HIGH),
            0xFF16 => Some(AUD2LEN),
            0xFF17 => Some(AUD2ENV),
            0xFF18 => Some(AUD2LOW),
            0xFF19 => Some(AUD2HIGH),
            0xFF1A => Some(AUD3ENA),
            0xFF1B => Some(AUD3LEN),
            0xFF1C => Some(AUD3LEVEL),
            0xFF1D => Some(AUD3LOW),
            0xFF1E => Some(AUD3HIGH),
            0xFF20 => Some(AUD4LEN),
            0xFF21 => Some(AUD4ENV),
            0xFF22 => Some(AUD4POLY),
            0xFF23 => Some(AUD4GO),
            0xFF24 => Some(AUDVOL),
            0xFF25 => Some(AUDTERM),
            0xFF26 => Some(AUDENA),
            0xFF30 => Some(WAV),
            0xFF40 => Some(LCDC),
            0xFF41 => Some(STAT),
            0xFF42 => Some(SCY),
            0xFF43 => Some(SCX),
            0xFF44 => Some(LY),
            0xFF45 => Some(LYC),
            0xFF46 => Some(DMA),
            0xFF47 => Some(BGP),
            0xFF48 => Some(OBP0),
            0xFF49 => Some(OBP1),
            0xFF4A => Some(WY),
            0xFF4B => Some(WX),
            0xFF4C => Some(SYS),
            0xFF4D => Some(SPD),
            0xFF4F => Some(VBK),
            0xFF50 => Some(BANK),
            0xFF51 => Some(HdmaSrcHi),
            0xFF52 => Some(HdmaSrcLo),
            0xFF53 => Some(HdmaDstHi),
            0xFF54 => Some(HdmaDstLo),
            0xFF55 => Some(HdmaLen),
            0xFF56 => Some(RP),
            0xFF68 => Some(BCPS),
            0xFF69 => Some(BCPD),
            0xFF6A => Some(OCPS),
            0xFF6B => Some(OCPD),
            0xFF6C => Some(OPRI),
            0xFF70 => Some(SVBK),
            0xFF76 => Some(PCM12),
            0xFF77 => Some(PCM34),
            0xFFFF => Some(IE),
            _ => None,
        }
    }

    pub fn address(&self) -> u16 {
        use IOType::*;
        match self {
            JOYP => 0xFF00,
            SB => 0xFF01,
            SC => 0xFF02,
            DIV => 0xFF04,
            TIMA => 0xFF05,
            TMA => 0xFF06,
            TAC => 0xFF07,
            IF => 0xFF0F,
            AUD1SWEEP => 0xFF10,
            AUD1LEN => 0xFF11,
            AUD1ENV => 0xFF12,
            AUD1LOW => 0xFF13,
            AUD1HIGH => 0xFF14,
            AUD2LEN => 0xFF16,
            AUD2ENV => 0xFF17,
            AUD2LOW => 0xFF18,
            AUD2HIGH => 0xFF19,
            AUD3ENA => 0xFF1A,
            AUD3LEN => 0xFF1B,
            AUD3LEVEL => 0xFF1C,
            AUD3LOW => 0xFF1D,
            AUD3HIGH => 0xFF1E,
            AUD4LEN => 0xFF20,
            AUD4ENV => 0xFF21,
            AUD4POLY => 0xFF22,
            AUD4GO => 0xFF23,
            AUDVOL => 0xFF24,
            AUDTERM => 0xFF25,
            AUDENA => 0xFF26,
            WAV => 0xFF30,
            LCDC => 0xFF40,
            STAT => 0xFF41,
            SCY => 0xFF42,
            SCX => 0xFF43,
            LY => 0xFF44,
            LYC => 0xFF45,
            DMA => 0xFF46,
            BGP => 0xFF47,
            OBP0 => 0xFF48,
            OBP1 => 0xFF49,
            WY => 0xFF4A,
            WX => 0xFF4B,
            SYS => 0xFF4C,
            SPD => 0xFF4D,
            VBK => 0xFF4F,
            BANK => 0xFF50,
            HdmaSrcHi => 0xFF51,
            HdmaSrcLo => 0xFF52,
            HdmaDstHi => 0xFF53,
            HdmaDstLo => 0xFF54,
            HdmaLen => 0xFF55,
            RP => 0xFF56,
            BCPS => 0xFF68,
            BCPD => 0xFF69,
            OCPS => 0xFF6A,
            OCPD => 0xFF6B,
            OPRI => 0xFF6C,
            SVBK => 0xFF70,
            PCM12 => 0xFF76,
            PCM34 => 0xFF77,
            IE => 0xFFFF,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_type_address() {
        for io_type in IOType::iter() {
            assert_eq!(
                io_type,
                IOType::from_address(io_type.address()).unwrap_or_else(|| {
                    panic!(
                        "IOType::from_address({:X}) returned None",
                        io_type.address()
                    )
                })
            );
        }
    }
}

use arbitrary_int::{u2, u3};

pub struct ConditionCode(u2);

impl ConditionCode {
    pub fn new(value: u2) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u8 {
        self.0.value()
    }
}

/// Almost all instructions are 1 byte long and the extras are present in the same as the opcode
/// imm8 and imm16 work a bit differently than the others, they are not present in the opcode
/// but in the next one and two bytes respectively
/// Since the instruction does not have access to the memory or pc, these values are passed
/// later by the instruction decoder
///
/// see [Cpu Instruction Set](https://gbdev.io/pandocs/CPU_Instruction_Set.html) for more details
#[derive(Debug, Default)]
pub struct Instruction {
    /// Cb prefix
    cb_prefix: bool,
    /// The instruction opcode
    opcode: u8,
    /// The instruction type
    instruction_type: InstructionType,
    /// The 8-bit immediate value, if applicable
    imm8: Option<u8>,
    /// The 16-bit immediate value, if applicable
    imm16: Option<u16>,
}

impl Instruction {
    /// Create a new instruction from the opcode
    pub fn new(opcode: u8) -> Self {
        Self {
            instruction_type: InstructionType::from_opcode(opcode),
            ..Self::default()
        }
    }

    /// Create a new instruction from the CB prefix opcode
    pub fn new_cb_prefix(opcode: u8) -> Self {
        Self {
            cb_prefix: true,
            instruction_type: InstructionType::from_opcode_cb_prefix(opcode),
            ..Self::default()
        }
    }

    /// Get the instruction type
    pub fn instruction_type(&self) -> &InstructionType {
        &self.instruction_type
    }

    /// Set the 8-bit immediate value
    pub fn set_imm8(&mut self, imm8: u8) {
        self.imm8 = Some(imm8);
    }

    /// Set the 16-bit immediate value
    pub fn set_imm16(&mut self, imm16: u16) {
        self.imm16 = Some(imm16);
    }

    /// Do we need to read a signed 8‑bit immediate after the opcode?
    pub fn requires_imm8(&self) -> bool {
        use InstructionType::*;
        matches!(
            self.instruction_type,
            // plain 8‑bit loads and jumps:
            LdR8Imm8
            | JrImm8
            | JrCondImm8
            // arithmetic on A with immediate:
            | AddAImm8
            | AdcAImm8
            | SubAImm8
            | SbcAImm8
            | AndAImm8
            | XorAImm8
            | OrAImm8
            | CpAImm8
            // special SP/HL adjustments:
            | AddSpImm8
            | LdHlSpImm8
            // high‑page (0xFF00+imm8) loads/stores:
            | LdhImm8A
            | LdhAImm8
        )
    }

    /// Do we need to read a 16‑bit immediate (little‑endian) after the opcode?
    pub fn requires_imm16(&self) -> bool {
        use InstructionType::*;
        matches!(
            self.instruction_type,
            // load 16‑bit constants:
            LdR16Imm16
            | LdImm16Sp
            // absolute jumps and calls:
            | JpImm16
            | JpCondImm16
            | CallImm16
            | CallCondImm16
            // absolute (0x0000–0xFFFF) loads:
            | LdImm16A
            | LdAImm16
        )
    }

    /// Get the 8-bit immediate value
    pub fn imm8(&self) -> Option<u8> {
        self.imm8
    }

    /// Get the 16-bit immediate value
    pub fn imm16(&self) -> Option<u16> {
        self.imm16
    }

    /// Get the r8 from instructions that use it
    /// r8: 3-bit, one of the 8-bit register
    pub fn r8(&self) -> Option<u3> {
        if self.cb_prefix {
            return Some(u3::from_u8(self.opcode & 0b111));
        }
        use InstructionType::*;
        match self.opcode >> 6 {
            0b10 => Some(self.opcode & 0b111),
            0b00 => match self.instruction_type {
                // 0b00??_?000
                IncR8 | DecR8 | LdR8Imm8 => Some((self.opcode >> 3) & 0b111),
                _ => None,
            },
            _ => None,
        }
        .map(u3::from_u8)
    }

    /// LdR8R8 instructions use two 3-bit registers
    pub fn r8_pair(&self) -> Option<(u3, u3)> {
        use InstructionType::*;
        match self.instruction_type {
            // 0b01??_????
            LdR8R8 => Some(((self.opcode >> 3) & 0b111, self.opcode & 0b111)),
            _ => None,
        }
        .map(|(a, b)| (u3::from_u8(a), u3::from_u8(b)))
    }

    /// Get the r16 from instructions that use it
    /// r16: 2-bit, one of the 16-bit registers
    /// r16stk: 2-bit, one of the 16-bit registers of the stack
    /// r16mem: 2-bit, one of the 16-bit registers of the memory
    pub fn r16(&self) -> Option<u2> {
        use InstructionType::*;

        if matches!(
            self.instruction_type,
            // 0b00??_0000
            LdR16Imm16
                | LdR16memA
                | LdAR16mem
                | IncR16
                | DecR16
                | AddHlR16
                | PopR16stk
                | PushR16stk
        ) {
            Some(u2::from_u8((self.opcode >> 4) & 0b11))
        } else {
            None
        }
    }

    /// Get the condition code from instructions that use it
    /// cond: condition code (z, nz, c, nc)
    /// The condition code is a 2-bit value
    pub fn cond(&self) -> Option<ConditionCode> {
        use InstructionType::*;
        match self.instruction_type {
            // 0b001?_?000
            JrCondImm8 | RetCond | JpCondImm16 | CallCondImm16 => {
                Some(ConditionCode::new(u2::from_u8((self.opcode >> 3) & 0b11)))
            }
            _ => None,
        }
    }

    /// Get the b3 from instructions that use it
    /// b3: 3-bit bit index
    /// The b3 is a 3-bit value
    pub fn b3(&self) -> Option<u3> {
        use InstructionType::*;
        match self.instruction_type {
            // 0b01??_????
            BitB3R8 | ResB3R8 | SetB3R8 => Some(u3::from_u8((self.opcode >> 3) & 0b111)),
            _ => None,
        }
    }

    /// Get the target 3 from instructions that use it
    /// tgt3: rst target address, divided by 8
    /// The tgt3 is a 3-bit value
    pub fn tgt3(&self) -> Option<u3> {
        use InstructionType::*;
        match self.instruction_type {
            // 0b11??_?111
            RstTgt3 => Some(u3::from_u8(self.opcode & 0b111)),
            _ => None,
        }
    }
}

/// Blocks are defined by the first two bits of the opcode
/// where instruction is the instruction name in assembly
#[derive(Debug, Default, Eq, PartialEq)]
pub enum InstructionType {
    // Block 0b00
    /// No operation
    /// binary: 0b0000_0000
    #[default]
    Nop,
    /// Load 16-bit immediate value into 16-bit register
    /// binary: 0b00??_0001
    LdR16Imm16,
    /// binary: 0b00??_0010
    LdR16memA,
    /// binary: 0b00??_1010
    LdAR16mem,
    /// binary: 0b00??_1000
    LdImm16Sp,
    /// binary: 0b00??_0011
    IncR16,
    /// binary: 0b00??_1011
    DecR16,
    /// binary: 0b00??_1001
    AddHlR16,
    /// binary: 0b00??_?100
    IncR8,
    /// binary: 0b00??_?101
    DecR8,
    /// binary: 0b00??_?110
    LdR8Imm8,
    /// binary: 0b0000_0111
    RlCA,
    /// binary: 0b0000_1111
    RrCA,
    /// binary: 0b0001_0111
    RlA,
    /// binary: 0b0001_1111
    RrA,
    /// binary: 0b0010_0111
    Daa,
    /// binary: 0b0010_1111
    Cpl,
    /// binary: 0b0011_0111
    Scf,
    /// binary: 0b0011_1111
    Ccf,
    /// binary: 0b0001_1000
    JrImm8,
    /// binary: 0b001?_?000
    JrCondImm8,
    /// binary: 0b0010_0000
    Stop,
    // Block 0b01
    /// binary: 0b01??_????
    LdR8R8,
    /// binary: 0b0111_0110
    Halt,
    // Block 0b10
    /// binary: 0b1000_00??
    AddAR8,
    /// binary: 0b1000_01??
    AdcAR8,
    /// binary: 0b1000_10??
    SubAR8,
    /// binary: 0b1000_11??
    SbcAR8,
    /// binary: 0b1001_00??
    AndAR8,
    /// binary: 0b1001_01??
    XorAR8,
    /// binary: 0b1001_10??
    OrAR8,
    /// binary: 0b1001_11??
    CpAR8,
    // Block 0b11
    /// binary: 0b1100_0110
    AddAImm8,
    /// binary: 0b1100_1110
    AdcAImm8,
    /// binary: 0b1101_0110
    SubAImm8,
    /// binary: 0b1101_1110
    SbcAImm8,
    /// binary: 0b1110_0110
    AndAImm8,
    /// binary: 0b1110_1110
    XorAImm8,
    /// binary: 0b1111_0110
    OrAImm8,
    /// binary: 0b1111_1110
    CpAImm8,
    /// binary: 0b110?_?000
    RetCond,
    /// binary: 0b1100_1001
    Ret,
    /// binary: 0b1101_1001
    RetI,
    /// binary: 0b110?_?010
    JpCondImm16,
    /// binary: 0b1100_0011
    JpImm16,
    /// binary: 0b1110_1001
    JpHl,
    /// binary: 0b110?_?100
    CallCondImm16,
    /// binary: 0b1100_1101
    CallImm16,
    /// binary: 0b11??_?111
    RstTgt3,
    /// binary: 0b11??_0001
    PopR16stk,
    /// binary: 0b11??_0101
    PushR16stk,
    /// binary: 0b1100_1011
    Prefix,
    /// binary: 0b1110_0010
    LdhCA,
    /// binary: 0b1110_0000
    LdhImm8A,
    /// binary: 0b1110_1010
    LdImm16A,
    /// binary: 0b1111_0010
    LdhAC,
    /// binary: 0b1111_0000
    LdhAImm8,
    /// binary: 0b1111_1010
    LdAImm16,
    /// binary: 0b1110_1000
    AddSpImm8,
    /// binary: 0b1111_1000
    LdHlSpImm8,
    /// binary: 0b1111_1001
    LdSpHl,
    /// binary: 0b1111_0011
    Di,
    /// binary: 0b1111_1011
    Ei,
    // $cb prefix instructions
    /// binary: 0b0000_0???
    RlcR8,
    /// binary: 0b0000_1???
    RrcR8,
    /// binary: 0b0001_0???
    RlR8,
    /// binary: 0b0001_1???
    RrR8,
    /// binary: 0b0010_0???
    SlaR8,
    /// binary: 0b0010_1???
    SraR8,
    /// binary: 0b0011_0???
    SwapR8,
    /// binary: 0b0011_1???
    SrlR8,
    /// binary: 0b01??_????
    BitB3R8,
    /// binary: 0b10??_????
    ResB3R8,
    /// binary: 0b11??_????
    SetB3R8,
}

impl InstructionType {
    /// Decode the opcode into an instruction type
    pub fn from_opcode(opcode: u8) -> Self {
        // Block 0b00
        if opcode == 0x00 {
            InstructionType::Nop
        } else if match_mask(opcode, 0b0000_0001, 0b1100_1110) {
            InstructionType::LdR16Imm16
        } else if match_mask(opcode, 0b0000_0010, 0b1100_1110) {
            InstructionType::LdR16memA
        } else if match_mask(opcode, 0b0000_1010, 0b1100_0011) {
            InstructionType::LdAR16mem
        } else if match_mask(opcode, 0b0000_1000, 0b1100_0011) {
            InstructionType::LdImm16Sp
        } else if match_mask(opcode, 0b0000_0011, 0b1100_1100) {
            InstructionType::IncR16
        } else if match_mask(opcode, 0b0000_1011, 0b1100_1100) {
            InstructionType::DecR16
        } else if match_mask(opcode, 0b0000_1001, 0b1100_1100) {
            InstructionType::AddHlR16
        } else if match_mask(opcode, 0b0000_0100, 0b1100_1011) {
            InstructionType::IncR8
        } else if match_mask(opcode, 0b0000_0101, 0b1100_1011) {
            InstructionType::DecR8
        } else if match_mask(opcode, 0b0000_0110, 0b1100_1001) {
            InstructionType::LdR8Imm8
        } else if opcode == 0x07 {
            InstructionType::RlCA
        } else if opcode == 0x0F {
            InstructionType::RrCA
        } else if opcode == 0x17 {
            InstructionType::RlA
        } else if opcode == 0x1F {
            InstructionType::RrA
        } else if opcode == 0x27 {
            InstructionType::Daa
        } else if opcode == 0x2F {
            InstructionType::Cpl
        } else if opcode == 0x37 {
            InstructionType::Scf
        } else if opcode == 0x3F {
            InstructionType::Ccf
        } else if opcode == 0x18 {
            InstructionType::JrImm8
        } else if match_mask(opcode, 0b0010_1000, 0b1101_0000) {
            InstructionType::JrCondImm8
        } else if opcode == 0x10 {
            InstructionType::Stop
        // Block 0b01
        } else if match_mask(opcode, 0b0100_0000, 0b1100_0000) {
            InstructionType::LdR8R8
        } else if opcode == 0x76 {
            InstructionType::Halt
        // Block 0b10
        } else if match_mask(opcode, 0b1000_0000, 0b1100_0000) {
            InstructionType::AddAR8
        } else if match_mask(opcode, 0b1000_0100, 0b1100_0000) {
            InstructionType::AdcAR8
        } else if match_mask(opcode, 0b1000_1000, 0b1100_0000) {
            InstructionType::SubAR8
        } else if match_mask(opcode, 0b1000_1100, 0b1100_0000) {
            InstructionType::SbcAR8
        } else if match_mask(opcode, 0b1001_0000, 0b1100_0000) {
            InstructionType::AndAR8
        } else if match_mask(opcode, 0b1001_0100, 0b1100_0000) {
            InstructionType::XorAR8
        } else if match_mask(opcode, 0b1001_1000, 0b1100_0000) {
            InstructionType::OrAR8
        } else if match_mask(opcode, 0b1001_1100, 0b1100_0000) {
            InstructionType::CpAR8
        // Block 0b11
        } else if opcode == 0xC6 {
            InstructionType::AddAImm8
        } else if opcode == 0xCE {
            InstructionType::AdcAImm8
        } else if opcode == 0xD6 {
            InstructionType::SubAImm8
        } else if opcode == 0xDE {
            InstructionType::SbcAImm8
        } else if opcode == 0xE6 {
            InstructionType::AndAImm8
        } else if opcode == 0xEE {
            InstructionType::XorAImm8
        } else if opcode == 0xF6 {
            InstructionType::OrAImm8
        } else if opcode == 0xFE {
            InstructionType::CpAImm8
        } else if match_mask(opcode, 0b1100_0000, 0b1101_1000) {
            InstructionType::RetCond
        } else if opcode == 0xC9 {
            InstructionType::Ret
        } else if opcode == 0xD9 {
            InstructionType::RetI
        } else if match_mask(opcode, 0b1100_0010, 0b1101_0000) {
            InstructionType::JpCondImm16
        } else if opcode == 0xC3 {
            InstructionType::JpImm16
        } else if opcode == 0xE9 {
            InstructionType::JpHl
        } else if match_mask(opcode, 0b1100_0100, 0b1101_0000) {
            InstructionType::CallCondImm16
        } else if opcode == 0xCD {
            InstructionType::CallImm16
        } else if match_mask(opcode, 0b1100_0111, 0b1100_1001) {
            InstructionType::RstTgt3
        } else if match_mask(opcode, 0b1100_0001, 0b1100_1011) {
            InstructionType::PopR16stk
        } else if match_mask(opcode, 0b1100_0101, 0b1100_1011) {
            InstructionType::PushR16stk
        } else if opcode == 0xCB {
            // CB‐prefix: call from_opcode_cb_prefix on the next byte
            InstructionType::Prefix
        } else if opcode == 0xE2 {
            InstructionType::LdhCA
        } else if opcode == 0xE0 {
            InstructionType::LdhImm8A
        } else if opcode == 0xEA {
            InstructionType::LdImm16A
        } else if opcode == 0xF2 {
            InstructionType::LdhAC
        } else if opcode == 0xF0 {
            InstructionType::LdhAImm8
        } else if opcode == 0xFA {
            InstructionType::LdAImm16
        } else if opcode == 0xE8 {
            InstructionType::AddSpImm8
        } else if opcode == 0xF8 {
            InstructionType::LdHlSpImm8
        } else if opcode == 0xF9 {
            InstructionType::LdSpHl
        } else if opcode == 0xF3 {
            InstructionType::Di
        } else if opcode == 0xFB {
            InstructionType::Ei
        } else {
            panic!("Unimplemented opcode: 0x{:02X}", opcode);
        }
    }

    /// Decode a CB‑prefixed opcode (i.e. the byte after 0xCB)
    pub fn from_opcode_cb_prefix(opcode: u8) -> Self {
        // Top two bits select the major group:
        // 00: rotate/shift/swap, 01: BIT b3, r8, 10: RES b3, r8, 11: SET b3, r8
        match opcode >> 6 {
            0b00 => {
                // within 0x00–0x3F: each 8‑byte slice is one of RLC/RRC/RL/RR/SLA/SRA/SWAP/SRL
                match opcode & 0xF8 {
                    0x00 => InstructionType::RlcR8,
                    0x08 => InstructionType::RrcR8,
                    0x10 => InstructionType::RlR8,
                    0x18 => InstructionType::RrR8,
                    0x20 => InstructionType::SlaR8,
                    0x28 => InstructionType::SraR8,
                    0x30 => InstructionType::SwapR8,
                    0x38 => InstructionType::SrlR8,
                    _ => unreachable!("invalid CB rotate/shift opcode: 0x{:02X}", opcode),
                }
            }
            0b01 => InstructionType::BitB3R8, // 0x40–0x7F
            0b10 => InstructionType::ResB3R8, // 0x80–0xBF
            0b11 => InstructionType::SetB3R8, // 0xC0–0xFF
            _ => unreachable!(),
        }
    }
}

/// Check if the opcode matches the mask
/// mask is the u8 with ones in the positions that should be ones
/// nmask is the u8 with ones in the positions that should be zeros
fn match_mask(opcode: u8, mask: u8, nmask: u8) -> bool {
    (opcode & mask) == mask && (opcode & nmask) == 0
}

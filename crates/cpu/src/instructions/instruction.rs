use arbitrary_int::{u2, u3};

use crate::instructions::{ConditionCode, InstructionType};

/// Almost all instructions are 1 byte long and the extras are present in the same as the opcode
/// imm8 and imm16 work a bit differently than the others, they are not present in the opcode
/// but in the next one and two bytes respectively
/// Since the instruction does not have access to the memory or pc, these values are passed
/// later by the instruction decoder
///
/// see [Cpu Instruction Set](https://gbdev.io/pandocs/CPU_Instruction_Set.html) for more details
#[derive(Default)]
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
            opcode,
            instruction_type: InstructionType::from_opcode(opcode),
            ..Self::default()
        }
    }

    /// Create a new instruction from the CB prefix opcode
    pub fn new_cb_prefix(opcode: u8) -> Self {
        Self {
            opcode,
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
    /// returns the pair (dst, src)
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
            RstTgt3 => Some(u3::from_u8((self.opcode >> 3) & 0b111)),
            _ => None,
        }
    }

    /// Get the duration in M-cycles of the instruction
    pub fn cycles(&self) -> u16 {
        use InstructionType::*;
        match self.instruction_type {
            // 0b00xxxxxx
            Nop => 1,
            LdR16Imm16 => 3,
            LdR16memA => 2,
            LdAR16mem => 2,
            LdImm16Sp => 3,
            IncR16 => 2,
            DecR16 => 2,
            AddHlR16 => 2,
            IncR8 => 1,
            DecR8 => 1,
            LdR8Imm8 => 2,
            RlCA | RrCA => 1,
            RlA | RrA => 1,
            Daa => 1,
            Cpl => 1,
            Scf => 1,
            Ccf => 1,
            JrImm8 => 2,
            JrCondImm8 => 3, // taken
            Stop => 1,
            // 0b01xxxxxx
            LdR8R8 => 1,
            Halt => 1,
            // 0b10xxxxxx
            AddAR8 | AdcAR8 | SubAR8 | SbcAR8 | AndAR8 | XorAR8 | OrAR8 | CpAR8 => 1,
            // 0b11xxxxxx
            AddAImm8 | AdcAImm8 | SubAImm8 | SbcAImm8 | AndAImm8 | XorAImm8 | OrAImm8 | CpAImm8 => {
                2
            }
            RetCond => 5, // taken
            Ret => 4,
            RetI => 4,
            JpCondImm16 => 4, // taken
            JpImm16 => 4,
            JpHl => 1,
            CallCondImm16 => 6, // taken
            CallImm16 => 6,
            RstTgt3 => 4,
            PopR16stk => 3,
            PushR16stk => 4,
            // CB‑prefix group (Prefix itself costs 1)
            Prefix => 1,
            RlcR8 | RrcR8 | RlR8 | RrR8 | SlaR8 | SraR8 | SwapR8 | SrlR8 => 2,
            BitB3R8 | ResB3R8 | SetB3R8 => 2,
            // High‑page / I/O style loads
            LdhCA | LdhAC => 2,
            LdhImm8A | LdhAImm8 => 3,
            // Absolute 16‑bit loads
            LdImm16A | LdAImm16 => 4,
            // SP/HL adjustments
            AddSpImm8 => 4,
            LdHlSpImm8 => 3,
            LdSpHl => 2,
            // DI/EI
            Di => 1,
            Ei => 1,
        }
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("Instruction");

        dbg.field("opcode", &format_args!("0x{:02X}", self.opcode))
            .field("type", &self.instruction_type);

        if self.cb_prefix {
            dbg.field("cb_prefix", &self.cb_prefix);
        }
        if let Some(imm8) = self.imm8 {
            dbg.field("imm8", &format_args!("0x{imm8:02X}"));
        }
        if let Some(imm16) = self.imm16 {
            dbg.field("imm16", &format_args!("0x{imm16:04X}"));
        }
        if let Some(r8) = self.r8() {
            dbg.field("r8", &format_args!("0x{r8:02X}"));
        }
        if let Some(r16) = self.r16() {
            dbg.field("r16", &format_args!("0x{r16:02X}"));
        }
        if let Some(cond) = self.cond() {
            dbg.field("cond", &format_args!("{cond:?}"));
        }
        if let Some(b3) = self.b3() {
            dbg.field("b3", &format_args!("0x{b3:02X}"));
        }
        if let Some(tgt3) = self.tgt3() {
            dbg.field("tgt3", &format_args!("0x{tgt3:02X}"));
        }

        dbg.finish_non_exhaustive()
    }
}

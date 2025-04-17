use crate::instruction::{Instruction, InstructionType};
use crate::registers::Registers;

pub struct Cpu {
    pub pc: u16,
    pub sp: u16,
    pub registers: Registers,
    pub cartridge: Vec<u8>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0x0100, // starts at 0x0100
            sp: 0,
            registers: Registers::default(),
            cartridge: Vec::new(),
        }
    }

    pub fn from_rom(rom: Vec<u8>) -> Self {
        Self {
            cartridge: rom,
            ..Self::default()
        }
    }

    pub fn step(&mut self) {
        // Fetch the next instruction
        let instruction = self.read_next_instruction();
        
        info!("instruction: {:?}", instruction);
    }

    pub fn read_next_instruction(&mut self) -> Instruction {
        // Fetch the next instruction
        let opcode = self.read_next_byte();

        // Decode the instruction
        let mut instruction = Instruction::new(opcode);
        if *instruction.instruction_type() == InstructionType::Prefix {
            // Handle prefix instructions
            let prefix_opcode = self.read_next_byte();
            instruction = Instruction::new_cb_prefix(prefix_opcode)
        }

        // Check if the instruction needs more bytes
        if instruction.requires_imm8() {
            // Read the immediate value
            let imm8 = self.read_next_byte();
            instruction.set_imm8(imm8);
        }

        if instruction.requires_imm16() {
            // Read the immediate value
            let lo = self.read_next_byte();
            let hi = self.read_next_byte();
            let imm16 = u16::from_le_bytes([lo, hi]);
            instruction.set_imm16(imm16);
        }

        instruction
    }

    pub fn read_next_byte(&mut self) -> u8 {
        let byte = self.cartridge[self.pc as usize];
        self.pc += 1;
        byte
    }
}
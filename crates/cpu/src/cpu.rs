use crate::alu::{Alu8, Alu16};
use crate::instruction::{ConditionCode, Instruction, InstructionType};
use crate::ram::Ram;
use crate::registers::Registers;
use arbitrary_int::{u2, u3};

pub struct Cpu {
    pub pc: u16,
    pub sp: u16,
    pub registers: Registers,
    pub ram: Ram,
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
            ram: Ram::default(),
            cartridge: Vec::default(),
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

    pub fn read_r8(&self, r8: u3) -> u8 {
        match r8.value() {
            0 => self.registers.b(),
            1 => self.registers.c(),
            2 => self.registers.d(),
            3 => self.registers.e(),
            4 => self.registers.h(),
            5 => self.registers.l(),
            6 => self.ram.read(self.registers.hl()),
            7 => self.registers.a(),
            _ => unreachable!(),
        }
    }

    pub fn write_r8(&mut self, r8: u3, value: u8) {
        match r8.value() {
            0 => self.registers.set_b(value),
            1 => self.registers.set_c(value),
            2 => self.registers.set_d(value),
            3 => self.registers.set_e(value),
            4 => self.registers.set_h(value),
            5 => self.registers.set_l(value),
            6 => self.ram.write(self.registers.hl(), value),
            7 => self.registers.set_a(value),
            _ => unreachable!(),
        }
    }

    pub fn read_r16(&self, r16: u2) -> u16 {
        match r16.value() {
            0 => self.registers.bc(),
            1 => self.registers.de(),
            2 => self.registers.hl(),
            3 => self.sp,
            _ => unreachable!(),
        }
    }

    pub fn write_r16(&mut self, r16: u2, value: u16) {
        match r16.value() {
            0 => self.registers.set_bc(value),
            1 => self.registers.set_de(value),
            2 => self.registers.set_hl(value),
            3 => self.sp = value,
            _ => unreachable!(),
        }
    }

    pub fn read_r16stk(&self, r16: u2) -> u16 {
        match r16.value() {
            0 => self.registers.bc(),
            1 => self.registers.de(),
            2 => self.registers.hl(),
            3 => self.registers.af(),
            _ => unreachable!(),
        }
    }

    pub fn write_r16stk(&mut self, r16: u2, value: u16) {
        match r16.value() {
            0 => self.registers.set_bc(value),
            1 => self.registers.set_de(value),
            2 => self.registers.set_hl(value),
            3 => self.registers.set_af(value),
            _ => unreachable!(),
        }
    }

    pub fn read_r16mem(&mut self, r16: u2) -> u16 {
        match r16.value() {
            0 => self.registers.bc(),
            1 => self.registers.de(),
            2 => self.registers.hl_inc(),
            3 => self.registers.hl_dec(),
            _ => unreachable!(),
        }
    }

    pub fn check_condition(&self, condition: ConditionCode) -> bool {
        match condition.value() {
            0 => self.registers.flags().z(),
            1 => !self.registers.flags().z(),
            2 => self.registers.flags().c(),
            3 => !self.registers.flags().c(),
            _ => unreachable!(),
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        // Execute the instruction
        use InstructionType::*;
        match instruction.instruction_type() {
            // Block 0b00
            Nop => {}
            LdR16Imm16 => {
                let r16 = instruction.r16().unwrap();
                let imm16 = instruction.imm16().unwrap();
                self.write_r16(r16, imm16);
            }
            LdR16memA => {
                let r16 = instruction.r16().unwrap();
                let hl = self.read_r16(r16);
                let a = self.registers.a();
                self.ram.write(hl, a);
            }
            LdAR16mem => {
                let r16 = instruction.r16().unwrap();
                let address = self.read_r16mem(r16);
                let a = self.ram.read(address);
                self.registers.set_a(a);
            }
            LdImm16Sp => {
                let imm16 = instruction.imm16().unwrap();
                self.ram.write_u16(imm16, self.sp);
            }
            IncR16 => {
                let r16 = instruction.r16().unwrap();
                let value = self.read_r16(r16);
                self.write_r16(r16, value.wrapping_add(1));
            }
            DecR16 => {
                let r16 = instruction.r16().unwrap();
                let value = self.read_r16(r16);
                self.write_r16(r16, value.wrapping_sub(1));
            }
            AddHlR16 => {
                let r_val = self.read_r16(instruction.r16().unwrap());
                let hl = self.registers.hl();

                let result = Alu16::add(r_val, hl);
                self.registers.set_hl(*result);

                self.registers
                    .flags_mut()
                    .set_n(false)
                    .set_h(result.cb11)
                    .set_c(result.cb15);
            }
            IncR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8);
                let result = Alu8::inc(r_val);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(result.cb3);
            }
            DecR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8);
                let result = Alu8::dec(r_val);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(true)
                    .set_h(result.cb3);
            }
            LdR8Imm8 => {
                let r8 = instruction.r8().unwrap();
                let imm8 = instruction.imm8().unwrap();
                self.write_r8(r8, imm8);
            }
            RlCA => {
                let a = self.registers.a();
                let result = Alu8::rlc(a);
                self.registers.set_a(*result);

                self.registers
                    .flags_mut()
                    .set_z(false)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            RrCA => {
                let a = self.registers.a();
                let result = Alu8::rrc(a);
                self.registers.set_a(*result);

                self.registers
                    .flags_mut()
                    .set_z(false)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            RlA => {
                let a = self.registers.a();
                let carry = self.registers.flags().c_u8();
                let result = Alu8::rl(a, carry);
                self.registers.set_a(*result);

                self.registers
                    .flags_mut()
                    .set_z(false)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            RrA => {
                let a = self.registers.a();
                let carry = self.registers.flags().c_u8();
                let result = Alu8::rr(a, carry);
                self.registers.set_a(*result);

                self.registers
                    .flags_mut()
                    .set_z(false)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            Daa => {
                let result = if self.registers.flags().n() {
                    let mut adj = 0;
                    if self.registers.flags().h() {
                        adj += 0x06;
                    }
                    if self.registers.flags().c() {
                        adj += 0x60;
                        self.registers.flags_mut().set_c(false);
                    }
                    let a = self.registers.a();
                    Alu8::sub(a, adj)
                } else {
                    let mut adj = 0;
                    if self.registers.flags().h() || (self.registers.a() & 0x0F) > 0x09 {
                        adj += 0x06;
                    }
                    if self.registers.flags().c() || self.registers.a() > 0x99 {
                        adj += 0x60;
                        self.registers.flags_mut().set_c(true);
                    }
                    let a = self.registers.a();
                    Alu8::add(a, adj)
                };
                self.registers.set_a(*result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_h(false);
            }
            Cpl => {
                let a = self.registers.a();
                self.registers.set_a(!a);
                self.registers.flags_mut().set_n(true).set_h(true);
            }
            Scf => {
                self.registers
                    .flags_mut()
                    .set_n(false)
                    .set_h(false)
                    .set_c(true);
            }
            Ccf => {
                let c = self.registers.flags().c();
                self.registers
                    .flags_mut()
                    .set_n(false)
                    .set_h(false)
                    .set_c(!c);
            }
            JrImm8 => {
                let imm8 = instruction.imm8().unwrap();
                let signed_imm = imm8 as i16;
                self.pc = self.pc.wrapping_add_signed(signed_imm);
            }
            JrCondImm8 => {
                let imm8 = instruction.imm8().unwrap();
                let signed_imm = imm8 as i16;
                self.pc = self.pc.wrapping_add_signed(signed_imm);
            }
            Stop => {
                panic!("STOP instruction encountered");
            }
            // Block 0b01
            LdR8R8 => {
                let (r8_src, r8_dst) = instruction.r8_pair().unwrap();
                let r_val = self.read_r8(r8_src);
                self.write_r8(r8_dst, r_val);
            }
            Halt => {
                panic!("HALT instruction encountered");
            }
            // Block 0b10
            AddAR8 => {
                let r_val = self.read_r8(instruction.r8().unwrap());
                let r_a = self.registers.a();
                let result = Alu8::add(r_a, r_val);
                self.registers.set_a(*result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            AdcAR8 => {
                let r_val = self.read_r8(instruction.r8().unwrap());
                let r_a = self.registers.a();
                let carry = self.registers.flags().c_u8();
                let result = Alu8::adc(r_a, r_val, carry);
                self.registers.set_a(*result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            SubAR8 => {
                let r_val = self.read_r8(instruction.r8().unwrap());
                let r_a = self.registers.a();
                let result = Alu8::sub(r_a, r_val);
                self.registers.set_a(*result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(true)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            SbcAR8 => {
                let r_val = self.read_r8(instruction.r8().unwrap());
                let r_a = self.registers.a();
                let carry = self.registers.flags().c_u8();
                let result = Alu8::sbc(r_a, r_val, carry);
                self.registers.set_a(*result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(true)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            AndAR8 => {
                let r_val = self.read_r8(instruction.r8().unwrap());
                let r_a = self.registers.a();
                let result = r_val & r_a;
                self.registers.set_a(result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(result)
                    .set_n(false)
                    .set_h(true)
                    .set_c(false);
            }
            XorAR8 => {
                let r_val = self.read_r8(instruction.r8().unwrap());
                let r_a = self.registers.a();
                let result = r_val ^ r_a;
                self.registers.set_a(result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(false);
            }
            OrAR8 => {
                let r_val = self.read_r8(instruction.r8().unwrap());
                let r_a = self.registers.a();
                let result = r_val | r_a;
                self.registers.set_a(result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(false);
            }
            CpAR8 => {
                let r_val = self.read_r8(instruction.r8().unwrap());
                let r_a = self.registers.a();
                let result = Alu8::sub(r_a, r_val);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(true)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            RetCond => {
                todo!("Implement RetCond");
            }
            Ret => {
                todo!("Implement Ret");
            }
            RetI => {
                todo!("Implement RetI");
            }
            JpCondImm16 => {
                let imm16 = instruction.imm16().unwrap();
                if self.check_condition(instruction.cond().unwrap()) {
                    self.pc = imm16;
                }
            }
            JpImm16 => {
                let imm16 = instruction.imm16().unwrap();
                self.pc = imm16;
            }
            JpHl => {
                let hl = self.registers.hl();
                self.pc = hl;
            }
            CallCondImm16 => {
                let imm16 = instruction.imm16().unwrap();
                if self.check_condition(instruction.cond().unwrap()) {
                    self.sp = self.sp.wrapping_sub(2);
                    self.ram.write_u16(self.sp, self.pc);
                    self.pc = imm16;
                }
            }
            _ => unreachable!(),
        }
    }
}

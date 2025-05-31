use crate::alu::{Alu8, Alu16};
use crate::ime::Ime;
use crate::instructions::{ConditionCode, Instruction, InstructionType};
use crate::registers::Registers;
use arbitrary_int::{u2, u3};
use yagber_ram::{InterruptType, Memory, Ram};

#[derive(Debug, Clone, Copy)]
pub struct Cpu {
    pc: u16,
    sp: u16,
    registers: Registers,
    ime: Ime,
    busy: u16,
    halt: bool,
    halt_bug: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0x0000,
            sp: 0x0000,
            registers: Registers::default(),
            ime: Ime::default(),
            busy: Default::default(),
            halt: false,
            halt_bug: false,
        }
    }

    /// Perform a single CPU step
    /// Respects instruction timing
    /// Represents a single M-cycle
    pub fn step(&mut self, ram: &mut Ram) {
        // If the CPU is busy, decrement the busy counter
        if self.busy > 0 {
            self.busy -= 1;
            return;
        }

        // If an interrupt is pending, the CPU wakes up from halt
        if self.any_interrupt_pending(ram) {
            self.halt = false;
        }

        // If the CPU is halted, do nothing
        if self.halt {
            return;
        }

        // Check for interrupts
        self.check_interrupt(ram);
        if self.busy != 0 {
            return;
        }

        // Handle interrupts
        if self.ime.interrupt_handling() {
            self.handle_interrupt(ram);
            self.ime.reset_interrupt_handling();
            return;
        }

        // Perform a step
        self.instruction_step(ram);
    }

    /// Perform a single CPU step
    /// returns the number of M-cycles taken by the instruction
    fn instruction_step(&mut self, ram: &mut Ram) {
        // Fetch the next instruction
        let instruction = self.read_instruction(ram);
        trace!("{:?}", instruction);

        if self.pc == 0x0100 {
            trace!("Boot Rom Completed, Starting cartridge");
        }

        // Execute the instruction
        self.execute_instruction(ram, &instruction);

        // Update the IME
        self.ime.update_ime();

        // number of M-cycles taken by the instruction
        self.busy = instruction.cycles() - 1;
    }

    fn read_instruction(&mut self, ram: &mut Ram) -> Instruction {
        // Fetch the next instruction
        let opcode = self.read_next_byte(ram);

        // Decode the instruction
        let mut instruction = Instruction::new(opcode);
        if *instruction.instruction_type() == InstructionType::Prefix {
            // Handle prefix instructions
            let prefix_opcode = self.read_next_byte(ram);
            instruction = Instruction::new_cb_prefix(prefix_opcode)
        }

        // Check if the instruction needs more bytes
        if instruction.requires_imm8() {
            // Read the immediate value
            let imm8 = self.read_next_byte(ram);
            instruction.set_imm8(imm8);
        }

        if instruction.requires_imm16() {
            // Read the immediate value
            let lo = self.read_next_byte(ram);
            let hi = self.read_next_byte(ram);
            let imm16 = u16::from_le_bytes([lo, hi]);
            instruction.set_imm16(imm16);
        }

        instruction
    }

    fn read_next_byte(&mut self, ram: &mut Ram) -> u8 {
        let byte = ram.read(self.pc);
        // If the halt bug is triggered the cpu fails to increment the PC
        if self.halt_bug {
            self.halt_bug = false;
        } else {
            self.pc += 1;
        }
        byte
    }

    fn read_r8(&self, r8: u3, ram: &mut Ram) -> u8 {
        match r8.value() {
            0 => self.registers.b(),
            1 => self.registers.c(),
            2 => self.registers.d(),
            3 => self.registers.e(),
            4 => self.registers.h(),
            5 => self.registers.l(),
            6 => ram.read(self.registers.hl()),
            7 => self.registers.a(),
            _ => unreachable!(),
        }
    }

    fn write_r8(&mut self, r8: u3, value: u8, ram: &mut Ram) {
        match r8.value() {
            0 => self.registers.set_b(value),
            1 => self.registers.set_c(value),
            2 => self.registers.set_d(value),
            3 => self.registers.set_e(value),
            4 => self.registers.set_h(value),
            5 => self.registers.set_l(value),
            6 => ram.write(self.registers.hl(), value),
            7 => self.registers.set_a(value),
            _ => unreachable!(),
        }
    }

    fn read_r16(&self, r16: u2) -> u16 {
        match r16.value() {
            0 => self.registers.bc(),
            1 => self.registers.de(),
            2 => self.registers.hl(),
            3 => self.sp,
            _ => unreachable!(),
        }
    }

    fn write_r16(&mut self, r16: u2, value: u16) {
        match r16.value() {
            0 => self.registers.set_bc(value),
            1 => self.registers.set_de(value),
            2 => self.registers.set_hl(value),
            3 => self.sp = value,
            _ => unreachable!(),
        }
    }

    fn read_r16stk(&self, r16: u2) -> u16 {
        match r16.value() {
            0 => self.registers.bc(),
            1 => self.registers.de(),
            2 => self.registers.hl(),
            3 => self.registers.af(),
            _ => unreachable!(),
        }
    }

    fn write_r16stk(&mut self, r16: u2, value: u16) {
        match r16.value() {
            0 => self.registers.set_bc(value),
            1 => self.registers.set_de(value),
            2 => self.registers.set_hl(value),
            3 => self.registers.set_af(value),
            _ => unreachable!(),
        }
    }

    fn read_r16mem(&mut self, r16: u2) -> u16 {
        match r16.value() {
            0 => self.registers.bc(),
            1 => self.registers.de(),
            2 => self.registers.hl_inc(),
            3 => self.registers.hl_dec(),
            _ => unreachable!(),
        }
    }

    fn check_condition(&self, condition: ConditionCode) -> bool {
        match condition.value() {
            0 => !self.registers.flags().z(), // NZ
            1 => self.registers.flags().z(),  // Z
            2 => !self.registers.flags().c(), // NC
            3 => self.registers.flags().c(),  // C
            _ => unreachable!(),
        }
    }

    fn stack_push(&mut self, ram: &mut Ram, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        // Game Boy stack must stay in WRAM/echo region (0xC000–0xFDFF)
        if self.sp < 0xC000 {
            panic!(
                "Stack overflow! SP went into invalid memory: {:#06X}",
                self.sp
            );
        }
        ram.write_u16(self.sp, value);
    }

    fn stack_pop(&mut self, ram: &mut Ram) -> u16 {
        // optionally guard underflow on pop
        if self.sp > 0xFFFE {
            panic!("Stack underflow! SP is already at {:#06X}", self.sp);
        }
        let value = ram.read_u16(self.sp);
        self.sp = self.sp.wrapping_add(2);
        value
    }

    fn call(&mut self, ram: &mut Ram, address: u16) {
        trace!("Calling to {:#06X}", address);
        self.stack_push(ram, self.pc);
        self.pc = address;
    }

    fn any_interrupt_pending(&self, ram: &Ram) -> bool {
        let ei = ram.read(InterruptType::IE_ADDRESS);
        let fi = ram.read(InterruptType::IF_ADDRESS);
        ei & fi != 0
    }

    fn check_interrupt(&mut self, ram: &mut Ram) {
        if self.any_interrupt_pending(ram) && self.ime.ime() {
            self.busy = 2; // 2 M-Cycles of NOP
            self.ime.set_interrupt_handling();
        }
    }

    fn handle_interrupt(&mut self, ram: &mut Ram) {
        let interrupt = ram.get_priority_interrupt();
        if let Some(interrupt) = interrupt {
            trace!("Handling interrupt: {:?}", interrupt);
            ram.clear_interrupt(interrupt);
            self.call(ram, interrupt.address());
            self.busy = 2; // 2 more M-Cycles of NOP
        } else {
            warn!("Requested interrupt handling but no interrupt is pending");
        }
    }

    fn execute_instruction(&mut self, ram: &mut Ram, instruction: &Instruction) {
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
                let hl = self.read_r16mem(r16);
                let a = self.registers.a();
                ram.write(hl, a);
            }
            LdAR16mem => {
                let r16 = instruction.r16().unwrap();
                let address = self.read_r16mem(r16);
                let a = ram.read(address);
                self.registers.set_a(a);
            }
            LdImm16Sp => {
                let imm16 = instruction.imm16().unwrap();
                ram.write_u16(imm16, self.sp);
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
                let r_val = self.read_r8(r8, ram);
                let result = Alu8::inc(r_val);
                self.write_r8(r8, *result, ram);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(result.cb3);
            }
            DecR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let result = Alu8::dec(r_val);
                self.write_r8(r8, *result, ram);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(true)
                    .set_h(result.cb3);
            }
            LdR8Imm8 => {
                let r8 = instruction.r8().unwrap();
                let imm8 = instruction.imm8().unwrap();
                self.write_r8(r8, imm8, ram);
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
                let a = self.registers.a();
                let flags = self.registers.flags();
                let mut carry = false;
                let result = if !flags.n() {
                    let mut adj = 0;
                    if flags.c() || a > 0x99 {
                        adj += 0x60;
                        carry = true;
                    }
                    if flags.h() || (a & 0x0F) > 0x09 {
                        adj += 0x06;
                    }
                    Alu8::add(a, adj)
                } else {
                    let mut adj = 0;
                    if flags.c() {
                        adj += 0x60;
                        carry = true;
                    }
                    if flags.h() {
                        adj += 0x06;
                    }
                    Alu8::sub(a, adj)
                };
                self.registers.set_a(*result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_h(false)
                    .set_c(carry);
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
                let signed_imm = (imm8 as i8) as i16;
                self.pc = self.pc.wrapping_add_signed(signed_imm);
                trace!("Jumping to {:#06X}", self.pc);
            }
            JrCondImm8 => {
                let imm8 = instruction.imm8().unwrap();
                let cc = instruction.cond().unwrap();
                if self.check_condition(cc) {
                    // sign‐extend 8→16 and add to PC
                    let offset = (imm8 as i8) as i16;
                    self.pc = self.pc.wrapping_add_signed(offset);
                    trace!("Jumping to {:#06X}", self.pc);
                }
            }
            Stop => {
                panic!("STOP instruction encountered");
            }
            // Block 0b01
            LdR8R8 => {
                let (r8_dst, r8_src) = instruction.r8_pair().unwrap();
                let r_val = self.read_r8(r8_src, ram);
                self.write_r8(r8_dst, r_val, ram);
            }
            Halt => {
                if !self.ime.ime() && self.any_interrupt_pending(ram) {
                    self.halt_bug = true;
                } else {
                    self.halt = true;
                }
            }
            // Block 0b10
            AddAR8 => {
                let r_val = self.read_r8(instruction.r8().unwrap(), ram);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), ram);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), ram);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), ram);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), ram);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), ram);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), ram);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), ram);
                let r_a = self.registers.a();
                let result = Alu8::sub(r_a, r_val);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(true)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            // Block 0b11
            AddAImm8 => {
                let a = self.registers.a();
                let imm8 = instruction.imm8().unwrap();
                let result = Alu8::add(a, imm8);
                self.registers.set_a(*result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            AdcAImm8 => {
                let a = self.registers.a();
                let imm8 = instruction.imm8().unwrap();
                let carry = self.registers.flags().c_u8();
                let result = Alu8::adc(a, imm8, carry);
                self.registers.set_a(*result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            SubAImm8 => {
                let a = self.registers.a();
                let imm8 = instruction.imm8().unwrap();
                let result = Alu8::sub(a, imm8);
                self.registers.set_a(*result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(true)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            SbcAImm8 => {
                let a = self.registers.a();
                let imm8 = instruction.imm8().unwrap();
                let carry = self.registers.flags().c_u8();
                let result = Alu8::sbc(a, imm8, carry);
                self.registers.set_a(*result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(true)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            AndAImm8 => {
                let a = self.registers.a();
                let imm8 = instruction.imm8().unwrap();
                let result = a & imm8;
                self.registers.set_a(result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(result)
                    .set_n(false)
                    .set_h(true)
                    .set_c(false);
            }
            XorAImm8 => {
                let a = self.registers.a();
                let imm8 = instruction.imm8().unwrap();
                let result = a ^ imm8;
                self.registers.set_a(result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(false);
            }
            OrAImm8 => {
                let a = self.registers.a();
                let imm8 = instruction.imm8().unwrap();
                let result = a | imm8;
                self.registers.set_a(result);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(false);
            }
            CpAImm8 => {
                let a = self.registers.a();
                let imm8 = instruction.imm8().unwrap();
                let result = Alu8::sub(a, imm8);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(true)
                    .set_h(result.cb3)
                    .set_c(result.cb7);
            }
            RetCond => {
                let condition = instruction.cond().unwrap();
                if self.check_condition(condition) {
                    let value = self.stack_pop(ram);
                    self.pc = value;
                }
            }
            Ret => {
                let value = self.stack_pop(ram);
                self.pc = value;
            }
            RetI => {
                let value = self.stack_pop(ram);
                self.pc = value;
                self.ime.set_ime();
            }
            JpCondImm16 => {
                let imm16 = instruction.imm16().unwrap();
                if self.check_condition(instruction.cond().unwrap()) {
                    self.pc = imm16;
                    trace!("Jumping to {:#06X}", imm16);
                }
            }
            JpImm16 => {
                let imm16 = instruction.imm16().unwrap();
                self.pc = imm16;
                trace!("Jumping to {:#06X}", imm16);
            }
            JpHl => {
                let hl = self.registers.hl();
                self.pc = hl;
                trace!("Jumping to HL: {:#06X}", hl);
            }
            CallCondImm16 => {
                let imm16 = instruction.imm16().unwrap();
                if self.check_condition(instruction.cond().unwrap()) {
                    self.call(ram, imm16);
                }
            }
            CallImm16 => {
                let imm16 = instruction.imm16().unwrap();
                self.call(ram, imm16);
            }
            RstTgt3 => {
                let tgt = instruction.tgt3().unwrap();
                let addr = (tgt.value() as u16) << 3;
                self.call(ram, addr);
            }
            PopR16stk => {
                let r16 = instruction.r16().unwrap();
                let value = self.stack_pop(ram);
                self.write_r16stk(r16, value);
            }
            PushR16stk => {
                let r16 = instruction.r16().unwrap();
                let value = self.read_r16stk(r16);
                self.stack_push(ram, value);
            }
            Prefix => trace!("Prefix instruction encountered"),
            LdhCA => {
                let a = self.registers.a();
                let c = self.registers.c() as u16;
                let address = 0xFF00 + c;
                ram.write(address, a);
            }
            LdhImm8A => {
                let a = self.registers.a();
                let imm8 = instruction.imm8().unwrap() as u16;
                let address = 0xFF00 + imm8;
                ram.write(address, a);
            }
            LdImm16A => {
                let a = self.registers.a();
                let imm16 = instruction.imm16().unwrap();
                ram.write(imm16, a);
            }
            LdhAC => {
                let c = self.registers.c() as u16;
                let address = 0xFF00 + c;
                let a = ram.read(address);
                self.registers.set_a(a);
            }
            LdhAImm8 => {
                let imm8 = instruction.imm8().unwrap() as u16;
                let address = 0xFF00 + imm8;
                let a = ram.read(address);
                self.registers.set_a(a);
            }
            LdAImm16 => {
                let imm16 = instruction.imm16().unwrap();
                let a = ram.read(imm16);
                self.registers.set_a(a);
            }
            AddSpImm8 => {
                let imm8 = instruction.imm8().unwrap();
                let sp = self.sp;
                let result = Alu16::add(sp, imm8 as u16);
                self.sp = *result;

                self.registers
                    .flags_mut()
                    .set_z(false)
                    .set_n(false)
                    .set_h(result.cb11)
                    .set_c(result.cb15);
            }
            LdHlSpImm8 => {
                let imm8 = instruction.imm8().unwrap() as i8;
                let sp = self.sp;
                let result = Alu16::add(sp, imm8 as u16);
                debug!("LD HL, {} + {} -> {:?}", sp, imm8, *result);
                self.registers.set_hl(*result);
                self.registers
                    .flags_mut()
                    .set_z(false)
                    .set_n(false)
                    .set_h(result.cb11)
                    .set_c(result.cb15);
            }
            LdSpHl => {
                let hl = self.registers.hl();
                self.sp = hl;
            }
            Di => {
                self.ime.reset_ime();
            }
            Ei => {
                self.ime.set_ime();
            }
            // CB Prefix instructions
            RlcR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let result = Alu8::rlc(r_val);
                self.write_r8(r8, *result, ram);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            RrcR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let result = Alu8::rrc(r_val);
                self.write_r8(r8, *result, ram);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            RlR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let carry = self.registers.flags().c_u8();
                let result = Alu8::rl(r_val, carry);
                self.write_r8(r8, *result, ram);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            RrR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let carry = self.registers.flags().c_u8();
                let result = Alu8::rr(r_val, carry);
                self.write_r8(r8, *result, ram);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            SlaR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let result = Alu8::sla(r_val);
                self.write_r8(r8, *result, ram);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            SraR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let result = Alu8::sra(r_val);
                self.write_r8(r8, *result, ram);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            SwapR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let result = r_val.rotate_right(4);
                self.write_r8(r8, result, ram);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(false);
            }
            SrlR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let result = Alu8::srl(r_val);
                self.write_r8(r8, *result, ram);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            BitB3R8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let bit = instruction.b3().unwrap();
                let result = r_val & (1 << bit.value());
                self.registers
                    .flags_mut()
                    .set_z_if_zero(result)
                    .set_n(false)
                    .set_h(true);
            }
            ResB3R8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let bit = instruction.b3().unwrap();
                let result = r_val & !(1 << bit.value());
                self.write_r8(r8, result, ram);
            }
            SetB3R8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, ram);
                let bit = instruction.b3().unwrap();
                let result = r_val | (1 << bit.value());
                self.write_r8(r8, result, ram);
            }
        }
    }
}

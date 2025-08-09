use crate::alu::{Alu16, Alu8};
use crate::ime::Ime;
use crate::instructions::{ConditionCode, Instruction, InstructionType};
use crate::registers::Registers;
use arbitrary_int::{u2, u3};
use yagber_memory::{Bus, Memory};

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

    pub fn on_mcycle(emulator: &mut yagber_app::Emulator) {
        #[cfg(feature = "trace-span")]
        let _span = tracing::info_span!("cpu step").entered();

        let (cpu, bus) = emulator
            .get_components_mut2::<Cpu, Bus>()
            .expect("Cpu and/or Bus component missing");
        cpu.step(bus);
    }

    /// Perform a single CPU step
    /// Respects instruction timing
    /// Represents a single M-cycle
    pub fn step(&mut self, bus: &mut Bus) {
        // If the CPU is busy, decrement the busy counter
        if self.busy > 0 {
            self.busy -= 1;
            return;
        }

        // If an interrupt is pending, the CPU wakes up from halt
        if self.any_interrupt_pending(bus) {
            self.halt = false;
        }

        // If the CPU is halted, do nothing
        if self.halt {
            return;
        }

        // Check for interrupts
        self.check_interrupt(bus);
        if self.busy != 0 {
            return;
        }

        // Handle interrupts
        if self.ime.interrupt_handling() {
            self.handle_interrupt(bus);
            self.ime.reset_interrupt_handling();
            return;
        }

        // Perform a step
        self.instruction_step(bus);
    }

    /// Perform a single CPU step
    /// returns the number of M-cycles taken by the instruction
    fn instruction_step(&mut self, bus: &mut Bus) {
        if self.pc == 0x0100 {
            #[cfg(feature = "trace")]
            tracing::info!("Boot Rom Completed, Starting cartridge");
        }

        // Fetch the next instruction
        let instruction = self.read_instruction(bus);
        #[cfg(feature = "trace")]
        tracing::trace!("{:?}", instruction);

        // Execute the instruction
        self.execute_instruction(bus, &instruction);

        // Update the IME
        self.ime.update_ime();

        // number of M-cycles taken by the instruction
        self.busy = instruction.cycles() - 1;
    }

    fn read_instruction(&mut self, bus: &mut Bus) -> Instruction {
        // Fetch the next instruction
        let opcode = self.read_next_byte(bus);

        // Decode the instruction
        let mut instruction = Instruction::new(opcode);
        if *instruction.instruction_type() == InstructionType::Prefix {
            // Handle prefix instructions
            let prefix_opcode = self.read_next_byte(bus);
            instruction = Instruction::new_cb_prefix(prefix_opcode)
        }

        // Check if the instruction needs more bytes
        if instruction.requires_imm8() {
            // Read the immediate value
            let imm8 = self.read_next_byte(bus);
            instruction.set_imm8(imm8);
        }

        if instruction.requires_imm16() {
            // Read the immediate value
            let lo = self.read_next_byte(bus);
            let hi = self.read_next_byte(bus);
            let imm16 = u16::from_le_bytes([lo, hi]);
            instruction.set_imm16(imm16);
        }

        instruction
    }

    fn read_next_byte(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read(self.pc);
        // If the halt bug is triggered the cpu fails to increment the PC
        if self.halt_bug {
            self.halt_bug = false;
        } else {
            self.pc += 1;
        }
        byte
    }

    fn read_r8(&self, r8: u3, bus: &mut Bus) -> u8 {
        match r8.value() {
            0 => self.registers.b(),
            1 => self.registers.c(),
            2 => self.registers.d(),
            3 => self.registers.e(),
            4 => self.registers.h(),
            5 => self.registers.l(),
            6 => bus.read(self.registers.hl()),
            7 => self.registers.a(),
            _ => unreachable!(),
        }
    }

    fn write_r8(&mut self, r8: u3, value: u8, bus: &mut Bus) {
        match r8.value() {
            0 => self.registers.set_b(value),
            1 => self.registers.set_c(value),
            2 => self.registers.set_d(value),
            3 => self.registers.set_e(value),
            4 => self.registers.set_h(value),
            5 => self.registers.set_l(value),
            6 => bus.write(self.registers.hl(), value),
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

    fn stack_push(&mut self, bus: &mut Bus, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        // Game Boy stack must stay in WRAM/echo region (0xC000–0xFDFF)
        if self.sp < 0xC000 {
            panic!(
                "Stack overflow! SP went into invalid memory: {:#06X}",
                self.sp
            );
        }
        bus.write_u16(self.sp, value);
    }

    fn stack_pop(&mut self, bus: &mut Bus) -> u16 {
        // optionally guard underflow on pop
        if self.sp > 0xFFFE {
            panic!("Stack underflow! SP is already at {:#06X}", self.sp);
        }
        let value = bus.read_u16(self.sp);
        self.sp = self.sp.wrapping_add(2);
        value
    }

    fn call(&mut self, bus: &mut Bus, address: u16) {
        #[cfg(feature = "trace")]
        tracing::trace!("Calling to {:#06X}", address);
        self.stack_push(bus, self.pc);
        self.pc = address;
    }

    fn any_interrupt_pending(&self, bus: &mut Bus) -> bool {
        let ei = bus.read(yagber_memory::IOType::IE.address());
        let fi = bus.read(yagber_memory::IOType::IF.address());
        ei & fi != 0
    }

    fn check_interrupt(&mut self, bus: &mut Bus) {
        if self.any_interrupt_pending(bus) && self.ime.ime() {
            self.busy = 2; // 2 M-Cycles of NOP
            self.ime.set_interrupt_handling();
        }
    }

    fn handle_interrupt(&mut self, bus: &mut Bus) {
        let interrupt = bus.get_priority_interrupt();
        if let Some(interrupt) = interrupt {
            #[cfg(feature = "trace")]
            tracing::trace!("Handling interrupt: {:?}", interrupt);
            bus.clear_interrupt(interrupt);
            self.call(bus, interrupt.address());
            self.busy = 2; // 2 more M-Cycles of NOP
        } else {
            #[cfg(feature = "trace")]
            tracing::warn!("Requested interrupt handling but no interrupt is pending");
        }
    }

    pub fn freeze_for(&mut self, cycles: u16) {
        self.busy = cycles;
    }

    fn execute_instruction(&mut self, bus: &mut Bus, instruction: &Instruction) {
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
                bus.write(hl, a);
            }
            LdAR16mem => {
                let r16 = instruction.r16().unwrap();
                let address = self.read_r16mem(r16);
                let a = bus.read(address);
                self.registers.set_a(a);
            }
            LdImm16Sp => {
                let imm16 = instruction.imm16().unwrap();
                bus.write_u16(imm16, self.sp);
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
                let r_val = self.read_r8(r8, bus);
                let result = Alu8::inc(r_val);
                self.write_r8(r8, *result, bus);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(result.cb3);
            }
            DecR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, bus);
                let result = Alu8::dec(r_val);
                self.write_r8(r8, *result, bus);
                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(true)
                    .set_h(result.cb3);
            }
            LdR8Imm8 => {
                let r8 = instruction.r8().unwrap();
                let imm8 = instruction.imm8().unwrap();
                self.write_r8(r8, imm8, bus);
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
                #[cfg(feature = "trace")]
                tracing::trace!("Jumping to {:#06X}", self.pc);
            }
            JrCondImm8 => {
                let imm8 = instruction.imm8().unwrap();
                let cc = instruction.cond().unwrap();
                if self.check_condition(cc) {
                    // sign‐extend 8→16 and add to PC
                    let offset = (imm8 as i8) as i16;
                    self.pc = self.pc.wrapping_add_signed(offset);
                    #[cfg(feature = "trace")]
                    tracing::trace!("Jumping to {:#06X}", self.pc);
                }
            }
            Stop => {
                // Speed switch
                let spd = yagber_memory::Spd::from_bus(bus);
                if spd.speed_switch_armed() {
                    let current_speed = spd.speed_mode();
                    let new_speed = current_speed.toggle();
                    bus.write(yagber_memory::IOType::SPD.address(), new_speed.as_spd_bit());
                }
            }
            // Block 0b01
            LdR8R8 => {
                let (r8_dst, r8_src) = instruction.r8_pair().unwrap();
                let r_val = self.read_r8(r8_src, bus);
                self.write_r8(r8_dst, r_val, bus);
            }
            Halt => {
                if !self.ime.ime() && self.any_interrupt_pending(bus) {
                    self.halt_bug = true;
                } else {
                    self.halt = true;
                }
            }
            // Block 0b10
            AddAR8 => {
                let r_val = self.read_r8(instruction.r8().unwrap(), bus);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), bus);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), bus);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), bus);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), bus);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), bus);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), bus);
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
                let r_val = self.read_r8(instruction.r8().unwrap(), bus);
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
                    let value = self.stack_pop(bus);
                    self.pc = value;
                }
            }
            Ret => {
                let value = self.stack_pop(bus);
                self.pc = value;
            }
            RetI => {
                let value = self.stack_pop(bus);
                self.pc = value;
                self.ime.set_ime();
            }
            JpCondImm16 => {
                let imm16 = instruction.imm16().unwrap();
                if self.check_condition(instruction.cond().unwrap()) {
                    self.pc = imm16;
                    #[cfg(feature = "trace")]
                    tracing::trace!("Jumping to {:#06X}", imm16);
                }
            }
            JpImm16 => {
                let imm16 = instruction.imm16().unwrap();
                self.pc = imm16;
                #[cfg(feature = "trace")]
                tracing::trace!("Jumping to {:#06X}", imm16);
            }
            JpHl => {
                let hl = self.registers.hl();
                self.pc = hl;
                #[cfg(feature = "trace")]
                tracing::trace!("Jumping to HL: {:#06X}", hl);
            }
            CallCondImm16 => {
                let imm16 = instruction.imm16().unwrap();
                if self.check_condition(instruction.cond().unwrap()) {
                    self.call(bus, imm16);
                }
            }
            CallImm16 => {
                let imm16 = instruction.imm16().unwrap();
                self.call(bus, imm16);
            }
            RstTgt3 => {
                let tgt = instruction.tgt3().unwrap();
                let addr = (tgt.value() as u16) << 3;
                self.call(bus, addr);
            }
            PopR16stk => {
                let r16 = instruction.r16().unwrap();
                let value = self.stack_pop(bus);
                self.write_r16stk(r16, value);
            }
            PushR16stk => {
                let r16 = instruction.r16().unwrap();
                let value = self.read_r16stk(r16);
                self.stack_push(bus, value);
            }
            Prefix => {
                #[cfg(feature = "trace")]
                tracing::trace!("Prefix instruction encountered");
            }
            LdhCA => {
                let a = self.registers.a();
                let c = self.registers.c() as u16;
                let address = 0xFF00 + c;
                bus.write(address, a);
            }
            LdhImm8A => {
                let a = self.registers.a();
                let imm8 = instruction.imm8().unwrap() as u16;
                let address = 0xFF00 + imm8;
                bus.write(address, a);
            }
            LdImm16A => {
                let a = self.registers.a();
                let imm16 = instruction.imm16().unwrap();
                bus.write(imm16, a);
            }
            LdhAC => {
                let c = self.registers.c() as u16;
                let address = 0xFF00 + c;
                let a = bus.read(address);
                self.registers.set_a(a);
            }
            LdhAImm8 => {
                let imm8 = instruction.imm8().unwrap() as u16;
                let address = 0xFF00 + imm8;
                let a = bus.read(address);
                self.registers.set_a(a);
            }
            LdAImm16 => {
                let imm16 = instruction.imm16().unwrap();
                let a = bus.read(imm16);
                self.registers.set_a(a);
            }
            AddSpImm8 => {
                let sp = self.sp;
                let imm8 = instruction.imm8().unwrap();
                let val_u16 = imm8 as i8 as u16; // sign-extend into 16-bit two's-complement

                // This does not use the Alu16::add function because it uses 8-bit flags
                let result = sp.wrapping_add(val_u16);
                let half_carry = ((sp & 0x0F) + (val_u16 & 0x0F)) > 0x0F;
                let carry = ((sp & 0xFF) + (val_u16 & 0xFF)) > 0xFF;

                self.sp = result;
                self.registers
                    .flags_mut()
                    .set_z(false)
                    .set_n(false)
                    .set_h(half_carry)
                    .set_c(carry);
            }
            LdHlSpImm8 => {
                let sp = self.sp;
                let imm8 = instruction.imm8().unwrap();
                let val_u16 = imm8 as i8 as u16; // sign-extend into 16-bit two's-complement

                // This does not use the Alu16::add function because it uses 8-bit flags
                let result = sp.wrapping_add(val_u16);
                let half_carry = ((sp & 0x0F) + (val_u16 & 0x0F)) > 0x0F;
                let carry = ((sp & 0xFF) + (val_u16 & 0xFF)) > 0xFF;

                self.registers.set_hl(result);
                self.registers
                    .flags_mut()
                    .set_z(false)
                    .set_n(false)
                    .set_h(half_carry)
                    .set_c(carry);
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
                let r_val = self.read_r8(r8, bus);
                let result = Alu8::rlc(r_val);
                self.write_r8(r8, *result, bus);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            RrcR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, bus);
                let result = Alu8::rrc(r_val);
                self.write_r8(r8, *result, bus);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            RlR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, bus);
                let carry = self.registers.flags().c_u8();
                let result = Alu8::rl(r_val, carry);
                self.write_r8(r8, *result, bus);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            RrR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, bus);
                let carry = self.registers.flags().c_u8();
                let result = Alu8::rr(r_val, carry);
                self.write_r8(r8, *result, bus);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            SlaR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, bus);
                let result = Alu8::sla(r_val);
                self.write_r8(r8, *result, bus);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            SraR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, bus);
                let result = Alu8::sra(r_val);
                self.write_r8(r8, *result, bus);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            SwapR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, bus);
                let result = r_val.rotate_right(4);
                self.write_r8(r8, result, bus);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(false);
            }
            SrlR8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, bus);
                let result = Alu8::srl(r_val);
                self.write_r8(r8, *result, bus);

                self.registers
                    .flags_mut()
                    .set_z_if_zero(*result)
                    .set_n(false)
                    .set_h(false)
                    .set_c(result.cb7);
            }
            BitB3R8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, bus);
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
                let r_val = self.read_r8(r8, bus);
                let bit = instruction.b3().unwrap();
                let result = r_val & !(1 << bit.value());
                self.write_r8(r8, result, bus);
            }
            SetB3R8 => {
                let r8 = instruction.r8().unwrap();
                let r_val = self.read_r8(r8, bus);
                let bit = instruction.b3().unwrap();
                let result = r_val | (1 << bit.value());
                self.write_r8(r8, result, bus);
            }
        }
    }
}

impl yagber_app::Component for Cpu {}

use crate::utils::{bytes_to_word, word_to_bytes};

use super::{
    instructions::{
        Instruction, JumpCondition, LoadType, PrefixedInstruction, RegisterSideEffect, XORTarget,
    },
    registers::{Flag, Register, RegisterFile},
};

pub struct CPU {
    program_counter: usize,
    registers: RegisterFile,
    memory: Box<[u8; 65536]>,
}

impl CPU {
    pub fn new() -> Self {
        let registers = RegisterFile::new();
        let memory = Box::new([0; 65536]);

        CPU {
            program_counter: 0,
            registers,
            memory,
        }
    }

    fn get_immediate_word(&mut self) -> u16 {
        self.program_counter += 1;
        let lower_byte = self.read_memory(self.program_counter);
        self.program_counter += 1;
        let higher_byte = self.read_memory(self.program_counter);

        bytes_to_word(higher_byte, lower_byte)
    }

    fn read_memory(&self, address: usize) -> u8 {
        self.memory[address]
    }

    fn write_memory(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    fn execute_load_instruction(&mut self, load_type: LoadType) {
        match load_type {
            LoadType::ImmediateWord(reg) => {
                let word = self.get_immediate_word();
                self.registers.write_register(reg, word);
            }

            LoadType::ImmediateByte(reg) => {
                self.program_counter += 1;
                let byte = self.read_memory(self.program_counter);

                self.registers.write_register(reg, byte as u16);
            }

            LoadType::RegToReg(reg, other_reg) => {
                let value = self.registers.read_register(other_reg);
                self.registers.write_register(reg, value);
            }

            LoadType::ImmediateByteToMemory(reg) => {
                self.program_counter += 1;
                let byte = self.read_memory(self.program_counter);
                let address = self.registers.read_register(reg);

                self.write_memory(address as usize, byte);
            }
            LoadType::StackPointerToMemory => {
                let address = self.get_immediate_word() as usize;
                let sp = self.registers.read_register(Register::StackPointer);

                let (high, low) = word_to_bytes(sp);

                self.write_memory(address as usize, low);
                self.write_memory((address + 1) as usize, high);
            }

            LoadType::FromMemory(destination, address_reg) => {
                let address = self.registers.read_register(address_reg) as usize;
                let value = self.read_memory(address) as u16;

                self.registers.write_register(destination, value);
            }

            LoadType::FromMemoryWithSideEffect(reg, side_effect) => {
                let address = self.registers.read_register(reg.clone()) as usize;
                let value = self.read_memory(address) as u16;

                self.registers.write_register(Register::A, value);

                match side_effect {
                    RegisterSideEffect::Inc => {
                        self.registers.write_register(reg, address as u16 + 1)
                    }
                    RegisterSideEffect::Dec => {
                        self.registers.write_register(reg, address as u16 - 1)
                    }
                }
            }

            LoadType::ToMemory(address_reg, source) => {
                let address = self.registers.read_register(address_reg) as usize;
                let value = self.registers.read_register(source);

                self.write_memory(address, value as u8);
            }

            LoadType::ToMemoryWithSideEffect(reg, side_effect) => {
                let address = self.registers.read_register(reg.clone()) as usize;
                let value = self.registers.read_register(Register::A);

                self.write_memory(address, value as u8);

                match side_effect {
                    RegisterSideEffect::Inc => {
                        self.registers.write_register(reg, address as u16 + 1)
                    }
                    RegisterSideEffect::Dec => {
                        self.registers.write_register(reg, address as u16 - 1)
                    }
                }
            }
        }
    }

    fn execute_xor_instruction(&mut self, target: XORTarget) {
        match target {
            XORTarget::Register(reg) => {
                let a_reg = self.registers.read_register(Register::A);
                let val = self.registers.read_register(reg);
                let xor_result = a_reg ^ val;

                self.registers.write_register(Register::A, xor_result);
                // Clear flags register
                self.registers.write_register(Register::F, 0);
                self.registers.set_flag(Flag::Z, xor_result == 0);
            }
        }
    }

    fn execute_bit_instruction(&mut self, index: u8, reg: Register) {
        let value = self.registers.read_register(reg);
        let bit = (value >> index) & 1 != 0;

        self.registers.set_flag(Flag::Z, !bit);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::H, true);
    }

    fn execute_jump_relative(&mut self, condition: JumpCondition) {
        self.program_counter += 1;
        let steps = self.read_memory(self.program_counter);

        match condition {
            JumpCondition::NegatedFlag(flag) => {
                if !self.registers.get_flag(flag) {
                    // We always increment the counter so ensure we go one back to land on the correct address
                    self.program_counter += (steps - 1) as usize;
                }
            }
        }
    }

    fn execute_prefixed_instruction(&mut self) {
        // All prefixed instructions are 2 bytes long
        self.program_counter += 1;

        let opcode = self.read_memory(self.program_counter);
        let instruction = PrefixedInstruction::decode(opcode);

        match instruction {
            PrefixedInstruction::Bit(index, reg) => self.execute_bit_instruction(index, reg),
            PrefixedInstruction::NoOp => (),
        };
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::XOR(target) => self.execute_xor_instruction(target),
            Instruction::Load(load_type) => self.execute_load_instruction(load_type),
            Instruction::Prefixed => self.execute_prefixed_instruction(),
            Instruction::JumpRelative(condition) => self.execute_jump_relative(condition),
            Instruction::NoOp => (),
        };

        // Increment program counter
        self.program_counter += 1;
    }

    pub fn step(&mut self) {
        let opcode = self.read_memory(self.program_counter);

        let instruction = Instruction::decode(opcode);

        self.execute(instruction);
    }
}

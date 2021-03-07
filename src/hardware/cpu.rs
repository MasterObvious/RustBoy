use crate::utils::bytes_to_word;

use super::{
    instructions::{Instruction, LoadType, XORTarget},
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

    fn execute_load_instruction(&mut self, load_type: LoadType) {
        match load_type {
            LoadType::ImmediateWord(reg) => {
                self.program_counter += 1;
                let lower_byte = self.memory[self.program_counter];
                self.program_counter += 1;
                let higher_byte = self.memory[self.program_counter];

                let word = bytes_to_word(higher_byte, lower_byte);

                self.registers.write_register(reg, word);
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

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::XOR(target) => self.execute_xor_instruction(target),
            Instruction::Load(load_type) => self.execute_load_instruction(load_type),
            Instruction::NoOp => (),
        };

        // Increment program counter
        self.program_counter += 1;
    }

    pub fn step(&mut self) {
        let opcode = self.memory[self.program_counter];

        let instruction = Instruction::decode(opcode);

        self.execute(instruction);
    }
}

use crate::utils::bytes_to_word;

use super::{
    instructions::{Instruction, LoadType},
    registers::RegisterFile,
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

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
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

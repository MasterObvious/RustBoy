use super::registers::RegisterFile;

pub struct CPU {
    program_counter: u16,
    registers: RegisterFile,
}

impl CPU {
    pub fn new() -> Self {
        let registers = RegisterFile::new();

        CPU {
            program_counter: 0,
            registers,
        }
    }
}

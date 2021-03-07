use super::registers::Register;

pub enum LoadType {
    ImmediateWord(Register),
}

pub enum Instruction {
    Load(LoadType),
    NoOp,
}

impl Instruction {
    pub fn decode(opcode: u8) -> Self {
        match opcode {
            0x31 => Instruction::Load(LoadType::ImmediateWord(Register::StackPointer)),
            _ => Instruction::NoOp,
        }
    }
}

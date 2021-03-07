use super::registers::Register;

pub enum LoadType {
    ImmediateWord(Register),
}

pub enum XORTarget {
    Register(Register),
}

pub enum Instruction {
    Load(LoadType),
    XOR(XORTarget),
    NoOp,
}

impl Instruction {
    pub fn decode(opcode: u8) -> Self {
        match opcode {
            0x31 => Instruction::Load(LoadType::ImmediateWord(Register::StackPointer)),
            0xAF => Instruction::XOR(XORTarget::Register(Register::A)),
            _ => Instruction::NoOp,
        }
    }
}

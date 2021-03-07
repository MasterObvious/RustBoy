use super::registers::{Flag, Register};

pub enum LoadType {
    ImmediateWord(Register),
}

pub enum XORTarget {
    Register(Register),
}

pub enum JumpCondition {
    NegatedFlag(Flag),
}

pub enum Instruction {
    Load(LoadType),
    XOR(XORTarget),
    JumpRelative(JumpCondition),
    Prefixed,
    NoOp,
}

impl Instruction {
    pub fn decode(opcode: u8) -> Self {
        match opcode {
            0x20 => Instruction::JumpRelative(JumpCondition::NegatedFlag(Flag::Z)),
            0x31 => Instruction::Load(LoadType::ImmediateWord(Register::StackPointer)),
            0xAF => Instruction::XOR(XORTarget::Register(Register::A)),
            0xCB => Instruction::Prefixed,
            _ => Instruction::NoOp,
        }
    }
}

pub enum PrefixedInstruction {
    Bit(u8, Register),
    NoOp,
}

impl PrefixedInstruction {
    pub fn decode(opcode: u8) -> Self {
        match opcode {
            0x7C => PrefixedInstruction::Bit(7, Register::H),
            _ => PrefixedInstruction::NoOp,
        }
    }
}

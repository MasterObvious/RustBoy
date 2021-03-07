use super::registers::{Flag, Register};

pub enum RegisterSideEffect {
    Inc,
    Dec,
}
pub enum LoadType {
    ImmediateWord(Register),
    ImmediateByte(Register),
    RegToReg(Register, Register),
    ImmediateByteToMemory(Register),
    ToMemory(Register, Register),
    ToMemoryWithSideEffect(Register, RegisterSideEffect),
    FromMemory(Register, Register),
    FromMemoryWithSideEffect(Register, RegisterSideEffect),
    StackPointerToMemory,
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
            0x01 => Instruction::Load(LoadType::ImmediateWord(Register::BC)), // LD BC, d16
            0x02 => Instruction::Load(LoadType::ToMemory(Register::BC, Register::A)), // LD (BC), A
            0x06 => Instruction::Load(LoadType::ImmediateByte(Register::B)),  // LD B, d8
            0x08 => Instruction::Load(LoadType::StackPointerToMemory),        // LD (a16), SP
            0x0A => Instruction::Load(LoadType::FromMemory(Register::A, Register::BC)), // LD A, (BC)
            0x0E => Instruction::Load(LoadType::ImmediateByte(Register::C)),            // LD C, d8
            0x11 => Instruction::Load(LoadType::ImmediateWord(Register::DE)), // LD DE, d16
            0x12 => Instruction::Load(LoadType::ToMemory(Register::DE, Register::A)), // LD (DE), A
            0x16 => Instruction::Load(LoadType::ImmediateByte(Register::D)),  // LD D, d8
            0x1A => Instruction::Load(LoadType::FromMemory(Register::A, Register::DE)), // LD A, (DE)
            0x1E => Instruction::Load(LoadType::ImmediateByte(Register::E)),            // LD E, d8
            0x20 => Instruction::JumpRelative(JumpCondition::NegatedFlag(Flag::Z)),     // JR NZ, s8
            0x21 => Instruction::Load(LoadType::ImmediateWord(Register::HL)), // LD HL, d16
            0x22 => Instruction::Load(LoadType::ToMemoryWithSideEffect(
                Register::HL,
                RegisterSideEffect::Inc,
            )), // LD (HL+), A
            0x26 => Instruction::Load(LoadType::ImmediateByte(Register::H)),  // LD H, d8
            0x2A => Instruction::Load(LoadType::FromMemoryWithSideEffect(
                Register::HL,
                RegisterSideEffect::Inc,
            )), // LD A, (HL+)
            0x2E => Instruction::Load(LoadType::ImmediateByte(Register::L)),  // LD L, d8
            0x31 => Instruction::Load(LoadType::ImmediateWord(Register::StackPointer)), // LD SP, d16
            0x32 => Instruction::Load(LoadType::ToMemoryWithSideEffect(
                Register::HL,
                RegisterSideEffect::Dec,
            )), // LD (HL-), A
            0x36 => Instruction::Load(LoadType::ImmediateByteToMemory(Register::HL)), // LD (HL), d8
            0x3A => Instruction::Load(LoadType::FromMemoryWithSideEffect(
                Register::HL,
                RegisterSideEffect::Dec,
            )), // LD A, (HL-)
            0x3E => Instruction::Load(LoadType::ImmediateByte(Register::A)),          // LD A, d8
            0x40 => Instruction::Load(LoadType::RegToReg(Register::B, Register::B)),  // LD B, B
            0x41 => Instruction::Load(LoadType::RegToReg(Register::B, Register::C)),  // LD B, C
            0x42 => Instruction::Load(LoadType::RegToReg(Register::B, Register::D)),  // LD B, D
            0x43 => Instruction::Load(LoadType::RegToReg(Register::B, Register::E)),  // LD B, E
            0x44 => Instruction::Load(LoadType::RegToReg(Register::B, Register::H)),  // LD B, H
            0x45 => Instruction::Load(LoadType::RegToReg(Register::B, Register::L)),  // LD B, L
            0x46 => Instruction::Load(LoadType::FromMemory(Register::B, Register::HL)), // LD B, (HL)
            0x47 => Instruction::Load(LoadType::RegToReg(Register::B, Register::A)),    // LD B, A
            0x48 => Instruction::Load(LoadType::RegToReg(Register::C, Register::B)),    // LD C, B
            0x49 => Instruction::Load(LoadType::RegToReg(Register::C, Register::C)),    // LD C, C
            0x4A => Instruction::Load(LoadType::RegToReg(Register::C, Register::D)),    // LD C, D
            0x4B => Instruction::Load(LoadType::RegToReg(Register::C, Register::E)),    // LD C, E
            0x4C => Instruction::Load(LoadType::RegToReg(Register::C, Register::H)),    // LD C, H
            0x4D => Instruction::Load(LoadType::RegToReg(Register::C, Register::L)),    // LD C, L
            0x4E => Instruction::Load(LoadType::FromMemory(Register::C, Register::HL)), // LD C, (HL)
            0x4F => Instruction::Load(LoadType::RegToReg(Register::C, Register::A)),    // LD C, A
            0x50 => Instruction::Load(LoadType::RegToReg(Register::D, Register::B)),    // LD D, B
            0x51 => Instruction::Load(LoadType::RegToReg(Register::D, Register::C)),    // LD D, C
            0x52 => Instruction::Load(LoadType::RegToReg(Register::D, Register::D)),    // LD D, D
            0x53 => Instruction::Load(LoadType::RegToReg(Register::D, Register::E)),    // LD D, E
            0x54 => Instruction::Load(LoadType::RegToReg(Register::D, Register::H)),    // LD D, H
            0x55 => Instruction::Load(LoadType::RegToReg(Register::D, Register::L)),    // LD D, L
            0x56 => Instruction::Load(LoadType::FromMemory(Register::D, Register::HL)), // LD D, (HL)
            0x57 => Instruction::Load(LoadType::RegToReg(Register::D, Register::A)),    // LD D, A
            0x58 => Instruction::Load(LoadType::RegToReg(Register::E, Register::B)),    // LD E, B
            0x59 => Instruction::Load(LoadType::RegToReg(Register::E, Register::C)),    // LD E, C
            0x5A => Instruction::Load(LoadType::RegToReg(Register::E, Register::D)),    // LD E, D
            0x5B => Instruction::Load(LoadType::RegToReg(Register::E, Register::E)),    // LD E, E
            0x5C => Instruction::Load(LoadType::RegToReg(Register::E, Register::H)),    // LD E, H
            0x5D => Instruction::Load(LoadType::RegToReg(Register::E, Register::L)),    // LD E, L
            0x5E => Instruction::Load(LoadType::FromMemory(Register::E, Register::HL)), // LD E, (HL)
            0x5F => Instruction::Load(LoadType::RegToReg(Register::E, Register::A)),    // LD E, A
            0x60 => Instruction::Load(LoadType::RegToReg(Register::H, Register::B)),    // LD H, B
            0x61 => Instruction::Load(LoadType::RegToReg(Register::H, Register::C)),    // LD H, C
            0x62 => Instruction::Load(LoadType::RegToReg(Register::H, Register::D)),    // LD H, D
            0x63 => Instruction::Load(LoadType::RegToReg(Register::H, Register::E)),    // LD H, E
            0x64 => Instruction::Load(LoadType::RegToReg(Register::H, Register::H)),    // LD H, H
            0x65 => Instruction::Load(LoadType::RegToReg(Register::H, Register::L)),    // LD H, L
            0x66 => Instruction::Load(LoadType::FromMemory(Register::H, Register::HL)), // LD H, (HL)
            0x67 => Instruction::Load(LoadType::RegToReg(Register::H, Register::A)),    // LD H, A
            0x68 => Instruction::Load(LoadType::RegToReg(Register::L, Register::B)),    // LD L, B
            0x69 => Instruction::Load(LoadType::RegToReg(Register::L, Register::C)),    // LD L, C
            0x6A => Instruction::Load(LoadType::RegToReg(Register::L, Register::D)),    // LD L, D
            0x6B => Instruction::Load(LoadType::RegToReg(Register::L, Register::E)),    // LD L, E
            0x6C => Instruction::Load(LoadType::RegToReg(Register::L, Register::H)),    // LD L, H
            0x6D => Instruction::Load(LoadType::RegToReg(Register::L, Register::L)),    // LD L, L
            0x6E => Instruction::Load(LoadType::FromMemory(Register::L, Register::HL)), // LD L, (HL)
            0x6F => Instruction::Load(LoadType::RegToReg(Register::L, Register::A)),    // LD L, A
            0x70 => Instruction::Load(LoadType::ToMemory(Register::HL, Register::B)), // LD (HL), B
            0x71 => Instruction::Load(LoadType::ToMemory(Register::HL, Register::C)), // LD (HL), C
            0x72 => Instruction::Load(LoadType::ToMemory(Register::HL, Register::D)), // LD (HL), D
            0x73 => Instruction::Load(LoadType::ToMemory(Register::HL, Register::E)), // LD (HL), E
            0x74 => Instruction::Load(LoadType::ToMemory(Register::HL, Register::H)), // LD (HL), H
            0x75 => Instruction::Load(LoadType::ToMemory(Register::HL, Register::L)), // LD (HL), L
            0x77 => Instruction::Load(LoadType::ToMemory(Register::HL, Register::A)), // LD (HL), A
            0x78 => Instruction::Load(LoadType::RegToReg(Register::A, Register::B)),  // LD A, B
            0x79 => Instruction::Load(LoadType::RegToReg(Register::A, Register::C)),  // LD A, C
            0x7A => Instruction::Load(LoadType::RegToReg(Register::A, Register::D)),  // LD A, D
            0x7B => Instruction::Load(LoadType::RegToReg(Register::A, Register::E)),  // LD A, E
            0x7C => Instruction::Load(LoadType::RegToReg(Register::A, Register::H)),  // LD A, H
            0x7D => Instruction::Load(LoadType::RegToReg(Register::A, Register::L)),  // LD A, L
            0x7E => Instruction::Load(LoadType::FromMemory(Register::A, Register::HL)), // LD A, (HL)
            0x7F => Instruction::Load(LoadType::RegToReg(Register::A, Register::A)),    // LD A, A
            0xAF => Instruction::XOR(XORTarget::Register(Register::A)),                 // XOR A
            0xCB => Instruction::Prefixed, // Any instruction that starts 0xCB
            _ => Instruction::NoOp,        // Not implemented
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

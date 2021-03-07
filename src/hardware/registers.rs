pub enum Register {
    A, // Accumulator
    F, // Flags
    B, // Data
    C, // Data
    D, // Data
    E, // Data
    H, // Data
    L, // Data

    AF, // Accumulator + Flags
    BC, // 16-bit data
    DE, // 16-bit data
    HL, // 16-bit data

    StackPointer, // Stack Pointer
}

pub struct RegisterFile {
    register_data: [u8; 10],
}

impl RegisterFile {
    pub fn new() -> Self {
        let register_data = [0; 10];

        RegisterFile { register_data }
    }

    pub fn read_register(&self, reg: Register) -> u16 {
        match reg {
            Register::A => self.register_data[0] as u16,
            Register::F => self.register_data[1] as u16,
            Register::B => self.register_data[2] as u16,
            Register::C => self.register_data[3] as u16,
            Register::D => self.register_data[4] as u16,
            Register::E => self.register_data[5] as u16,
            Register::H => self.register_data[6] as u16,
            Register::L => self.register_data[7] as u16,

            Register::AF => self.get_word(0),
            Register::BC => self.get_word(2),
            Register::DE => self.get_word(4),
            Register::HL => self.get_word(6),
            Register::StackPointer => self.get_word(8),
        }
    }

    pub fn write_register(&mut self, reg: Register, value: u16) {
        match reg {
            Register::A => self.register_data[0] = value as u8,
            Register::F => self.register_data[1] = value as u8,
            Register::B => self.register_data[2] = value as u8,
            Register::C => self.register_data[3] = value as u8,
            Register::D => self.register_data[4] = value as u8,
            Register::E => self.register_data[5] = value as u8,
            Register::H => self.register_data[6] = value as u8,
            Register::L => self.register_data[7] = value as u8,

            Register::AF => self.set_word(0, value),
            Register::BC => self.set_word(2, value),
            Register::DE => self.set_word(4, value),
            Register::HL => self.set_word(6, value),
            Register::StackPointer => self.set_word(8, value),
        };
    }

    fn set_word(&mut self, index: usize, value: u16) {
        let higher_byte = (value >> 8) as u8;
        let lower_byte = value as u8;

        self.register_data[index] = higher_byte;
        self.register_data[index + 1] = lower_byte;
    }

    fn get_word(&self, index: usize) -> u16 {
        let higher_byte = self.register_data[index] as u16;
        let lower_byte = self.register_data[index + 1] as u16;

        println!("{}, {}", higher_byte, lower_byte);

        (higher_byte << 8) | lower_byte
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_read_simple() {
        let mut register_file = RegisterFile::new();
        register_file.write_register(Register::B, 69);

        assert_eq!(register_file.read_register(Register::B), 69);
    }

    #[test]
    fn write_read_word() {
        let mut register_file = RegisterFile::new();
        register_file.write_register(Register::BC, 69);

        assert_eq!(register_file.read_register(Register::BC), 69);

        register_file.write_register(Register::BC, 65535);
        assert_eq!(register_file.read_register(Register::BC), 65535);
    }

    #[test]
    fn write_short_read_word() {
        let mut register_file = RegisterFile::new();
        register_file.write_register(Register::B, 0x80);
        register_file.write_register(Register::C, 0x08);
        assert_eq!(register_file.read_register(Register::BC), 0x8008);
    }

    #[test]
    fn truncation() {
        let mut register_file = RegisterFile::new();
        register_file.write_register(Register::B, 65535);
        assert_eq!(register_file.read_register(Register::B), 255);
    }
}

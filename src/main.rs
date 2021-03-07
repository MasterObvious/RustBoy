mod hardware;
mod utils;

use crate::hardware::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.step();
}

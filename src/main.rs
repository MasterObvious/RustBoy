mod hardware;

use crate::hardware::CPU;

fn main() {
    let cpu = CPU::new();
    println!("Hello, world!");
}

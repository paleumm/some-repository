mod cpu;
use cpu::{RIA2, RIA3};

fn main() {
    let mut cpu = RIA2 {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    // 8 => signifies that the operation involves two registers.
    // 0 => maps to cpu.registers[0].
    // 1 => maps to cpu.registers[1].
    // 4 => indicates addition.
    // 0x8014

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80; //
    mem[1] = 0x14; // 0x8014 -> add reg1 to reg0
    mem[2] = 0x80; //
    mem[3] = 0x24; // 0x8024 -> add reg2 to reg0
    mem[4] = 0x80; //
    mem[5] = 0x34; // 0x8034 -> add reg3 to reg0

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);

    let mut ria3 = RIA3 {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    ria3.registers[0] = 5;
    ria3.registers[1] = 10;

    let ria3_mem = &mut ria3.memory;
    ria3_mem[0x000] = 0x21;
    ria3_mem[0x001] = 0x00;
    ria3_mem[0x002] = 0x21;
    ria3_mem[0x003] = 0x00;
    ria3_mem[0x004] = 0x00;
    ria3_mem[0x005] = 0x00;

    ria3_mem[0x100] = 0x80;
    ria3_mem[0x101] = 0x14;
    ria3_mem[0x102] = 0x80;
    ria3_mem[0x103] = 0x14;
    ria3_mem[0x104] = 0x00;
    ria3_mem[0x105] = 0xEE;

    ria3.run();

    assert_eq!(ria3.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", ria3.registers[0]);
}

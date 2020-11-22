#![allow(dead_code)]

// Please allow me to use the new line brace formatting
#![allow(clippy::suspicious_else_formatting)]

mod riscv;

fn main()
{
    println!("RISCV Emulator");

    let mut cpu = riscv::ChipCPU::new();

    cpu.debug_display = true;

    cpu.write_to_memory(0, vec![
        0x13,
0x00,
0x00,
0x00,
0x73,
0x00,
0x00,
0x00,

    ]);

    for _ in 0..100
    {
        cpu.clock_to_instruction();
        // println!("{:?}", cpu);
    }
}


fn main() {
    // let mut ram: Ram = Ram{ram: [0; 0x10000]};
    // let mut vdg6847: Vdg6847 = Vdg6847{reg: [0; 256]};
    // let mut decoder = Decoder {
    //     bus: system::Bus::new(),
    //     cpu: cpu6502::Cpu::new(),
    // }
    let mut bus = system::Bus::new();
    let mut cpu = cpu6502::Cpu::new();
    for _i in 0..260 {
        cpu6502::decode(&mut bus, &mut cpu);
    } // PC now 0xFF5B

    println!("PC {:04X} Acc: {:02X}, X: {:02X}, Y: {:02X}", cpu.pc, cpu.a, cpu.x, cpu.y);
}

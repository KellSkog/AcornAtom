
fn main() {
    // let mut ram: Ram = Ram{ram: [0; 0x10000]};
    // let mut vdg6847: Vdg6847 = Vdg6847{reg: [0; 256]};
    // let mut decoder = Decoder {
    //     bus: system::Bus::new(),
    //     cpu: cpu6502::Cpu::new(),
    // }
    let mut ram = system::Ram{ram: [0u8; 0x10000]};
    let mut vdg = system::Vdg6847{reg :[0u8; 0x100]};
    let mut via = system::Via6522{reg :[0u8; 0x100]};
    let mut bus = system::Bus::new(&mut ram, &mut vdg, &mut via);
    let mut cpu = cpu6502::Cpu::new();
    for _i in 0..260 {
        cpu6502::decode(&mut bus, &mut cpu);
    } // PC now 0xFF5B

    println!("PC {:04X} Acc: {:02X}, X: {:02X}, Y: {:02X}", cpu.pc, cpu.a, cpu.x, cpu.y);
}

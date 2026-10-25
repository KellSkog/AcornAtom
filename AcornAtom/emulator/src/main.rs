
fn main() {
    // let mut ram: Ram = Ram{ram: [0; 0x10000]};
    // let mut vdg6847: Vdg6847 = Vdg6847{reg: [0; 256]};
    let mut bus = system::Bus::new();
    let mut cpu = cpu6502::Cpu::new(&mut bus);
    cpu.decode();
    bus.write(0,0);
}

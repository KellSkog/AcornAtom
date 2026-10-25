const BASIC_ROM: &[u8; 4096] = include_bytes!("../Atom_Basic.rom"); // 0xC000
// const Fp_ROM: &[u8; 4096] = include_bytes!("../Atom_Basic.rom"); // 0xD000
const COS_ROM: &[u8; 4096] = include_bytes!("../Atom_Kernel.rom"); // 0xF000
// static ram: [u8; 65536] = [0; 65536];

struct Ram {
    ram: [u8; 0x10000]
}
impl Ram {
    pub fn read(&self, index: usize) -> u8 {
        return self.ram[index];
    }
    pub fn write(&mut self, index: usize, data: u8) {
        self.ram[index] = data;
    }
}

struct Vdg6847 {
    reg: [u8;0x100]
}
impl Vdg6847 {
    pub fn read(&self, index: usize) -> u8 {
        return self.reg[index];
    }
    pub fn write(&mut self, index: usize, data: u8) {
        self.reg[index] = data;
    }
}

pub struct Bus {
    ram: Ram,
    vdg: Vdg6847,
}
impl Bus {
    pub fn new() -> Self {
        Self {
            ram: Ram {
                ram: [0; 0x10000],
            },
            vdg: Vdg6847 {
                reg: [0; 0x100],
            },
        }
    }
    pub fn read(&self, adr: usize) -> u8 {
        match adr {
            0x0800..0x0900 => self.vdg.read(adr - 0x0800),

            0xC000..0xD000 => BASIC_ROM[adr - 0xC000],
            0xF000..0x10000 => COS_ROM[adr - 0xF000],
            _ => self.ram.read(adr),
        }
    }
    pub fn write(&mut self, adr: u16, data: u8) {
        match adr {
            0x0800..0x0900 => {
                self.vdg.write((adr - 0x0800) as usize, data)
            }
            _ => self.ram.write(adr as usize, data),
        }
    }
}

// static ppi8255 = Ppi8255 {
//     reg: [u8;0xFF]
// };
// impl Ppi8255 {
//     pub fn read(&self, index: usize) -> u8 {
//         return self.reg[index];
//     }
//     pub fn write(&self, index: usize, data: u8) {
//         self.reg[index] = data;
//     }
// }
// // static ppi8255 = Ppi8255;

// static Via6522 = Via6522 {
//     reg: [u8;0xFF]
// };
// impl Via6522 {
//     pub fn read(&self, index: usize) -> u8 {
//         return self.reg[index];
//     }
//     pub fn write(&self, index: usize, data: u8) {
//         self.reg[index] = data;
//     }
// }
// static Via6522 = via6522;

// pub fn read(bus: Bus, adr: usize) -> u8 {
//     unsafe {
//     return match adr {
//         0x0000..0x0800 => bus.ram.read(adr), // Zero page, Teletext VDU RAM
//         0x0800..0x0900 => bus.vdg6847.read(adr),
//         0x2800..0x3C00 => bus.ram.read(adr), // Ram expansion
//         0x8000..0x9800 => bus.ram.read(adr), // Graphics RAM
//         // 0xB000..0xB800 => ppi8255.read(adr),
//         // 0xB800..0xC000 => via6522.read(adr),
//         0xC000..0xD000 => BASIC_ROM[adr - 0xC000],
//         0xF000..0x10000 => COS_ROM[adr - 0xF000],
//         _ => 0xFF,
//     }
// }
// }
// pub fn write(adr: usize, data: u8) {
//     unsafe {
//     match adr {
//         0x0000..0x0800 => ram.write(adr, data), // Zero page, Teletext VDU RAM
//         0x0800..0x0900 => vdg6847.write(adr, data),
//         0x2800..0x3C00 => ram.write(adr, data), // Ram expansion
//         0x8000..0x9800 => ram.write(adr, data), // Graphics RAM
//         // 0xB000..0xB800 => ppi8255.write(adr, data),
//         // 0xB800..0xC000 => via6522.write(adr, data),
//         _ => {},
//     }
// }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_rom() {
        let bus = Bus::new();
        assert_eq!(bus.read(0xF000), 0x50);
    }
    #[test]
    fn vdg() {
        let mut bus = Bus::new();
        assert_eq!(bus.read(0xF000), 0x50);
        bus.write(0x800, 0x42);
        assert_eq!(bus.read(0x800), 0x42);
    }
}

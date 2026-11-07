const BASIC_ROM: &[u8; 4096] = include_bytes!("../Atom_Basic.rom"); // 0xC000
const FP_ROM: &[u8; 4096] = include_bytes!("../Atom_FloatingPoint.rom"); // 0xD000
const COS_ROM: &[u8; 4096] = include_bytes!("../Atom_Kernel.rom"); // 0xF000

/* VIA:
Register: Address: Name:
Data Register B              #B800  DB
Data Register A              #B801  DA
Data Direction Register B    #B802  DDRB
Data Direction Register A    #B803  DDRA
Timer 1 low counter, latch   #B804  T1CL
Timer 1 high counter         #B805  T1CH
Timer 1 low latch            #B806  T1LL
Timer 1 high latch           #B807  T1LH
Timer 2 low counter, latch   #B808  T2CL
Timer 2 high counter         #B809  T2CH
Shift Register               #B80A  SR
Auxiliary Control Register   #B80B  ACR
Peripheral Control Register  #B80C  PCR
Interrupt Flag Register      #B80D  IFR
Interrupt Enable Register    #B80E  IER
Data Register A              #B80F  DA
*/
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
            0xD000..0xE000 => FP_ROM[adr - 0xD000],
            0xF000..0x10000 => COS_ROM[adr - 0xF000],
            _ => self.ram.read(adr),
        }
    }
    pub fn write(&mut self, adr: usize, data: u8) {
        match adr {
            0x0800..0x0900 => {
                self.vdg.write(adr - 0x0800, data)
            }
            _ => self.ram.write(adr, data),
        }
    }
}

// static ppu8255 = Ppu8255 {
//     reg: [u8;0xFF]
// };
// impl Ppu8255 {
//     pub fn read(&self, index: usize) -> u8 {
//         return self.reg[index];
//     }
//     pub fn write(&self, index: usize, data: u8) {
//         self.reg[index] = data;
//     }
// }
// // static ppu8255 = Ppu8255;

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
//         // 0xB000..0xB800 => ppu8255.read(adr),
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
//         // 0xB000..0xB800 => ppu8255.write(adr, data),
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

#![allow(non_snake_case)]
#![allow(unused_variables)]
/* Addressing modes:
Mode,               Cycles, Bytes
Immediate           4(3)
Absolute            3(3)
Zero page
Accumulator
Zero page INDEXed Indirect
Zero page Indirect INDEXed
Zero page, X
Zero page, Y
Absolute, X
Absolute, Y
Implied             2
Relative
Indirect Absolute (Jump)    5,   3

Bit 0 – Carry flag (C). Set if a carry occurs during an add
operation; cleared if a borrow occurs during a subtract operation;
used as a ninth bit in the shift and rotate instructions.
Bit 1 – Zero flag (Z). Set if the result of an operation is zero;
cleared otherwise.
Bit 2 – Interrupt disable (I). If set, disables the effect of the
IRQ interrupt. Is set by the processor during interrupts.
Bit 3 – Decimal mode flag (0). If set, the add and subtract
operations work in binary-coded-decimal arithmetic; if clear, the
add and subtract operations work in binary arithmetic.
Bit 4 – Break command (B). Set by the processor during a BRK
interrupt; otherwise cleared.
Bit 5 – Unused.
Bit 6 – Overflow flag (V). Set if a carry occurred from bit 6 during
an add operation; cleared if a borrow occurred to bit 6 in a
subtract operation.
Bit 7 – Negative flag (N). Set if bit 7 of the result of an
operation is set; otherwise cleared.
*/
const FLAG_C: u8 = 0x01 << 0;
const FLAG_Z: u8 = 0x01 << 1;
const FLAG_I: u8 = 0x01 << 2;
const FLAG_D: u8 = 0x01 << 3;
const FLAG_B: u8 = 0x01 << 4;
const FLAG_V: u8 = 0x01 << 6;
const FLAG_N: u8 = 0x01 << 7; // the N flag is set equal to bit 7 of the resulting value in all data movement and data arithmetic.

// {
    // #![allow(non_snake_case)] // error: an inner attribute is not permitted in this context
    fn BRK<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ORAindX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ORAzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ASLzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn PHP<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ORAimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ASLA<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ORAabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ASLabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn BPLrel<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ORAindY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ORAzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ASLzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn CLC<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ORAabsY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ORAabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ASLabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn JSRabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ANDindX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn BITzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ANDzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ROLzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn PLP<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ANDimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ROLA<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn BITABS<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ANDabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ROLabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn LDXimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) { // 0xA2
        cpu.x = bus.read((cpu.pc +1) as usize);
        cpu.pc += 2;
        if cpu.x == 0 {
        cpu.flags |= FLAG_Z;
        }

        if cpu.x & 0x80 != 0 {
            cpu.flags |= FLAG_N;
        }
    }
    fn LDAabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) { // 0xBD
        let addrl: usize = bus.read((cpu.pc + 1)  as usize) as usize;
        let addrh: usize = (bus.read((cpu.pc + 2) as usize) as usize) << 8;
        let addr: usize = addrh + addrl + (cpu.x  as usize);
        cpu.a = bus.read(addr);
        if cpu.a == 0 {
        cpu.flags |= FLAG_Z;
        }

        if cpu.a & 0x80 != 0 {
            cpu.flags |= FLAG_N;
        }
    }
    fn ILLEGAL<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
        panic!("Illegal opcode");
    }
    /*fn () {}
    fn () {}
    fn () {}
    fn () {}
    fn () {}
    fn () {}*/
// }
#[warn(non_snake_case)]

static OP_CODE: [for<'a>fn(bus: &'a mut system::Bus, cpu: &'a mut Cpu); 31] = [ // Dense array of function pointers
BRK, ORAindX, ORAzpg, ASLzpg, PHP, ORAimm, ASLA, ORAabs,
ASLabs, BPLrel, ORAindY, ORAzpgX, ASLzpgX, CLC, ORAabsY, ORAabsX, 
ASLabsX, JSRabs, ANDindX, BITzpg, ANDzpg, ROLzpg, PLP, ANDimm,
ROLA, BITABS, ANDabs, ROLabs, LDXimm, LDAabsX,
ILLEGAL];
static OP_COUNT: usize = OP_CODE.len() - 1; // Points to ILLEGAL
static INDEX: [usize; 0x100] = [ // 256 byte sparse array pointing to INDEX of function pointer for op code
0, 0x1, OP_COUNT, OP_COUNT, OP_COUNT, 0x2, 0x3, OP_COUNT, 0x4, 0x5, 0x6, OP_COUNT, OP_COUNT, 0x7, 0x8, OP_COUNT,  // 0 - F, 9 instructions
0x9, 0xA, OP_COUNT, OP_COUNT, OP_COUNT, 0xB, 0xC, OP_COUNT, 0xD, 0xE, OP_COUNT, OP_COUNT, OP_COUNT, 0xF, 0x10, OP_COUNT,   // 10 - 1F, 8 instructions
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // 20 - 2F
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // 30 - 3F
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // 40 - 4F
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // 50 - 5F
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // 60 - 6F
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // 70 - 7F
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // 80 - 8F
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // 90 - 9F
0, 1, 28, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // A0 - AF
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 29, 0xe, 0xf,  // B0 - BF
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // C0 - CF
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // D0 - DF
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // E0 - EF
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // F0 - FF
];

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub flags: u8, // Negative, Zero, Carry, Interupt disable, Decimal, oVerflow
    // bus: &'a mut system::Bus, // A Cpu contains a mutable reference to a Bus, and that reference lives for lifetime 'a.
}
impl Cpu {
//     pub fn new(bus: &'a mut system::Bus) -> Self {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0, // offset in indexed and pre-indexed indirect addressing modes.
            y: 0, // offset in indexed and post-indexed indirect addressing modes.
            sp: 0xFD,
            pc: 0xFF3F, // PCL of reset vector, PCH in 0xFFFD
            flags: 0,
            // bus,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.pc = 0xFF3F; // PCL of reset vector, PCH in 0xFFFD
        self.flags = 0;
    }
}

pub fn decode<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) { //}, opcode: u8) {
    let opcode = bus.read(cpu.pc as usize);
    OP_CODE[INDEX[opcode as usize]](bus, cpu);
}
    
// Unit tests - Has access to private members!
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let mut bus = system::Bus::new();
        let mut cpu = Cpu::new();
        assert_eq!(bus.read(cpu.pc as usize), 0xA2);
        decode(&mut bus, &mut cpu); // LDX @0x17
        assert_eq!(cpu.pc, 0xFF41);
        assert_eq!(bus.read(cpu.pc as usize), 0xBD);
        assert_eq!(cpu.x, 0x17);

        let addrl: usize = bus.read((cpu.pc + 1)  as usize) as usize;
        let addrh: usize = (bus.read((cpu.pc + 2) as usize) as usize) << 8;
        let addr: usize = addrh + addrl + (cpu.x  as usize);

        assert_eq!(addr, 0xff9a + 0x17); 
        decode(&mut bus, &mut cpu); // LDA 0xff9a, X
        assert_eq!(cpu.a, 0xC2); // (0xff9a + 0x17) = 0xC2

    }
    #[test]
    fn arithmetics() {
        let a: u8 = 1;
        let b: u8 = 16;
        let c: u8 = 1;
        let res: usize = ((a as usize) << 8) + b as usize + c as usize;
        assert_eq!(res, 0x111);
    }
}

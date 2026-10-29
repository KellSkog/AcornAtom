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
const FLAG_C: u8 = 0x01;
const FLAG_Z: u8 = 0x02;
const FLAG_I: u8 = 0x04;
const FLAG_D: u8 = 0x08;
const FLAG_B: u8 = 0x10;
const FLAG_V: u8 = 0x40;
const FLAG_N: u8 = 0x80;

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
    fn LDXimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    fn ILLEGAL<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {}
    /*fn () {}
    fn () {}
    fn () {}
    fn () {}
    fn () {}
    fn () {}*/
// }
#[warn(non_snake_case)]

static OP_CODE: [for<'a>fn(bus: &'a mut system::Bus, cpu: &'a mut Cpu); 30] = [ // Dense array of function pointers
BRK, ORAindX, ORAzpg, ASLzpg, PHP, ORAimm, ASLA, ORAabs,
ASLabs, BPLrel, ORAindY, ORAzpgX, ASLzpgX, CLC, ORAabsY, ORAabsX, 
ASLabsX, JSRabs, ANDindX, BITzpg, ANDzpg, ROLzpg, PLP, ANDimm,
ROLA, BITABS, ANDabs, ROLabs, LDXimm,
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
0, 1, 29, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // A0 - AF
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // B0 - BF
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // C0 - CF
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // D0 - DF
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // E0 - EF
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,  // F0 - FF
];
// pub fn decode(op: u8)  {
//         OP_CODE[INDEX[op as usize]]();
//     }

pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    flags: u8, // Negative, Zero, Carry, Interupt disable, Decimal, oVerflow
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
        decode(&mut bus, &mut cpu);
        assert_eq!(cpu.pc, 0xFF3F)
        // assert_eq!(OP_CODE[INDEX[0x1E]] as usize,
        // ASLabsX as *const () as usize);
    }
}

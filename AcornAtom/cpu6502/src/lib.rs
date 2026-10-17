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
*/
#![allow(non_snake_case)]
fn BRK() {}
fn ORAindX() {}
fn ORAzpg() {}
fn ASLzpg() {}
fn PHP() {}
fn ORAimm() {}
fn ASLA() {}
fn ORAabs() {}
fn ASLabs() {}
fn BPLrel() {}
fn ORAindY() {}
fn ORAzpgX() {}
fn ASLzpgX() {}
fn CLC() {}
fn ORAabsY() {}
fn ORAabsX() {}
fn ASLabsX() {}
fn JSRabs() {}
fn ANDindX() {}
fn BITzpg() {}
fn ANDzpg() {}
fn ROLzpg() {}
fn PLP() {}
fn ANDimm() {}
fn ROLA() {}
fn BITABS() {}
fn ANDabs() {}
fn ROLabs() {}
fn ILLEGAL() {}
/*fn () {}
fn () {}
fn () {}
fn () {}
fn () {}
fn () {}
fn () {}*/
#[warn(non_snake_case)]

const OP_CODE: [fn(); 29] = [ // Dense array of function pointers
    BRK, ORAindX, ORAzpg, ASLzpg, PHP, ORAimm, ASLA, ORAabs,
    ASLabs, BPLrel, ORAindY, ORAzpgX, ASLzpgX, CLC, ORAabsY, ORAabsX, 
    ASLabsX, JSRabs, ANDindX, BITzpg, ANDzpg, ROLzpg, PLP, ANDimm,
    ROLA, BITABS, ANDabs, ROLabs,
    ILLEGAL];
const OP_COUNT: usize = OP_CODE.len() - 1; // Points to ILLEGAL
const INDEX: [usize; 32] = [ // 256 byte sparse array pointing to INDEX of function pointer for op code
    0, 0x1, OP_COUNT, OP_COUNT, OP_COUNT, 0x2, 0x3, OP_COUNT, 0x4, 0x5, 0x6, OP_COUNT, OP_COUNT, 0x7, 0x8, OP_COUNT,  // 0 - F, 9 instructions
    0x9, 0xA, OP_COUNT, OP_COUNT, OP_COUNT, 0xB, 0xC, OP_COUNT, 0xD, 0xE, OP_COUNT, OP_COUNT, OP_COUNT, 0xF, 0x10, OP_COUNT   // 10 - 1F, 8 instructions
];
pub fn decode(op: u8)  {
    OP_CODE[INDEX[op as usize]]();
}


// Unit tests - Has access to private members!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(OP_CODE[INDEX[0x1E]] as usize,
        ASLabsX as *const () as usize);
    }
}

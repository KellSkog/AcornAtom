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

Stack:
In the same manner as the microprocessor forces locations 00 on to the high order 8 bits of the
address lines for Zero Page operations, the microprocessor automatically puts 01 Hex on to the high order 8-bit address lines during each stack operation.
*/
const FLAG_C: u8 = 0x01 << 0; // Operations which affect the carry are ADC, ASL, CLC, CMP, CPX, CPY, LSR, PLP, ROL, :RTT, SBC, sfc.
const FLAG_Z: u8 = 0x01 << 1; // set when the 8 bits of results of an opera­tion are 0.
const FLAG_I: u8 = 0x01 << 2; // Instructions which affect the interrupt disable are BRK, CLI, PLP, RTI and SEI.
const FLAG_D: u8 = 0x01 << 3; // affects the decimal mode flag: CLD, PLP, RTI and SED.
const FLAG_B: u8 = 0x01 << 4; // determined during an interrupt service sequence whether or not the inter­rupt was caused by BRK command or by a real interrupt.
const FLAG_V: u8 = 0x01 << 6;
const FLAG_N: u8 = 0x01 << 7; // the N flag is set equal to bit 7 of the resulting value in all data movement and data arithmetic.

fn get_NZ_flags(value: u8, mut flags: u8) -> u8 {
    if value == 0 {
        flags |= FLAG_Z;
        }
        if value >= 0x80 {
            flags |= FLAG_N;
        } else {
            flags &= !FLAG_N;
        }
    flags
}
fn unimplemented<'a>(bus: &'a system::Bus, cpu: &'a Cpu) {
    println!("PC: {:04X}, Acc: {:02X}, SP: {:04X}, X: {:02X}, Y: {:02X}", cpu.pc, cpu.a, cpu.sp, cpu.x, cpu.y);
    panic!("UIOp: {:02X}", bus.read(cpu.pc as usize));
}
fn CMP_helper<'a>(reg: u8, mem: u8, cpu: &'a mut Cpu) {
    let res = cpu.a.wrapping_sub(mem);

    if reg >= mem { // the carry flag is set when the value in memory is less than or equal to the accumulator
        cpu.flags |= FLAG_C;
    } else {
        cpu.flags &= !FLAG_C;
    }
    cpu.flags = get_NZ_flags(res, cpu.flags);
    cpu.pc += 2;
}
// #![allow(non_snake_case)] // error: an inner attribute is not permitted in this context
fn BRK<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* 00 */{unimplemented(&bus, &cpu);}
fn ORAindX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ORAzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ASLzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn PHP<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ORAimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ASLA<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ORAabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ASLabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn BPLrel<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* 10 */{
    cpu.pc +=2;
    if (cpu.flags & FLAG_N) == 0 {
        let offset: i8 = bus.read((cpu.pc - 1).into()).cast_signed() as i8;
        cpu.pc = cpu.pc.wrapping_add_signed(offset as i16); // rel is signed....
    }
}
fn ORAindY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ORAzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ASLzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CLC<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ORAabsY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ORAabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ASLabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn JSRabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* 20 */{
    /*
    JSR instruction stores the program counter address which points to the last byte of the 
    jump instruc­tion onto the stack using the stack pointer.
    The stack byte contains the program count high first, followed by program count low.
    The JSR then transfers the addresses following the jump instruction to the program
    counter low and the program counter high, thereby directing the program to begin at that new address.
    The JSR instruction affects no flags, causes the stack pointer to be decremented by 2 
    and substitutes new values into the program counter low and the program counter high.
    The addressing mode for the JSR is always Absolute. */
    // Get jmp target
    cpu.pc += 1;
    let adrl = bus.read(cpu.pc as usize) as u16;
    cpu.pc += 1; // Now points to last byte of JSR instruction!
    let adrh = bus.read(cpu.pc as usize) as u16;
    // Push PC, last byte of the JSR instruc­tion
    bus.write(cpu.sp as usize, (cpu.pc >> 8) as u8);
    cpu.sp -= 1;
    bus.write(cpu.sp as usize, (cpu.pc & 0xFF) as u8);
    cpu.sp -= 1;
    // Update PC
    cpu.pc = adrh << 8 | adrl;
    // The JSR instruction affects no flags
}
fn ANDindX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn BITzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ANDzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ROLzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn PLP<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ANDimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    cpu.a &= bus.read((cpu.pc +1) as usize);
    cpu.flags = get_NZ_flags(cpu.a, cpu.flags);
    cpu.pc +=2;
}
fn ROLA<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn BITABS<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ANDabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ROLabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn BMIrel<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* 30 */{
    cpu.pc +=2;
    if (cpu.flags & FLAG_N) == FLAG_N {
        let offset: i8 = bus.read((cpu.pc - 1).into()).cast_signed() as i8;
        cpu.pc = cpu.pc.wrapping_add_signed(offset as i16); // rel is signed....
    }
}
fn ANDindY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ANDzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ROLzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn SEC<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ANDabsY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ANDabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ROLabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn RTI<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* 40 */{unimplemented(&bus, &cpu);}
fn EORindX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn EORzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LSRzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn PHA<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    bus.write(cpu.sp as usize, cpu.a);
    cpu.sp -= 1;
    cpu.pc += 1;
}
fn EORimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LSRA<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn JMPabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn EORabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LSRabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn BVCrel<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* 50 */ {unimplemented(&bus, &cpu);}
fn EORindY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn EORzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LSRzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CLI<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn EORabsY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn EORabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LSRabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn RTS<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* 60 */ {unimplemented(&bus, &cpu);}
fn ADCindX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ADCzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn RORzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn PLA<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    cpu.a = bus.read(cpu.sp as usize);
    cpu.flags = get_NZ_flags(cpu.a, cpu.flags);
    cpu.sp += 1;
    cpu.pc += 1;
}
fn ADCimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn RORA<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn JMPind<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    let ial = bus.read((cpu.pc + 1) as usize) as u16;
    let iah = bus.read((cpu.pc + 2) as usize) as u16;
    let adl = bus.read(((iah << 8) | ial) as usize) as u16;
    let adh = bus.read(((iah << 8) | ial + 1) as usize) as u16;
    cpu.pc = (adh << 8) | adl;
}
fn ADCabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn RORabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn BVSrel<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* 70 */ {unimplemented(&bus, &cpu);}
fn ADCindY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ADCzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn RORzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn SEI<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ADCabsY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn ADCabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn RORabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn STAindX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* 81 */ {unimplemented(&bus, &cpu);}
fn STYzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn STAzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    let addr: usize = bus.read((cpu.pc + 1) as usize) as usize;
    bus.write(addr, cpu.a);
    cpu.pc += 2;
}
fn STXzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    let addr: usize = bus.read((cpu.pc + 1) as usize) as usize;
    bus.write(addr, cpu.x);
    cpu.pc += 2;
}
fn DEY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn TXA<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    cpu.a = cpu.x;
    cpu.flags = get_NZ_flags(cpu.a, cpu.flags);
    cpu.pc += 1;
}
fn STYabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn STAabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) { // 8D
    let addr: usize = 
        bus.read((cpu.pc + 1) as usize) as usize |
        (bus.read((cpu.pc + 2) as usize) as usize) << 8;
    bus.write(addr, cpu.a);
    cpu.pc += 3;
}
fn STXabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn BCCrel<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* 90 */ {unimplemented(&bus, &cpu);}
fn STAindY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn STYzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn STAzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn STXzpgY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn TYA<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn STAabsY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn TXS<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    cpu.sp = 0x100 | cpu.x as u16;
    cpu.pc += 1;
}
fn STAabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    let addr: usize = 
        ((bus.read((cpu.pc + 1) as usize) as usize) | 
        ((bus.read((cpu.pc + 2) as usize) as usize) << 8)) +
        cpu.x as usize;
    bus.write(addr, cpu.a);
    cpu.pc += 3;
}
fn LDYimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* A0 */ {
    cpu.y = bus.read((cpu.pc +1) as usize);
    cpu.flags = get_NZ_flags(cpu.y, cpu.flags);
    cpu.pc += 2;
}
fn LDAindX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDXimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    cpu.x = bus.read((cpu.pc +1) as usize);
    cpu.flags = get_NZ_flags(cpu.x, cpu.flags);
    cpu.pc += 2;
}
fn LDYzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDAzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDXzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn TAY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDAimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    cpu.a = bus.read((cpu.pc +1) as usize);
    cpu.flags = get_NZ_flags(cpu.x, cpu.flags);
    cpu.pc += 2;
}
fn TAX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDYabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDAabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    let adr: usize = 
        bus.read((cpu.pc + 1) as usize) as usize | 
        (bus.read((cpu.pc + 2) as usize) as usize) << 8;
    cpu.a = bus.read(adr);
    cpu.flags = get_NZ_flags(cpu.a, cpu.flags);
    cpu.pc += 3;
}
fn LDXabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn BCSrel<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* B0 */ {unimplemented(&bus, &cpu);}
fn LDAindY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    /*  INDIRECT INDEXED ADDRESSING
        adrl = (PC + 1) e.g zpg (#E8) = A0
        adrh = ((PC + 1) + 1) e.g. zpg (#E9) = FE
        addr = adrl +adrh << 8 + Y => FEA0 + Y */
    let adrl = bus.read((cpu.pc + 1) as usize)as usize;
    let adrh = bus.read((adrl + 1) as usize)as usize;
    cpu.a = bus.read((adrh << 8) | adrl + cpu.y as usize);
    cpu.flags = get_NZ_flags(cpu.a, cpu.flags);
    cpu.pc += 2;
}
fn LDYzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDAzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDXzpgY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CLV<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDAabsY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn TSX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDYabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn LDAabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) { // 0xBD
    let addr: usize = 
        ((bus.read((cpu.pc + 1) as usize)as usize) | 
        ((bus.read((cpu.pc + 2) as usize) as usize) << 8)) +
        cpu.x as usize;
    cpu.a = bus.read(addr);
    cpu.flags = get_NZ_flags(cpu.a, cpu.flags);
    cpu.pc += 3;
}
fn LDXabsY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn CPYimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* C0 */ {
    // Flags: N, Z, C
    let mem = bus.read((cpu.pc + 1) as usize);
    CMP_helper(cpu.y, mem, cpu);
}
fn CMPindX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CPYzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CMPzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    let adr: usize = bus.read((cpu.pc + 1) as usize) as usize;
    let mem = bus.read(adr);
    CMP_helper(cpu.a, mem, cpu);
}
fn DECzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn INY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CMPimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    /* This instruction subtracts the contents of memory from the contents of the accumulator. 
    Flags: N, Z, C */
    let mem = bus.read((cpu.pc + 1) as usize);
    CMP_helper(cpu.a, mem, cpu);
}
fn DEX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    cpu.x = cpu.x.wrapping_sub(1);
    cpu.flags = get_NZ_flags(cpu.x, cpu.flags);
    cpu.pc += 1;
}
fn CPYabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    let adr: usize = 
        bus.read((cpu.pc + 1) as usize) as usize | 
        (bus.read((cpu.pc + 2) as usize) as usize) << 8;
    let mem = bus.read(adr);
    CMP_helper(cpu.y, mem, cpu);
}
fn CMPabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    let adr: usize = 
        bus.read((cpu.pc + 1) as usize) as usize | 
        (bus.read((cpu.pc + 2) as usize) as usize) << 8;
    let mem = bus.read(adr);
    CMP_helper(cpu.a, mem, cpu);
}
fn DECabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn BNErel<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* D0 */ {
    cpu.pc += 2;
    if (cpu.flags & FLAG_Z) == 0 {
        let offset: i8 = bus.read((cpu.pc - 1) as usize).cast_signed() as i8;
        cpu.pc = cpu.pc.wrapping_add_signed(offset as i16); // rel is signed....
    }
}
fn CMPindY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CMPzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn DECzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CLD<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CMPabsY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CMPabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn DECabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn CPXimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* E0 */ {unimplemented(&bus, &cpu);}
fn SBCindX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CPXzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn SBCzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn INCzpg<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    let zpg = (cpu.pc + 1) as usize;
    let data = bus.read(zpg) + 1;
    cpu.flags = get_NZ_flags(data, cpu.flags);
    bus.write(zpg, data);
    cpu.pc += 2;
}
fn INX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    cpu.x = cpu.x.wrapping_add(1);
    cpu.flags = get_NZ_flags(cpu.x, cpu.flags);
    cpu.pc += 1;
}
fn SBCimm<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn NOP<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn CPXabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn SBCabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn INCabs<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn BEQrel<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) /* F0 */ {
    cpu.pc += 2;
    if (cpu.flags & FLAG_Z) == 1 {
        let offset: i8 = bus.read((cpu.pc - 1) as usize).cast_signed() as i8;
        cpu.pc = cpu.pc.wrapping_add_signed(offset as i16); // rel is signed....
    }
}
fn SBCindY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn SBCzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn INCzpgX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn SED<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn SBCabsY<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn SBCabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}
fn INCabsX<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {unimplemented(&bus, &cpu);}

fn ILLEGAL<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) {
    panic!("Illegal opcode");
}
#[warn(non_snake_case)]

/* Todo: Remove this fossil will double indirection to save memory (Cycles more critical than memory)
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
]; */

static OP_CODE: [for<'a>fn(bus: &'a mut system::Bus, cpu: &'a mut Cpu); 0x100] = [ 
BRK, ORAindX, ILLEGAL, ILLEGAL, ILLEGAL, ORAzpg, ASLzpg, ILLEGAL, PHP, ORAimm, ASLA, ILLEGAL, ILLEGAL, ORAabs, ASLabs, ILLEGAL, // 00 - 0F
BPLrel, ORAindY, ILLEGAL, ILLEGAL, ILLEGAL, ORAzpgX, ASLzpgX, ILLEGAL, CLC, ORAabsY, ILLEGAL, ILLEGAL, ILLEGAL, ORAabsX, ASLabsX, ILLEGAL, // 10 - 1F
JSRabs, ANDindX, ILLEGAL, ILLEGAL, BITzpg, ANDzpg, ROLzpg, ILLEGAL, PLP, ANDimm, ROLA, ILLEGAL, BITABS, ANDabs, ROLabs, ILLEGAL, // 20 - 2F
BMIrel, ANDindY, ILLEGAL, ILLEGAL, ILLEGAL, ANDzpgX, ROLzpgX, ILLEGAL, SEC, ANDabsY, ILLEGAL, ILLEGAL, ILLEGAL, ANDabsX, ROLabsX, ILLEGAL,  // 30 - 3F
RTI, EORindX, ILLEGAL, ILLEGAL, ILLEGAL, EORzpg, LSRzpg, ILLEGAL, PHA, EORimm, LSRA, ILLEGAL, JMPabs, EORabs, LSRabs, ILLEGAL, // 40 - 4F
BVCrel, EORindY, ILLEGAL, ILLEGAL, ILLEGAL, EORzpgX, LSRzpgX, ILLEGAL, CLI, EORabsY, ILLEGAL, ILLEGAL, ILLEGAL, EORabsX, LSRabsX,  ILLEGAL, // 50 - 5F
RTS, ADCindX, ILLEGAL, ILLEGAL, ILLEGAL, ADCzpg, RORzpg, ILLEGAL, PLA, ADCimm, RORA, ILLEGAL, JMPind, ADCabs, RORabs, ILLEGAL, // 60 -6F
BVSrel, ADCindY, ILLEGAL, ILLEGAL, ILLEGAL, ADCzpgX, RORzpgX, ILLEGAL, SEI, ADCabsY, ILLEGAL, ILLEGAL, ILLEGAL, ADCabsX, RORabsX,  ILLEGAL, // 70 - 7F
ILLEGAL, STAindX, ILLEGAL, ILLEGAL, STYzpg, STAzpg, STXzpg, ILLEGAL, DEY, ILLEGAL, TXA, ILLEGAL, STYabs, STAabs, STXabs,  ILLEGAL, // 80 -8F
BCCrel, STAindY, ILLEGAL, ILLEGAL, STYzpgX, STAzpgX, STXzpgY, ILLEGAL, TYA, STAabsY, TXS, ILLEGAL, ILLEGAL, STAabsX, ILLEGAL, ILLEGAL, // 90 -9F
LDYimm, LDAindX, LDXimm, ILLEGAL, LDYzpg, LDAzpg, LDXzpg, ILLEGAL, TAY, LDAimm, TAX, ILLEGAL, LDYabs, LDAabs, LDXabs, ILLEGAL, // A0 -AF
BCSrel, LDAindY, ILLEGAL, ILLEGAL, LDYzpgX, LDAzpgX, LDXzpgY, ILLEGAL, CLV, LDAabsY, TSX, ILLEGAL, LDYabsX, LDAabsX, LDXabsY, ILLEGAL, // B0 - BF
CPYimm, CMPindX, ILLEGAL, ILLEGAL, CPYzpg, CMPzpg, DECzpg, ILLEGAL, INY, CMPimm, DEX, ILLEGAL, CPYabs, CMPabs, DECabs, ILLEGAL, // C0 - CF
BNErel, CMPindY, ILLEGAL, ILLEGAL, ILLEGAL, CMPzpgX, DECzpgX, ILLEGAL, CLD, CMPabsY, ILLEGAL, ILLEGAL, ILLEGAL, CMPabsX, DECabsX, ILLEGAL, // D0 -DF
CPXimm, SBCindX, ILLEGAL, ILLEGAL, CPXzpg, SBCzpg, INCzpg, ILLEGAL, INX, SBCimm, NOP, ILLEGAL, CPXabs, SBCabs, INCabs, ILLEGAL, // E0- EF
BEQrel, SBCindY, ILLEGAL, ILLEGAL, ILLEGAL, SBCzpgX, INCzpgX, ILLEGAL, SED, SBCabsY, ILLEGAL, ILLEGAL, ILLEGAL, SBCabsX, INCabsX, ILLEGAL, // F0 -FF
];

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u16, // 0x100 <= sp <= 0x1FF, i.e. sph allways 0x01!
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
            sp: 0x1FD,
            pc: 0xFF3F, // PCL of reset vector, PCH in 0xFFFD
            flags: 0,
            // bus,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0x1FD;
        self.pc = 0xFF3F; // PCL of reset vector, PCH in 0xFFFD
        self.flags = 0;
    }
}

pub fn decode<'a>(bus: &'a mut system::Bus, cpu: &'a mut Cpu) { //}, opcode: u8) {
    let opcode = bus.read(cpu.pc as usize) as u8;
    // println!("PC: {:04X} OP: {:02X}", cpu.pc, opcode);
    // OP_CODE[INDEX[opcode as usize]](bus, cpu);
    OP_CODE[opcode as usize](bus, cpu);
}
    
// Unit tests - Has access to private members!
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn reset() {
        // FF9A                                00 A0 EF F8 52 FE
        // FFA0  94 FE 6E F9 E5 FA AC C2 AC C2 EE FB 7C FC 38 FC
        // FFB0  78 C2 
        let mut ram = system::Ram{ram: [0u8; 0x10000]};
        let mut vdg = system::Vdg6847{reg :[0u8; 0x100]};
        let mut via = system::Via6522{reg :[0u8; 0x100]};
        let mut bus = system::Bus::new(&mut ram, &mut vdg, &mut via);
        let mut cpu = Cpu::new();
        for _i in 0..0x68 {
            decode(&mut bus, &mut cpu);
        }
        assert_eq!(cpu.sp, 0x1FF);
        assert_eq!(cpu.pc, 0xFF55);
        for _i in 0x68..266 {
            decode(&mut bus, &mut cpu);
        }
        assert_eq!(cpu.pc, 0xFF69); // FF69  20 D1 F7  JSR #F7D1
        assert_eq!(cpu.sp, 0x1FF);

        for _i in 266..271 { // sp -=2, +=2
            decode(&mut bus, &mut cpu);
        }
        assert_eq!(cpu.pc, 0xF7D7);
        assert_eq!(cpu.sp, 0x1FF);

        for _i in 271..276 { // sp -=2, +=2
            decode(&mut bus, &mut cpu);
        }
        assert_eq!(cpu.pc, 0xF7E1);

        decode(&mut bus, &mut cpu);
        decode(&mut bus, &mut cpu);
        assert_eq!(cpu.sp, 0x1FD);
        assert_eq!(cpu.pc, 0xFFF4);
        decode(&mut bus, &mut cpu);
        assert_eq!(cpu.pc, 0xFE52); // OSWRCH
        decode(&mut bus, &mut cpu);
        assert_eq!(cpu.sp, 0x1FB);

        for _i in 280..291 { // sp -=2, +=2
            decode(&mut bus, &mut cpu);
        }
        assert_eq!(cpu.pc, 0xFF10);
        assert_eq!(cpu.sp, 0x1FB); // TODO: Why!!!!!
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

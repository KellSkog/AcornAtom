/*  The graphics of Acorn Atom on the RPI 1 is achived via the Linux fb0 device
    to which a descriptor fd is opened and used to get a pointer to U8 video memory mem.
    Views fb_u8 and fb_16 are created to access the video memory as bytes and
    16 bit pixels:
    $ kmsprint Connector 0 (35) HDMI-A-1 (connected) Encoder 0 (34) TMDS Crtc 3 (97) 1920x1080@60.00 148.500 1920/88/44/148/+ 1080/4/5/36/+ 60 (60.00) P|D Plane 3 (86) fb-id: 670 (crtcs: 3) 0,0 1920x1080 -> 0,0 1920x1080 (XR24 AR24 AB24 XB24 RX24 RA24 BX24 BA24 RG16 BG16 AR15 XR15 RG24 BG24 YU16 YV16 YU24 YV24 YU12 YV12 NV12 NV21 NV16 NV61 RGB8 BGR8 XR12 AR12 XB12 AB12 BX12 BA12 RX12 RA12) FB 670 1920x1080 RG16

    FB 670 1920x1080 RG16 -> fb-id 670 with 16 bit pixel mode (it's RGB565)

    With the Atom screen being 256*192 and console 32*16, 256/32=8, 192/16=12 so a
    8*12 font fits and can be directly drawn on the U16 frame buffer using offsets.

    fb_u6:
    0                                           1919
    1920                                        3839
    .
    .
    2,071,680                               2,073,599

    From cursor position (cx=15, cy=7) we get 256x192 relative by
    cx*8, cy*12 > px=120, py=84
    at (cx, cy) we iterate over a character:
    for y= 0..12
        put_pixel_row(FONT[char + y], cx, cy);
*/

use libc::{close, mmap, munmap, open, MAP_SHARED, O_RDWR, PROT_READ, PROT_WRITE};
use std::ffi::CString;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;

const FB_PATH: &str = "/dev/fb0";

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

const BYTES_PER_PIXEL: usize = 2;
const ATOM_WIDTH: usize = 256;
const ATOM_HEIGHT: usize = 192;
const FB_SIZE: usize = WIDTH * ATOM_HEIGHT * BYTES_PER_PIXEL;
const FRAMEBUFFER_SIZE: usize = WIDTH * HEIGHT * BYTES_PER_PIXEL;

const OFFSET_X: usize = 100; //WIDTH - ATOM_WIDTH; // 1664
const OFFSET_Y: usize = 100;

const BLACK: u16 = 0x0;
const RED: u16 = 0xF800;
const GREEN: u16 = 0x07E0;
const BLUE: u16 = 0x001F;
const WHITE: u16 = 0xFFFF;

const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 12;

const FONT_6847: [u8; 48] = [
    0, 0, 0, 0x1C, 0x22, 0x2, 0x1A, 0x2A, 0x2A, 0x1C, 0, 0, // @
    0, 0, 0, 0x8, 0x14, 0x22, 0x22, 0x3E, 0x22, 0x22, 0, 0, // A
    0, 0, 0, 0b00111100, 0b00010010, 0b00010010, 0b00011100, 0b00010010, 0b00010010, 0b00111100, 0, 0, // B
    0, 0, 0, 0b00011100, 0b00100010, 0b00100000, 0b00100000, 0b00100000, 0b00100010, 0b00011100, 0, 0, // C
];

// fn draw_square(
//     fb: &mut [u16],
//     x0: usize,
//     y0: usize,
//     size: usize,
//     color: u16,
// ) {
//     for y in y0..(y0 + size) {
//         for x in x0..(x0 + size) {
//             let idx = y * ATOM_WIDTH + x;
//             fb[idx] = color;
//         }
//     }
// }
fn put_pixel_row(fb: &mut [u16], symbol: u8, px: usize, py: usize) {//-> [usize; 8] {
    // px, py are 256x192 pixel references
    let mut indices = [0usize; 8];
    for pixel in (0..8) {
        let index:usize = (OFFSET_Y + py) * WIDTH + (OFFSET_X + px + pixel);
        let mut col: u16;
        if symbol & (1 << pixel) == 0 {
            col = BLACK;
        } else {
            col = WHITE;
        }
        fb[index] = col;
        // indices[pixel] = col as usize;
    }
    // indices
}
fn test_hdmi() -> std::io::Result<()> {
    let path = CString::new(FB_PATH).unwrap();

    unsafe {
        let fd = open(path.as_ptr(), O_RDWR);

        if fd < 0 {
            panic!("Failed to open {}", FB_PATH);
        }

        let mem = mmap(
            ptr::null_mut(),
            FRAMEBUFFER_SIZE,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        );

        if mem == libc::MAP_FAILED {
            close(fd);
            panic!("mmap() failed");
        }

        let fb_u8 =
            slice::from_raw_parts_mut(mem as *mut u8, FRAMEBUFFER_SIZE);

        let backup = fb_u8.to_vec();

        let fb_u16 =
            slice::from_raw_parts_mut(mem as *mut u16, WIDTH * HEIGHT);

        // let mut atom_fb = [0u16; ATOM_WIDTH * ATOM_HEIGHT];
        // Draw a character of 8 by 12 pixels 96*2 bytes
        let row: usize = 8;
        let col: usize = 16;
        let char_width: usize = 8;
        let char_heigth: usize = 12;
        // Top left pixel
        let pix_x: usize = 3;
        let pix_y: usize = 4;
        let pix_i : usize = pix_y * char_width + pix_y;
        let atom_x:usize = col * FONT_WIDTH + pix_x; // 16 * 8 + 3 = 131
        let atom_y:usize = row * FONT_HEIGHT + pix_y; // 8 * 12 + 4 = 100
        let fb_u16_x: usize = atom_x + OFFSET_X; // 131 + 1664 = 1794
        let fb_u16_y: usize = atom_y + OFFSET_Y; // 100 + 0 = 100
        let index:usize = fb_u16_y * WIDTH +fb_u16_x;

        let symbol: usize = 1; // 'A'
        let mut indices = [0usize; 12 * 8];
        for y in 0..12 { // Glyph row
            put_pixel_row(&mut *fb_u16, FONT_6847[symbol * 12 + y].reverse_bits(), col * char_width, row * char_heigth + y);
            // indices[y * 8..(y + 1) * 8].copy_from_slice(&put_pixel_row(&mut *fb_u16, FONT_6847[symbol * 12 + y], col * char_width, row * char_heigth + y));
        }
        for y in 0..12 { // Glyph row
            put_pixel_row(&mut *fb_u16, FONT_6847[3 * 12 + y].reverse_bits(), (col + 1) * char_width, row * char_heigth + y);
        }

        // Index = (row * FONT_HEIGHT + OFFSET_y) * WIDTH + atom_x + OFFSET_X

        // let FONT: [u16; 96] = [
        //     BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
        //     BLACK, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, BLACK,
        //     BLACK, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, BLACK,
        //     BLACK, WHITE, WHITE, BLACK, BLACK, BLACK, WHITE, BLACK,
        //     BLACK, WHITE, WHITE, BLACK, BLACK, BLACK, WHITE, BLACK,
        //     BLACK, WHITE, WHITE, BLACK, BLACK, BLACK, WHITE, BLACK,
        //     BLACK, WHITE, WHITE, BLACK, BLACK, BLACK, WHITE, BLACK,
        //     BLACK, WHITE, WHITE, BLACK, BLACK, BLACK, WHITE, BLACK,
        //     BLACK, WHITE, WHITE, BLACK, BLACK, BLACK, WHITE, BLACK,
        //     BLACK, WHITE, WHITE, BLACK, BLACK, BLACK, WHITE, BLACK,
        //     BLACK, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, BLACK,
        //     BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,];
        // for y in 0..12 {
        //     for x in 0..8 {
        //         fb_u16[(OFFSET_X + x) * (OFFSET_Y + y)] = FONT[x * y];
        //     }
        // }


        // draw_square(&mut atom_fb, 20, 20, 60, RED);

        // draw_square(
        //     &mut atom_fb,
        //     ATOM_WIDTH - 80,
        //     20,
        //     60,
        //     GREEN,
        // );

        // draw_square(
        //     &mut atom_fb,
        //     20,
        //     ATOM_HEIGHT - 80,
        //     60,
        //     BLUE,
        // );

        // draw_square(
        //     &mut atom_fb,
        //     ATOM_WIDTH - 80,
        //     ATOM_HEIGHT - 80,
        //     60,
        //     WHITE,
        // );

    //     for y in 0..ATOM_HEIGHT {
    //         let src_start = y * ATOM_WIDTH;
    //         let src_end = src_start + ATOM_WIDTH;
    //         let dst_start = (OFFSET_Y + y) * WIDTH + OFFSET_X;

    //         fb[dst_start..dst_start + ATOM_WIDTH]
    //             .copy_from_slice(&atom_fb[src_start..src_end]);
    // }

        // println!("Sleeping for 5 seconds...");
        thread::sleep(Duration::from_secs(5));

        println!("Restoring framebuffer...");

        fb_u8.copy_from_slice(&backup);

        munmap(mem, FRAMEBUFFER_SIZE);
        close(fd);

        // for row in 0..12 {
        //     for col in 0..8 {
        //         print!("{:04X}, ", &indices[row * col]);
        //     }
        //     println!();
        // }
    }
    // println!("Row 4: {:02x} = 0x14", FONT_6847[1 * 12 + 4]);
    println!("Done.");

    Ok(())
}
// pub struct Framebuffer {
//     pub WIDTH: usize = 192,
//     pub HEIGHT: usize = 256,
//     pub pixels: [u16; ATOM_WIDTH * ATOM_HEIGHT],
// }

// fn draw_char(
//     fb: &mut Framebuffer,
//     x: usize,
//     y: usize,
//     ch: u8,) {
//     for glyph_row in 0..12 {
//         let bits = FONT[ch as usize][glyph_row];

//         for glyph_col in 0..8 {
//             if bits & (0x80 >> glyph_col) != 0 {
//                 fb.set_pixel(
//                     x + glyph_col,
//                     y + glyph_row,
//                     1,
//                 );
//             }
//         }
//     }
// }
// fn test_text() {
//     let msg = b"ACORN ATOM";

//     for (i, c) in msg.iter().enumerate() {
//         text_ram[3 * 32 + 10 + i] = *c;
//     }
// }
// use std::time::Instant;
fn main() {
    let res: std::io::Result<()> = test_hdmi();
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
    // let start = Instant::now();
    for _i in 0..260 { // Timed to 49 us on RPi 1 => 188 ns/instr. :-)
        cpu6502::decode(&mut bus, &mut cpu);
    } // PC now 0xFF5B
    // println!(
    //         "Executed 260 instructions in {:.3?} us",
    //         start.elapsed().as_micros()
    //     );
    println!("PC {:04X} Acc: {:02X}, X: {:02X}, Y: {:02X}", cpu.pc, cpu.a, cpu.x, cpu.y);
}

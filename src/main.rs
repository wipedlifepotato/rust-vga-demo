#![no_std]
#![no_main]
use core::panic::PanicInfo;
const TABLE: [u8; 128] = {
	let mut arr = [0u8; 128];
	let mut i = 0;
	while i < 128 {
		arr[i]=i as u8;
		i+=1;
	}
	arr
};
static mut SEED: u32 = 42; 
#[repr(u8)]
#[derive(Copy, Clone)]
enum Color {
    Black        = 0x0,
    Blue         = 0x1,
    Green        = 0x2,
    Cyan         = 0x3,
    Red          = 0x4,
    Magenta      = 0x5,
    Brown        = 0x6,
    LightGray    = 0x7,
    DarkGray     = 0x8,
    LightBlue    = 0x9,
    LightGreen   = 0xA,
    LightCyan    = 0xB,
    LightRed     = 0xC,
    LightMagenta = 0xD,
    Yellow       = 0xE,
    White        = 0xF,
}
const COLORS: [Color; 16] = [
    Color::Black,
    Color::Blue,
    Color::Green,
    Color::Cyan,
    Color::Red,
    Color::Magenta,
    Color::Brown,
    Color::LightGray,
    Color::DarkGray,
    Color::LightBlue,
    Color::LightGreen,
    Color::LightCyan,
    Color::LightRed,
    Color::LightMagenta,
    Color::Yellow,
    Color::White,
];
fn make_vga_byte(fg: Color, bg: Color) -> u8 {
    ((bg as u8) << 4) | (fg as u8)
}
fn rdtsc() -> u32 {
    let lo: u32;
    let hi: u32;
    unsafe {
        core::arch::asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi
        );
    }
    lo ^ hi
}

fn init_seed() {
    unsafe { SEED = rdtsc(); }
}
fn rand() -> u32 {
    unsafe {
        SEED = SEED.wrapping_mul(1664525).wrapping_add(1013904223);
        SEED
    }
}
fn rand_color() -> Color {
    let index = (rand() % 16) as usize;
    COLORS[index]
}
fn delay(count: u32) {
    for _ in 0..count {
        unsafe { core::arch::asm!("nop") }; 
    }
}
//static HELLO: &[u8] = b"abcdefghczsdfqwerasdfwqerq1123!@#!@#";
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let mut position:isize = 0;

    loop {
    	unsafe {
    		init_seed();
    		let index = (rand() % 128) as usize;
    		*vga_buffer.offset(position%2000 as isize * 2) = TABLE[index];
    		let fg = rand_color();
		let bg = rand_color();
		let color = make_vga_byte(fg, bg);
    		*vga_buffer.offset(position%2000 as isize *2 + 1)=color;
    		position+=1;
    		delay(60000);

    	}
    }

}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

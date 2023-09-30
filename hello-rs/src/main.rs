#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
use bios::{add_intc_handler, enable_intc};
use core::panic::PanicInfo;
use core::fmt::Write;
use graphics_2d::{Color, Point};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut out = debug::EEOut;

    writeln!(out, "PANIC! {}", info).ok();
    loop {}
}

extern crate prussia_2d as graphics_2d;
extern crate prussia_bios as bios;
extern crate prussia_debug as debug;
extern crate prussia_dma as dma;
extern crate prussia_gs as gs;
extern crate prussia_rt as rt;

static mut count: u32 = 0;
fn draw_cb(cause: i32) -> i32 {
    let mut triangle = [
        Point { x: 0, y: 100, z: 1 },
        Point {
            x: 50,
            y: 150,
            z: 1,
        },
        Point { x: 0, y: 150, z: 1 },
    ];

    unsafe {
        count += 5;

        triangle[0].x += count;
        if triangle[0].x > 640 {
            triangle[0].x = 0;
            count = 0;
        }
        triangle[1].x += count;
        triangle[2].x += count;
    }
    graphics_2d::draw_tri(
        Point { x: 0, y: 0, z: 1 },
        Point { x: 0, y: 448, z: 1 },
        Point { x: 640, y: 0, z: 1 },
        Color { r: 0x33, g: 0x33, b: 0x33, a: 0 },
    );
    graphics_2d::draw_tri(
        Point { x: 0, y: 448, z: 1 },
        Point { x: 640, y: 0, z: 1 },
        Point { x: 640, y: 448, z: 1 },
        Color { r: 0x33, g: 0x33, b: 0x33, a: 0 },
    );

    graphics_2d::draw_tri(
        triangle[0],
        triangle[1],
        triangle[2],
        Color { r: unsafe { (count & u8::MAX as u32) as u8 },
            g: 0x66,
            b: 0x66,
            a: 0xff,
        },
    );
    return 0;
}

#[no_mangle]
fn main() -> ! {
    let mut out = debug::EEOut;
    writeln!(out, "[EE] Hello, from rust!").unwrap();

    graphics_2d::setup_env();
    add_intc_handler(bios::INTCCause::VBON, draw_cb, -1);
    enable_intc(bios::INTCCause::VBON);

    writeln!(out, "Finished setup").unwrap();
    bios::sleep_thread();
    bios::exit(0);
}

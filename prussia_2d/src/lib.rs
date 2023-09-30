//! Routines for the PlayStation 2 Emotion Engine Interrupt Controller.

#![no_std]
#![deny(missing_docs)]
#![feature(array_chunks)]
use aligned::{Aligned, A16};
use core::fmt::Write;
use core::mem;
use gs::gs_set_test;
use prussia_bios as bios;
use prussia_debug as debug;
use prussia_dma as dma;
use prussia_gs as gs;
mod draw;

/// Sets up a simple 640x448 framebuffer.
pub fn setup_env() {
    // Reset the GIF
    gs::privileged::CSR::new().with_reset(1).store();
    gs::privileged::DISPFB2::new()
        .with_fbp(0x00)
        .with_fbw(10)
        .with_psm(0x00)
        .store();
    gs::privileged::DISPLAY2::new()
        .with_dx(0)
        .with_dy(0)
        .with_magh(3)
        .with_dw(2559)
        .with_dh(447)
        .store();
    gs::privileged::PMODE::new()
        .with_en1(false)
        .with_en2(true)
        .with_mmod(false)
        .with_amod(true)
        .with_alp(0x80)
        .store();
    bios::set_gs_crt(
        bios::Interlacing::Interlaced,
        bios::VideoMode::NTSC,
        bios::FieldFrameMode::Frame,
    );
    static mut GIF_BUFFER: Aligned<A16, [u128; 10]> = Aligned([16; 10]);
    unsafe {
        GIF_BUFFER[0] = gs::pack(gs::gif_set_tag(9, 1, 0, 0, 0, 1), 0x0e);
        GIF_BUFFER[1] = gs::packreg(gs::GP::FRAME_1, gs::gs_set_frame(0x00, 10, 0, 0));
        GIF_BUFFER[2] = gs::packreg(gs::GP::ZBUF_1, gs::gs_set_zbuf(0x46000 / 2048, 0, 0));
        GIF_BUFFER[3] = gs::packreg(gs::GP::PRMODECONT, gs::gs_set_prmodecont(1));
        GIF_BUFFER[4] = gs::packreg(gs::GP::XYOFFSET_1, gs::gs_set_xyoffset(0, 0));
        GIF_BUFFER[5] = gs::packreg(
            gs::GP::SCISSOR_1,
            gs::gs_set_scissor(0, 639 as u64, 0, 447 as u64),
        );
        GIF_BUFFER[6] = gs::packreg(gs::GP::TEST_1, gs_set_test(0, 0, 0, 0, 0, 0, 1, 1));
        GIF_BUFFER[7] = gs::packreg(gs::GP::ALPHA_1, gs::gs_set_alpha(0, 1, 0, 1, 0));
        GIF_BUFFER[8] = gs::packreg(gs::GP::TEXA, gs::gs_set_texa(0x80, 0, 0x80));
        GIF_BUFFER[9] = gs::packreg(gs::GP::FBA_1, gs::gs_set_fba(0));

        let dma_from = dma::Transfer::from_mem(dma::Gif, &mut GIF_BUFFER);

        dma_from.wait();
    }
}

/// A simple color
#[derive(Copy, Clone)]
pub struct Color {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// Alpha component
    pub a: u8,
}

/// A simple point
#[derive(Copy, Clone)]
pub struct Point {
    /// X coordinate
    pub x: u32,
    /// Y coordinate
    pub y: u32,
    /// Z coordinate
    pub z: u32,
}

/// Draw a simple flat shaded triangle
pub fn draw_tri(v0: Point, v1: Point, v2: Point, color: Color) {
    static mut GIF_BUFFER: Aligned<A16, [u128; 5]> = Aligned([16; 5]);
    unsafe {
        GIF_BUFFER[0] = gs::pack(
            gs::gif_set_tag(1, 1, 1, gs::gs_set_prim(3, 0, 0, 0, 0, 1, 0, 0, 0), 0, 4),
            0x0e | 0x0e << 4 | 0x0e << 8 | 0x0e << 12,
        );
        GIF_BUFFER[1] = gs::packreg(
            gs::GP::RGBAQ,
            gs::gs_set_rgbaq(color.r, color.g, color.b, color.a, 0.0),
        );
        GIF_BUFFER[2] = gs::packreg(gs::GP::XYZ2, gs::gs_set_xyz(v0.x << 4, v0.y << 4, v0.z));
        GIF_BUFFER[3] = gs::packreg(gs::GP::XYZ2, gs::gs_set_xyz(v1.x << 4, v1.y << 4, v1.z));
        GIF_BUFFER[4] = gs::packreg(gs::GP::XYZ2, gs::gs_set_xyz(v2.x << 4, v2.y << 4, v2.z));

        let dma_from = dma::Transfer::from_mem(dma::Gif, &mut GIF_BUFFER);
        dma_from.wait();
    }
}

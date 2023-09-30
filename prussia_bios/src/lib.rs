//! A Rust implementation of the PlayStation 2 BIOS.

#![no_std]
#![deny(missing_docs)]
#![feature(asm_experimental_arch)]

use core::arch::asm;

/// EE Component Reset Flags, passed to `reset_ee`.
pub enum EEResetFlag {
    /// Reset the DMAC.
    DMAC = 0x1,
    /// Reset VU1.
    VU1 = 0x2,
    /// Reset the VIF1.
    VIF1 = 0x4,
    /// Reset the GIF.
    GIF = 0x8,
    /// Reset VU0.
    VU0 = 0x10,
    /// Reset the VIF0.
    VIF0 = 0x20,
    /// Reset the IPU.
    IPU = 0x40,
}

/// Reset the EE components depending on the given reset flags.
pub fn reset_ee(reset_flag: u32) {
    unsafe {
        asm!(
            "syscall",
            in("$3") 0x1, // v1 (Syscall ID for ResetEE)
            in("$4") reset_flag, // a0 (Reset flag)
        );
    }
}

/// The Graphics Synthesizer video mode, passed to `set_gs_crt`.
#[repr(i16)]
pub enum VideoMode {
    /// PAL, 720x576@50Hz interlaced, as used in most of Europe.
    NTSC = 2,
    /// NTSC, 720x480@59.97Hz interlaced, as used in North America and Japan.
    PAL = 3,
}

/// Whether interlacing is enabled, passed to `set_gs_crt`.
#[repr(i16)]
pub enum Interlacing {
    /// Do not interlace frames.
    Noninterlaced = 0,
    /// Interlace frames, blending between them.
    Interlaced = 1,
}

/// Whether to read every line or every other line, passed to `set_gs_crt`.
#[repr(i16)]
pub enum FieldFrameMode {
    /// Read every line from a frame.
    Frame = 0,
    /// Read every other line from a frame.
    Field = 1,
}

/// Configure the Graphics Synthesizer's display controller to output a given VideoMode.
pub fn set_gs_crt(imode: Interlacing, vmode: VideoMode, ffmode: FieldFrameMode) {
    unsafe {
        asm!(
            "syscall",
            in("$3") 0x2, // v1
            in("$4") imode as i16, // a0
            in("$5") vmode as i16, // a1
            in("$6") ffmode as i16, // a2
        );
    }
}

/// Exit the program and return to the PS2 browser.
///
/// This function is not recommended for use if developing with PS2Link; it won't return to PS2Link
/// but instead go straight to the browser, meaning you'll need to relaunch PS2Link. Instead, use
/// `sleep_thread`.
pub fn exit(return_code: u32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("$3") 0x4, // v1
            in("$4") return_code, // a0
            options(noreturn),
        );
    }
}

/// Loads the given ELF file and executes it.
/// Threads, semaphores, and kernel state is not preserved.
/// This function does not return.
pub fn load_exec_ps2(filename: &str, argc: i32, argv: *const *const u8) {
    unsafe {
        asm!(
            "syscall",
            in("$3") 0x6, // v1
            in("$4") filename.as_ptr(), // a0
            in("$5") argc, // a1
            in("$6") argv, // a2
            options(noreturn),
        );
    }
}

/// Clears the kernel state and creates a thread with priority 0.
/// This thread will execute the given entry point with the given arguments.
/// This function does not return.
pub fn exec_ps2(entry: u32, gp: u32, argc: i32, argv: *const *const u8) {
    unsafe {
        asm!(
            "syscall",
            in("$3") 0x7, // v1
            in("$4") entry, // a0
            in("$5") gp, // a1
            in("$6") argc, // a2
            in("$7") argv, // a3
            options(noreturn),
        );
    }
}

/// The interrupt controller status. Each flag is set if the interrupt controller detects a
/// change in state. Writing a bit acknowledges that interrupt. If the status and
/// corresponding mask bit are both set, INT0 is raised.
pub enum INTCCause {
    /// An interrupt from the Graphics Synthesizer.
    GS,
    /// An interrupt from the SBUS connection to the IOP.
    SBUS,
    /// The start of VBlank.
    VBON,
    /// The end of VBlank.
    VBOF,
    /// An interrupt from VU Interface 0.
    VIF0,
    /// An interrupt from VU Interface 1.
    VIF1,
    /// An interrupt from Vector Unit 0.
    VU0,
    /// An interrupt from Vector Unit 1.
    VU1,
    /// An interrupt from the Image Processing Unit.
    IPU,
    /// An interrupt from Timer 0.
    TIM0,
    /// An interrupt from Timer 1.
    TIM1,
    /// An interrupt from Timer 2.
    TIM2,
    /// An interrupt from Timer 3.
    TIM3,
    /// An interrupt from the SBUS FIFO queue.
    SFIFO,
    /// An interrupt from the Vector Unit 0 watchdog.
    VU0WD,
}

/// Add an interrupt handler
/// Returns the handler ID if the operation succeeds, -1 if it fails
pub fn add_intc_handler(int_cause: INTCCause, handler: fn(i32) -> i32, next: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            out("$2") result, // v0
            in("$3") 0x10, // v1
            in("$4") int_cause as i32, // a0
            in("$5") handler as usize, // a1
            in("$6") next, // a2
        );
    }
    result
}

/// Remove an interrupt handler
/// Returns -1 if the operation fails
pub fn remove_intc_handler(int_case: INTCCause, handler_id: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            out("$2") result, // v0
            in("$3") 0x11, // v1
            in("$4") int_case as i32, // a0
            in("$5") handler_id, // a1
        );
    }
    result
}

/// The interrupt controller status. Each flag is set if the interrupt controller detects a
/// change in state. Writing a bit acknowledges that interrupt. If the status and
/// corresponding mask bit are both set, INT0 is raised.
pub enum DMACause {
    /// An interrupt from the Vector InterFace 0 channel.
    VIFO,
    /// An interrupt from the Vector InterFace 1 channel.
    VIF1,
    /// An interrupt from the GIF channel.
    GIF,
    /// An interrupt from the IPU FROM channel.
    FROMIPU,
    /// An interrupt from the IPU TO channel.
    TOIPU,
    /// An interrupt from the Serial InterFace 0 channel.
    SIF0,
    /// An interrupt from the Serial InterFace 1 channel.
    SIF1,
    /// An interrupt from the Serial InterFace 2 channel.
    SIF2,
    /// An interrupt from the Scratch Pad Ram FROM channel.
    FROMSPR,
    /// An interrupt from the Scratch Pad Ram TO channel.
    TOSPR
}

/// Add a DMAC handler
/// Returns the handler ID if the operation succeeds, -1 if it fails
pub fn add_dmac_handler(dma_cause: DMACause, handler: fn(i32) -> i32, next: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            out("$2") result, // v0
            in("$3") 0x12, // v1
            in("$4") dma_cause as i32, // a0
            in("$5") handler as usize, // a1
            in("$6") next, // a2
        );
    }
    result
}

/// Remove a DMAC handler
/// Returns -1 if the operation fails
pub fn remove_dmac_handler(dma_cause: DMACause, handler_id: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            out("$2") result, // v0
            in("$3") 0x13, // v1
            in("$4") dma_cause as i32, // a0
            in("$5") handler_id, // a1
        );
    }
    result
}

/// Enables the bit in the interrupt mask register.
/// Returns true if the bit was set to 0 and false if it was already set to 1.
pub fn enable_intc(int_cause: INTCCause) -> bool {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            out("$2") result, // v0
            in("$3") 0x14, // v1
            in("$4") int_cause as i32, // a0
        );
    }
    result == 1
}

/// Disables the bit in the interrupt mask register.
/// Returns true if the bit was set to 1 and false if it was already set to 0.
pub fn disable_intc(int_cause: INTCCause) -> bool {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            out("$2") result, // v0
            in("$3") 0x15, // v1
            in("$4") int_cause as i32, // a0
        );
    }
    result == 1
}

/// Enables the bit in the dmac interrupt mask register.
/// Returns true if the bit was set to 0 and false if it was already set to 1.
pub fn enable_dmac(int_cause: DMACause) -> bool {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            out("$2") result, // v0
            in("$3") 0x16, // v1
            in("$4") int_cause as i32, // a0
        );
    }
    result == 1
}

/// Disables the bit in the dmac interrupt mask register.
/// Returns true if the bit was set to 1 and false if it was already set to 0.
pub fn disable_dmac(int_cause: DMACause) -> bool {
    let result: i32;
    unsafe {
        asm!(
            "syscall",
            out("$2") result, // v0
            in("$3") 0x17, // v1
            in("$4") int_cause as i32, // a0
        );
    }
    result == 1
}

/// Decrements the current threads 'wakeup_count' if it is greater than one.
/// If the wakeup count is 0 the current thread is set to WAIT status and a thread reschedule occurs
pub fn sleep_thread() {
    unsafe {
        asm!(
            "syscall",
            in("$3") 0x32, // v1
        );
    }
}

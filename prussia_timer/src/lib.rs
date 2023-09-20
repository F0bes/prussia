//! Routines for the PlayStation 2 EE Timers

#![no_std]
#![deny(missing_docs)]
use bitflags::bitflags;
use core::ptr;

/// The available EE Timers
#[derive(PartialEq, Clone, Copy)]
pub enum EETimers {
    /// Timer 0
    Timer0,
    /// Timer 1
    Timer1,
    /// Timer 2
    Timer2,
    /// Timer 3
    Timer3,
}

bitflags! {
    /// Timer mode flags.
    pub struct TimerMode : u16 {
        /// Bus Clock (~147 MHz)
        const Clock_Bus = 0x0 << 0;
        /// Bus Clock (~147 MHz) / 16
        const Clock_Bus_Div_16 = 0x1 << 0;
        /// Bus Clock (~147 MHz) / 256
        const Clock_Bus_Div_256 = 0x2 << 0;
        /// HBLANK Clock
        const Clock_HBLANK = 0x3 << 0;

        /// Enable Gate
        const Gate_Enable = 0x1 << 2;

        /// Trigger Gate on HBLANK
        const Gate_Type_HBLANK = 0x0 << 3;
        /// Trigger Gate on VBLANK
        const Gate_Type_VBLANK = 0x1 << 3;

        /// Count While the Gate is Not Active
        const Gate_Mode_Not_Active = 0x0 << 4;
        /// Reset Counter When the Gate Goes From Low to High
        const Gate_Mode_LH = 0x1 << 4;
        /// Reset Counter When the Gate Goes From High to Low
        const Gate_Mode_HL = 0x2 << 4;
        /// Reset Counter When the Gate Goes From Low to High or High to Low
        const Gate_Mode_LH_HL = 0x3 << 4;

        /// Clear the Counter When the Compare Value is Reached
        const Clear_Comp = 0x1 << 6;
        /// Enable the timer
        const Enable = 0x1 << 7;
        /// Trigger an interrupt when the compare value is reached
        const Compare_Interrupt_Enable = 0x1 << 8;
        /// Trigger an interrupt when the counter overflows
        const Overflow_Interrupt_Enable = 0x1 << 9;
        /// Compare Interrupt flag (Write to clear)
        const Compare_Interrupt_Flag = 0x1 << 10;
        /// Overflow Interrupt flag (Write to clear)
        const Overflow_Interrupt_Flag = 0x1 << 11;
    }
}

/// A wrapper around an EE hardware timer
pub struct EETimer {
    base_addr: u32,
    timer: EETimers,
}

impl EETimer {
    /// Creates a wrapper around an EE hardware timer
    pub fn new(timer: EETimers) -> Self {
        let base_addr = match timer {
            EETimers::Timer0 => 0x10000000,
            EETimers::Timer1 => 0x10000800,
            EETimers::Timer2 => 0x10001000,
            EETimers::Timer3 => 0x10001800,
        };
        EETimer { base_addr, timer }
    }
    /// Returns the current count of the timer
    pub fn get_count(&self) -> u16 {
        unsafe { ptr::read_volatile((self.base_addr + 0x00) as *mut u16) }
    }
    /// Sets the current count of the timer
    pub fn set_count(&self, count: u16) {
        unsafe { ptr::write_volatile((self.base_addr + 0x00) as *mut u16, count) }
    }
    /// Returns the current mode of the timer
    pub fn get_mode(&self) -> TimerMode {
        unsafe { ptr::read_volatile((self.base_addr + 0x10) as *mut TimerMode) }
    }
    /// Sets the current mode of the timer
    pub fn set_mode(&self, mode: TimerMode) {
        unsafe { ptr::write_volatile((self.base_addr + 0x10) as *mut TimerMode, mode) }
    }
    /// Returns the current compare value of the timer
    pub fn get_compare(&self) -> u16 {
        unsafe { ptr::read_volatile((self.base_addr + 0x20) as *mut u16) }
    }
    /// Sets the current compare value of the timer
    pub fn set_compare(&self, compare: u16) {
        unsafe { ptr::write_volatile((self.base_addr + 0x20) as *mut u16, compare) }
    }
    /// Returns the current hold value of the timer
    /// Only valid for Timer0 and Timer1
    pub fn get_hold(&self) -> u16 {
        if self.timer == EETimers::Timer0 || self.timer == EETimers::Timer1 {
            unsafe { ptr::read_volatile((self.base_addr + 0x30) as *mut u16) }
        } else {
            panic!(
                "What are you doing? Timer {} does not have a hold register!",
                self.timer as u8
            );
        }
    }
    /// Sets the current hold value of the timer
    /// Only valid for Timer0 and Timer1
    pub fn set_hold(&self, hold: u16) {
        if self.timer == EETimers::Timer0 || self.timer == EETimers::Timer1 {
            unsafe { ptr::write_volatile((self.base_addr + 0x30) as *mut u16, hold) }
        } else {
            panic!(
                "What are you doing? Timer {} does not have a hold register!",
                self.timer as u8
            );
        }
    }
    /// Helper function to clear the interrupt flags
    pub fn clear_interrupts(&self) {
        self.set_mode(
            self.get_mode() | TimerMode::Compare_Interrupt_Enable | TimerMode::Overflow_Interrupt_Flag,
        );
    }
}

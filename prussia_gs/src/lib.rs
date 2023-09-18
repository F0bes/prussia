//! Routines for the PlayStation 2 Graphics Synthesizer.

#![no_std]
#![deny(missing_docs)]
use core::ptr;

use bitflags::bitflags;

static mut PMODE: *mut u64 = 0x1200_0000 as *mut u64;
static mut SMODE1: *mut u64 = 0x1200_0010 as *mut u64;
static mut SMODE2: *mut u64 = 0x1200_0020 as *mut u64;
static mut SRFSH: *mut u64 = 0x1200_0030 as *mut u64;
static mut SYNCH1: *mut u64 = 0x1200_0040 as *mut u64;
static mut SYNCH2: *mut u64 = 0x1200_0050 as *mut u64;
static mut SYNCV: *mut u64 = 0x1200_0060 as *mut u64;
static mut DISPFB1: *mut u64 = 0x1200_0070 as *mut u64;
static mut DISPLAY1: *mut u64 = 0x1200_0080 as *mut u64;
static mut DISPFB2: *mut u64 = 0x1200_0090 as *mut u64;
static mut DISPLAY2: *mut u64 = 0x1200_00A0 as *mut u64;
static mut EXTBUF: *mut u64 = 0x1200_00B0 as *mut u64;
static mut EXTDATA: *mut u64 = 0x1200_00C0 as *mut u64;
static mut EXTWRITE: *mut u64 = 0x1200_00D0 as *mut u64;
static mut BGCOLOR: *mut u64 = 0x1200_00E0 as *mut u64;
static mut CSR: *mut u64 = 0x1200_1000 as *mut u64;
static mut IMR: *mut u64 = 0x1200_1010 as *mut u64;
static mut BUSDIR: *mut u64 = 0x1200_1040 as *mut u64;
static mut SIGLBLID: *mut u64 = 0x1200_1080 as *mut u64;

bitflags! {
    /// PCRTC Mode Settings
    pub struct PMODE: u64 {
        /// Read Circuit 1 Enable
        const EN1   = 0x1 << 0;
        /// Read Circuit 2 Enable
        const EN2   = 0x1 << 1;
        /// CRT Output Switching (Should be 001)
        const CRTMD = 0x7 << 2;
        /// Alpha Value Selection for Alpha Blending
        const MMOD  = 0x1 << 5;
        /// OUT1 Alpha Output Selection
        const AMOD  = 0x1 << 6;
        /// Alpha Blending Method Selection
        const SLBG  = 0x1 << 7;
        /// Fixed Alpha Value (0xff = 1.0)
        const ALP   = 0xFF << 8;
    }
}

impl PMODE {
    /// Set the PCRTC Mode Setting Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(PMODE, self.bits) };
    }
}

bitflags! {
    /// Video Synchronization Settings
    pub struct SMODE1: u64 {
        /// PLL Reference Divider
        const RC     = 0x7 << 0;
        /// PLL Loop Divider
        const LC     = 0x3F8 << 3;
        /// ?
        const T1248  = 0x3 << 10;
        /// ?
        const SLCK   = 0x1 << 12;
        /// Color Subcarrier
        /// 2 -> NTSC
        /// 3 -> PAL
        const CMOD   = 0x3 << 13;
        /// ?
        const EX     = 0x1 << 15;
        /// PCRTC Reset
        const PRST   = 0x1 << 16;
        /// PLL (Phase-locked  loop)
        const SINT   = 0x1 << 17;
        /// ?
        const XPCK   = 0x1 << 18;
        /// ?
        const PCK    = 0x3 << 19;
        /// Sub-Pixel Magnification Level
        const SPML   = 0xF << 21;
        /// Component Color Mode
        /// 0 -> RGB
        /// 1 -> YPbPr
        const GCONT  = 0x1 << 25;
        /// ?
        const PHS    = 0x1 << 26;
        /// ?
        const PVS    = 0x1 << 27;
        /// ?
        const PEHS   = 0x1 << 28;
        /// ?
        const PEVS   = 0x1 << 29;
        /// ?
        const CLKSEL = 0x3 << 30;
        /// ?
        const NVCK   = 0x1 << 32;
        /// ?
        const SLCK2  = 0x1 << 33;
        /// ?
        const VCKSEL = 0x3 << 34;
        /// Mode Choice
        /// 0 -> Interlaced
        /// 1 -> Progressive
        const VHP    = 0x1 << 36;
    }
}

impl SMODE1 {
    /// Set the video synchronization settings Register
    #[deprecated(note = "**This function uses an undocumented register**
    There is the possibility that you can damage any video device connected to your PS2!
    Please be careful when changing these parameters!")]
    pub fn store(self) {
        unsafe { ptr::write_volatile(SMODE1, self.bits) };
    }
}

bitflags! {
    /// Video Synchronization Settings
    pub struct SMODE2: u64 {
        /// Interlace Mode Setting
        /// 0 -> Non-Interlace
        /// 1 -> Interlace
        const INT = 0x1 << 0;
        /// Setting in Interlace Mode
        /// 0 -> FIELD Mode (Every other line)
        /// 1 -> FRAME Mode (Every line)
        const FFMD = 0x1 << 1;
        /// VESA DPMS Mode Settings
        /// 00 -> On
        /// 01 -> Standby
        /// 10 -> Suspend
        /// 11 -> Off
        const DPMS = 0x3 << 2;
    }
}

impl SMODE2 {
    /// Set the video synchronization settings Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(SMODE2, self.bits) };
    }
}

bitflags! {
    /// DRAM Refresh Settings
    pub struct SRFSH: u64 {
        /// ?
        const SRFSH = 0xFFFFFFFFFFFFFFFF << 0;
    }
}

impl SRFSH {
    /// Set the DRAM Refresh Settings Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(SRFSH, self.bits) };
    }
}

bitflags! {
    /// Video Synchronization Settings
    pub struct SYNCH1: u64 {
        /// ?
        const HFP  = 0x7FF << 0;
        /// ?
        const HBP  = 0x7FF << 11;
        /// ?
        const HSEQ = 0x7FF << 22;
        /// ?
        const HSVS = 0x7FF << 33;
        /// ?
        const HS   = 0x7FF << 44;
    }
}

impl SYNCH1 {
    /// Set the video synchronization settings Register
    #[deprecated(note = "**This function uses an undocumented register**
    There is the possibility that you can damage any video device connected to your PS2!
    Please be careful when changing these parameters!")]
    pub fn store(self) {
        unsafe { ptr::write_volatile(SYNCH1, self.bits) };
    }
}

bitflags! {
    /// Video Synchronization Settings
    pub struct SYNCH2: u64 {
        /// ?
        const HF  = 0x7FF << 0;
        /// ?
        const HB  = 0x7FF << 11;
    }
}

impl SYNCH2 {
    /// Set the video synchronization settings Register
    #[deprecated(note = "**This function uses an undocumented register**
    There is the possibility that you can damage any video device connected to your PS2!
    Please be careful when changing these parameters!")]
    pub fn store(self) {
        unsafe { ptr::write_volatile(SYNCH2, self.bits) };
    }
}

bitflags! {
    /// Video Synchronization Settings
    pub struct SYNCV: u64 {
        /// Vertical Front Porch?
        const VFP  = 0x3FF << 0;
        /// ?
        const VFPE = 0x3FF << 10;
        /// Vertical Back Porch?
        const VBP  = 0xFFF << 20;
        /// ?
        const VBPE = 0xFFF << 32;
        /// ?
        const VDP  = 0x7FF << 42;
        /// ?
        const VS   = 0xF << 53;
    }
}

impl SYNCV {
    /// Set the video synchronization settings
    #[deprecated(note = "**This function uses an undocumented register**
    There is the possibility that you can damage any video device connected to your PS2!
    Please be careful when changing these parameters!")]
    pub fn store(self) {
        unsafe { ptr::write_volatile(SYNCV, self.bits) };
    }
}

bitflags! {
    /// Rectangle Display Area for Circuit 1 Register Settings
    pub struct DISPFB1: u64 {
        /// Frame Buffer Address
        const FBP  = 0x1FF << 0;
        /// Frame Buffer Width
        const FBW  = 0x3F << 9;
        /// Frame Buffer Pixel Storage Format
        const PSM  = 0x1F << 15;
        /// X position of the upper left corner of the display area
        const DBX  = 0x7FF << 32;
        /// Y position of the upper left corner of the display area
        const DBY  = 0x7FF << 43;
    }
}

impl DISPFB1 {
    /// Set Rectangle Display Area for Circuit 1 Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(DISPFB1, self.bits) };
    }
}

bitflags! {
    /// Rectangle Display Area for Circuit 1 Register Settings
    pub struct DISPLAY1: u64 {
        /// x Position in the Display Area
        const DX   = 0xFFF << 0;
        /// y Position in the Display Area
        const DY   = 0x7FF << 12;
        /// Magnification in H Direction
        const MAGH = 0xF << 23;
        /// Magnification in V Direction
        const MAGV = 0x3 << 27;
        /// Display Area Width
        const DW   = 0xFFF << 32;
        /// Display Area Height
        const DH   = 0x7FF << 44;
    }
}

impl DISPLAY1 {
    /// Set Rectangle Display Area for Circuit 1 Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(DISPLAY1, self.bits) };
    }
}

bitflags! {
    /// Rectangle Display Area for Circuit 2 Register Settings
    pub struct DISPFB2: u64 {
        /// Frame Buffer Address
        const FBP  = 0x1FF << 0;
        /// Frame Buffer Width
        const FBW  = 0x3F << 9;
        /// Frame Buffer Pixel Storage Format
        const PSM  = 0x1F << 15;
        /// X position of the upper left corner of the display area
        const DBX  = 0x7FF << 32;
        /// Y position of the upper left corner of the display area
        const DBY  = 0x7FF << 43;
    }
}

impl DISPFB2 {
    /// Set Rectangle Display Area for Circuit 2 Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(DISPFB2, self.bits) };
    }
}

bitflags! {
    /// Rectangle Display Area for Circuit 2 Register Settings
    pub struct DISPLAY2: u64 {
        /// x Position in the Display Area
        const DX   = 0xFFF << 0;
        /// y Position in the Display Area
        const DY   = 0x7FF << 12;
        /// Magnification in H Direction
        const MAGH = 0xF << 23;
        /// Magnification in V Direction
        const MAGV = 0x3 << 27;
        /// Display Area Width
        const DW   = 0xFFF << 32;
        /// Display Area Height
        const DH   = 0x7FF << 44;
    }
}

impl DISPLAY2 {
    /// Set Rectangle Display Area for Circuit 2 Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(DISPLAY2, self.bits) };
    }
}

bitflags! {
    /// Feedback Write Settings
    pub struct EXTBUF: u64 {
        /// Base Address of the Feedback Buffer
        const EXBP  = 0x3FFF << 0;
        /// Width of the Feedback Buffer
        const EXBW  = 0x3F << 14;
        /// Selection of Input Source
        const FBIN  = 0x3 << 20;
        /// Interlace Mode
        const WFFMD = 0x1 << 22;
        /// Method of Processing Input Alpha
        const EMODA = 0x3 << 23;
        /// Method of Processing Input Color
        const EMODC = 0x3 << 25;
        /// X Coordinate of the Top Left Corner of the Feedback Buffer
        const WDX   = 0x7FFF << 32;
        /// Y Coordinate of the Top Left Corner of the Feedback Buffer
        const WDY   = 0x7FFF << 43;
    }
}

impl EXTBUF {
    /// Set the Feedback Write Buffer Setting Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(EXTBUF, self.bits) };
    }
}

bitflags! {
    /// Feedback Write Settings
    pub struct EXTDATA: u64 {
        /// X Coordinate of the Top Left Corner of the Feedback Buffer
        const SX  = 0x1FFF << 0;
        /// Y Coordinate of the Top Left Corner of the Feedback Buffer
        const SY  = 0xFFF << 12;
        /// Sampling Rate in H Direction
        const SMPH  = 0xF << 23;
        /// Sampling Rate in V Direction
        const SMPV = 0x3 << 27;
        /// Rectangular Area Width - 1
        const WW = 0x1FFF << 32;
        /// Rectangular Area Height - 1
        const WH = 0xFFF << 44;
    }
}

impl EXTDATA {
    /// Set the Feedback Write Buffer Setting Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(EXTDATA, self.bits) };
    }
}

bitflags! {
    /// Feedback Write Function Control
    pub struct EXTWRITE: u64 {
        /// Activation / Deactivation of Write
        const WRITE = 0x1 << 0;
    }
}

impl EXTWRITE {
    /// Set the Feedback Write Control Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(EXTWRITE, self.bits) };
    }
}

bitflags! {
    /// Background Color Settings
    pub struct BGCOLOR: u64 {
        /// Luminance of R element
        const R = 0xFF << 0;
        /// Luminance of G element
        const G = 0xFF << 8;
        /// Luminance of B element
        const B = 0xFF << 16;
    }
}

impl BGCOLOR {
    /// Set the Feedback Write Control Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(BGCOLOR, self.bits) };
    }
}

bitflags! {
    /// GS System Status
    pub struct CSR: u64 {
        /// Signal Event Control
        const SIGNAL = 0x1 << 0;
        /// Finish Event Control
        const FINISH = 0x1 << 1;
        /// HSync Interrupt Control
        const HSINT  = 0x1 << 2;
        /// VSync Interrupt Control
        const VSINT  = 0x1 << 3;
        /// Rectangular Area Write Termination Interrupt Control
        const EDWINT = 0x1 << 4;
        /// Drawing Suspend and FIFO Clear
        const FLUSH  = 0x1 << 8;
        /// GS System Reset
        const RESET  = 0x1 << 9;
        /// Output Value of NFIELD Output
        const NFIELD = 0x1 << 12;
        /// Field Displayed Currently
        const FIELD  = 0x1 << 13;
        /// Host Interface FIFO Status
        const FIFO   = 0x3 << 14;
        /// Revision No. of the GS
        const REV    = 0xFF << 16;
        /// ID of the GS
        const ID     = 0xFF << 24;
    }
}

impl CSR {
    /// Set GS Status Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(CSR, self.bits) };
    }

    /// Read GS Status Register
    pub fn load() -> Self {
        let csr = unsafe { ptr::read_volatile(CSR) };
        CSR { bits: csr }
    }
}

bitflags! {
    /// Interrupt Mask Control
    pub struct IMR: u64 {
        /// Signal Event Interrupt Mask
        const SIGMSK    = 0x1 << 8;
        /// Finish Event Interrupt Mask
        const FINISHMSK = 0x1 << 9;
        /// HSynch Interrupt Mask
        const HSMSK     = 0x1 << 10;
        /// VSynch Interrupt Mask
        const VSMSK     = 0x1 << 11;
        /// Rectangular Area Write Termination Interrupt Mask
        const EDWMSK    = 0x1 << 12;
    }
}

impl IMR {
    /// Set the Interrupt Mask Control Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(IMR, self.bits) };
    }
}

bitflags! {
    /// Host Interface Bus Switching
    pub struct BUSDIR: u64 {
        /// Transmission Direction
        /// 0 -> EE -> GS
        /// 1 -> GS -> EE
        const DIR = 0x1 << 0;
    }
}

impl BUSDIR {
    /// Set the Host Interface Bus Direction Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(BUSDIR, self.bits) };
    }
}

bitflags! {
    /// Singal ID Value
    pub struct SIGLBLID: u64 {
        /// ID Set by SIGNAL Register
        const SIGID = 0xFFFFFFFF << 0;
        /// ID Set by LABEL Register
        const LBLID = 0xFFFFFFFF << 32;
    }
}

impl SIGLBLID {
    /// Set the Singal ID Value Register
    pub fn store(self) {
        unsafe { ptr::write_volatile(SIGLBLID, self.bits) };
    }

    /// Read the Singal ID Value Register
    pub fn load() -> Self {
        let siglblid = unsafe { ptr::read_volatile(SIGLBLID) };
        SIGLBLID { bits: siglblid }
    }
}

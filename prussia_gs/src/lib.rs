//! Routines for the PlayStation 2 Graphics Synthesizer.

#![no_std]
#![deny(missing_docs)]

/// Wrappers around GS privileged registers.
pub mod privileged {
    use bitfield_struct::bitfield;
    use core::ptr;
    const PMODE_ADDR: *mut u64 = 0x1200_0000 as *mut u64;
    const SMODE1_ADDR: *mut u64 = 0x1200_0010 as *mut u64;
    const SMODE2_ADDR: *mut u64 = 0x1200_0020 as *mut u64;
    const SRFSH_ADDR: *mut u64 = 0x1200_0030 as *mut u64;
    const SYNCH1_ADDR: *mut u64 = 0x1200_0040 as *mut u64;
    const SYNCH2_ADDR: *mut u64 = 0x1200_0050 as *mut u64;
    const SYNCV_ADDR: *mut u64 = 0x1200_0060 as *mut u64;
    const DISPFB1_ADDR: *mut u64 = 0x1200_0070 as *mut u64;
    const DISPLAY1_ADDR: *mut u64 = 0x1200_0080 as *mut u64;
    const DISPFB2_ADDR: *mut u64 = 0x1200_0090 as *mut u64;
    const DISPLAY2_ADDR: *mut u64 = 0x1200_00A0 as *mut u64;
    const EXTBUF_ADDR: *mut u64 = 0x1200_00B0 as *mut u64;
    const EXTDATA_ADDR: *mut u64 = 0x1200_00C0 as *mut u64;
    const EXTWRITE_ADDR: *mut u64 = 0x1200_00D0 as *mut u64;
    const BGCOLOR_ADDR: *mut u64 = 0x1200_00E0 as *mut u64;
    const CSR_ADDR: *mut u64 = 0x1200_1000 as *mut u64;
    const IMR_ADDR: *mut u64 = 0x1200_1010 as *mut u64;
    const BUSDIR_ADDR: *mut u64 = 0x1200_1040 as *mut u64;
    const SIGLBLID_ADDR: *mut u64 = 0x1200_1080 as *mut u64;

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// PCRTC Mode Settings
    pub struct PMODE {
        pub en1: bool,
        pub en2: bool,
        #[bits(3, default = 001)]
        pub crtmd: u8,
        pub mmod: bool,
        pub amod: bool,
        pub slbg: bool,
        pub alp: u8,
        #[bits(48)]
        __: i64,
    }

    impl PMODE {
        /// Set the PCRTC Mode Setting Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(PMODE_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Video Synchronization Settings
    pub struct SMODE1 {
        #[bits(3)]
        pub rc: u8,
        #[bits(7)]
        pub kc: u8,
        #[bits(2)]
        pub t1248: u8,
        #[bits(1)]
        pub slck: u8,
        #[bits(2)]
        pub cmod: u8,
        #[bits(1)]
        pub ex: u8,
        #[bits(1)]
        pub prst: u8,
        #[bits(1)]
        pub sint: u8,
        #[bits(1)]
        pub xpck: u8,
        #[bits(2)]
        pub pck2: u8,
        #[bits(4)]
        pub spml: u8,
        #[bits(1)]
        pub gcont: u8,
        #[bits(1)]
        pub phs: u8,
        #[bits(1)]
        pub pvs: u8,
        #[bits(1)]
        pub pehs: u8,
        #[bits(1)]
        pub pevs: u8,
        #[bits(2)]
        pub clksel: u8,
        #[bits(1)]
        pub nvck: u8,
        #[bits(1)]
        pub slck2: u8,
        #[bits(2)]
        pub vcksel: u8,
        #[bits(1)]
        pub vck2: u8,
        #[bits(27)]
        __: u32,
    }

    impl SMODE1 {
        /// Set the video synchronization settings Register
        ///
        /// **This function uses an undocumented register**
        /// There is the possibility that you can damage any video device connected to your PS2!
        /// Please be careful when changing these parameters!
        pub fn store(self) {
            unsafe { ptr::write_volatile(SMODE1_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Video Synchronization Settings
    pub struct SMODE2 {
        pub int: bool,
        pub ffmd: bool,
        #[bits(2)]
        pub dpms: u8,
        #[bits(60)]
        __: u64,
    }

    impl SMODE2 {
        /// Set the video synchronization settings Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(SMODE2_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// DRAM Refresh Settings
    pub struct SRFSH {
        pub srfsh: u64,
    }

    impl SRFSH {
        /// Set the DRAM Refresh Settings Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(SRFSH_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Video Synchronization Settings
    pub struct SYNCH1 {
        #[bits(11)]
        pub hfp: u16,
        #[bits(11)]
        pub hbp: u16,
        #[bits(11)]
        pub hseq: u16,
        #[bits(11)]
        pub hsvs: u16,
        #[bits(11)]
        pub hs: u16,
        #[bits(9)]
        __: u16,
    }

    impl SYNCH1 {
        /// Set the video synchronization settings Register
        ///
        /// **This function uses an undocumented register**
        /// There is the possibility that you can damage any video device connected to your PS2!
        /// Please be careful when changing these parameters!
        pub fn store(self) {
            unsafe { ptr::write_volatile(SYNCH1_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Video Synchronization Settings
    pub struct SYNCH2 {
        #[bits(11)]
        pub hf: u16,
        #[bits(11)]
        pub hb: u16,
        #[bits(42)]
        __: u64,
    }

    impl SYNCH2 {
        /// Set the video synchronization settings Register
        ///
        /// **This function uses an undocumented register**
        /// There is the possibility that you can damage any video device connected to your PS2!
        /// Please be careful when changing these parameters!
        pub fn store(self) {
            unsafe { ptr::write_volatile(SYNCH2_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Video Synchronization Settings
    pub struct SYNCV {
        #[bits(10)]
        pub vfp: u16,
        #[bits(10)]
        pub vfpe: u16,
        #[bits(12)]
        pub vbp: u16,
        #[bits(12)]
        pub vbpe: u16,
        #[bits(11)]
        pub vdp: u16,
        #[bits(9)]
        pub vs: u16,
    }

    impl SYNCV {
        /// Set the video synchronization settings Register
        ///
        /// **This function uses an undocumented register**
        /// There is the possibility that you can damage any video device connected to your PS2!
        /// Please be careful when changing these parameters!
        pub fn store(self) {
            unsafe { ptr::write_volatile(SYNCV_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Rectangle Display Area for Circuit 1 Register Settings
    pub struct DISPFB1 {
        #[bits(9)]
        pub fbp: u16,
        #[bits(6)]
        pub fbw: u16,
        #[bits(5)]
        pub psm: u16,
        #[bits(12)]
        __: u16,
        #[bits(11)]
        pub dbx: u16,
        #[bits(11)]
        pub dby: u16,
        #[bits(10)]
        __: u16,
    }

    impl DISPFB1 {
        /// Set Rectangle Display Area for Circuit 1 Register
        pub fn store(self) {
            unsafe {
                ptr::write_volatile(DISPFB1_ADDR, self.into());
            };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Rectangle Display Area for Circuit 1 Register Settings
    pub struct DISPLAY1 {
        #[bits(12)]
        pub dx: u16,
        #[bits(11)]
        pub dy: u16,
        #[bits(4)]
        pub magh: u16,
        #[bits(2)]
        pub magv: u16,
        #[bits(12)]
        pub dw: u16,
        #[bits(11)]
        pub dh: u16,
        #[bits(12)]
        __: u16,
    }

    impl DISPLAY1 {
        /// Set Rectangle Display Area for Circuit 1 Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(DISPLAY1_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Rectangle Display Area for Circuit 2 Register Settings
    pub struct DISPFB2 {
        #[bits(9)]
        pub fbp: u16,
        #[bits(6)]
        pub fbw: u16,
        #[bits(5)]
        pub psm: u16,
        #[bits(12)]
        __: u16,
        #[bits(11)]
        pub dbx: u16,
        #[bits(11)]
        pub dby: u16,
        #[bits(10)]
        __: u16,
    }

    impl DISPFB2 {
        /// Set Rectangle Display Area for Circuit 2 Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(DISPFB2_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Rectangle Display Area for Circuit 1 Register Settings
    pub struct DISPLAY2 {
        #[bits(12)]
        pub dx: u16,
        #[bits(11)]
        pub dy: u16,
        #[bits(4)]
        pub magh: u16,
        #[bits(2)]
        pub magv: u16,
        #[bits(12)]
        pub dw: u16,
        #[bits(11)]
        pub dh: u16,
        #[bits(12)]
        __: u16,
    }

    impl DISPLAY2 {
        /// Set Rectangle Display Area for Circuit 2 Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(DISPLAY2_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Feedback Write Settings
    pub struct EXTBUF {
        #[bits(14)]
        pub exbp: u16,
        #[bits(6)]
        pub exbw: u8,
        #[bits(2)]
        pub fbin: u8,
        #[bits(1)]
        pub wwfmd: u8,
        #[bits(2)]
        pub emoda: u8,
        #[bits(2)]
        pub emodc: u8,
        #[bits(5)]
        __: u8,
        #[bits(11)]
        pub wdx: u16,
        #[bits(11)]
        pub wdy: u16,
        #[bits(10)]
        __: u16,
    }

    impl EXTBUF {
        /// Set the Feedback Write Buffer Setting Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(EXTBUF_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Feedback Write Settings
    pub struct EXTDATA {
        #[bits(12)]
        pub sx: u16,
        #[bits(11)]
        pub sy: u16,
        #[bits(4)]
        pub smph: u16,
        #[bits(2)]
        pub smpv: u16,
        #[bits(3)]
        __: u16,
        #[bits(12)]
        pub ww: u16,
        #[bits(11)]
        pub wh: u16,
        #[bits(9)]
        __: u16,
    }

    impl EXTDATA {
        /// Set the Feedback Write Buffer Setting Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(EXTDATA_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Feedback Write Function Control
    pub struct EXTWRITE {
        pub write: bool,
        #[bits(63)]
        __: u64,
    }

    impl EXTWRITE {
        /// Set the Feedback Write Control Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(EXTWRITE_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Background Color Settings
    pub struct BGCOLOR {
        #[bits(8)]
        pub r: u8,
        #[bits(8)]
        pub g: u8,
        #[bits(8)]
        pub b: u8,
        #[bits(40)]
        __: u64,
    }

    impl BGCOLOR {
        /// Set the Feedback Write Control Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(BGCOLOR_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// GS System Status
    pub struct CSR {
        #[bits(1)]
        pub signal: u8,
        #[bits(1)]
        pub finish: u8,
        #[bits(1)]
        pub hsint: u8,
        #[bits(1)]
        pub vsint: u8,
        #[bits(1)]
        pub edwint: u8,
        #[bits(3)]
        __: u8,
        #[bits(1)]
        pub flush: u8,
        #[bits(1)]
        pub reset: u8,
        #[bits(2)]
        __: u8,
        #[bits(1)]
        pub nfield: u8,
        #[bits(1)]
        pub field: u8,
        #[bits(2)]
        pub fifo: u8,
        #[bits(8)]
        pub rev: u8,
        #[bits(8)]
        pub id: u8,
        #[bits(32)]
        __: u32,
    }

    impl CSR {
        /// Set GS Status Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(CSR_ADDR, self.into()) };
        }

        /// Read GS Status Register
        pub fn load() -> Self {
            let csr = unsafe { ptr::read_volatile(CSR_ADDR) };
            return CSR::from(csr);
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Interrupt Mask Control
    pub struct IMR {
        #[bits(8)]
        __: u8,
        #[bits(1)]
        pub sigmsk: u8,
        #[bits(1)]
        pub finmsk: u8,
        #[bits(1)]
        pub hsmsk: u8,
        #[bits(1)]
        pub vsmsk: u8,
        #[bits(2, default = 0b11)]
        __: u8,
        #[bits(50)]
        __: u64,
    }

    impl IMR {
        /// Set the Interrupt Mask Control Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(IMR_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Host Interface Bus Switching
    pub struct BUSDIR {
        #[bits(1)]
        pub dir: u8,
        #[bits(63)]
        __: u64,
    }

    impl BUSDIR {
        /// Set the Host Interface Bus Direction Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(BUSDIR_ADDR, self.into()) };
        }
    }

    #[bitfield(u64)]
    #[derive(PartialEq, Eq)]
    /// Signal ID Value
    pub struct SIGLBLID {
        pub siglblid: u32,
        pub lblid: u32,
    }

    impl SIGLBLID {
        /// Set the Singal ID Value Register
        pub fn store(self) {
            unsafe { ptr::write_volatile(SIGLBLID_ADDR, self.into()) };
        }

        /// Read the Singal ID Value Register
        pub fn load() -> Self {
            let siglblid = unsafe { ptr::read_volatile(SIGLBLID_ADDR) };
            return SIGLBLID::from(siglblid);
        }
    }
}

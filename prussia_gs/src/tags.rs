#![no_std]
#![deny(missing_docs)]

/// GS General Purpose Registers
#[repr(u64)]
pub enum GP {
    /// Primitive Settings
    PRIM = 0x00,
    /// Vertex Color
    RGBAQ = 0x01,
    /// Texture Coordinates
    ST = 0x02,
    /// Texture Coordinates
    UV = 0x03,
    /// XYZ Coordinates (with fog value)
    XYZF2 = 0x04,
    /// XYZ Coordinates
    XYZ2 = 0x05,
    /// Texture Settings
    TEX0_1 = 0x06,
    /// Texture Settings
    TEX0_2 = 0x07,
    /// Clamping Settings
    CLAMP_1 = 0x08,
    /// Clamping Settings
    CLAMP_2 = 0x09,
    /// Fog Settings
    FOG = 0x0A,
    /// XYZ Coordinates (with fog value) (no vertex kick)
    XYZF3 = 0x0C,
    /// XYZ Coordinates (no vertex kick)
    XYZ3 = 0x0D,
    /// Texture Settings
    TEX1_1 = 0x14,
    /// Texture Settings
    TEX1_2 = 0x15,
    /// Texture Settings
    TEX2_1 = 0x16,
    /// Texture Settings
    TEX2_2 = 0x17,
    /// XY Offset
    XYOFFSET_1 = 0x18,
    /// XY Offset
    XYOFFSET_2 = 0x19,
    /// Primitive Settings Switch (0 = PRMODE, 1 = PRIM)
    PRMODECONT = 0x1A,
    /// Primitive Primitive Settings
    PRMODE = 0x1B,
    /// Texture Color Lookup Table Settings
    TEXCLUT = 0x1C,
    /// Address Mask Settings
    SCANMSK = 0x22,
    /// Mip Mapping Settings
    MIPTBP1_1 = 0x34,
    /// Mip Mapping Settings
    MIPTBP1_2 = 0x35,
    /// Mip Mapping Settings
    MIPTBP2_1 = 0x36,
    /// Mip Mapping Settings
    MIPTBP2_2 = 0x37,
    /// Texture Alpha Settings
    TEXA = 0x3B,
    /// Fog Color
    FOGCOL = 0x3D,
    /// Texture Flush
    TEXFLUSH = 0x3F,
    /// Scissor Settings
    SCISSOR_1 = 0x40,
    /// Scissor Settings
    SCISSOR_2 = 0x41,
    /// Alpha Settings
    ALPHA_1 = 0x42,
    /// Alpha Settings
    ALPHA_2 = 0x43,
    /// Dithering Settings
    DIMX = 0x44,
    /// Dithering Settings
    DTHE = 0x45,
    /// Color Clamping Settings
    COLCLAMP = 0x46,
    /// Alpha / Z-Test Settings
    TEST_1 = 0x47,
    /// Alpha / Z-Test Settings
    TEST_2 = 0x48,
    /// Primitive Alpha Enable
    PABE = 0x49,
    /// Frame Buffer Alpha Setting
    FBA_1 = 0x4A,
    /// Frame Buffer Alpha Setting
    FBA_2 = 0x4B,
    /// Frame Buffer Settings
    FRAME_1 = 0x4C,
    /// Frame Buffer Settings
    FRAME_2 = 0x4D,
    /// Z-Buffer Settings
    ZBUF_1 = 0x4E,
    /// Z-Buffer Settings
    ZBUF_2 = 0x4F,
    /// Transmission Settings
    BITBLTBUF = 0x50,
    /// Transmission Settings
    TRXPOS = 0x51,
    /// Transmission Settings
    TRXREG = 0x52,
    /// Transmission Settings
    TRXDIR = 0x53,
    /// Data port
    HWREG = 0x54,
    /// Signal Event
    SIGNAL = 0x60,
    /// Finish Event
    FINISH = 0x61,
    /// Label Event
    LABEL = 0x62,
    /// NOP
    NOP = 0x7F,
}

/// Pack two 64-bit values into a 128-bit value.
pub fn pack(d0: u64, d1: u64) -> u128 {
    (d0 as u128) | ((d1 as u128) << 64)
}

/// Pack two 64-bit values into a 128-bit value.
pub fn packreg(d0: GP, d1: u64) -> u128 {
    (d1 as u128) | ((d0 as u128) << 64)
}

/// Generate a GIF tag.
pub fn gif_set_tag(nloop: u64, eop: u64, pre: u64, prim: u64, flg: u64, nreg: u64) -> u64 {
    ((nloop & 0x00007FFF) << 0)
        | ((eop & 0x00000001) << 15)
        | ((pre & 0x00000001) << 46)
        | ((prim & 0x000007FF) << 47)
        | ((flg & 0x00000003) << 58)
        | ((nreg & 0x0000000F) << 60)
}

/// Generate a PRIM register value.
pub fn gif_set_prim(
    prm: u64,
    iip: u64,
    tme: u64,
    fge: u64,
    abe: u64,
    aa1: u64,
    fst: u64,
    ctxt: u64,
    fix: u64,
) -> u64 {
    ((prm & 0x00000007) << 0)
        | ((iip & 0x00000001) << 3)
        | ((tme & 0x00000001) << 4)
        | ((fge & 0x00000001) << 5)
        | ((abe & 0x00000001) << 6)
        | ((aa1 & 0x00000001) << 7)
        | ((fst & 0x00000001) << 8)
        | ((ctxt & 0x00000001) << 9)
        | ((fix & 0x00000001) << 10)
}

/// Generate a RGBAQ register value.
pub fn gif_set_rgbaq(r: u64, g: u64, b: u64, a: u64, q: u64) -> u64 {
    ((r & 0x000000FF) << 0)
        | ((g & 0x000000FF) << 8)
        | ((b & 0x000000FF) << 16)
        | ((a & 0x000000FF) << 24)
        | ((q & 0xFFFFFFFF) << 32)
}

/// Generate an ST register value.
pub fn gif_set_st(s: u64, t: u64) -> u64 {
    ((s & 0xFFFFFFFF) << 0) | ((t & 0xFFFFFFFF) << 32)
}

/// Generate a UV register value.
pub fn gif_set_uv(u: u64, v: u64) -> u64 {
    ((u & 0x00003FFF) << 0) | ((v & 0x00003FFF) << 16)
}

/// Generate an XYZ2 register value.
pub fn gif_set_xyz(x: u64, y: u64, z: u64) -> u64 {
    ((x & 0x0000FFFF) << 0) | ((y & 0x0000FFFF) << 16) | ((z & 0xFFFFFFFF) << 32)
}

/// Generate an XYZF register value.
pub fn gif_set_xyzf(x: u64, y: u64, z: u64, f: u64) -> u64 {
    ((x & 0x0000FFFF) << 0)
        | ((y & 0x0000FFFF) << 16)
        | ((z & 0x00FFFFFF) << 32)
        | ((f & 0x000000FF) << 56)
}

/// Generate a TEX0 register value.
pub fn gif_set_tex0(
    tba: u64,
    tbw: u64,
    psm: u64,
    tw: u64,
    th: u64,
    tcc: u64,
    tfnct: u64,
    cba: u64,
    cpsm: u64,
    csm: u64,
    csa: u64,
    cld: u64,
) -> u64 {
    ((tba & 0x00003FFF) << 0)
        | ((tbw & 0x0000003F) << 14)
        | ((psm & 0x0000003F) << 20)
        | ((tw & 0x0000000F) << 26)
        | ((th & 0x0000000F) << 30)
        | ((tcc & 0x00000001) << 34)
        | ((tfnct & 0x00000003) << 35)
        | ((cba & 0x00003FFF) << 37)
        | ((cpsm & 0x0000000F) << 51)
        | ((csm & 0x00000001) << 55)
        | ((csa & 0x0000001F) << 56)
        | ((cld & 0x00000007) << 61)
}

/// Generate a CLAMP register value.
pub fn gif_set_clamp(wms: u64, wmt: u64, minu: u64, maxu: u64, minv: u64, maxv: u64) -> u64 {
    ((wms & 0x00000003) << 0)
        | ((wmt & 0x00000003) << 2)
        | ((minu & 0x000003FF) << 4)
        | ((maxu & 0x000003FF) << 14)
        | ((minv & 0x000003FF) << 24)
        | ((maxv & 0x000003FF) << 34)
}

/// Generate a FOG register value.
pub fn gif_set_fog(fog: u64) -> u64 {
    (fog & 0x000000FF) << 56
}

/// Generate an ALPHA register value.
pub fn gs_set_alpha(a: u64, b: u64, c: u64, d: u64, alpha: u64) -> u64 {
    ((a & 0x00000003) << 0)
        | ((b & 0x00000003) << 2)
        | ((c & 0x00000003) << 4)
        | ((d & 0x00000003) << 6)
        | ((alpha & 0x000000FF) << 32)
}

/// Generate a BITBLTBUF register value.
pub fn gs_set_bitbltbuf(sba: u64, sbw: u64, spsm: u64, dba: u64, dbw: u64, dpsm: u64) -> u64 {
    ((sba & 0x00003FFF) << 0)
        | ((sbw & 0x0000003F) << 16)
        | ((spsm & 0x0000003F) << 24)
        | ((dba & 0x00003FFF) << 32)
        | ((dbw & 0x0000003F) << 48)
        | ((dpsm & 0x0000003F) << 56)
}

/// Generate a CLAMP register value.
pub fn gs_set_clamp(wms: u64, wmt: u64, minu: u64, maxu: u64, minv: u64, maxv: u64) -> u64 {
    ((wms & 0x00000003) << 0)
        | ((wmt & 0x00000003) << 2)
        | ((minu & 0x000003FF) << 4)
        | ((maxu & 0x000003FF) << 14)
        | ((minv & 0x000003FF) << 24)
        | ((maxv & 0x000003FF) << 34)
}

/// Generate a COLCLAMP register value.
pub fn gs_set_colclamp(clamp: u64) -> u64 {
    clamp & 0x00000001
}

/// Generate a DIMX register value.
pub fn gs_set_dimx(
    d00: u64,
    d01: u64,
    d02: u64,
    d03: u64,
    d10: u64,
    d11: u64,
    d12: u64,
    d13: u64,
    d20: u64,
    d21: u64,
    d22: u64,
    d23: u64,
    d30: u64,
    d31: u64,
    d32: u64,
    d33: u64,
) -> u64 {
    ((d00 & 0x00000003) << 0)
        | ((d01 & 0x00000003) << 4)
        | ((d02 & 0x00000003) << 8)
        | ((d03 & 0x00000003) << 12)
        | ((d10 & 0x00000003) << 16)
        | ((d11 & 0x00000003) << 20)
        | ((d12 & 0x00000003) << 24)
        | ((d13 & 0x00000003) << 28)
        | ((d20 & 0x00000003) << 32)
        | ((d21 & 0x00000003) << 36)
        | ((d22 & 0x00000003) << 40)
        | ((d23 & 0x00000003) << 44)
        | ((d30 & 0x00000003) << 48)
        | ((d31 & 0x00000003) << 52)
        | ((d32 & 0x00000003) << 56)
        | ((d33 & 0x00000003) << 60)
}

/// Generate a DTHE register value.
pub fn gs_set_dthe(enable: u64) -> u64 {
    enable & 0x00000001
}

/// Generate a FBA register value.
pub fn gs_set_fba(alpha: u64) -> u64 {
    alpha & 0x00000001
}

/// Generate a FINISH register value.
pub fn gs_set_finish(a: u64) -> u64 {
    a & 0xFFFFFFFF
}

/// Generate a FOG register value.
pub fn gs_set_fog(fog: u64) -> u64 {
    (fog & 0x000000FF) << 56
}

/// Generate a FOGCOL register value.
pub fn gs_set_fogcol(r: u64, g: u64, b: u64) -> u64 {
    ((r & 0x000000FF) << 0) | ((g & 0x000000FF) << 8) | ((b & 0x000000FF) << 16)
}

/// Generate a FRAME register value.
pub fn gs_set_frame(fba: u64, fbw: u64, psm: u64, fmsk: u64) -> u64 {
    ((fba & 0x000001FF) << 0)
        | ((fbw & 0x0000003F) << 16)
        | ((psm & 0x0000003F) << 24)
        | ((fmsk & 0xFFFFFFFF) << 32)
}

/// Generate a FRAME register value (for CT16).
pub fn gs_set_fmsk16(r: u32, g: u32, b: u32, a: u32) -> u32 {
    ((r & 0x0000001F) << 3)
        | ((g & 0x0000001F) << 11)
        | ((b & 0x0000001F) << 19)
        | ((a & 0x00000001) << 31)
}

/// Generate a HWREG register value.
pub fn gs_set_hwreg(a: u64) -> u64 {
    a & 0xFFFFFFFFFFFFFFFF
}

/// Generate a LABEL register value.
pub fn gs_set_label(id: u64, msk: u64) -> u64 {
    ((id & 0xFFFFFFFF) << 0) | ((msk & 0xFFFFFFFF) << 32)
}

/// Generate a MIPTBP1 register value.
pub fn gs_set_miptbp1(tba1: u64, tbw1: u64, tba2: u64, tbw2: u64, tba3: u64, tbw3: u64) -> u64 {
    ((tba1 & 0x00003FFF) << 0)
        | ((tbw1 & 0x0000003F) << 14)
        | ((tba2 & 0x00003FFF) << 20)
        | ((tbw2 & 0x0000003F) << 34)
        | ((tba3 & 0x00003FFF) << 40)
        | ((tbw3 & 0x0000003F) << 54)
}

/// Generate a MIPTBP2 register value.
pub fn gs_set_miptbp2(tba4: u64, tbw4: u64, tba5: u64, tbw5: u64, tba6: u64, tbw6: u64) -> u64 {
    ((tba4 & 0x00003FFF) << 0)
        | ((tbw4 & 0x0000003F) << 14)
        | ((tba5 & 0x00003FFF) << 20)
        | ((tbw5 & 0x0000003F) << 34)
        | ((tba6 & 0x00003FFF) << 40)
        | ((tbw6 & 0x0000003F) << 54)
}

/// Generate a NOP register value.
pub fn gs_set_nop(a: u64) -> u64 {
    a & 0xFFFFFFFF
}

/// Generate a PABE register value.
pub fn gs_set_pabe(enable: u64) -> u64 {
    enable & 0x00000001
}

/// Generate a PRIM register value.
pub fn gs_set_prim(
    prim: u64,
    iip: u64,
    tme: u64,
    fge: u64,
    abe: u64,
    aa1: u64,
    fst: u64,
    ctxt: u64,
    fix: u64,
) -> u64 {
    ((prim & 0x00000007) << 0)
        | ((iip & 0x00000001) << 3)
        | ((tme & 0x00000001) << 4)
        | ((fge & 0x00000001) << 5)
        | ((abe & 0x00000001) << 6)
        | ((aa1 & 0x00000001) << 7)
        | ((fst & 0x00000001) << 8)
        | ((ctxt & 0x00000001) << 9)
        | ((fix & 0x00000001) << 10)
}

/// Generate a PRMODE register value.
pub fn gs_set_prmode(
    iip: u64,
    tme: u64,
    fge: u64,
    abe: u64,
    aa1: u64,
    fst: u64,
    ctxt: u64,
    fix: u64,
) -> u64 {
    ((iip & 0x00000001) << 3)
        | ((tme & 0x00000001) << 4)
        | ((fge & 0x00000001) << 5)
        | ((abe & 0x00000001) << 6)
        | ((aa1 & 0x00000001) << 7)
        | ((fst & 0x00000001) << 8)
        | ((ctxt & 0x00000001) << 9)
        | ((fix & 0x00000001) << 10)
}

/// Generate a PRMODECONT register value.
pub fn gs_set_prmodecont(ctrl: u64) -> u64 {
    ctrl & 0x00000001
}

/// Generate a RGBAQ register value.
pub fn gs_set_rgbaq(r: u8, g: u8, b: u8, a: u8, q: f32) -> u64 {
    ((r as u64 & 0x000000FF) << 0)
        | ((g as u64 & 0x000000FF) << 8)
        | ((b as u64 & 0x000000FF) << 16)
        | ((a as u64 & 0x000000FF) << 24)
        | ((q.to_bits() as u64 & 0xFFFFFFFF) << 32)
}

/// Generate a SCANMSK register value.
pub fn gs_set_scanmsk(msk: u64) -> u64 {
    msk & 0x00000003
}

/// Generate a SCISSOR register value.
pub fn gs_set_scissor(x0: u64, x1: u64, y0: u64, y1: u64) -> u64 {
    ((x0 & 0x000007FF) << 0)
        | ((x1 & 0x000007FF) << 16)
        | ((y0 & 0x000007FF) << 32)
        | ((y1 & 0x000007FF) << 48)
}

/// Generate a SIGNAL register value.
pub fn gs_set_signal(id: u64, msk: u64) -> u64 {
    ((id & 0xFFFFFFFF) << 0) | ((msk & 0xFFFFFFFF) << 32)
}

/// Generate a ST register value.
pub fn gs_set_st(s: u64, t: u64) -> u64 {
    ((s & 0xFFFFFFFF) << 0) | ((t & 0xFFFFFFFF) << 32)
}

/// Generate a TEST register value.
pub fn gs_set_test(
    aten: u64,
    atmeth: u64,
    atref: u64,
    atfail: u64,
    daten: u64,
    datmd: u64,
    zten: u64,
    ztmeth: u64,
) -> u64 {
    ((aten & 0x00000001) << 0)
        | ((atmeth & 0x00000007) << 1)
        | ((atref & 0x000000FF) << 4)
        | ((atfail & 0x00000003) << 12)
        | ((daten & 0x00000001) << 14)
        | ((datmd & 0x00000001) << 15)
        | ((zten & 0x00000001) << 16)
        | ((ztmeth & 0x00000003) << 17)
}

/// Generate a TEX0_SMALL register value.
pub fn gs_set_tex0_small(
    tba: u64,
    tbw: u64,
    psm: u64,
    tw: u64,
    th: u64,
    tcc: u64,
    tfnct: u64,
) -> u64 {
    ((tba & 0x00003FFF) << 0)
        | ((tbw & 0x0000003F) << 14)
        | ((psm & 0x0000003F) << 20)
        | ((tw & 0x0000000F) << 26)
        | ((th & 0x0000000F) << 30)
        | ((tcc & 0x00000001) << 34)
        | ((tfnct & 0x00000003) << 35)
}

/// Generate a TEX0 register value.
pub fn gs_set_tex0(
    tba: u64,
    tbw: u64,
    psm: u64,
    tw: u64,
    th: u64,
    tcc: u64,
    tfnct: u64,
    cba: u64,
    cpsm: u64,
    csm: u64,
    csa: u64,
    cld: u64,
) -> u64 {
    ((tba & 0x00003FFF) << 0)
        | ((tbw & 0x0000003F) << 14)
        | ((psm & 0x0000003F) << 20)
        | ((tw & 0x0000000F) << 26)
        | ((th & 0x0000000F) << 30)
        | ((tcc & 0x00000001) << 34)
        | ((tfnct & 0x00000003) << 35)
        | ((cba & 0x00003FFF) << 37)
        | ((cpsm & 0x0000000F) << 51)
        | ((csm & 0x00000001) << 55)
        | ((csa & 0x0000001F) << 56)
        | ((cld & 0x00000007) << 61)
}

/// Generate a TEX1 register value.
pub fn gs_set_tex1(lcm: u64, mxl: u64, mmag: u64, mmin: u64, mtba: u64, l: u64, k: u64) -> u64 {
    ((lcm & 0x00000001) << 0)
        | ((mxl & 0x00000007) << 2)
        | ((mmag & 0x00000001) << 5)
        | ((mmin & 0x00000007) << 6)
        | ((mtba & 0x00000001) << 9)
        | ((l & 0x00000003) << 19)
        | ((k & 0x00000FFF) << 32)
}

/// Generate a TEX2 register value.
pub fn gs_set_tex2(psm: u64, cba: u64, cpsm: u64, csm: u64, csa: u64, cld: u64) -> u64 {
    ((psm & 0x0000003F) << 20)
        | ((cba & 0x00003FFF) << 37)
        | ((cpsm & 0x0000000F) << 51)
        | ((csm & 0x00000001) << 55)
        | ((csa & 0x0000001F) << 56)
        | ((cld & 0x00000007) << 61)
}

/// Generate a TEXA register value.
pub fn gs_set_texa(a0: u64, am: u64, a1: u64) -> u64 {
    ((a0 & 0x000000FF) << 0) | ((am & 0x00000001) << 15) | ((a1 & 0x000000FF) << 32)
}

/// Generate a TEXCLUT register value.
pub fn gs_set_texclut(cbw: u64, cu: u64, cv: u64) -> u64 {
    ((cbw & 0x0000003F) << 0) | ((cu & 0x0000003F) << 6) | ((cv & 0x00000FFF) << 12)
}

/// Generate a TRXDIR register value.
pub fn gs_set_trxdir(dir: u64) -> u64 {
    dir & 0x00000003
}

/// Generate a TEXFLUSH register value.
pub fn gs_set_texflush(a: u64) -> u64 {
    a & 0xFFFFFFFF
}

/// Generate a TRXPOS register value.
pub fn gs_set_trxpos(sx: u64, sy: u64, dx: u64, dy: u64, dir: u64) -> u64 {
    ((sx & 0x000007FF) << 0)
        | ((sy & 0x000007FF) << 16)
        | ((dx & 0x000007FF) << 32)
        | ((dy & 0x000007FF) << 48)
        | ((dir & 0x00000003) << 59)
}

/// Generate a TRXREG register value.
pub fn gs_set_trxreg(w: u64, h: u64) -> u64 {
    ((w & 0x00000FFF) << 0) | ((h & 0x00000FFF) << 32)
}

/// Generate a UV register value.
pub fn gs_set_uv(u: u64, v: u64) -> u64 {
    ((u & 0x00003FFF) << 0) | ((v & 0x00003FFF) << 16)
}

/// Generate a XYOFFSET register value.
pub fn gs_set_xyoffset(x: u64, y: u64) -> u64 {
    ((x & 0x0000FFFF) << 0) | ((y & 0x0000FFFF) << 32)
}

/// Generate a XYZ register value.
pub fn gs_set_xyz(x: u32, y: u32, z: u32) -> u64 {
    ((x as u64 & 0x0000FFFF) << 0) | ((y as u64 & 0x0000FFFF) << 16) | ((z as u64 & 0xFFFFFFFF) << 32)
}

/// Generate a XYZF register value.
pub fn gs_set_xyzf(x: u64, y: u64, z: u64, f: u64) -> u64 {
    ((x & 0x0000FFFF) << 0)
        | ((y & 0x0000FFFF) << 16)
        | ((z & 0x00FFFFFF) << 32)
        | ((f & 0x000000FF) << 56)
}

/// Generate a ZBUF register value.
pub fn gs_set_zbuf(zba: u64, zsm: u64, zmsk: u64) -> u64 {
    ((zba & 0x000001FF) << 0) | ((zsm & 0x0000000F) << 24) | ((zmsk & 0x00000001) << 32)
}

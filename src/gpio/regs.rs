#[doc = "Alternate Pad Configuration reg0 (Pads 3,2,1,0)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfga(pub u32);
impl Altpadcfga {
    #[doc = "Pad 0 high order drive strength selection. Used in conjunction with PAD0STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad0_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 0 high order drive strength selection. Used in conjunction with PAD0STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad0_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 0 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad0_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 0 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad0_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 1 high order drive strength selection. Used in conjunction with PAD1STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad1_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 1 high order drive strength selection. Used in conjunction with PAD1STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad1_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 1 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad1_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 1 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad1_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 2 high order drive strength selection. Used in conjunction with PAD2STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad2_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 2 high order drive strength selection. Used in conjunction with PAD2STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad2_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 2 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad2_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 2 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad2_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 3 high order drive strength selection. Used in conjunction with PAD3STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad3_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 3 high order drive strength selection. Used in conjunction with PAD3STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad3_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 3 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad3_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 3 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad3_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfga {
    #[inline(always)]
    fn default() -> Altpadcfga {
        Altpadcfga(0)
    }
}
impl core::fmt::Debug for Altpadcfga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfga")
            .field("pad0_ds1", &self.pad0_ds1())
            .field("pad0_sr", &self.pad0_sr())
            .field("pad1_ds1", &self.pad1_ds1())
            .field("pad1_sr", &self.pad1_sr())
            .field("pad2_ds1", &self.pad2_ds1())
            .field("pad2_sr", &self.pad2_sr())
            .field("pad3_ds1", &self.pad3_ds1())
            .field("pad3_sr", &self.pad3_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfga {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfga {{ pad0_ds1: {=bool:?}, pad0_sr: {=bool:?}, pad1_ds1: {=bool:?}, pad1_sr: {=bool:?}, pad2_ds1: {=bool:?}, pad2_sr: {=bool:?}, pad3_ds1: {=bool:?}, pad3_sr: {=bool:?} }}" , self . pad0_ds1 () , self . pad0_sr () , self . pad1_ds1 () , self . pad1_sr () , self . pad2_ds1 () , self . pad2_sr () , self . pad3_ds1 () , self . pad3_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg1 (Pads 7,6,5,4)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgb(pub u32);
impl Altpadcfgb {
    #[doc = "Pad 4 high order drive strength selection. Used in conjunction with PAD4STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad4_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 4 high order drive strength selection. Used in conjunction with PAD4STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad4_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 4 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad4_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 4 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad4_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 5 high order drive strength selection. Used in conjunction with PAD5STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad5_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 5 high order drive strength selection. Used in conjunction with PAD5STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad5_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 5 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad5_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 5 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad5_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 6 high order drive strength selection. Used in conjunction with PAD6STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad6_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 6 high order drive strength selection. Used in conjunction with PAD6STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad6_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 6 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad6_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 6 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad6_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 7 high order drive strength selection. Used in conjunction with PAD7STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad7_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 7 high order drive strength selection. Used in conjunction with PAD7STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad7_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 7 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad7_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 7 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad7_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfgb {
    #[inline(always)]
    fn default() -> Altpadcfgb {
        Altpadcfgb(0)
    }
}
impl core::fmt::Debug for Altpadcfgb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgb")
            .field("pad4_ds1", &self.pad4_ds1())
            .field("pad4_sr", &self.pad4_sr())
            .field("pad5_ds1", &self.pad5_ds1())
            .field("pad5_sr", &self.pad5_sr())
            .field("pad6_ds1", &self.pad6_ds1())
            .field("pad6_sr", &self.pad6_sr())
            .field("pad7_ds1", &self.pad7_ds1())
            .field("pad7_sr", &self.pad7_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgb {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgb {{ pad4_ds1: {=bool:?}, pad4_sr: {=bool:?}, pad5_ds1: {=bool:?}, pad5_sr: {=bool:?}, pad6_ds1: {=bool:?}, pad6_sr: {=bool:?}, pad7_ds1: {=bool:?}, pad7_sr: {=bool:?} }}" , self . pad4_ds1 () , self . pad4_sr () , self . pad5_ds1 () , self . pad5_sr () , self . pad6_ds1 () , self . pad6_sr () , self . pad7_ds1 () , self . pad7_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg2 (Pads 11,10,9,8)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgc(pub u32);
impl Altpadcfgc {
    #[doc = "Pad 8 high order drive strength selection. Used in conjunction with PAD8STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad8_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 8 high order drive strength selection. Used in conjunction with PAD8STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad8_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 8 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad8_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 8 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad8_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 9 high order drive strength selection. Used in conjunction with PAD9STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad9_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 9 high order drive strength selection. Used in conjunction with PAD9STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad9_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 9 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad9_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 9 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad9_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 10 high order drive strength selection. Used in conjunction with PAD10STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad10_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 10 high order drive strength selection. Used in conjunction with PAD10STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad10_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 10 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad10_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 10 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad10_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 11 high order drive strength selection. Used in conjunction with PAD11STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad11_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 11 high order drive strength selection. Used in conjunction with PAD11STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad11_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 11 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad11_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 11 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad11_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfgc {
    #[inline(always)]
    fn default() -> Altpadcfgc {
        Altpadcfgc(0)
    }
}
impl core::fmt::Debug for Altpadcfgc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgc")
            .field("pad8_ds1", &self.pad8_ds1())
            .field("pad8_sr", &self.pad8_sr())
            .field("pad9_ds1", &self.pad9_ds1())
            .field("pad9_sr", &self.pad9_sr())
            .field("pad10_ds1", &self.pad10_ds1())
            .field("pad10_sr", &self.pad10_sr())
            .field("pad11_ds1", &self.pad11_ds1())
            .field("pad11_sr", &self.pad11_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgc {{ pad8_ds1: {=bool:?}, pad8_sr: {=bool:?}, pad9_ds1: {=bool:?}, pad9_sr: {=bool:?}, pad10_ds1: {=bool:?}, pad10_sr: {=bool:?}, pad11_ds1: {=bool:?}, pad11_sr: {=bool:?} }}" , self . pad8_ds1 () , self . pad8_sr () , self . pad9_ds1 () , self . pad9_sr () , self . pad10_ds1 () , self . pad10_sr () , self . pad11_ds1 () , self . pad11_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg3 (Pads 15,14,13,12)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgd(pub u32);
impl Altpadcfgd {
    #[doc = "Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad12_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad12_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 12 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad12_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 12 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad12_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad13_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad13_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 13 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad13_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 13 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad13_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad14_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad14_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 14 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad14_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 14 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad14_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad15_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad15_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 15 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad15_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 15 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad15_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfgd {
    #[inline(always)]
    fn default() -> Altpadcfgd {
        Altpadcfgd(0)
    }
}
impl core::fmt::Debug for Altpadcfgd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgd")
            .field("pad12_ds1", &self.pad12_ds1())
            .field("pad12_sr", &self.pad12_sr())
            .field("pad13_ds1", &self.pad13_ds1())
            .field("pad13_sr", &self.pad13_sr())
            .field("pad14_ds1", &self.pad14_ds1())
            .field("pad14_sr", &self.pad14_sr())
            .field("pad15_ds1", &self.pad15_ds1())
            .field("pad15_sr", &self.pad15_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgd {{ pad12_ds1: {=bool:?}, pad12_sr: {=bool:?}, pad13_ds1: {=bool:?}, pad13_sr: {=bool:?}, pad14_ds1: {=bool:?}, pad14_sr: {=bool:?}, pad15_ds1: {=bool:?}, pad15_sr: {=bool:?} }}" , self . pad12_ds1 () , self . pad12_sr () , self . pad13_ds1 () , self . pad13_sr () , self . pad14_ds1 () , self . pad14_sr () , self . pad15_ds1 () , self . pad15_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg4 (Pads 19,18,17,16)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfge(pub u32);
impl Altpadcfge {
    #[doc = "Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad16_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad16_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 16 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad16_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 16 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad16_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad17_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad17_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 17 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad17_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 17 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad17_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad18_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad18_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 18 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad18_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 18 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad18_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad19_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad19_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 19 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad19_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 19 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad19_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfge {
    #[inline(always)]
    fn default() -> Altpadcfge {
        Altpadcfge(0)
    }
}
impl core::fmt::Debug for Altpadcfge {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfge")
            .field("pad16_ds1", &self.pad16_ds1())
            .field("pad16_sr", &self.pad16_sr())
            .field("pad17_ds1", &self.pad17_ds1())
            .field("pad17_sr", &self.pad17_sr())
            .field("pad18_ds1", &self.pad18_ds1())
            .field("pad18_sr", &self.pad18_sr())
            .field("pad19_ds1", &self.pad19_ds1())
            .field("pad19_sr", &self.pad19_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfge {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfge {{ pad16_ds1: {=bool:?}, pad16_sr: {=bool:?}, pad17_ds1: {=bool:?}, pad17_sr: {=bool:?}, pad18_ds1: {=bool:?}, pad18_sr: {=bool:?}, pad19_ds1: {=bool:?}, pad19_sr: {=bool:?} }}" , self . pad16_ds1 () , self . pad16_sr () , self . pad17_ds1 () , self . pad17_sr () , self . pad18_ds1 () , self . pad18_sr () , self . pad19_ds1 () , self . pad19_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg5 (Pads 23,22,21,20)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgf(pub u32);
impl Altpadcfgf {
    #[doc = "Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad20_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad20_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 20 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad20_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 20 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad20_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad21_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad21_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 21 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad21_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 21 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad21_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad22_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad22_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 22 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad22_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 22 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad22_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad23_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad23_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 23 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad23_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 23 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad23_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfgf {
    #[inline(always)]
    fn default() -> Altpadcfgf {
        Altpadcfgf(0)
    }
}
impl core::fmt::Debug for Altpadcfgf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgf")
            .field("pad20_ds1", &self.pad20_ds1())
            .field("pad20_sr", &self.pad20_sr())
            .field("pad21_ds1", &self.pad21_ds1())
            .field("pad21_sr", &self.pad21_sr())
            .field("pad22_ds1", &self.pad22_ds1())
            .field("pad22_sr", &self.pad22_sr())
            .field("pad23_ds1", &self.pad23_ds1())
            .field("pad23_sr", &self.pad23_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgf {{ pad20_ds1: {=bool:?}, pad20_sr: {=bool:?}, pad21_ds1: {=bool:?}, pad21_sr: {=bool:?}, pad22_ds1: {=bool:?}, pad22_sr: {=bool:?}, pad23_ds1: {=bool:?}, pad23_sr: {=bool:?} }}" , self . pad20_ds1 () , self . pad20_sr () , self . pad21_ds1 () , self . pad21_sr () , self . pad22_ds1 () , self . pad22_sr () , self . pad23_ds1 () , self . pad23_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg6 (Pads 27,26,25,24)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgg(pub u32);
impl Altpadcfgg {
    #[doc = "Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad24_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad24_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 24 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad24_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 24 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad24_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad25_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad25_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 25 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad25_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 25 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad25_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad26_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad26_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 26 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad26_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 26 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad26_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad27_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad27_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 27 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad27_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 27 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad27_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfgg {
    #[inline(always)]
    fn default() -> Altpadcfgg {
        Altpadcfgg(0)
    }
}
impl core::fmt::Debug for Altpadcfgg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgg")
            .field("pad24_ds1", &self.pad24_ds1())
            .field("pad24_sr", &self.pad24_sr())
            .field("pad25_ds1", &self.pad25_ds1())
            .field("pad25_sr", &self.pad25_sr())
            .field("pad26_ds1", &self.pad26_ds1())
            .field("pad26_sr", &self.pad26_sr())
            .field("pad27_ds1", &self.pad27_ds1())
            .field("pad27_sr", &self.pad27_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgg {{ pad24_ds1: {=bool:?}, pad24_sr: {=bool:?}, pad25_ds1: {=bool:?}, pad25_sr: {=bool:?}, pad26_ds1: {=bool:?}, pad26_sr: {=bool:?}, pad27_ds1: {=bool:?}, pad27_sr: {=bool:?} }}" , self . pad24_ds1 () , self . pad24_sr () , self . pad25_ds1 () , self . pad25_sr () , self . pad26_ds1 () , self . pad26_sr () , self . pad27_ds1 () , self . pad27_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg7 (Pads 31,30,29,28)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgh(pub u32);
impl Altpadcfgh {
    #[doc = "Pad 28 high order drive strength selection. Used in conjunction with PAD28STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad28_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 28 high order drive strength selection. Used in conjunction with PAD28STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad28_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 28 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad28_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 28 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad28_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 29 high order drive strength selection. Used in conjunction with PAD29STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad29_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 29 high order drive strength selection. Used in conjunction with PAD29STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad29_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 29 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad29_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 29 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad29_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 30 high order drive strength selection. Used in conjunction with PAD30STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad30_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 30 high order drive strength selection. Used in conjunction with PAD30STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad30_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 30 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad30_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 30 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad30_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 31 high order drive strength selection. Used in conjunction with PAD31STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad31_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 31 high order drive strength selection. Used in conjunction with PAD31STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad31_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 31 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad31_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 31 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad31_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfgh {
    #[inline(always)]
    fn default() -> Altpadcfgh {
        Altpadcfgh(0)
    }
}
impl core::fmt::Debug for Altpadcfgh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgh")
            .field("pad28_ds1", &self.pad28_ds1())
            .field("pad28_sr", &self.pad28_sr())
            .field("pad29_ds1", &self.pad29_ds1())
            .field("pad29_sr", &self.pad29_sr())
            .field("pad30_ds1", &self.pad30_ds1())
            .field("pad30_sr", &self.pad30_sr())
            .field("pad31_ds1", &self.pad31_ds1())
            .field("pad31_sr", &self.pad31_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgh {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgh {{ pad28_ds1: {=bool:?}, pad28_sr: {=bool:?}, pad29_ds1: {=bool:?}, pad29_sr: {=bool:?}, pad30_ds1: {=bool:?}, pad30_sr: {=bool:?}, pad31_ds1: {=bool:?}, pad31_sr: {=bool:?} }}" , self . pad28_ds1 () , self . pad28_sr () , self . pad29_ds1 () , self . pad29_sr () , self . pad30_ds1 () , self . pad30_sr () , self . pad31_ds1 () , self . pad31_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg8 (Pads 35,34,33,32)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgi(pub u32);
impl Altpadcfgi {
    #[doc = "Pad 32 high order drive strength selection. Used in conjunction with PAD32STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad32_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 32 high order drive strength selection. Used in conjunction with PAD32STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad32_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 32 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad32_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 32 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad32_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 33 high order drive strength selection. Used in conjunction with PAD33STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad33_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 33 high order drive strength selection. Used in conjunction with PAD33STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad33_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 33 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad33_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 33 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad33_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 34 high order drive strength selection. Used in conjunction with PAD34STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad34_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 34 high order drive strength selection. Used in conjunction with PAD34STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad34_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 34 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad34_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 34 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad34_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 35 high order drive strength selection. Used in conjunction with PAD35STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad35_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 35 high order drive strength selection. Used in conjunction with PAD35STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad35_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 35 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad35_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 35 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad35_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfgi {
    #[inline(always)]
    fn default() -> Altpadcfgi {
        Altpadcfgi(0)
    }
}
impl core::fmt::Debug for Altpadcfgi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgi")
            .field("pad32_ds1", &self.pad32_ds1())
            .field("pad32_sr", &self.pad32_sr())
            .field("pad33_ds1", &self.pad33_ds1())
            .field("pad33_sr", &self.pad33_sr())
            .field("pad34_ds1", &self.pad34_ds1())
            .field("pad34_sr", &self.pad34_sr())
            .field("pad35_ds1", &self.pad35_ds1())
            .field("pad35_sr", &self.pad35_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgi {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgi {{ pad32_ds1: {=bool:?}, pad32_sr: {=bool:?}, pad33_ds1: {=bool:?}, pad33_sr: {=bool:?}, pad34_ds1: {=bool:?}, pad34_sr: {=bool:?}, pad35_ds1: {=bool:?}, pad35_sr: {=bool:?} }}" , self . pad32_ds1 () , self . pad32_sr () , self . pad33_ds1 () , self . pad33_sr () , self . pad34_ds1 () , self . pad34_sr () , self . pad35_ds1 () , self . pad35_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg9 (Pads 39,38,37,36)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgj(pub u32);
impl Altpadcfgj {
    #[doc = "Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad36_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad36_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 36 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad36_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 36 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad36_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad37_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad37_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 37 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad37_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 37 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad37_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad38_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad38_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 38 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad38_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 38 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad38_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad39_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad39_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 39 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad39_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 39 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad39_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfgj {
    #[inline(always)]
    fn default() -> Altpadcfgj {
        Altpadcfgj(0)
    }
}
impl core::fmt::Debug for Altpadcfgj {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgj")
            .field("pad36_ds1", &self.pad36_ds1())
            .field("pad36_sr", &self.pad36_sr())
            .field("pad37_ds1", &self.pad37_ds1())
            .field("pad37_sr", &self.pad37_sr())
            .field("pad38_ds1", &self.pad38_ds1())
            .field("pad38_sr", &self.pad38_sr())
            .field("pad39_ds1", &self.pad39_ds1())
            .field("pad39_sr", &self.pad39_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgj {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgj {{ pad36_ds1: {=bool:?}, pad36_sr: {=bool:?}, pad37_ds1: {=bool:?}, pad37_sr: {=bool:?}, pad38_ds1: {=bool:?}, pad38_sr: {=bool:?}, pad39_ds1: {=bool:?}, pad39_sr: {=bool:?} }}" , self . pad36_ds1 () , self . pad36_sr () , self . pad37_ds1 () , self . pad37_sr () , self . pad38_ds1 () , self . pad38_sr () , self . pad39_ds1 () , self . pad39_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg10 (Pads 43,42,41,40)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgk(pub u32);
impl Altpadcfgk {
    #[doc = "Pad 40 high order drive strength selection. Used in conjunction with PAD40STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad40_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 40 high order drive strength selection. Used in conjunction with PAD40STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad40_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 40 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad40_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 40 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad40_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 41 high order drive strength selection. Used in conjunction with PAD41STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad41_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 41 high order drive strength selection. Used in conjunction with PAD41STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad41_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 41 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad41_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 41 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad41_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 42 high order drive strength selection. Used in conjunction with PAD42STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad42_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 42 high order drive strength selection. Used in conjunction with PAD42STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad42_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 42 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad42_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 42 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad42_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 43 high order drive strength selection. Used in conjunction with PAD43STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad43_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 43 high order drive strength selection. Used in conjunction with PAD43STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad43_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 43 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad43_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 43 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad43_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfgk {
    #[inline(always)]
    fn default() -> Altpadcfgk {
        Altpadcfgk(0)
    }
}
impl core::fmt::Debug for Altpadcfgk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgk")
            .field("pad40_ds1", &self.pad40_ds1())
            .field("pad40_sr", &self.pad40_sr())
            .field("pad41_ds1", &self.pad41_ds1())
            .field("pad41_sr", &self.pad41_sr())
            .field("pad42_ds1", &self.pad42_ds1())
            .field("pad42_sr", &self.pad42_sr())
            .field("pad43_ds1", &self.pad43_ds1())
            .field("pad43_sr", &self.pad43_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgk {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgk {{ pad40_ds1: {=bool:?}, pad40_sr: {=bool:?}, pad41_ds1: {=bool:?}, pad41_sr: {=bool:?}, pad42_ds1: {=bool:?}, pad42_sr: {=bool:?}, pad43_ds1: {=bool:?}, pad43_sr: {=bool:?} }}" , self . pad40_ds1 () , self . pad40_sr () , self . pad41_ds1 () , self . pad41_sr () , self . pad42_ds1 () , self . pad42_sr () , self . pad43_ds1 () , self . pad43_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg11 (Pads 47,46,45,44)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgl(pub u32);
impl Altpadcfgl {
    #[doc = "Pad 44 high order drive strength selection. Used in conjunction with PAD44STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad44_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 44 high order drive strength selection. Used in conjunction with PAD44STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad44_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 44 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad44_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 44 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad44_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 45 high order drive strength selection. Used in conjunction with PAD45STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad45_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 45 high order drive strength selection. Used in conjunction with PAD45STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad45_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 45 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad45_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 45 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad45_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pad 46 high order drive strength selection. Used in conjunction with PAD46STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad46_ds1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 46 high order drive strength selection. Used in conjunction with PAD46STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad46_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 46 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad46_sr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 46 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad46_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pad 47 high order drive strength selection. Used in conjunction with PAD47STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad47_ds1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 47 high order drive strength selection. Used in conjunction with PAD47STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad47_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 47 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad47_sr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 47 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad47_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Altpadcfgl {
    #[inline(always)]
    fn default() -> Altpadcfgl {
        Altpadcfgl(0)
    }
}
impl core::fmt::Debug for Altpadcfgl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgl")
            .field("pad44_ds1", &self.pad44_ds1())
            .field("pad44_sr", &self.pad44_sr())
            .field("pad45_ds1", &self.pad45_ds1())
            .field("pad45_sr", &self.pad45_sr())
            .field("pad46_ds1", &self.pad46_ds1())
            .field("pad46_sr", &self.pad46_sr())
            .field("pad47_ds1", &self.pad47_ds1())
            .field("pad47_sr", &self.pad47_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgl {{ pad44_ds1: {=bool:?}, pad44_sr: {=bool:?}, pad45_ds1: {=bool:?}, pad45_sr: {=bool:?}, pad46_ds1: {=bool:?}, pad46_sr: {=bool:?}, pad47_ds1: {=bool:?}, pad47_sr: {=bool:?} }}" , self . pad44_ds1 () , self . pad44_sr () , self . pad45_ds1 () , self . pad45_sr () , self . pad46_ds1 () , self . pad46_sr () , self . pad47_ds1 () , self . pad47_sr ())
    }
}
#[doc = "Alternate Pad Configuration reg12 (Pads 49,48)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Altpadcfgm(pub u32);
impl Altpadcfgm {
    #[doc = "Pad 48 high order drive strength selection. Used in conjunction with PAD48STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad48_ds1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 48 high order drive strength selection. Used in conjunction with PAD48STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad48_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 48 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad48_sr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 48 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad48_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pad 49 high order drive strength selection. Used in conjunction with PAD49STRNG field to set the pad drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad49_ds1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 49 high order drive strength selection. Used in conjunction with PAD49STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub const fn set_pad49_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 49 slew rate selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad49_sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 49 slew rate selection."]
    #[inline(always)]
    pub const fn set_pad49_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Altpadcfgm {
    #[inline(always)]
    fn default() -> Altpadcfgm {
        Altpadcfgm(0)
    }
}
impl core::fmt::Debug for Altpadcfgm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Altpadcfgm")
            .field("pad48_ds1", &self.pad48_ds1())
            .field("pad48_sr", &self.pad48_sr())
            .field("pad49_ds1", &self.pad49_ds1())
            .field("pad49_sr", &self.pad49_sr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altpadcfgm {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Altpadcfgm {{ pad48_ds1: {=bool:?}, pad48_sr: {=bool:?}, pad49_ds1: {=bool:?}, pad49_sr: {=bool:?} }}" , self . pad48_ds1 () , self . pad48_sr () , self . pad49_ds1 () , self . pad49_sr ())
    }
}
#[doc = "BLEIF Flow Control IRQ Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleifirq(pub u32);
impl Bleifirq {
    #[doc = "BLEIF IRQ pad select."]
    #[must_use]
    #[inline(always)]
    pub const fn bleifirq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "BLEIF IRQ pad select."]
    #[inline(always)]
    pub const fn set_bleifirq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Bleifirq {
    #[inline(always)]
    fn default() -> Bleifirq {
        Bleifirq(0)
    }
}
impl core::fmt::Debug for Bleifirq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleifirq")
            .field("bleifirq", &self.bleifirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleifirq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bleifirq {{ bleifirq: {=u8:?} }}", self.bleifirq())
    }
}
#[doc = "GPIO Configuration Register A (Pads 0-7)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfga(pub u32);
impl Cfga {
    #[doc = "GPIO0 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0incfg(&self) -> super::vals::Gpio0incfg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpio0incfg::from_bits(val as u8)
    }
    #[doc = "GPIO0 input enable."]
    #[inline(always)]
    pub const fn set_gpio0incfg(&mut self, val: super::vals::Gpio0incfg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO0 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0outcfg(&self) -> super::vals::Gpio0outcfg {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Gpio0outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO0 output configuration."]
    #[inline(always)]
    pub const fn set_gpio0outcfg(&mut self, val: super::vals::Gpio0outcfg) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "GPIO0 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0intd(&self) -> super::vals::Gpio0intd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gpio0intd::from_bits(val as u8)
    }
    #[doc = "GPIO0 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio0intd(&mut self, val: super::vals::Gpio0intd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO1 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1incfg(&self) -> super::vals::Gpio1incfg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Gpio1incfg::from_bits(val as u8)
    }
    #[doc = "GPIO1 input enable."]
    #[inline(always)]
    pub const fn set_gpio1incfg(&mut self, val: super::vals::Gpio1incfg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO1 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1outcfg(&self) -> super::vals::Gpio1outcfg {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Gpio1outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO1 output configuration."]
    #[inline(always)]
    pub const fn set_gpio1outcfg(&mut self, val: super::vals::Gpio1outcfg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "GPIO1 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1intd(&self) -> super::vals::Gpio1intd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Gpio1intd::from_bits(val as u8)
    }
    #[doc = "GPIO1 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio1intd(&mut self, val: super::vals::Gpio1intd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO2 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2incfg(&self) -> super::vals::Gpio2incfg {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Gpio2incfg::from_bits(val as u8)
    }
    #[doc = "GPIO2 input enable."]
    #[inline(always)]
    pub const fn set_gpio2incfg(&mut self, val: super::vals::Gpio2incfg) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO2 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2outcfg(&self) -> super::vals::Gpio2outcfg {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Gpio2outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO2 output configuration."]
    #[inline(always)]
    pub const fn set_gpio2outcfg(&mut self, val: super::vals::Gpio2outcfg) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "GPIO2 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2intd(&self) -> super::vals::Gpio2intd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Gpio2intd::from_bits(val as u8)
    }
    #[doc = "GPIO2 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio2intd(&mut self, val: super::vals::Gpio2intd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO3 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3incfg(&self) -> super::vals::Gpio3incfg {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Gpio3incfg::from_bits(val as u8)
    }
    #[doc = "GPIO3 input enable."]
    #[inline(always)]
    pub const fn set_gpio3incfg(&mut self, val: super::vals::Gpio3incfg) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO3 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3outcfg(&self) -> super::vals::Gpio3outcfg {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Gpio3outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO3 output configuration."]
    #[inline(always)]
    pub const fn set_gpio3outcfg(&mut self, val: super::vals::Gpio3outcfg) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "GPIO3 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3intd(&self) -> super::vals::Gpio3intd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Gpio3intd::from_bits(val as u8)
    }
    #[doc = "GPIO3 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio3intd(&mut self, val: super::vals::Gpio3intd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO4 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4incfg(&self) -> super::vals::Gpio4incfg {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpio4incfg::from_bits(val as u8)
    }
    #[doc = "GPIO4 input enable."]
    #[inline(always)]
    pub const fn set_gpio4incfg(&mut self, val: super::vals::Gpio4incfg) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO4 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4outcfg(&self) -> super::vals::Gpio4outcfg {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Gpio4outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO4 output configuration."]
    #[inline(always)]
    pub const fn set_gpio4outcfg(&mut self, val: super::vals::Gpio4outcfg) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "GPIO4 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4intd(&self) -> super::vals::Gpio4intd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpio4intd::from_bits(val as u8)
    }
    #[doc = "GPIO4 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio4intd(&mut self, val: super::vals::Gpio4intd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO5 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5incfg(&self) -> super::vals::Gpio5incfg {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpio5incfg::from_bits(val as u8)
    }
    #[doc = "GPIO5 input enable."]
    #[inline(always)]
    pub const fn set_gpio5incfg(&mut self, val: super::vals::Gpio5incfg) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO5 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5outcfg(&self) -> super::vals::Gpio5outcfg {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Gpio5outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO5 output configuration."]
    #[inline(always)]
    pub const fn set_gpio5outcfg(&mut self, val: super::vals::Gpio5outcfg) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "GPIO5 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5intd(&self) -> super::vals::Gpio5intd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpio5intd::from_bits(val as u8)
    }
    #[doc = "GPIO5 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio5intd(&mut self, val: super::vals::Gpio5intd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO6 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio6incfg(&self) -> super::vals::Gpio6incfg {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpio6incfg::from_bits(val as u8)
    }
    #[doc = "GPIO6 input enable."]
    #[inline(always)]
    pub const fn set_gpio6incfg(&mut self, val: super::vals::Gpio6incfg) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO6 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio6outcfg(&self) -> super::vals::Gpio6outcfg {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Gpio6outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO6 output configuration."]
    #[inline(always)]
    pub const fn set_gpio6outcfg(&mut self, val: super::vals::Gpio6outcfg) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "GPIO6 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio6intd(&self) -> super::vals::Gpio6intd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Gpio6intd::from_bits(val as u8)
    }
    #[doc = "GPIO6 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio6intd(&mut self, val: super::vals::Gpio6intd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "GPIO7 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio7incfg(&self) -> super::vals::Gpio7incfg {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gpio7incfg::from_bits(val as u8)
    }
    #[doc = "GPIO7 input enable."]
    #[inline(always)]
    pub const fn set_gpio7incfg(&mut self, val: super::vals::Gpio7incfg) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO7 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio7outcfg(&self) -> super::vals::Gpio7outcfg {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Gpio7outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO7 output configuration."]
    #[inline(always)]
    pub const fn set_gpio7outcfg(&mut self, val: super::vals::Gpio7outcfg) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "GPIO7 interrupt direction, nCE polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio7intd(&self) -> super::vals::Gpio7intd {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gpio7intd::from_bits(val as u8)
    }
    #[doc = "GPIO7 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub const fn set_gpio7intd(&mut self, val: super::vals::Gpio7intd) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cfga {
    #[inline(always)]
    fn default() -> Cfga {
        Cfga(0)
    }
}
impl core::fmt::Debug for Cfga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfga")
            .field("gpio0incfg", &self.gpio0incfg())
            .field("gpio0outcfg", &self.gpio0outcfg())
            .field("gpio0intd", &self.gpio0intd())
            .field("gpio1incfg", &self.gpio1incfg())
            .field("gpio1outcfg", &self.gpio1outcfg())
            .field("gpio1intd", &self.gpio1intd())
            .field("gpio2incfg", &self.gpio2incfg())
            .field("gpio2outcfg", &self.gpio2outcfg())
            .field("gpio2intd", &self.gpio2intd())
            .field("gpio3incfg", &self.gpio3incfg())
            .field("gpio3outcfg", &self.gpio3outcfg())
            .field("gpio3intd", &self.gpio3intd())
            .field("gpio4incfg", &self.gpio4incfg())
            .field("gpio4outcfg", &self.gpio4outcfg())
            .field("gpio4intd", &self.gpio4intd())
            .field("gpio5incfg", &self.gpio5incfg())
            .field("gpio5outcfg", &self.gpio5outcfg())
            .field("gpio5intd", &self.gpio5intd())
            .field("gpio6incfg", &self.gpio6incfg())
            .field("gpio6outcfg", &self.gpio6outcfg())
            .field("gpio6intd", &self.gpio6intd())
            .field("gpio7incfg", &self.gpio7incfg())
            .field("gpio7outcfg", &self.gpio7outcfg())
            .field("gpio7intd", &self.gpio7intd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfga {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfga {{ gpio0incfg: {:?}, gpio0outcfg: {:?}, gpio0intd: {:?}, gpio1incfg: {:?}, gpio1outcfg: {:?}, gpio1intd: {:?}, gpio2incfg: {:?}, gpio2outcfg: {:?}, gpio2intd: {:?}, gpio3incfg: {:?}, gpio3outcfg: {:?}, gpio3intd: {:?}, gpio4incfg: {:?}, gpio4outcfg: {:?}, gpio4intd: {:?}, gpio5incfg: {:?}, gpio5outcfg: {:?}, gpio5intd: {:?}, gpio6incfg: {:?}, gpio6outcfg: {:?}, gpio6intd: {:?}, gpio7incfg: {:?}, gpio7outcfg: {:?}, gpio7intd: {:?} }}" , self . gpio0incfg () , self . gpio0outcfg () , self . gpio0intd () , self . gpio1incfg () , self . gpio1outcfg () , self . gpio1intd () , self . gpio2incfg () , self . gpio2outcfg () , self . gpio2intd () , self . gpio3incfg () , self . gpio3outcfg () , self . gpio3intd () , self . gpio4incfg () , self . gpio4outcfg () , self . gpio4intd () , self . gpio5incfg () , self . gpio5outcfg () , self . gpio5intd () , self . gpio6incfg () , self . gpio6outcfg () , self . gpio6intd () , self . gpio7incfg () , self . gpio7outcfg () , self . gpio7intd ())
    }
}
#[doc = "GPIO Configuration Register B (Pads 8-15)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgb(pub u32);
impl Cfgb {
    #[doc = "GPIO8 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio8incfg(&self) -> super::vals::Gpio8incfg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpio8incfg::from_bits(val as u8)
    }
    #[doc = "GPIO8 input enable."]
    #[inline(always)]
    pub const fn set_gpio8incfg(&mut self, val: super::vals::Gpio8incfg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO8 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio8outcfg(&self) -> super::vals::Gpio8outcfg {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Gpio8outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO8 output configuration."]
    #[inline(always)]
    pub const fn set_gpio8outcfg(&mut self, val: super::vals::Gpio8outcfg) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "GPIO8 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio8intd(&self) -> super::vals::Gpio8intd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gpio8intd::from_bits(val as u8)
    }
    #[doc = "GPIO8 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio8intd(&mut self, val: super::vals::Gpio8intd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO9 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio9incfg(&self) -> super::vals::Gpio9incfg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Gpio9incfg::from_bits(val as u8)
    }
    #[doc = "GPIO9 input enable."]
    #[inline(always)]
    pub const fn set_gpio9incfg(&mut self, val: super::vals::Gpio9incfg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO9 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio9outcfg(&self) -> super::vals::Gpio9outcfg {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Gpio9outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO9 output configuration."]
    #[inline(always)]
    pub const fn set_gpio9outcfg(&mut self, val: super::vals::Gpio9outcfg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "GPIO9 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio9intd(&self) -> super::vals::Gpio9intd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Gpio9intd::from_bits(val as u8)
    }
    #[doc = "GPIO9 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio9intd(&mut self, val: super::vals::Gpio9intd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO10 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio10incfg(&self) -> super::vals::Gpio10incfg {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Gpio10incfg::from_bits(val as u8)
    }
    #[doc = "GPIO10 input enable."]
    #[inline(always)]
    pub const fn set_gpio10incfg(&mut self, val: super::vals::Gpio10incfg) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO10 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio10outcfg(&self) -> super::vals::Gpio10outcfg {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Gpio10outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO10 output configuration."]
    #[inline(always)]
    pub const fn set_gpio10outcfg(&mut self, val: super::vals::Gpio10outcfg) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "GPIO10 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio10intd(&self) -> super::vals::Gpio10intd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Gpio10intd::from_bits(val as u8)
    }
    #[doc = "GPIO10 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio10intd(&mut self, val: super::vals::Gpio10intd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO11 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio11incfg(&self) -> super::vals::Gpio11incfg {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Gpio11incfg::from_bits(val as u8)
    }
    #[doc = "GPIO11 input enable."]
    #[inline(always)]
    pub const fn set_gpio11incfg(&mut self, val: super::vals::Gpio11incfg) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO11 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio11outcfg(&self) -> super::vals::Gpio11outcfg {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Gpio11outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO11 output configuration."]
    #[inline(always)]
    pub const fn set_gpio11outcfg(&mut self, val: super::vals::Gpio11outcfg) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "GPIO11 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio11intd(&self) -> super::vals::Gpio11intd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Gpio11intd::from_bits(val as u8)
    }
    #[doc = "GPIO11 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio11intd(&mut self, val: super::vals::Gpio11intd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO12 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio12incfg(&self) -> super::vals::Gpio12incfg {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpio12incfg::from_bits(val as u8)
    }
    #[doc = "GPIO12 input enable."]
    #[inline(always)]
    pub const fn set_gpio12incfg(&mut self, val: super::vals::Gpio12incfg) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO12 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio12outcfg(&self) -> super::vals::Gpio12outcfg {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Gpio12outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO12 output configuration."]
    #[inline(always)]
    pub const fn set_gpio12outcfg(&mut self, val: super::vals::Gpio12outcfg) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "GPIO12 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio12intd(&self) -> super::vals::Gpio12intd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpio12intd::from_bits(val as u8)
    }
    #[doc = "GPIO12 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio12intd(&mut self, val: super::vals::Gpio12intd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO13 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio13incfg(&self) -> super::vals::Gpio13incfg {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpio13incfg::from_bits(val as u8)
    }
    #[doc = "GPIO13 input enable."]
    #[inline(always)]
    pub const fn set_gpio13incfg(&mut self, val: super::vals::Gpio13incfg) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO13 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio13outcfg(&self) -> super::vals::Gpio13outcfg {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Gpio13outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO13 output configuration."]
    #[inline(always)]
    pub const fn set_gpio13outcfg(&mut self, val: super::vals::Gpio13outcfg) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "GPIO13 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio13intd(&self) -> super::vals::Gpio13intd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpio13intd::from_bits(val as u8)
    }
    #[doc = "GPIO13 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio13intd(&mut self, val: super::vals::Gpio13intd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO14 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio14incfg(&self) -> super::vals::Gpio14incfg {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpio14incfg::from_bits(val as u8)
    }
    #[doc = "GPIO14 input enable."]
    #[inline(always)]
    pub const fn set_gpio14incfg(&mut self, val: super::vals::Gpio14incfg) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO14 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio14outcfg(&self) -> super::vals::Gpio14outcfg {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Gpio14outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO14 output configuration."]
    #[inline(always)]
    pub const fn set_gpio14outcfg(&mut self, val: super::vals::Gpio14outcfg) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "GPIO14 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio14intd(&self) -> super::vals::Gpio14intd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Gpio14intd::from_bits(val as u8)
    }
    #[doc = "GPIO14 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio14intd(&mut self, val: super::vals::Gpio14intd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "GPIO15 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio15incfg(&self) -> super::vals::Gpio15incfg {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gpio15incfg::from_bits(val as u8)
    }
    #[doc = "GPIO15 input enable."]
    #[inline(always)]
    pub const fn set_gpio15incfg(&mut self, val: super::vals::Gpio15incfg) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO15 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio15outcfg(&self) -> super::vals::Gpio15outcfg {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Gpio15outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO15 output configuration."]
    #[inline(always)]
    pub const fn set_gpio15outcfg(&mut self, val: super::vals::Gpio15outcfg) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "GPIO15 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio15intd(&self) -> super::vals::Gpio15intd {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gpio15intd::from_bits(val as u8)
    }
    #[doc = "GPIO15 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio15intd(&mut self, val: super::vals::Gpio15intd) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cfgb {
    #[inline(always)]
    fn default() -> Cfgb {
        Cfgb(0)
    }
}
impl core::fmt::Debug for Cfgb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgb")
            .field("gpio8incfg", &self.gpio8incfg())
            .field("gpio8outcfg", &self.gpio8outcfg())
            .field("gpio8intd", &self.gpio8intd())
            .field("gpio9incfg", &self.gpio9incfg())
            .field("gpio9outcfg", &self.gpio9outcfg())
            .field("gpio9intd", &self.gpio9intd())
            .field("gpio10incfg", &self.gpio10incfg())
            .field("gpio10outcfg", &self.gpio10outcfg())
            .field("gpio10intd", &self.gpio10intd())
            .field("gpio11incfg", &self.gpio11incfg())
            .field("gpio11outcfg", &self.gpio11outcfg())
            .field("gpio11intd", &self.gpio11intd())
            .field("gpio12incfg", &self.gpio12incfg())
            .field("gpio12outcfg", &self.gpio12outcfg())
            .field("gpio12intd", &self.gpio12intd())
            .field("gpio13incfg", &self.gpio13incfg())
            .field("gpio13outcfg", &self.gpio13outcfg())
            .field("gpio13intd", &self.gpio13intd())
            .field("gpio14incfg", &self.gpio14incfg())
            .field("gpio14outcfg", &self.gpio14outcfg())
            .field("gpio14intd", &self.gpio14intd())
            .field("gpio15incfg", &self.gpio15incfg())
            .field("gpio15outcfg", &self.gpio15outcfg())
            .field("gpio15intd", &self.gpio15intd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgb {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfgb {{ gpio8incfg: {:?}, gpio8outcfg: {:?}, gpio8intd: {:?}, gpio9incfg: {:?}, gpio9outcfg: {:?}, gpio9intd: {:?}, gpio10incfg: {:?}, gpio10outcfg: {:?}, gpio10intd: {:?}, gpio11incfg: {:?}, gpio11outcfg: {:?}, gpio11intd: {:?}, gpio12incfg: {:?}, gpio12outcfg: {:?}, gpio12intd: {:?}, gpio13incfg: {:?}, gpio13outcfg: {:?}, gpio13intd: {:?}, gpio14incfg: {:?}, gpio14outcfg: {:?}, gpio14intd: {:?}, gpio15incfg: {:?}, gpio15outcfg: {:?}, gpio15intd: {:?} }}" , self . gpio8incfg () , self . gpio8outcfg () , self . gpio8intd () , self . gpio9incfg () , self . gpio9outcfg () , self . gpio9intd () , self . gpio10incfg () , self . gpio10outcfg () , self . gpio10intd () , self . gpio11incfg () , self . gpio11outcfg () , self . gpio11intd () , self . gpio12incfg () , self . gpio12outcfg () , self . gpio12intd () , self . gpio13incfg () , self . gpio13outcfg () , self . gpio13intd () , self . gpio14incfg () , self . gpio14outcfg () , self . gpio14intd () , self . gpio15incfg () , self . gpio15outcfg () , self . gpio15intd ())
    }
}
#[doc = "GPIO Configuration Register C (Pads 16-23)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgc(pub u32);
impl Cfgc {
    #[doc = "GPIO16 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio16incfg(&self) -> super::vals::Gpio16incfg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpio16incfg::from_bits(val as u8)
    }
    #[doc = "GPIO16 input enable."]
    #[inline(always)]
    pub const fn set_gpio16incfg(&mut self, val: super::vals::Gpio16incfg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO16 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio16outcfg(&self) -> super::vals::Gpio16outcfg {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Gpio16outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO16 output configuration."]
    #[inline(always)]
    pub const fn set_gpio16outcfg(&mut self, val: super::vals::Gpio16outcfg) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "GPIO16 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio16intd(&self) -> super::vals::Gpio16intd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gpio16intd::from_bits(val as u8)
    }
    #[doc = "GPIO16 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio16intd(&mut self, val: super::vals::Gpio16intd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO17 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio17incfg(&self) -> super::vals::Gpio17incfg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Gpio17incfg::from_bits(val as u8)
    }
    #[doc = "GPIO17 input enable."]
    #[inline(always)]
    pub const fn set_gpio17incfg(&mut self, val: super::vals::Gpio17incfg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO17 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio17outcfg(&self) -> super::vals::Gpio17outcfg {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Gpio17outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO17 output configuration."]
    #[inline(always)]
    pub const fn set_gpio17outcfg(&mut self, val: super::vals::Gpio17outcfg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "GPIO17 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio17intd(&self) -> super::vals::Gpio17intd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Gpio17intd::from_bits(val as u8)
    }
    #[doc = "GPIO17 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio17intd(&mut self, val: super::vals::Gpio17intd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO18 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio18incfg(&self) -> super::vals::Gpio18incfg {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Gpio18incfg::from_bits(val as u8)
    }
    #[doc = "GPIO18 input enable."]
    #[inline(always)]
    pub const fn set_gpio18incfg(&mut self, val: super::vals::Gpio18incfg) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO18 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio18outcfg(&self) -> super::vals::Gpio18outcfg {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Gpio18outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO18 output configuration."]
    #[inline(always)]
    pub const fn set_gpio18outcfg(&mut self, val: super::vals::Gpio18outcfg) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "GPIO18 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio18intd(&self) -> super::vals::Gpio18intd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Gpio18intd::from_bits(val as u8)
    }
    #[doc = "GPIO18 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio18intd(&mut self, val: super::vals::Gpio18intd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO19 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio19incfg(&self) -> super::vals::Gpio19incfg {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Gpio19incfg::from_bits(val as u8)
    }
    #[doc = "GPIO19 input enable."]
    #[inline(always)]
    pub const fn set_gpio19incfg(&mut self, val: super::vals::Gpio19incfg) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO19 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio19outcfg(&self) -> super::vals::Gpio19outcfg {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Gpio19outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO19 output configuration."]
    #[inline(always)]
    pub const fn set_gpio19outcfg(&mut self, val: super::vals::Gpio19outcfg) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "GPIO19 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio19intd(&self) -> super::vals::Gpio19intd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Gpio19intd::from_bits(val as u8)
    }
    #[doc = "GPIO19 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio19intd(&mut self, val: super::vals::Gpio19intd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO20 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio20incfg(&self) -> super::vals::Gpio20incfg {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpio20incfg::from_bits(val as u8)
    }
    #[doc = "GPIO20 input enable."]
    #[inline(always)]
    pub const fn set_gpio20incfg(&mut self, val: super::vals::Gpio20incfg) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO20 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio20outcfg(&self) -> super::vals::Gpio20outcfg {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Gpio20outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO20 output configuration."]
    #[inline(always)]
    pub const fn set_gpio20outcfg(&mut self, val: super::vals::Gpio20outcfg) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "GPIO20 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio20intd(&self) -> super::vals::Gpio20intd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpio20intd::from_bits(val as u8)
    }
    #[doc = "GPIO20 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio20intd(&mut self, val: super::vals::Gpio20intd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO21 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio21incfg(&self) -> super::vals::Gpio21incfg {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpio21incfg::from_bits(val as u8)
    }
    #[doc = "GPIO21 input enable."]
    #[inline(always)]
    pub const fn set_gpio21incfg(&mut self, val: super::vals::Gpio21incfg) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO21 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio21outcfg(&self) -> super::vals::Gpio21outcfg {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Gpio21outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO21 output configuration."]
    #[inline(always)]
    pub const fn set_gpio21outcfg(&mut self, val: super::vals::Gpio21outcfg) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "GPIO21 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio21intd(&self) -> super::vals::Gpio21intd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpio21intd::from_bits(val as u8)
    }
    #[doc = "GPIO21 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio21intd(&mut self, val: super::vals::Gpio21intd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO22 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio22incfg(&self) -> super::vals::Gpio22incfg {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpio22incfg::from_bits(val as u8)
    }
    #[doc = "GPIO22 input enable."]
    #[inline(always)]
    pub const fn set_gpio22incfg(&mut self, val: super::vals::Gpio22incfg) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO22 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio22outcfg(&self) -> super::vals::Gpio22outcfg {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Gpio22outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO22 output configuration."]
    #[inline(always)]
    pub const fn set_gpio22outcfg(&mut self, val: super::vals::Gpio22outcfg) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "GPIO22 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio22intd(&self) -> super::vals::Gpio22intd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Gpio22intd::from_bits(val as u8)
    }
    #[doc = "GPIO22 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio22intd(&mut self, val: super::vals::Gpio22intd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "GPIO23 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio23incfg(&self) -> super::vals::Gpio23incfg {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gpio23incfg::from_bits(val as u8)
    }
    #[doc = "GPIO23 input enable."]
    #[inline(always)]
    pub const fn set_gpio23incfg(&mut self, val: super::vals::Gpio23incfg) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO23 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio23outcfg(&self) -> super::vals::Gpio23outcfg {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Gpio23outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO23 output configuration."]
    #[inline(always)]
    pub const fn set_gpio23outcfg(&mut self, val: super::vals::Gpio23outcfg) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "GPIO23 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio23intd(&self) -> super::vals::Gpio23intd {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gpio23intd::from_bits(val as u8)
    }
    #[doc = "GPIO23 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio23intd(&mut self, val: super::vals::Gpio23intd) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cfgc {
    #[inline(always)]
    fn default() -> Cfgc {
        Cfgc(0)
    }
}
impl core::fmt::Debug for Cfgc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgc")
            .field("gpio16incfg", &self.gpio16incfg())
            .field("gpio16outcfg", &self.gpio16outcfg())
            .field("gpio16intd", &self.gpio16intd())
            .field("gpio17incfg", &self.gpio17incfg())
            .field("gpio17outcfg", &self.gpio17outcfg())
            .field("gpio17intd", &self.gpio17intd())
            .field("gpio18incfg", &self.gpio18incfg())
            .field("gpio18outcfg", &self.gpio18outcfg())
            .field("gpio18intd", &self.gpio18intd())
            .field("gpio19incfg", &self.gpio19incfg())
            .field("gpio19outcfg", &self.gpio19outcfg())
            .field("gpio19intd", &self.gpio19intd())
            .field("gpio20incfg", &self.gpio20incfg())
            .field("gpio20outcfg", &self.gpio20outcfg())
            .field("gpio20intd", &self.gpio20intd())
            .field("gpio21incfg", &self.gpio21incfg())
            .field("gpio21outcfg", &self.gpio21outcfg())
            .field("gpio21intd", &self.gpio21intd())
            .field("gpio22incfg", &self.gpio22incfg())
            .field("gpio22outcfg", &self.gpio22outcfg())
            .field("gpio22intd", &self.gpio22intd())
            .field("gpio23incfg", &self.gpio23incfg())
            .field("gpio23outcfg", &self.gpio23outcfg())
            .field("gpio23intd", &self.gpio23intd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfgc {{ gpio16incfg: {:?}, gpio16outcfg: {:?}, gpio16intd: {:?}, gpio17incfg: {:?}, gpio17outcfg: {:?}, gpio17intd: {:?}, gpio18incfg: {:?}, gpio18outcfg: {:?}, gpio18intd: {:?}, gpio19incfg: {:?}, gpio19outcfg: {:?}, gpio19intd: {:?}, gpio20incfg: {:?}, gpio20outcfg: {:?}, gpio20intd: {:?}, gpio21incfg: {:?}, gpio21outcfg: {:?}, gpio21intd: {:?}, gpio22incfg: {:?}, gpio22outcfg: {:?}, gpio22intd: {:?}, gpio23incfg: {:?}, gpio23outcfg: {:?}, gpio23intd: {:?} }}" , self . gpio16incfg () , self . gpio16outcfg () , self . gpio16intd () , self . gpio17incfg () , self . gpio17outcfg () , self . gpio17intd () , self . gpio18incfg () , self . gpio18outcfg () , self . gpio18intd () , self . gpio19incfg () , self . gpio19outcfg () , self . gpio19intd () , self . gpio20incfg () , self . gpio20outcfg () , self . gpio20intd () , self . gpio21incfg () , self . gpio21outcfg () , self . gpio21intd () , self . gpio22incfg () , self . gpio22outcfg () , self . gpio22intd () , self . gpio23incfg () , self . gpio23outcfg () , self . gpio23intd ())
    }
}
#[doc = "GPIO Configuration Register D (Pads 24-31)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgd(pub u32);
impl Cfgd {
    #[doc = "GPIO24 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio24incfg(&self) -> super::vals::Gpio24incfg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpio24incfg::from_bits(val as u8)
    }
    #[doc = "GPIO24 input enable."]
    #[inline(always)]
    pub const fn set_gpio24incfg(&mut self, val: super::vals::Gpio24incfg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO24 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio24outcfg(&self) -> super::vals::Gpio24outcfg {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Gpio24outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO24 output configuration."]
    #[inline(always)]
    pub const fn set_gpio24outcfg(&mut self, val: super::vals::Gpio24outcfg) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "GPIO24 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio24intd(&self) -> super::vals::Gpio24intd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gpio24intd::from_bits(val as u8)
    }
    #[doc = "GPIO24 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio24intd(&mut self, val: super::vals::Gpio24intd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO25 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio25incfg(&self) -> super::vals::Gpio25incfg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Gpio25incfg::from_bits(val as u8)
    }
    #[doc = "GPIO25 input enable."]
    #[inline(always)]
    pub const fn set_gpio25incfg(&mut self, val: super::vals::Gpio25incfg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO25 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio25outcfg(&self) -> super::vals::Gpio25outcfg {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Gpio25outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO25 output configuration."]
    #[inline(always)]
    pub const fn set_gpio25outcfg(&mut self, val: super::vals::Gpio25outcfg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "GPIO25 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio25intd(&self) -> super::vals::Gpio25intd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Gpio25intd::from_bits(val as u8)
    }
    #[doc = "GPIO25 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio25intd(&mut self, val: super::vals::Gpio25intd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO26 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio26incfg(&self) -> super::vals::Gpio26incfg {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Gpio26incfg::from_bits(val as u8)
    }
    #[doc = "GPIO26 input enable."]
    #[inline(always)]
    pub const fn set_gpio26incfg(&mut self, val: super::vals::Gpio26incfg) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO26 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio26outcfg(&self) -> super::vals::Gpio26outcfg {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Gpio26outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO26 output configuration."]
    #[inline(always)]
    pub const fn set_gpio26outcfg(&mut self, val: super::vals::Gpio26outcfg) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "GPIO26 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio26intd(&self) -> super::vals::Gpio26intd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Gpio26intd::from_bits(val as u8)
    }
    #[doc = "GPIO26 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio26intd(&mut self, val: super::vals::Gpio26intd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO27 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio27incfg(&self) -> super::vals::Gpio27incfg {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Gpio27incfg::from_bits(val as u8)
    }
    #[doc = "GPIO27 input enable."]
    #[inline(always)]
    pub const fn set_gpio27incfg(&mut self, val: super::vals::Gpio27incfg) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO27 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio27outcfg(&self) -> super::vals::Gpio27outcfg {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Gpio27outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO27 output configuration."]
    #[inline(always)]
    pub const fn set_gpio27outcfg(&mut self, val: super::vals::Gpio27outcfg) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "GPIO27 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio27intd(&self) -> super::vals::Gpio27intd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Gpio27intd::from_bits(val as u8)
    }
    #[doc = "GPIO27 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio27intd(&mut self, val: super::vals::Gpio27intd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO28 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio28incfg(&self) -> super::vals::Gpio28incfg {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpio28incfg::from_bits(val as u8)
    }
    #[doc = "GPIO28 input enable."]
    #[inline(always)]
    pub const fn set_gpio28incfg(&mut self, val: super::vals::Gpio28incfg) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO28 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio28outcfg(&self) -> super::vals::Gpio28outcfg {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Gpio28outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO28 output configuration."]
    #[inline(always)]
    pub const fn set_gpio28outcfg(&mut self, val: super::vals::Gpio28outcfg) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "GPIO28 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio28intd(&self) -> super::vals::Gpio28intd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpio28intd::from_bits(val as u8)
    }
    #[doc = "GPIO28 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio28intd(&mut self, val: super::vals::Gpio28intd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO29 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio29incfg(&self) -> super::vals::Gpio29incfg {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpio29incfg::from_bits(val as u8)
    }
    #[doc = "GPIO29 input enable."]
    #[inline(always)]
    pub const fn set_gpio29incfg(&mut self, val: super::vals::Gpio29incfg) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO29 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio29outcfg(&self) -> super::vals::Gpio29outcfg {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Gpio29outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO29 output configuration."]
    #[inline(always)]
    pub const fn set_gpio29outcfg(&mut self, val: super::vals::Gpio29outcfg) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "GPIO29 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio29intd(&self) -> super::vals::Gpio29intd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpio29intd::from_bits(val as u8)
    }
    #[doc = "GPIO29 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio29intd(&mut self, val: super::vals::Gpio29intd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO30 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio30incfg(&self) -> super::vals::Gpio30incfg {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpio30incfg::from_bits(val as u8)
    }
    #[doc = "GPIO30 input enable."]
    #[inline(always)]
    pub const fn set_gpio30incfg(&mut self, val: super::vals::Gpio30incfg) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO30 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio30outcfg(&self) -> super::vals::Gpio30outcfg {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Gpio30outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO30 output configuration."]
    #[inline(always)]
    pub const fn set_gpio30outcfg(&mut self, val: super::vals::Gpio30outcfg) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "GPIO30 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio30intd(&self) -> super::vals::Gpio30intd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Gpio30intd::from_bits(val as u8)
    }
    #[doc = "GPIO30 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio30intd(&mut self, val: super::vals::Gpio30intd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "GPIO31 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio31incfg(&self) -> super::vals::Gpio31incfg {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gpio31incfg::from_bits(val as u8)
    }
    #[doc = "GPIO31 input enable."]
    #[inline(always)]
    pub const fn set_gpio31incfg(&mut self, val: super::vals::Gpio31incfg) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO31 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio31outcfg(&self) -> super::vals::Gpio31outcfg {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Gpio31outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO31 output configuration."]
    #[inline(always)]
    pub const fn set_gpio31outcfg(&mut self, val: super::vals::Gpio31outcfg) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "GPIO31 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio31intd(&self) -> super::vals::Gpio31intd {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gpio31intd::from_bits(val as u8)
    }
    #[doc = "GPIO31 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio31intd(&mut self, val: super::vals::Gpio31intd) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cfgd {
    #[inline(always)]
    fn default() -> Cfgd {
        Cfgd(0)
    }
}
impl core::fmt::Debug for Cfgd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgd")
            .field("gpio24incfg", &self.gpio24incfg())
            .field("gpio24outcfg", &self.gpio24outcfg())
            .field("gpio24intd", &self.gpio24intd())
            .field("gpio25incfg", &self.gpio25incfg())
            .field("gpio25outcfg", &self.gpio25outcfg())
            .field("gpio25intd", &self.gpio25intd())
            .field("gpio26incfg", &self.gpio26incfg())
            .field("gpio26outcfg", &self.gpio26outcfg())
            .field("gpio26intd", &self.gpio26intd())
            .field("gpio27incfg", &self.gpio27incfg())
            .field("gpio27outcfg", &self.gpio27outcfg())
            .field("gpio27intd", &self.gpio27intd())
            .field("gpio28incfg", &self.gpio28incfg())
            .field("gpio28outcfg", &self.gpio28outcfg())
            .field("gpio28intd", &self.gpio28intd())
            .field("gpio29incfg", &self.gpio29incfg())
            .field("gpio29outcfg", &self.gpio29outcfg())
            .field("gpio29intd", &self.gpio29intd())
            .field("gpio30incfg", &self.gpio30incfg())
            .field("gpio30outcfg", &self.gpio30outcfg())
            .field("gpio30intd", &self.gpio30intd())
            .field("gpio31incfg", &self.gpio31incfg())
            .field("gpio31outcfg", &self.gpio31outcfg())
            .field("gpio31intd", &self.gpio31intd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfgd {{ gpio24incfg: {:?}, gpio24outcfg: {:?}, gpio24intd: {:?}, gpio25incfg: {:?}, gpio25outcfg: {:?}, gpio25intd: {:?}, gpio26incfg: {:?}, gpio26outcfg: {:?}, gpio26intd: {:?}, gpio27incfg: {:?}, gpio27outcfg: {:?}, gpio27intd: {:?}, gpio28incfg: {:?}, gpio28outcfg: {:?}, gpio28intd: {:?}, gpio29incfg: {:?}, gpio29outcfg: {:?}, gpio29intd: {:?}, gpio30incfg: {:?}, gpio30outcfg: {:?}, gpio30intd: {:?}, gpio31incfg: {:?}, gpio31outcfg: {:?}, gpio31intd: {:?} }}" , self . gpio24incfg () , self . gpio24outcfg () , self . gpio24intd () , self . gpio25incfg () , self . gpio25outcfg () , self . gpio25intd () , self . gpio26incfg () , self . gpio26outcfg () , self . gpio26intd () , self . gpio27incfg () , self . gpio27outcfg () , self . gpio27intd () , self . gpio28incfg () , self . gpio28outcfg () , self . gpio28intd () , self . gpio29incfg () , self . gpio29outcfg () , self . gpio29intd () , self . gpio30incfg () , self . gpio30outcfg () , self . gpio30intd () , self . gpio31incfg () , self . gpio31outcfg () , self . gpio31intd ())
    }
}
#[doc = "GPIO Configuration Register E (Pads 32-39)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfge(pub u32);
impl Cfge {
    #[doc = "GPIO32 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio32incfg(&self) -> super::vals::Gpio32incfg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpio32incfg::from_bits(val as u8)
    }
    #[doc = "GPIO32 input enable."]
    #[inline(always)]
    pub const fn set_gpio32incfg(&mut self, val: super::vals::Gpio32incfg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO32 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio32outcfg(&self) -> super::vals::Gpio32outcfg {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Gpio32outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO32 output configuration."]
    #[inline(always)]
    pub const fn set_gpio32outcfg(&mut self, val: super::vals::Gpio32outcfg) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "GPIO32 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio32intd(&self) -> super::vals::Gpio32intd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gpio32intd::from_bits(val as u8)
    }
    #[doc = "GPIO32 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio32intd(&mut self, val: super::vals::Gpio32intd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO33 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio33incfg(&self) -> super::vals::Gpio33incfg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Gpio33incfg::from_bits(val as u8)
    }
    #[doc = "GPIO33 input enable."]
    #[inline(always)]
    pub const fn set_gpio33incfg(&mut self, val: super::vals::Gpio33incfg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO33 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio33outcfg(&self) -> super::vals::Gpio33outcfg {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Gpio33outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO33 output configuration."]
    #[inline(always)]
    pub const fn set_gpio33outcfg(&mut self, val: super::vals::Gpio33outcfg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "GPIO33 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio33intd(&self) -> super::vals::Gpio33intd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Gpio33intd::from_bits(val as u8)
    }
    #[doc = "GPIO33 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio33intd(&mut self, val: super::vals::Gpio33intd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO34 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio34incfg(&self) -> super::vals::Gpio34incfg {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Gpio34incfg::from_bits(val as u8)
    }
    #[doc = "GPIO34 input enable."]
    #[inline(always)]
    pub const fn set_gpio34incfg(&mut self, val: super::vals::Gpio34incfg) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO34 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio34outcfg(&self) -> super::vals::Gpio34outcfg {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Gpio34outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO34 output configuration."]
    #[inline(always)]
    pub const fn set_gpio34outcfg(&mut self, val: super::vals::Gpio34outcfg) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "GPIO34 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio34intd(&self) -> super::vals::Gpio34intd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Gpio34intd::from_bits(val as u8)
    }
    #[doc = "GPIO34 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio34intd(&mut self, val: super::vals::Gpio34intd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO35 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio35incfg(&self) -> super::vals::Gpio35incfg {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Gpio35incfg::from_bits(val as u8)
    }
    #[doc = "GPIO35 input enable."]
    #[inline(always)]
    pub const fn set_gpio35incfg(&mut self, val: super::vals::Gpio35incfg) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO35 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio35outcfg(&self) -> super::vals::Gpio35outcfg {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Gpio35outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO35 output configuration."]
    #[inline(always)]
    pub const fn set_gpio35outcfg(&mut self, val: super::vals::Gpio35outcfg) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "GPIO35 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio35intd(&self) -> super::vals::Gpio35intd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Gpio35intd::from_bits(val as u8)
    }
    #[doc = "GPIO35 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio35intd(&mut self, val: super::vals::Gpio35intd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO36 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio36incfg(&self) -> super::vals::Gpio36incfg {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpio36incfg::from_bits(val as u8)
    }
    #[doc = "GPIO36 input enable."]
    #[inline(always)]
    pub const fn set_gpio36incfg(&mut self, val: super::vals::Gpio36incfg) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO36 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio36outcfg(&self) -> super::vals::Gpio36outcfg {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Gpio36outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO36 output configuration."]
    #[inline(always)]
    pub const fn set_gpio36outcfg(&mut self, val: super::vals::Gpio36outcfg) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "GPIO36 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio36intd(&self) -> super::vals::Gpio36intd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpio36intd::from_bits(val as u8)
    }
    #[doc = "GPIO36 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio36intd(&mut self, val: super::vals::Gpio36intd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO37 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio37incfg(&self) -> super::vals::Gpio37incfg {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpio37incfg::from_bits(val as u8)
    }
    #[doc = "GPIO37 input enable."]
    #[inline(always)]
    pub const fn set_gpio37incfg(&mut self, val: super::vals::Gpio37incfg) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO37 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio37outcfg(&self) -> super::vals::Gpio37outcfg {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Gpio37outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO37 output configuration."]
    #[inline(always)]
    pub const fn set_gpio37outcfg(&mut self, val: super::vals::Gpio37outcfg) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "GPIO37 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio37intd(&self) -> super::vals::Gpio37intd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpio37intd::from_bits(val as u8)
    }
    #[doc = "GPIO37 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio37intd(&mut self, val: super::vals::Gpio37intd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO38 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio38incfg(&self) -> super::vals::Gpio38incfg {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpio38incfg::from_bits(val as u8)
    }
    #[doc = "GPIO38 input enable."]
    #[inline(always)]
    pub const fn set_gpio38incfg(&mut self, val: super::vals::Gpio38incfg) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO38 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio38outcfg(&self) -> super::vals::Gpio38outcfg {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Gpio38outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO38 output configuration."]
    #[inline(always)]
    pub const fn set_gpio38outcfg(&mut self, val: super::vals::Gpio38outcfg) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "GPIO38 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio38intd(&self) -> super::vals::Gpio38intd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Gpio38intd::from_bits(val as u8)
    }
    #[doc = "GPIO38 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio38intd(&mut self, val: super::vals::Gpio38intd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "GPIO39 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio39incfg(&self) -> super::vals::Gpio39incfg {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gpio39incfg::from_bits(val as u8)
    }
    #[doc = "GPIO39 input enable."]
    #[inline(always)]
    pub const fn set_gpio39incfg(&mut self, val: super::vals::Gpio39incfg) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO39 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio39outcfg(&self) -> super::vals::Gpio39outcfg {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Gpio39outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO39 output configuration."]
    #[inline(always)]
    pub const fn set_gpio39outcfg(&mut self, val: super::vals::Gpio39outcfg) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "GPIO39 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio39intd(&self) -> super::vals::Gpio39intd {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gpio39intd::from_bits(val as u8)
    }
    #[doc = "GPIO39 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio39intd(&mut self, val: super::vals::Gpio39intd) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cfge {
    #[inline(always)]
    fn default() -> Cfge {
        Cfge(0)
    }
}
impl core::fmt::Debug for Cfge {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfge")
            .field("gpio32incfg", &self.gpio32incfg())
            .field("gpio32outcfg", &self.gpio32outcfg())
            .field("gpio32intd", &self.gpio32intd())
            .field("gpio33incfg", &self.gpio33incfg())
            .field("gpio33outcfg", &self.gpio33outcfg())
            .field("gpio33intd", &self.gpio33intd())
            .field("gpio34incfg", &self.gpio34incfg())
            .field("gpio34outcfg", &self.gpio34outcfg())
            .field("gpio34intd", &self.gpio34intd())
            .field("gpio35incfg", &self.gpio35incfg())
            .field("gpio35outcfg", &self.gpio35outcfg())
            .field("gpio35intd", &self.gpio35intd())
            .field("gpio36incfg", &self.gpio36incfg())
            .field("gpio36outcfg", &self.gpio36outcfg())
            .field("gpio36intd", &self.gpio36intd())
            .field("gpio37incfg", &self.gpio37incfg())
            .field("gpio37outcfg", &self.gpio37outcfg())
            .field("gpio37intd", &self.gpio37intd())
            .field("gpio38incfg", &self.gpio38incfg())
            .field("gpio38outcfg", &self.gpio38outcfg())
            .field("gpio38intd", &self.gpio38intd())
            .field("gpio39incfg", &self.gpio39incfg())
            .field("gpio39outcfg", &self.gpio39outcfg())
            .field("gpio39intd", &self.gpio39intd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfge {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfge {{ gpio32incfg: {:?}, gpio32outcfg: {:?}, gpio32intd: {:?}, gpio33incfg: {:?}, gpio33outcfg: {:?}, gpio33intd: {:?}, gpio34incfg: {:?}, gpio34outcfg: {:?}, gpio34intd: {:?}, gpio35incfg: {:?}, gpio35outcfg: {:?}, gpio35intd: {:?}, gpio36incfg: {:?}, gpio36outcfg: {:?}, gpio36intd: {:?}, gpio37incfg: {:?}, gpio37outcfg: {:?}, gpio37intd: {:?}, gpio38incfg: {:?}, gpio38outcfg: {:?}, gpio38intd: {:?}, gpio39incfg: {:?}, gpio39outcfg: {:?}, gpio39intd: {:?} }}" , self . gpio32incfg () , self . gpio32outcfg () , self . gpio32intd () , self . gpio33incfg () , self . gpio33outcfg () , self . gpio33intd () , self . gpio34incfg () , self . gpio34outcfg () , self . gpio34intd () , self . gpio35incfg () , self . gpio35outcfg () , self . gpio35intd () , self . gpio36incfg () , self . gpio36outcfg () , self . gpio36intd () , self . gpio37incfg () , self . gpio37outcfg () , self . gpio37intd () , self . gpio38incfg () , self . gpio38outcfg () , self . gpio38intd () , self . gpio39incfg () , self . gpio39outcfg () , self . gpio39intd ())
    }
}
#[doc = "GPIO Configuration Register F (Pads 40 -47)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgf(pub u32);
impl Cfgf {
    #[doc = "GPIO40 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio40incfg(&self) -> super::vals::Gpio40incfg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpio40incfg::from_bits(val as u8)
    }
    #[doc = "GPIO40 input enable."]
    #[inline(always)]
    pub const fn set_gpio40incfg(&mut self, val: super::vals::Gpio40incfg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO40 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio40outcfg(&self) -> super::vals::Gpio40outcfg {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Gpio40outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO40 output configuration."]
    #[inline(always)]
    pub const fn set_gpio40outcfg(&mut self, val: super::vals::Gpio40outcfg) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "GPIO40 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio40intd(&self) -> super::vals::Gpio40intd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gpio40intd::from_bits(val as u8)
    }
    #[doc = "GPIO40 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio40intd(&mut self, val: super::vals::Gpio40intd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO41 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio41incfg(&self) -> super::vals::Gpio41incfg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Gpio41incfg::from_bits(val as u8)
    }
    #[doc = "GPIO41 input enable."]
    #[inline(always)]
    pub const fn set_gpio41incfg(&mut self, val: super::vals::Gpio41incfg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO41 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio41outcfg(&self) -> super::vals::Gpio41outcfg {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Gpio41outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO41 output configuration."]
    #[inline(always)]
    pub const fn set_gpio41outcfg(&mut self, val: super::vals::Gpio41outcfg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "GPIO41 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio41intd(&self) -> super::vals::Gpio41intd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Gpio41intd::from_bits(val as u8)
    }
    #[doc = "GPIO41 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio41intd(&mut self, val: super::vals::Gpio41intd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO42 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio42incfg(&self) -> super::vals::Gpio42incfg {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Gpio42incfg::from_bits(val as u8)
    }
    #[doc = "GPIO42 input enable."]
    #[inline(always)]
    pub const fn set_gpio42incfg(&mut self, val: super::vals::Gpio42incfg) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO42 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio42outcfg(&self) -> super::vals::Gpio42outcfg {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Gpio42outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO42 output configuration."]
    #[inline(always)]
    pub const fn set_gpio42outcfg(&mut self, val: super::vals::Gpio42outcfg) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "GPIO42 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio42intd(&self) -> super::vals::Gpio42intd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Gpio42intd::from_bits(val as u8)
    }
    #[doc = "GPIO42 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio42intd(&mut self, val: super::vals::Gpio42intd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO43 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio43incfg(&self) -> super::vals::Gpio43incfg {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Gpio43incfg::from_bits(val as u8)
    }
    #[doc = "GPIO43 input enable."]
    #[inline(always)]
    pub const fn set_gpio43incfg(&mut self, val: super::vals::Gpio43incfg) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO43 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio43outcfg(&self) -> super::vals::Gpio43outcfg {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Gpio43outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO43 output configuration."]
    #[inline(always)]
    pub const fn set_gpio43outcfg(&mut self, val: super::vals::Gpio43outcfg) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "GPIO43 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio43intd(&self) -> super::vals::Gpio43intd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Gpio43intd::from_bits(val as u8)
    }
    #[doc = "GPIO43 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio43intd(&mut self, val: super::vals::Gpio43intd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO44 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio44incfg(&self) -> super::vals::Gpio44incfg {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpio44incfg::from_bits(val as u8)
    }
    #[doc = "GPIO44 input enable."]
    #[inline(always)]
    pub const fn set_gpio44incfg(&mut self, val: super::vals::Gpio44incfg) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO44 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio44outcfg(&self) -> super::vals::Gpio44outcfg {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Gpio44outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO44 output configuration."]
    #[inline(always)]
    pub const fn set_gpio44outcfg(&mut self, val: super::vals::Gpio44outcfg) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "GPIO44 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio44intd(&self) -> super::vals::Gpio44intd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpio44intd::from_bits(val as u8)
    }
    #[doc = "GPIO44 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio44intd(&mut self, val: super::vals::Gpio44intd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO45 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio45incfg(&self) -> super::vals::Gpio45incfg {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpio45incfg::from_bits(val as u8)
    }
    #[doc = "GPIO45 input enable."]
    #[inline(always)]
    pub const fn set_gpio45incfg(&mut self, val: super::vals::Gpio45incfg) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO45 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio45outcfg(&self) -> super::vals::Gpio45outcfg {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Gpio45outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO45 output configuration."]
    #[inline(always)]
    pub const fn set_gpio45outcfg(&mut self, val: super::vals::Gpio45outcfg) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "GPIO45 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio45intd(&self) -> super::vals::Gpio45intd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpio45intd::from_bits(val as u8)
    }
    #[doc = "GPIO45 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio45intd(&mut self, val: super::vals::Gpio45intd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO46 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio46incfg(&self) -> super::vals::Gpio46incfg {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpio46incfg::from_bits(val as u8)
    }
    #[doc = "GPIO46 input enable."]
    #[inline(always)]
    pub const fn set_gpio46incfg(&mut self, val: super::vals::Gpio46incfg) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO46 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio46outcfg(&self) -> super::vals::Gpio46outcfg {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Gpio46outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO46 output configuration."]
    #[inline(always)]
    pub const fn set_gpio46outcfg(&mut self, val: super::vals::Gpio46outcfg) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "GPIO46 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio46intd(&self) -> super::vals::Gpio46intd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Gpio46intd::from_bits(val as u8)
    }
    #[doc = "GPIO46 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio46intd(&mut self, val: super::vals::Gpio46intd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "GPIO47 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio47incfg(&self) -> super::vals::Gpio47incfg {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gpio47incfg::from_bits(val as u8)
    }
    #[doc = "GPIO47 input enable."]
    #[inline(always)]
    pub const fn set_gpio47incfg(&mut self, val: super::vals::Gpio47incfg) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO47 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio47outcfg(&self) -> super::vals::Gpio47outcfg {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Gpio47outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO47 output configuration."]
    #[inline(always)]
    pub const fn set_gpio47outcfg(&mut self, val: super::vals::Gpio47outcfg) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "GPIO47 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio47intd(&self) -> super::vals::Gpio47intd {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gpio47intd::from_bits(val as u8)
    }
    #[doc = "GPIO47 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio47intd(&mut self, val: super::vals::Gpio47intd) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cfgf {
    #[inline(always)]
    fn default() -> Cfgf {
        Cfgf(0)
    }
}
impl core::fmt::Debug for Cfgf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgf")
            .field("gpio40incfg", &self.gpio40incfg())
            .field("gpio40outcfg", &self.gpio40outcfg())
            .field("gpio40intd", &self.gpio40intd())
            .field("gpio41incfg", &self.gpio41incfg())
            .field("gpio41outcfg", &self.gpio41outcfg())
            .field("gpio41intd", &self.gpio41intd())
            .field("gpio42incfg", &self.gpio42incfg())
            .field("gpio42outcfg", &self.gpio42outcfg())
            .field("gpio42intd", &self.gpio42intd())
            .field("gpio43incfg", &self.gpio43incfg())
            .field("gpio43outcfg", &self.gpio43outcfg())
            .field("gpio43intd", &self.gpio43intd())
            .field("gpio44incfg", &self.gpio44incfg())
            .field("gpio44outcfg", &self.gpio44outcfg())
            .field("gpio44intd", &self.gpio44intd())
            .field("gpio45incfg", &self.gpio45incfg())
            .field("gpio45outcfg", &self.gpio45outcfg())
            .field("gpio45intd", &self.gpio45intd())
            .field("gpio46incfg", &self.gpio46incfg())
            .field("gpio46outcfg", &self.gpio46outcfg())
            .field("gpio46intd", &self.gpio46intd())
            .field("gpio47incfg", &self.gpio47incfg())
            .field("gpio47outcfg", &self.gpio47outcfg())
            .field("gpio47intd", &self.gpio47intd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfgf {{ gpio40incfg: {:?}, gpio40outcfg: {:?}, gpio40intd: {:?}, gpio41incfg: {:?}, gpio41outcfg: {:?}, gpio41intd: {:?}, gpio42incfg: {:?}, gpio42outcfg: {:?}, gpio42intd: {:?}, gpio43incfg: {:?}, gpio43outcfg: {:?}, gpio43intd: {:?}, gpio44incfg: {:?}, gpio44outcfg: {:?}, gpio44intd: {:?}, gpio45incfg: {:?}, gpio45outcfg: {:?}, gpio45intd: {:?}, gpio46incfg: {:?}, gpio46outcfg: {:?}, gpio46intd: {:?}, gpio47incfg: {:?}, gpio47outcfg: {:?}, gpio47intd: {:?} }}" , self . gpio40incfg () , self . gpio40outcfg () , self . gpio40intd () , self . gpio41incfg () , self . gpio41outcfg () , self . gpio41intd () , self . gpio42incfg () , self . gpio42outcfg () , self . gpio42intd () , self . gpio43incfg () , self . gpio43outcfg () , self . gpio43intd () , self . gpio44incfg () , self . gpio44outcfg () , self . gpio44intd () , self . gpio45incfg () , self . gpio45outcfg () , self . gpio45intd () , self . gpio46incfg () , self . gpio46outcfg () , self . gpio46intd () , self . gpio47incfg () , self . gpio47outcfg () , self . gpio47intd ())
    }
}
#[doc = "GPIO Configuration Register G (Pads 48-49)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgg(pub u32);
impl Cfgg {
    #[doc = "GPIO48 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio48incfg(&self) -> super::vals::Gpio48incfg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpio48incfg::from_bits(val as u8)
    }
    #[doc = "GPIO48 input enable."]
    #[inline(always)]
    pub const fn set_gpio48incfg(&mut self, val: super::vals::Gpio48incfg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO48 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio48outcfg(&self) -> super::vals::Gpio48outcfg {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Gpio48outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO48 output configuration."]
    #[inline(always)]
    pub const fn set_gpio48outcfg(&mut self, val: super::vals::Gpio48outcfg) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "GPIO48 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio48intd(&self) -> super::vals::Gpio48intd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gpio48intd::from_bits(val as u8)
    }
    #[doc = "GPIO48 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio48intd(&mut self, val: super::vals::Gpio48intd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO49 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio49incfg(&self) -> super::vals::Gpio49incfg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Gpio49incfg::from_bits(val as u8)
    }
    #[doc = "GPIO49 input enable."]
    #[inline(always)]
    pub const fn set_gpio49incfg(&mut self, val: super::vals::Gpio49incfg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO49 output configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio49outcfg(&self) -> super::vals::Gpio49outcfg {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Gpio49outcfg::from_bits(val as u8)
    }
    #[doc = "GPIO49 output configuration."]
    #[inline(always)]
    pub const fn set_gpio49outcfg(&mut self, val: super::vals::Gpio49outcfg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "GPIO49 interrupt direction."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio49intd(&self) -> super::vals::Gpio49intd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Gpio49intd::from_bits(val as u8)
    }
    #[doc = "GPIO49 interrupt direction."]
    #[inline(always)]
    pub const fn set_gpio49intd(&mut self, val: super::vals::Gpio49intd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Cfgg {
    #[inline(always)]
    fn default() -> Cfgg {
        Cfgg(0)
    }
}
impl core::fmt::Debug for Cfgg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgg")
            .field("gpio48incfg", &self.gpio48incfg())
            .field("gpio48outcfg", &self.gpio48outcfg())
            .field("gpio48intd", &self.gpio48intd())
            .field("gpio49incfg", &self.gpio49incfg())
            .field("gpio49outcfg", &self.gpio49outcfg())
            .field("gpio49intd", &self.gpio49intd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfgg {{ gpio48incfg: {:?}, gpio48outcfg: {:?}, gpio48intd: {:?}, gpio49incfg: {:?}, gpio49outcfg: {:?}, gpio49intd: {:?} }}" , self . gpio48incfg () , self . gpio48outcfg () , self . gpio48intd () , self . gpio49incfg () , self . gpio49outcfg () , self . gpio49intd ())
    }
}
#[doc = "Counter/Timer Enable Config."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctencfg(pub u32);
impl Ctencfg {
    #[doc = "CT0 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en0(&self) -> super::vals::En0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::En0::from_bits(val as u8)
    }
    #[doc = "CT0 Enable."]
    #[inline(always)]
    pub const fn set_en0(&mut self, val: super::vals::En0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CT1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en1(&self) -> super::vals::En1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::En1::from_bits(val as u8)
    }
    #[doc = "CT1 Enable."]
    #[inline(always)]
    pub const fn set_en1(&mut self, val: super::vals::En1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CT2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en2(&self) -> super::vals::En2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::En2::from_bits(val as u8)
    }
    #[doc = "CT2 Enable."]
    #[inline(always)]
    pub const fn set_en2(&mut self, val: super::vals::En2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CT3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en3(&self) -> super::vals::En3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::En3::from_bits(val as u8)
    }
    #[doc = "CT3 Enable."]
    #[inline(always)]
    pub const fn set_en3(&mut self, val: super::vals::En3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "CT4 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en4(&self) -> super::vals::En4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::En4::from_bits(val as u8)
    }
    #[doc = "CT4 Enable."]
    #[inline(always)]
    pub const fn set_en4(&mut self, val: super::vals::En4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "CT5 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en5(&self) -> super::vals::En5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::En5::from_bits(val as u8)
    }
    #[doc = "CT5 Enable."]
    #[inline(always)]
    pub const fn set_en5(&mut self, val: super::vals::En5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "CT6 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en6(&self) -> super::vals::En6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::En6::from_bits(val as u8)
    }
    #[doc = "CT6 Enable."]
    #[inline(always)]
    pub const fn set_en6(&mut self, val: super::vals::En6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "CT7 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en7(&self) -> super::vals::En7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::En7::from_bits(val as u8)
    }
    #[doc = "CT7 Enable."]
    #[inline(always)]
    pub const fn set_en7(&mut self, val: super::vals::En7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CT8 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en8(&self) -> super::vals::En8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::En8::from_bits(val as u8)
    }
    #[doc = "CT8 Enable."]
    #[inline(always)]
    pub const fn set_en8(&mut self, val: super::vals::En8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CT9 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CT9 Enable."]
    #[inline(always)]
    pub const fn set_en9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CT10 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en10(&self) -> super::vals::En10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::En10::from_bits(val as u8)
    }
    #[doc = "CT10 Enable."]
    #[inline(always)]
    pub const fn set_en10(&mut self, val: super::vals::En10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "CT11 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en11(&self) -> super::vals::En11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::En11::from_bits(val as u8)
    }
    #[doc = "CT11 Enable."]
    #[inline(always)]
    pub const fn set_en11(&mut self, val: super::vals::En11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "CT12 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en12(&self) -> super::vals::En12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::En12::from_bits(val as u8)
    }
    #[doc = "CT12 Enable."]
    #[inline(always)]
    pub const fn set_en12(&mut self, val: super::vals::En12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "CT13 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en13(&self) -> super::vals::En13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::En13::from_bits(val as u8)
    }
    #[doc = "CT13 Enable."]
    #[inline(always)]
    pub const fn set_en13(&mut self, val: super::vals::En13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "CT14 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en14(&self) -> super::vals::En14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::En14::from_bits(val as u8)
    }
    #[doc = "CT14 Enable."]
    #[inline(always)]
    pub const fn set_en14(&mut self, val: super::vals::En14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "CT15 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en15(&self) -> super::vals::En15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::En15::from_bits(val as u8)
    }
    #[doc = "CT15 Enable."]
    #[inline(always)]
    pub const fn set_en15(&mut self, val: super::vals::En15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "CT16 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en16(&self) -> super::vals::En16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::En16::from_bits(val as u8)
    }
    #[doc = "CT16 Enable."]
    #[inline(always)]
    pub const fn set_en16(&mut self, val: super::vals::En16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "CT17 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en17(&self) -> super::vals::En17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::En17::from_bits(val as u8)
    }
    #[doc = "CT17 Enable."]
    #[inline(always)]
    pub const fn set_en17(&mut self, val: super::vals::En17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "CT18 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en18(&self) -> super::vals::En18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::En18::from_bits(val as u8)
    }
    #[doc = "CT18 Enable."]
    #[inline(always)]
    pub const fn set_en18(&mut self, val: super::vals::En18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "CT19 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en19(&self) -> super::vals::En19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::En19::from_bits(val as u8)
    }
    #[doc = "CT19 Enable."]
    #[inline(always)]
    pub const fn set_en19(&mut self, val: super::vals::En19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "CT20 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en20(&self) -> super::vals::En20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::En20::from_bits(val as u8)
    }
    #[doc = "CT20 Enable."]
    #[inline(always)]
    pub const fn set_en20(&mut self, val: super::vals::En20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "CT21 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en21(&self) -> super::vals::En21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::En21::from_bits(val as u8)
    }
    #[doc = "CT21 Enable."]
    #[inline(always)]
    pub const fn set_en21(&mut self, val: super::vals::En21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "CT22 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en22(&self) -> super::vals::En22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::En22::from_bits(val as u8)
    }
    #[doc = "CT22 Enable."]
    #[inline(always)]
    pub const fn set_en22(&mut self, val: super::vals::En22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "CT23 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en23(&self) -> super::vals::En23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::En23::from_bits(val as u8)
    }
    #[doc = "CT23 Enable."]
    #[inline(always)]
    pub const fn set_en23(&mut self, val: super::vals::En23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "CT24 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en24(&self) -> super::vals::En24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::En24::from_bits(val as u8)
    }
    #[doc = "CT24 Enable."]
    #[inline(always)]
    pub const fn set_en24(&mut self, val: super::vals::En24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "CT25 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en25(&self) -> super::vals::En25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::En25::from_bits(val as u8)
    }
    #[doc = "CT25 Enable."]
    #[inline(always)]
    pub const fn set_en25(&mut self, val: super::vals::En25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "CT26 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en26(&self) -> super::vals::En26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::En26::from_bits(val as u8)
    }
    #[doc = "CT26 Enable."]
    #[inline(always)]
    pub const fn set_en26(&mut self, val: super::vals::En26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "CT27 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en27(&self) -> super::vals::En27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::En27::from_bits(val as u8)
    }
    #[doc = "CT27 Enable."]
    #[inline(always)]
    pub const fn set_en27(&mut self, val: super::vals::En27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "CT28 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en28(&self) -> super::vals::En28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::En28::from_bits(val as u8)
    }
    #[doc = "CT28 Enable."]
    #[inline(always)]
    pub const fn set_en28(&mut self, val: super::vals::En28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "CT29 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en29(&self) -> super::vals::En29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::En29::from_bits(val as u8)
    }
    #[doc = "CT29 Enable."]
    #[inline(always)]
    pub const fn set_en29(&mut self, val: super::vals::En29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "CT30 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en30(&self) -> super::vals::En30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::En30::from_bits(val as u8)
    }
    #[doc = "CT30 Enable."]
    #[inline(always)]
    pub const fn set_en30(&mut self, val: super::vals::En30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "CT31 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en31(&self) -> super::vals::En31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::En31::from_bits(val as u8)
    }
    #[doc = "CT31 Enable."]
    #[inline(always)]
    pub const fn set_en31(&mut self, val: super::vals::En31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctencfg {
    #[inline(always)]
    fn default() -> Ctencfg {
        Ctencfg(0)
    }
}
impl core::fmt::Debug for Ctencfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctencfg")
            .field("en0", &self.en0())
            .field("en1", &self.en1())
            .field("en2", &self.en2())
            .field("en3", &self.en3())
            .field("en4", &self.en4())
            .field("en5", &self.en5())
            .field("en6", &self.en6())
            .field("en7", &self.en7())
            .field("en8", &self.en8())
            .field("en9", &self.en9())
            .field("en10", &self.en10())
            .field("en11", &self.en11())
            .field("en12", &self.en12())
            .field("en13", &self.en13())
            .field("en14", &self.en14())
            .field("en15", &self.en15())
            .field("en16", &self.en16())
            .field("en17", &self.en17())
            .field("en18", &self.en18())
            .field("en19", &self.en19())
            .field("en20", &self.en20())
            .field("en21", &self.en21())
            .field("en22", &self.en22())
            .field("en23", &self.en23())
            .field("en24", &self.en24())
            .field("en25", &self.en25())
            .field("en26", &self.en26())
            .field("en27", &self.en27())
            .field("en28", &self.en28())
            .field("en29", &self.en29())
            .field("en30", &self.en30())
            .field("en31", &self.en31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctencfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctencfg {{ en0: {:?}, en1: {:?}, en2: {:?}, en3: {:?}, en4: {:?}, en5: {:?}, en6: {:?}, en7: {:?}, en8: {:?}, en9: {=bool:?}, en10: {:?}, en11: {:?}, en12: {:?}, en13: {:?}, en14: {:?}, en15: {:?}, en16: {:?}, en17: {:?}, en18: {:?}, en19: {:?}, en20: {:?}, en21: {:?}, en22: {:?}, en23: {:?}, en24: {:?}, en25: {:?}, en26: {:?}, en27: {:?}, en28: {:?}, en29: {:?}, en30: {:?}, en31: {:?} }}" , self . en0 () , self . en1 () , self . en2 () , self . en3 () , self . en4 () , self . en5 () , self . en6 () , self . en7 () , self . en8 () , self . en9 () , self . en10 () , self . en11 () , self . en12 () , self . en13 () , self . en14 () , self . en15 () , self . en16 () , self . en17 () , self . en18 () , self . en19 () , self . en20 () , self . en21 () , self . en22 () , self . en23 () , self . en24 () , self . en25 () , self . en26 () , self . en27 () , self . en28 () , self . en29 () , self . en30 () , self . en31 ())
    }
}
#[doc = "GPIO Enable Register B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enb(pub u32);
impl Enb {
    #[doc = "GPIO49-32 output enables."]
    #[must_use]
    #[inline(always)]
    pub const fn enb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "GPIO49-32 output enables."]
    #[inline(always)]
    pub const fn set_enb(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Enb {
    #[inline(always)]
    fn default() -> Enb {
        Enb(0)
    }
}
impl core::fmt::Debug for Enb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enb").field("enb", &self.enb()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enb {{ enb: {=u32:?} }}", self.enb())
    }
}
#[doc = "GPIO Enable Register B Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Encb(pub u32);
impl Encb {
    #[doc = "Clear the GPIO49-32 output enables."]
    #[must_use]
    #[inline(always)]
    pub const fn encb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Clear the GPIO49-32 output enables."]
    #[inline(always)]
    pub const fn set_encb(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Encb {
    #[inline(always)]
    fn default() -> Encb {
        Encb(0)
    }
}
impl core::fmt::Debug for Encb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Encb").field("encb", &self.encb()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Encb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Encb {{ encb: {=u32:?} }}", self.encb())
    }
}
#[doc = "GPIO Enable Register B Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ensb(pub u32);
impl Ensb {
    #[doc = "Set the GPIO49-32 output enables."]
    #[must_use]
    #[inline(always)]
    pub const fn ensb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Set the GPIO49-32 output enables."]
    #[inline(always)]
    pub const fn set_ensb(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Ensb {
    #[inline(always)]
    fn default() -> Ensb {
        Ensb(0)
    }
}
impl core::fmt::Debug for Ensb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ensb").field("ensb", &self.ensb()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ensb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ensb {{ ensb: {=u32:?} }}", self.ensb())
    }
}
#[doc = "GPIO Observation Mode Sample register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpioobs(pub u32);
impl Gpioobs {
    #[doc = "Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only."]
    #[must_use]
    #[inline(always)]
    pub const fn obs_data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only."]
    #[inline(always)]
    pub const fn set_obs_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Gpioobs {
    #[inline(always)]
    fn default() -> Gpioobs {
        Gpioobs(0)
    }
}
impl core::fmt::Debug for Gpioobs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpioobs")
            .field("obs_data", &self.obs_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpioobs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpioobs {{ obs_data: {=u16:?} }}", self.obs_data())
    }
}
#[doc = "GPIO Interrupt Registers 31-0: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int0clr(pub u32);
impl Int0clr {
    #[doc = "GPIO0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0 interrupt."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1 interrupt."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO2 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2 interrupt."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO3 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3 interrupt."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO4 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4 interrupt."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO5 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO5 interrupt."]
    #[inline(always)]
    pub const fn set_gpio5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO6 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO6 interrupt."]
    #[inline(always)]
    pub const fn set_gpio6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO7 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO7 interrupt."]
    #[inline(always)]
    pub const fn set_gpio7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO8 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO8 interrupt."]
    #[inline(always)]
    pub const fn set_gpio8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO9 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO9 interrupt."]
    #[inline(always)]
    pub const fn set_gpio9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GPIO10 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO10 interrupt."]
    #[inline(always)]
    pub const fn set_gpio10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GPIO11 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO11 interrupt."]
    #[inline(always)]
    pub const fn set_gpio11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO12 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO12 interrupt."]
    #[inline(always)]
    pub const fn set_gpio12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO13 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO13 interrupt."]
    #[inline(always)]
    pub const fn set_gpio13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO14 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO14 interrupt."]
    #[inline(always)]
    pub const fn set_gpio14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO15 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO15 interrupt."]
    #[inline(always)]
    pub const fn set_gpio15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO16 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO16 interrupt."]
    #[inline(always)]
    pub const fn set_gpio16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO17 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO17 interrupt."]
    #[inline(always)]
    pub const fn set_gpio17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPIO18interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO18interrupt."]
    #[inline(always)]
    pub const fn set_gpio18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "GPIO19 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO19 interrupt."]
    #[inline(always)]
    pub const fn set_gpio19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO20 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO20 interrupt."]
    #[inline(always)]
    pub const fn set_gpio20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO21 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO21 interrupt."]
    #[inline(always)]
    pub const fn set_gpio21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO22 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO22 interrupt."]
    #[inline(always)]
    pub const fn set_gpio22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO23 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO23 interrupt."]
    #[inline(always)]
    pub const fn set_gpio23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO24 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO24 interrupt."]
    #[inline(always)]
    pub const fn set_gpio24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO25 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO25 interrupt."]
    #[inline(always)]
    pub const fn set_gpio25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "GPIO26 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO26 interrupt."]
    #[inline(always)]
    pub const fn set_gpio26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "GPIO27 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO27 interrupt."]
    #[inline(always)]
    pub const fn set_gpio27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "GPIO28 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO28 interrupt."]
    #[inline(always)]
    pub const fn set_gpio28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO29 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO29 interrupt."]
    #[inline(always)]
    pub const fn set_gpio29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "GPIO30 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO30 interrupt."]
    #[inline(always)]
    pub const fn set_gpio30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "GPIO31 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO31 interrupt."]
    #[inline(always)]
    pub const fn set_gpio31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Int0clr {
    #[inline(always)]
    fn default() -> Int0clr {
        Int0clr(0)
    }
}
impl core::fmt::Debug for Int0clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int0clr")
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("gpio5", &self.gpio5())
            .field("gpio6", &self.gpio6())
            .field("gpio7", &self.gpio7())
            .field("gpio8", &self.gpio8())
            .field("gpio9", &self.gpio9())
            .field("gpio10", &self.gpio10())
            .field("gpio11", &self.gpio11())
            .field("gpio12", &self.gpio12())
            .field("gpio13", &self.gpio13())
            .field("gpio14", &self.gpio14())
            .field("gpio15", &self.gpio15())
            .field("gpio16", &self.gpio16())
            .field("gpio17", &self.gpio17())
            .field("gpio18", &self.gpio18())
            .field("gpio19", &self.gpio19())
            .field("gpio20", &self.gpio20())
            .field("gpio21", &self.gpio21())
            .field("gpio22", &self.gpio22())
            .field("gpio23", &self.gpio23())
            .field("gpio24", &self.gpio24())
            .field("gpio25", &self.gpio25())
            .field("gpio26", &self.gpio26())
            .field("gpio27", &self.gpio27())
            .field("gpio28", &self.gpio28())
            .field("gpio29", &self.gpio29())
            .field("gpio30", &self.gpio30())
            .field("gpio31", &self.gpio31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int0clr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int0clr {{ gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, gpio5: {=bool:?}, gpio6: {=bool:?}, gpio7: {=bool:?}, gpio8: {=bool:?}, gpio9: {=bool:?}, gpio10: {=bool:?}, gpio11: {=bool:?}, gpio12: {=bool:?}, gpio13: {=bool:?}, gpio14: {=bool:?}, gpio15: {=bool:?}, gpio16: {=bool:?}, gpio17: {=bool:?}, gpio18: {=bool:?}, gpio19: {=bool:?}, gpio20: {=bool:?}, gpio21: {=bool:?}, gpio22: {=bool:?}, gpio23: {=bool:?}, gpio24: {=bool:?}, gpio25: {=bool:?}, gpio26: {=bool:?}, gpio27: {=bool:?}, gpio28: {=bool:?}, gpio29: {=bool:?}, gpio30: {=bool:?}, gpio31: {=bool:?} }}" , self . gpio0 () , self . gpio1 () , self . gpio2 () , self . gpio3 () , self . gpio4 () , self . gpio5 () , self . gpio6 () , self . gpio7 () , self . gpio8 () , self . gpio9 () , self . gpio10 () , self . gpio11 () , self . gpio12 () , self . gpio13 () , self . gpio14 () , self . gpio15 () , self . gpio16 () , self . gpio17 () , self . gpio18 () , self . gpio19 () , self . gpio20 () , self . gpio21 () , self . gpio22 () , self . gpio23 () , self . gpio24 () , self . gpio25 () , self . gpio26 () , self . gpio27 () , self . gpio28 () , self . gpio29 () , self . gpio30 () , self . gpio31 ())
    }
}
#[doc = "GPIO Interrupt Registers 31-0: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int0en(pub u32);
impl Int0en {
    #[doc = "GPIO0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0 interrupt."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1 interrupt."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO2 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2 interrupt."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO3 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3 interrupt."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO4 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4 interrupt."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO5 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO5 interrupt."]
    #[inline(always)]
    pub const fn set_gpio5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO6 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO6 interrupt."]
    #[inline(always)]
    pub const fn set_gpio6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO7 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO7 interrupt."]
    #[inline(always)]
    pub const fn set_gpio7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO8 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO8 interrupt."]
    #[inline(always)]
    pub const fn set_gpio8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO9 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO9 interrupt."]
    #[inline(always)]
    pub const fn set_gpio9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GPIO10 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO10 interrupt."]
    #[inline(always)]
    pub const fn set_gpio10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GPIO11 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO11 interrupt."]
    #[inline(always)]
    pub const fn set_gpio11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO12 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO12 interrupt."]
    #[inline(always)]
    pub const fn set_gpio12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO13 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO13 interrupt."]
    #[inline(always)]
    pub const fn set_gpio13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO14 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO14 interrupt."]
    #[inline(always)]
    pub const fn set_gpio14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO15 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO15 interrupt."]
    #[inline(always)]
    pub const fn set_gpio15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO16 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO16 interrupt."]
    #[inline(always)]
    pub const fn set_gpio16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO17 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO17 interrupt."]
    #[inline(always)]
    pub const fn set_gpio17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPIO18interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO18interrupt."]
    #[inline(always)]
    pub const fn set_gpio18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "GPIO19 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO19 interrupt."]
    #[inline(always)]
    pub const fn set_gpio19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO20 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO20 interrupt."]
    #[inline(always)]
    pub const fn set_gpio20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO21 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO21 interrupt."]
    #[inline(always)]
    pub const fn set_gpio21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO22 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO22 interrupt."]
    #[inline(always)]
    pub const fn set_gpio22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO23 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO23 interrupt."]
    #[inline(always)]
    pub const fn set_gpio23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO24 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO24 interrupt."]
    #[inline(always)]
    pub const fn set_gpio24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO25 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO25 interrupt."]
    #[inline(always)]
    pub const fn set_gpio25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "GPIO26 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO26 interrupt."]
    #[inline(always)]
    pub const fn set_gpio26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "GPIO27 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO27 interrupt."]
    #[inline(always)]
    pub const fn set_gpio27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "GPIO28 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO28 interrupt."]
    #[inline(always)]
    pub const fn set_gpio28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO29 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO29 interrupt."]
    #[inline(always)]
    pub const fn set_gpio29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "GPIO30 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO30 interrupt."]
    #[inline(always)]
    pub const fn set_gpio30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "GPIO31 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO31 interrupt."]
    #[inline(always)]
    pub const fn set_gpio31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Int0en {
    #[inline(always)]
    fn default() -> Int0en {
        Int0en(0)
    }
}
impl core::fmt::Debug for Int0en {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int0en")
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("gpio5", &self.gpio5())
            .field("gpio6", &self.gpio6())
            .field("gpio7", &self.gpio7())
            .field("gpio8", &self.gpio8())
            .field("gpio9", &self.gpio9())
            .field("gpio10", &self.gpio10())
            .field("gpio11", &self.gpio11())
            .field("gpio12", &self.gpio12())
            .field("gpio13", &self.gpio13())
            .field("gpio14", &self.gpio14())
            .field("gpio15", &self.gpio15())
            .field("gpio16", &self.gpio16())
            .field("gpio17", &self.gpio17())
            .field("gpio18", &self.gpio18())
            .field("gpio19", &self.gpio19())
            .field("gpio20", &self.gpio20())
            .field("gpio21", &self.gpio21())
            .field("gpio22", &self.gpio22())
            .field("gpio23", &self.gpio23())
            .field("gpio24", &self.gpio24())
            .field("gpio25", &self.gpio25())
            .field("gpio26", &self.gpio26())
            .field("gpio27", &self.gpio27())
            .field("gpio28", &self.gpio28())
            .field("gpio29", &self.gpio29())
            .field("gpio30", &self.gpio30())
            .field("gpio31", &self.gpio31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int0en {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int0en {{ gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, gpio5: {=bool:?}, gpio6: {=bool:?}, gpio7: {=bool:?}, gpio8: {=bool:?}, gpio9: {=bool:?}, gpio10: {=bool:?}, gpio11: {=bool:?}, gpio12: {=bool:?}, gpio13: {=bool:?}, gpio14: {=bool:?}, gpio15: {=bool:?}, gpio16: {=bool:?}, gpio17: {=bool:?}, gpio18: {=bool:?}, gpio19: {=bool:?}, gpio20: {=bool:?}, gpio21: {=bool:?}, gpio22: {=bool:?}, gpio23: {=bool:?}, gpio24: {=bool:?}, gpio25: {=bool:?}, gpio26: {=bool:?}, gpio27: {=bool:?}, gpio28: {=bool:?}, gpio29: {=bool:?}, gpio30: {=bool:?}, gpio31: {=bool:?} }}" , self . gpio0 () , self . gpio1 () , self . gpio2 () , self . gpio3 () , self . gpio4 () , self . gpio5 () , self . gpio6 () , self . gpio7 () , self . gpio8 () , self . gpio9 () , self . gpio10 () , self . gpio11 () , self . gpio12 () , self . gpio13 () , self . gpio14 () , self . gpio15 () , self . gpio16 () , self . gpio17 () , self . gpio18 () , self . gpio19 () , self . gpio20 () , self . gpio21 () , self . gpio22 () , self . gpio23 () , self . gpio24 () , self . gpio25 () , self . gpio26 () , self . gpio27 () , self . gpio28 () , self . gpio29 () , self . gpio30 () , self . gpio31 ())
    }
}
#[doc = "GPIO Interrupt Registers 31-0: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int0set(pub u32);
impl Int0set {
    #[doc = "GPIO0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0 interrupt."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1 interrupt."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO2 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2 interrupt."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO3 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3 interrupt."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO4 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4 interrupt."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO5 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO5 interrupt."]
    #[inline(always)]
    pub const fn set_gpio5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO6 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO6 interrupt."]
    #[inline(always)]
    pub const fn set_gpio6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO7 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO7 interrupt."]
    #[inline(always)]
    pub const fn set_gpio7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO8 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO8 interrupt."]
    #[inline(always)]
    pub const fn set_gpio8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO9 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO9 interrupt."]
    #[inline(always)]
    pub const fn set_gpio9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GPIO10 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO10 interrupt."]
    #[inline(always)]
    pub const fn set_gpio10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GPIO11 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO11 interrupt."]
    #[inline(always)]
    pub const fn set_gpio11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO12 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO12 interrupt."]
    #[inline(always)]
    pub const fn set_gpio12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO13 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO13 interrupt."]
    #[inline(always)]
    pub const fn set_gpio13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO14 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO14 interrupt."]
    #[inline(always)]
    pub const fn set_gpio14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO15 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO15 interrupt."]
    #[inline(always)]
    pub const fn set_gpio15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO16 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO16 interrupt."]
    #[inline(always)]
    pub const fn set_gpio16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO17 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO17 interrupt."]
    #[inline(always)]
    pub const fn set_gpio17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPIO18interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO18interrupt."]
    #[inline(always)]
    pub const fn set_gpio18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "GPIO19 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO19 interrupt."]
    #[inline(always)]
    pub const fn set_gpio19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO20 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO20 interrupt."]
    #[inline(always)]
    pub const fn set_gpio20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO21 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO21 interrupt."]
    #[inline(always)]
    pub const fn set_gpio21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO22 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO22 interrupt."]
    #[inline(always)]
    pub const fn set_gpio22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO23 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO23 interrupt."]
    #[inline(always)]
    pub const fn set_gpio23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO24 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO24 interrupt."]
    #[inline(always)]
    pub const fn set_gpio24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO25 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO25 interrupt."]
    #[inline(always)]
    pub const fn set_gpio25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "GPIO26 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO26 interrupt."]
    #[inline(always)]
    pub const fn set_gpio26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "GPIO27 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO27 interrupt."]
    #[inline(always)]
    pub const fn set_gpio27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "GPIO28 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO28 interrupt."]
    #[inline(always)]
    pub const fn set_gpio28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO29 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO29 interrupt."]
    #[inline(always)]
    pub const fn set_gpio29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "GPIO30 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO30 interrupt."]
    #[inline(always)]
    pub const fn set_gpio30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "GPIO31 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO31 interrupt."]
    #[inline(always)]
    pub const fn set_gpio31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Int0set {
    #[inline(always)]
    fn default() -> Int0set {
        Int0set(0)
    }
}
impl core::fmt::Debug for Int0set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int0set")
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("gpio5", &self.gpio5())
            .field("gpio6", &self.gpio6())
            .field("gpio7", &self.gpio7())
            .field("gpio8", &self.gpio8())
            .field("gpio9", &self.gpio9())
            .field("gpio10", &self.gpio10())
            .field("gpio11", &self.gpio11())
            .field("gpio12", &self.gpio12())
            .field("gpio13", &self.gpio13())
            .field("gpio14", &self.gpio14())
            .field("gpio15", &self.gpio15())
            .field("gpio16", &self.gpio16())
            .field("gpio17", &self.gpio17())
            .field("gpio18", &self.gpio18())
            .field("gpio19", &self.gpio19())
            .field("gpio20", &self.gpio20())
            .field("gpio21", &self.gpio21())
            .field("gpio22", &self.gpio22())
            .field("gpio23", &self.gpio23())
            .field("gpio24", &self.gpio24())
            .field("gpio25", &self.gpio25())
            .field("gpio26", &self.gpio26())
            .field("gpio27", &self.gpio27())
            .field("gpio28", &self.gpio28())
            .field("gpio29", &self.gpio29())
            .field("gpio30", &self.gpio30())
            .field("gpio31", &self.gpio31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int0set {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int0set {{ gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, gpio5: {=bool:?}, gpio6: {=bool:?}, gpio7: {=bool:?}, gpio8: {=bool:?}, gpio9: {=bool:?}, gpio10: {=bool:?}, gpio11: {=bool:?}, gpio12: {=bool:?}, gpio13: {=bool:?}, gpio14: {=bool:?}, gpio15: {=bool:?}, gpio16: {=bool:?}, gpio17: {=bool:?}, gpio18: {=bool:?}, gpio19: {=bool:?}, gpio20: {=bool:?}, gpio21: {=bool:?}, gpio22: {=bool:?}, gpio23: {=bool:?}, gpio24: {=bool:?}, gpio25: {=bool:?}, gpio26: {=bool:?}, gpio27: {=bool:?}, gpio28: {=bool:?}, gpio29: {=bool:?}, gpio30: {=bool:?}, gpio31: {=bool:?} }}" , self . gpio0 () , self . gpio1 () , self . gpio2 () , self . gpio3 () , self . gpio4 () , self . gpio5 () , self . gpio6 () , self . gpio7 () , self . gpio8 () , self . gpio9 () , self . gpio10 () , self . gpio11 () , self . gpio12 () , self . gpio13 () , self . gpio14 () , self . gpio15 () , self . gpio16 () , self . gpio17 () , self . gpio18 () , self . gpio19 () , self . gpio20 () , self . gpio21 () , self . gpio22 () , self . gpio23 () , self . gpio24 () , self . gpio25 () , self . gpio26 () , self . gpio27 () , self . gpio28 () , self . gpio29 () , self . gpio30 () , self . gpio31 ())
    }
}
#[doc = "GPIO Interrupt Registers 31-0: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int0stat(pub u32);
impl Int0stat {
    #[doc = "GPIO0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0 interrupt."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1 interrupt."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO2 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2 interrupt."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO3 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3 interrupt."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO4 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4 interrupt."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO5 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO5 interrupt."]
    #[inline(always)]
    pub const fn set_gpio5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO6 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO6 interrupt."]
    #[inline(always)]
    pub const fn set_gpio6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO7 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO7 interrupt."]
    #[inline(always)]
    pub const fn set_gpio7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO8 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO8 interrupt."]
    #[inline(always)]
    pub const fn set_gpio8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO9 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO9 interrupt."]
    #[inline(always)]
    pub const fn set_gpio9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GPIO10 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO10 interrupt."]
    #[inline(always)]
    pub const fn set_gpio10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GPIO11 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO11 interrupt."]
    #[inline(always)]
    pub const fn set_gpio11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO12 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO12 interrupt."]
    #[inline(always)]
    pub const fn set_gpio12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO13 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO13 interrupt."]
    #[inline(always)]
    pub const fn set_gpio13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO14 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO14 interrupt."]
    #[inline(always)]
    pub const fn set_gpio14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO15 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO15 interrupt."]
    #[inline(always)]
    pub const fn set_gpio15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO16 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO16 interrupt."]
    #[inline(always)]
    pub const fn set_gpio16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO17 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO17 interrupt."]
    #[inline(always)]
    pub const fn set_gpio17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPIO18interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO18interrupt."]
    #[inline(always)]
    pub const fn set_gpio18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "GPIO19 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO19 interrupt."]
    #[inline(always)]
    pub const fn set_gpio19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO20 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO20 interrupt."]
    #[inline(always)]
    pub const fn set_gpio20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO21 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO21 interrupt."]
    #[inline(always)]
    pub const fn set_gpio21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO22 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO22 interrupt."]
    #[inline(always)]
    pub const fn set_gpio22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO23 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO23 interrupt."]
    #[inline(always)]
    pub const fn set_gpio23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO24 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO24 interrupt."]
    #[inline(always)]
    pub const fn set_gpio24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO25 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO25 interrupt."]
    #[inline(always)]
    pub const fn set_gpio25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "GPIO26 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO26 interrupt."]
    #[inline(always)]
    pub const fn set_gpio26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "GPIO27 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO27 interrupt."]
    #[inline(always)]
    pub const fn set_gpio27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "GPIO28 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO28 interrupt."]
    #[inline(always)]
    pub const fn set_gpio28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO29 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO29 interrupt."]
    #[inline(always)]
    pub const fn set_gpio29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "GPIO30 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO30 interrupt."]
    #[inline(always)]
    pub const fn set_gpio30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "GPIO31 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO31 interrupt."]
    #[inline(always)]
    pub const fn set_gpio31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Int0stat {
    #[inline(always)]
    fn default() -> Int0stat {
        Int0stat(0)
    }
}
impl core::fmt::Debug for Int0stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int0stat")
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("gpio5", &self.gpio5())
            .field("gpio6", &self.gpio6())
            .field("gpio7", &self.gpio7())
            .field("gpio8", &self.gpio8())
            .field("gpio9", &self.gpio9())
            .field("gpio10", &self.gpio10())
            .field("gpio11", &self.gpio11())
            .field("gpio12", &self.gpio12())
            .field("gpio13", &self.gpio13())
            .field("gpio14", &self.gpio14())
            .field("gpio15", &self.gpio15())
            .field("gpio16", &self.gpio16())
            .field("gpio17", &self.gpio17())
            .field("gpio18", &self.gpio18())
            .field("gpio19", &self.gpio19())
            .field("gpio20", &self.gpio20())
            .field("gpio21", &self.gpio21())
            .field("gpio22", &self.gpio22())
            .field("gpio23", &self.gpio23())
            .field("gpio24", &self.gpio24())
            .field("gpio25", &self.gpio25())
            .field("gpio26", &self.gpio26())
            .field("gpio27", &self.gpio27())
            .field("gpio28", &self.gpio28())
            .field("gpio29", &self.gpio29())
            .field("gpio30", &self.gpio30())
            .field("gpio31", &self.gpio31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int0stat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int0stat {{ gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, gpio5: {=bool:?}, gpio6: {=bool:?}, gpio7: {=bool:?}, gpio8: {=bool:?}, gpio9: {=bool:?}, gpio10: {=bool:?}, gpio11: {=bool:?}, gpio12: {=bool:?}, gpio13: {=bool:?}, gpio14: {=bool:?}, gpio15: {=bool:?}, gpio16: {=bool:?}, gpio17: {=bool:?}, gpio18: {=bool:?}, gpio19: {=bool:?}, gpio20: {=bool:?}, gpio21: {=bool:?}, gpio22: {=bool:?}, gpio23: {=bool:?}, gpio24: {=bool:?}, gpio25: {=bool:?}, gpio26: {=bool:?}, gpio27: {=bool:?}, gpio28: {=bool:?}, gpio29: {=bool:?}, gpio30: {=bool:?}, gpio31: {=bool:?} }}" , self . gpio0 () , self . gpio1 () , self . gpio2 () , self . gpio3 () , self . gpio4 () , self . gpio5 () , self . gpio6 () , self . gpio7 () , self . gpio8 () , self . gpio9 () , self . gpio10 () , self . gpio11 () , self . gpio12 () , self . gpio13 () , self . gpio14 () , self . gpio15 () , self . gpio16 () , self . gpio17 () , self . gpio18 () , self . gpio19 () , self . gpio20 () , self . gpio21 () , self . gpio22 () , self . gpio23 () , self . gpio24 () , self . gpio25 () , self . gpio26 () , self . gpio27 () , self . gpio28 () , self . gpio29 () , self . gpio30 () , self . gpio31 ())
    }
}
#[doc = "GPIO Interrupt Registers 49-32: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int1clr(pub u32);
impl Int1clr {
    #[doc = "GPIO32 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO32 interrupt."]
    #[inline(always)]
    pub const fn set_gpio32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO33 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO33 interrupt."]
    #[inline(always)]
    pub const fn set_gpio33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO34 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO34 interrupt."]
    #[inline(always)]
    pub const fn set_gpio34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO35 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO35 interrupt."]
    #[inline(always)]
    pub const fn set_gpio35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO36 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO36 interrupt."]
    #[inline(always)]
    pub const fn set_gpio36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO37 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO37 interrupt."]
    #[inline(always)]
    pub const fn set_gpio37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO38 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO38 interrupt."]
    #[inline(always)]
    pub const fn set_gpio38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO39 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO39 interrupt."]
    #[inline(always)]
    pub const fn set_gpio39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO40 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio40(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO40 interrupt."]
    #[inline(always)]
    pub const fn set_gpio40(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO41 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio41(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO41 interrupt."]
    #[inline(always)]
    pub const fn set_gpio41(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GPIO42 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio42(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO42 interrupt."]
    #[inline(always)]
    pub const fn set_gpio42(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GPIO43 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio43(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO43 interrupt."]
    #[inline(always)]
    pub const fn set_gpio43(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO44 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio44(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO44 interrupt."]
    #[inline(always)]
    pub const fn set_gpio44(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO45 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio45(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO45 interrupt."]
    #[inline(always)]
    pub const fn set_gpio45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO46 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio46(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO46 interrupt."]
    #[inline(always)]
    pub const fn set_gpio46(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO47 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio47(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO47 interrupt."]
    #[inline(always)]
    pub const fn set_gpio47(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO48 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio48(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO48 interrupt."]
    #[inline(always)]
    pub const fn set_gpio48(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO49 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio49(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO49 interrupt."]
    #[inline(always)]
    pub const fn set_gpio49(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Int1clr {
    #[inline(always)]
    fn default() -> Int1clr {
        Int1clr(0)
    }
}
impl core::fmt::Debug for Int1clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int1clr")
            .field("gpio32", &self.gpio32())
            .field("gpio33", &self.gpio33())
            .field("gpio34", &self.gpio34())
            .field("gpio35", &self.gpio35())
            .field("gpio36", &self.gpio36())
            .field("gpio37", &self.gpio37())
            .field("gpio38", &self.gpio38())
            .field("gpio39", &self.gpio39())
            .field("gpio40", &self.gpio40())
            .field("gpio41", &self.gpio41())
            .field("gpio42", &self.gpio42())
            .field("gpio43", &self.gpio43())
            .field("gpio44", &self.gpio44())
            .field("gpio45", &self.gpio45())
            .field("gpio46", &self.gpio46())
            .field("gpio47", &self.gpio47())
            .field("gpio48", &self.gpio48())
            .field("gpio49", &self.gpio49())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int1clr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int1clr {{ gpio32: {=bool:?}, gpio33: {=bool:?}, gpio34: {=bool:?}, gpio35: {=bool:?}, gpio36: {=bool:?}, gpio37: {=bool:?}, gpio38: {=bool:?}, gpio39: {=bool:?}, gpio40: {=bool:?}, gpio41: {=bool:?}, gpio42: {=bool:?}, gpio43: {=bool:?}, gpio44: {=bool:?}, gpio45: {=bool:?}, gpio46: {=bool:?}, gpio47: {=bool:?}, gpio48: {=bool:?}, gpio49: {=bool:?} }}" , self . gpio32 () , self . gpio33 () , self . gpio34 () , self . gpio35 () , self . gpio36 () , self . gpio37 () , self . gpio38 () , self . gpio39 () , self . gpio40 () , self . gpio41 () , self . gpio42 () , self . gpio43 () , self . gpio44 () , self . gpio45 () , self . gpio46 () , self . gpio47 () , self . gpio48 () , self . gpio49 ())
    }
}
#[doc = "GPIO Interrupt Registers 49-32: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int1en(pub u32);
impl Int1en {
    #[doc = "GPIO32 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO32 interrupt."]
    #[inline(always)]
    pub const fn set_gpio32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO33 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO33 interrupt."]
    #[inline(always)]
    pub const fn set_gpio33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO34 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO34 interrupt."]
    #[inline(always)]
    pub const fn set_gpio34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO35 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO35 interrupt."]
    #[inline(always)]
    pub const fn set_gpio35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO36 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO36 interrupt."]
    #[inline(always)]
    pub const fn set_gpio36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO37 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO37 interrupt."]
    #[inline(always)]
    pub const fn set_gpio37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO38 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO38 interrupt."]
    #[inline(always)]
    pub const fn set_gpio38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO39 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO39 interrupt."]
    #[inline(always)]
    pub const fn set_gpio39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO40 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio40(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO40 interrupt."]
    #[inline(always)]
    pub const fn set_gpio40(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO41 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio41(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO41 interrupt."]
    #[inline(always)]
    pub const fn set_gpio41(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GPIO42 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio42(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO42 interrupt."]
    #[inline(always)]
    pub const fn set_gpio42(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GPIO43 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio43(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO43 interrupt."]
    #[inline(always)]
    pub const fn set_gpio43(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO44 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio44(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO44 interrupt."]
    #[inline(always)]
    pub const fn set_gpio44(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO45 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio45(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO45 interrupt."]
    #[inline(always)]
    pub const fn set_gpio45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO46 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio46(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO46 interrupt."]
    #[inline(always)]
    pub const fn set_gpio46(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO47 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio47(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO47 interrupt."]
    #[inline(always)]
    pub const fn set_gpio47(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO48 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio48(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO48 interrupt."]
    #[inline(always)]
    pub const fn set_gpio48(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO49 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio49(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO49 interrupt."]
    #[inline(always)]
    pub const fn set_gpio49(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Int1en {
    #[inline(always)]
    fn default() -> Int1en {
        Int1en(0)
    }
}
impl core::fmt::Debug for Int1en {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int1en")
            .field("gpio32", &self.gpio32())
            .field("gpio33", &self.gpio33())
            .field("gpio34", &self.gpio34())
            .field("gpio35", &self.gpio35())
            .field("gpio36", &self.gpio36())
            .field("gpio37", &self.gpio37())
            .field("gpio38", &self.gpio38())
            .field("gpio39", &self.gpio39())
            .field("gpio40", &self.gpio40())
            .field("gpio41", &self.gpio41())
            .field("gpio42", &self.gpio42())
            .field("gpio43", &self.gpio43())
            .field("gpio44", &self.gpio44())
            .field("gpio45", &self.gpio45())
            .field("gpio46", &self.gpio46())
            .field("gpio47", &self.gpio47())
            .field("gpio48", &self.gpio48())
            .field("gpio49", &self.gpio49())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int1en {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int1en {{ gpio32: {=bool:?}, gpio33: {=bool:?}, gpio34: {=bool:?}, gpio35: {=bool:?}, gpio36: {=bool:?}, gpio37: {=bool:?}, gpio38: {=bool:?}, gpio39: {=bool:?}, gpio40: {=bool:?}, gpio41: {=bool:?}, gpio42: {=bool:?}, gpio43: {=bool:?}, gpio44: {=bool:?}, gpio45: {=bool:?}, gpio46: {=bool:?}, gpio47: {=bool:?}, gpio48: {=bool:?}, gpio49: {=bool:?} }}" , self . gpio32 () , self . gpio33 () , self . gpio34 () , self . gpio35 () , self . gpio36 () , self . gpio37 () , self . gpio38 () , self . gpio39 () , self . gpio40 () , self . gpio41 () , self . gpio42 () , self . gpio43 () , self . gpio44 () , self . gpio45 () , self . gpio46 () , self . gpio47 () , self . gpio48 () , self . gpio49 ())
    }
}
#[doc = "GPIO Interrupt Registers 49-32: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int1set(pub u32);
impl Int1set {
    #[doc = "GPIO32 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO32 interrupt."]
    #[inline(always)]
    pub const fn set_gpio32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO33 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO33 interrupt."]
    #[inline(always)]
    pub const fn set_gpio33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO34 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO34 interrupt."]
    #[inline(always)]
    pub const fn set_gpio34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO35 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO35 interrupt."]
    #[inline(always)]
    pub const fn set_gpio35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO36 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO36 interrupt."]
    #[inline(always)]
    pub const fn set_gpio36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO37 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO37 interrupt."]
    #[inline(always)]
    pub const fn set_gpio37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO38 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO38 interrupt."]
    #[inline(always)]
    pub const fn set_gpio38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO39 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO39 interrupt."]
    #[inline(always)]
    pub const fn set_gpio39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO40 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio40(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO40 interrupt."]
    #[inline(always)]
    pub const fn set_gpio40(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO41 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio41(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO41 interrupt."]
    #[inline(always)]
    pub const fn set_gpio41(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GPIO42 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio42(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO42 interrupt."]
    #[inline(always)]
    pub const fn set_gpio42(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GPIO43 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio43(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO43 interrupt."]
    #[inline(always)]
    pub const fn set_gpio43(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO44 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio44(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO44 interrupt."]
    #[inline(always)]
    pub const fn set_gpio44(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO45 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio45(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO45 interrupt."]
    #[inline(always)]
    pub const fn set_gpio45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO46 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio46(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO46 interrupt."]
    #[inline(always)]
    pub const fn set_gpio46(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO47 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio47(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO47 interrupt."]
    #[inline(always)]
    pub const fn set_gpio47(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO48 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio48(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO48 interrupt."]
    #[inline(always)]
    pub const fn set_gpio48(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO49 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio49(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO49 interrupt."]
    #[inline(always)]
    pub const fn set_gpio49(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Int1set {
    #[inline(always)]
    fn default() -> Int1set {
        Int1set(0)
    }
}
impl core::fmt::Debug for Int1set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int1set")
            .field("gpio32", &self.gpio32())
            .field("gpio33", &self.gpio33())
            .field("gpio34", &self.gpio34())
            .field("gpio35", &self.gpio35())
            .field("gpio36", &self.gpio36())
            .field("gpio37", &self.gpio37())
            .field("gpio38", &self.gpio38())
            .field("gpio39", &self.gpio39())
            .field("gpio40", &self.gpio40())
            .field("gpio41", &self.gpio41())
            .field("gpio42", &self.gpio42())
            .field("gpio43", &self.gpio43())
            .field("gpio44", &self.gpio44())
            .field("gpio45", &self.gpio45())
            .field("gpio46", &self.gpio46())
            .field("gpio47", &self.gpio47())
            .field("gpio48", &self.gpio48())
            .field("gpio49", &self.gpio49())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int1set {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int1set {{ gpio32: {=bool:?}, gpio33: {=bool:?}, gpio34: {=bool:?}, gpio35: {=bool:?}, gpio36: {=bool:?}, gpio37: {=bool:?}, gpio38: {=bool:?}, gpio39: {=bool:?}, gpio40: {=bool:?}, gpio41: {=bool:?}, gpio42: {=bool:?}, gpio43: {=bool:?}, gpio44: {=bool:?}, gpio45: {=bool:?}, gpio46: {=bool:?}, gpio47: {=bool:?}, gpio48: {=bool:?}, gpio49: {=bool:?} }}" , self . gpio32 () , self . gpio33 () , self . gpio34 () , self . gpio35 () , self . gpio36 () , self . gpio37 () , self . gpio38 () , self . gpio39 () , self . gpio40 () , self . gpio41 () , self . gpio42 () , self . gpio43 () , self . gpio44 () , self . gpio45 () , self . gpio46 () , self . gpio47 () , self . gpio48 () , self . gpio49 ())
    }
}
#[doc = "GPIO Interrupt Registers 49-32: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int1stat(pub u32);
impl Int1stat {
    #[doc = "GPIO32 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO32 interrupt."]
    #[inline(always)]
    pub const fn set_gpio32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO33 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO33 interrupt."]
    #[inline(always)]
    pub const fn set_gpio33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO34 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO34 interrupt."]
    #[inline(always)]
    pub const fn set_gpio34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO35 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO35 interrupt."]
    #[inline(always)]
    pub const fn set_gpio35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO36 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO36 interrupt."]
    #[inline(always)]
    pub const fn set_gpio36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO37 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO37 interrupt."]
    #[inline(always)]
    pub const fn set_gpio37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO38 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO38 interrupt."]
    #[inline(always)]
    pub const fn set_gpio38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO39 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO39 interrupt."]
    #[inline(always)]
    pub const fn set_gpio39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO40 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio40(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO40 interrupt."]
    #[inline(always)]
    pub const fn set_gpio40(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO41 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio41(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO41 interrupt."]
    #[inline(always)]
    pub const fn set_gpio41(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "GPIO42 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio42(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO42 interrupt."]
    #[inline(always)]
    pub const fn set_gpio42(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GPIO43 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio43(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO43 interrupt."]
    #[inline(always)]
    pub const fn set_gpio43(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO44 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio44(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO44 interrupt."]
    #[inline(always)]
    pub const fn set_gpio44(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO45 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio45(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO45 interrupt."]
    #[inline(always)]
    pub const fn set_gpio45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO46 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio46(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO46 interrupt."]
    #[inline(always)]
    pub const fn set_gpio46(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO47 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio47(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO47 interrupt."]
    #[inline(always)]
    pub const fn set_gpio47(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO48 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio48(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO48 interrupt."]
    #[inline(always)]
    pub const fn set_gpio48(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO49 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio49(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO49 interrupt."]
    #[inline(always)]
    pub const fn set_gpio49(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Int1stat {
    #[inline(always)]
    fn default() -> Int1stat {
        Int1stat(0)
    }
}
impl core::fmt::Debug for Int1stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int1stat")
            .field("gpio32", &self.gpio32())
            .field("gpio33", &self.gpio33())
            .field("gpio34", &self.gpio34())
            .field("gpio35", &self.gpio35())
            .field("gpio36", &self.gpio36())
            .field("gpio37", &self.gpio37())
            .field("gpio38", &self.gpio38())
            .field("gpio39", &self.gpio39())
            .field("gpio40", &self.gpio40())
            .field("gpio41", &self.gpio41())
            .field("gpio42", &self.gpio42())
            .field("gpio43", &self.gpio43())
            .field("gpio44", &self.gpio44())
            .field("gpio45", &self.gpio45())
            .field("gpio46", &self.gpio46())
            .field("gpio47", &self.gpio47())
            .field("gpio48", &self.gpio48())
            .field("gpio49", &self.gpio49())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int1stat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int1stat {{ gpio32: {=bool:?}, gpio33: {=bool:?}, gpio34: {=bool:?}, gpio35: {=bool:?}, gpio36: {=bool:?}, gpio37: {=bool:?}, gpio38: {=bool:?}, gpio39: {=bool:?}, gpio40: {=bool:?}, gpio41: {=bool:?}, gpio42: {=bool:?}, gpio43: {=bool:?}, gpio44: {=bool:?}, gpio45: {=bool:?}, gpio46: {=bool:?}, gpio47: {=bool:?}, gpio48: {=bool:?}, gpio49: {=bool:?} }}" , self . gpio32 () , self . gpio33 () , self . gpio34 () , self . gpio35 () , self . gpio36 () , self . gpio37 () , self . gpio38 () , self . gpio39 () , self . gpio40 () , self . gpio41 () , self . gpio42 () , self . gpio43 () , self . gpio44 () , self . gpio45 () , self . gpio46 () , self . gpio47 () , self . gpio48 () , self . gpio49 ())
    }
}
#[doc = "IOM0 Flow Control IRQ Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iom0irq(pub u32);
impl Iom0irq {
    #[doc = "IOMSTR0 IRQ pad select."]
    #[must_use]
    #[inline(always)]
    pub const fn iom0irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "IOMSTR0 IRQ pad select."]
    #[inline(always)]
    pub const fn set_iom0irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Iom0irq {
    #[inline(always)]
    fn default() -> Iom0irq {
        Iom0irq(0)
    }
}
impl core::fmt::Debug for Iom0irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iom0irq")
            .field("iom0irq", &self.iom0irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iom0irq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iom0irq {{ iom0irq: {=u8:?} }}", self.iom0irq())
    }
}
#[doc = "IOM1 Flow Control IRQ Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iom1irq(pub u32);
impl Iom1irq {
    #[doc = "IOMSTR1 IRQ pad select."]
    #[must_use]
    #[inline(always)]
    pub const fn iom1irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "IOMSTR1 IRQ pad select."]
    #[inline(always)]
    pub const fn set_iom1irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Iom1irq {
    #[inline(always)]
    fn default() -> Iom1irq {
        Iom1irq(0)
    }
}
impl core::fmt::Debug for Iom1irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iom1irq")
            .field("iom1irq", &self.iom1irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iom1irq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iom1irq {{ iom1irq: {=u8:?} }}", self.iom1irq())
    }
}
#[doc = "IOM2 Flow Control IRQ Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iom2irq(pub u32);
impl Iom2irq {
    #[doc = "IOMSTR2 IRQ pad select."]
    #[must_use]
    #[inline(always)]
    pub const fn iom2irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "IOMSTR2 IRQ pad select."]
    #[inline(always)]
    pub const fn set_iom2irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Iom2irq {
    #[inline(always)]
    fn default() -> Iom2irq {
        Iom2irq(0)
    }
}
impl core::fmt::Debug for Iom2irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iom2irq")
            .field("iom2irq", &self.iom2irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iom2irq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iom2irq {{ iom2irq: {=u8:?} }}", self.iom2irq())
    }
}
#[doc = "IOM3 Flow Control IRQ Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iom3irq(pub u32);
impl Iom3irq {
    #[doc = "IOMSTR3 IRQ pad select."]
    #[must_use]
    #[inline(always)]
    pub const fn iom3irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "IOMSTR3 IRQ pad select."]
    #[inline(always)]
    pub const fn set_iom3irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Iom3irq {
    #[inline(always)]
    fn default() -> Iom3irq {
        Iom3irq(0)
    }
}
impl core::fmt::Debug for Iom3irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iom3irq")
            .field("iom3irq", &self.iom3irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iom3irq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iom3irq {{ iom3irq: {=u8:?} }}", self.iom3irq())
    }
}
#[doc = "IOM4 Flow Control IRQ Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iom4irq(pub u32);
impl Iom4irq {
    #[doc = "IOMSTR4 IRQ pad select."]
    #[must_use]
    #[inline(always)]
    pub const fn iom4irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "IOMSTR4 IRQ pad select."]
    #[inline(always)]
    pub const fn set_iom4irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Iom4irq {
    #[inline(always)]
    fn default() -> Iom4irq {
        Iom4irq(0)
    }
}
impl core::fmt::Debug for Iom4irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iom4irq")
            .field("iom4irq", &self.iom4irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iom4irq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iom4irq {{ iom4irq: {=u8:?} }}", self.iom4irq())
    }
}
#[doc = "IOM5 Flow Control IRQ Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iom5irq(pub u32);
impl Iom5irq {
    #[doc = "IOMSTR5 IRQ pad select."]
    #[must_use]
    #[inline(always)]
    pub const fn iom5irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "IOMSTR5 IRQ pad select."]
    #[inline(always)]
    pub const fn set_iom5irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Iom5irq {
    #[inline(always)]
    fn default() -> Iom5irq {
        Iom5irq(0)
    }
}
impl core::fmt::Debug for Iom5irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iom5irq")
            .field("iom5irq", &self.iom5irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iom5irq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iom5irq {{ iom5irq: {=u8:?} }}", self.iom5irq())
    }
}
#[doc = "Key Register for all pad configuration registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padkey(pub u32);
impl Padkey {
    #[doc = "Key register value."]
    #[must_use]
    #[inline(always)]
    pub const fn padkey(&self) -> super::vals::Padkey {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Padkey::from_bits(val as u32)
    }
    #[doc = "Key register value."]
    #[inline(always)]
    pub const fn set_padkey(&mut self, val: super::vals::Padkey) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Padkey {
    #[inline(always)]
    fn default() -> Padkey {
        Padkey(0)
    }
}
impl core::fmt::Debug for Padkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padkey")
            .field("padkey", &self.padkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padkey {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Padkey {{ padkey: {:?} }}", self.padkey())
    }
}
#[doc = "Pad Configuration Register A (Pads 0-3)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padrega(pub u32);
impl Padrega {
    #[doc = "Pad 0 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad0pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 0 pullup enable."]
    #[inline(always)]
    pub const fn set_pad0pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 0 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad0inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 0 input enable."]
    #[inline(always)]
    pub const fn set_pad0inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 0 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad0strng(&self) -> super::vals::Pad0strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad0strng::from_bits(val as u8)
    }
    #[doc = "Pad 0 drive strength."]
    #[inline(always)]
    pub const fn set_pad0strng(&mut self, val: super::vals::Pad0strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 0 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad0fncsel(&self) -> super::vals::Pad0fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad0fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 0 function select."]
    #[inline(always)]
    pub const fn set_pad0fncsel(&mut self, val: super::vals::Pad0fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 0 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad0rsel(&self) -> super::vals::Pad0rsel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pad0rsel::from_bits(val as u8)
    }
    #[doc = "Pad 0 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad0rsel(&mut self, val: super::vals::Pad0rsel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Pad 1 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad1pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 1 pullup enable."]
    #[inline(always)]
    pub const fn set_pad1pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 1 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad1inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 1 input enable."]
    #[inline(always)]
    pub const fn set_pad1inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 1 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad1strng(&self) -> super::vals::Pad1strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad1strng::from_bits(val as u8)
    }
    #[doc = "Pad 1 drive strength."]
    #[inline(always)]
    pub const fn set_pad1strng(&mut self, val: super::vals::Pad1strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 1 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad1fncsel(&self) -> super::vals::Pad1fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad1fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 1 function select."]
    #[inline(always)]
    pub const fn set_pad1fncsel(&mut self, val: super::vals::Pad1fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 1 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad1rsel(&self) -> super::vals::Pad1rsel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pad1rsel::from_bits(val as u8)
    }
    #[doc = "Pad 1 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad1rsel(&mut self, val: super::vals::Pad1rsel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Pad 2 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad2pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 2 pullup enable."]
    #[inline(always)]
    pub const fn set_pad2pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 2 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad2inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 2 input enable."]
    #[inline(always)]
    pub const fn set_pad2inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 2 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad2strng(&self) -> super::vals::Pad2strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad2strng::from_bits(val as u8)
    }
    #[doc = "Pad 2 drive strength."]
    #[inline(always)]
    pub const fn set_pad2strng(&mut self, val: super::vals::Pad2strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 2 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad2fncsel(&self) -> super::vals::Pad2fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad2fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 2 function select."]
    #[inline(always)]
    pub const fn set_pad2fncsel(&mut self, val: super::vals::Pad2fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 3 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad3pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 3 pullup enable."]
    #[inline(always)]
    pub const fn set_pad3pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 3 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad3inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 3 input enable."]
    #[inline(always)]
    pub const fn set_pad3inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 3 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad3strng(&self) -> super::vals::Pad3strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad3strng::from_bits(val as u8)
    }
    #[doc = "Pad 3 drive strength."]
    #[inline(always)]
    pub const fn set_pad3strng(&mut self, val: super::vals::Pad3strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 3 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad3fncsel(&self) -> super::vals::Pad3fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad3fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 3 function select."]
    #[inline(always)]
    pub const fn set_pad3fncsel(&mut self, val: super::vals::Pad3fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Pad 3 VDD power switch enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad3pwrup(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 3 VDD power switch enable."]
    #[inline(always)]
    pub const fn set_pad3pwrup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Padrega {
    #[inline(always)]
    fn default() -> Padrega {
        Padrega(0)
    }
}
impl core::fmt::Debug for Padrega {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padrega")
            .field("pad0pull", &self.pad0pull())
            .field("pad0inpen", &self.pad0inpen())
            .field("pad0strng", &self.pad0strng())
            .field("pad0fncsel", &self.pad0fncsel())
            .field("pad0rsel", &self.pad0rsel())
            .field("pad1pull", &self.pad1pull())
            .field("pad1inpen", &self.pad1inpen())
            .field("pad1strng", &self.pad1strng())
            .field("pad1fncsel", &self.pad1fncsel())
            .field("pad1rsel", &self.pad1rsel())
            .field("pad2pull", &self.pad2pull())
            .field("pad2inpen", &self.pad2inpen())
            .field("pad2strng", &self.pad2strng())
            .field("pad2fncsel", &self.pad2fncsel())
            .field("pad3pull", &self.pad3pull())
            .field("pad3inpen", &self.pad3inpen())
            .field("pad3strng", &self.pad3strng())
            .field("pad3fncsel", &self.pad3fncsel())
            .field("pad3pwrup", &self.pad3pwrup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padrega {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padrega {{ pad0pull: {=bool:?}, pad0inpen: {=bool:?}, pad0strng: {:?}, pad0fncsel: {:?}, pad0rsel: {:?}, pad1pull: {=bool:?}, pad1inpen: {=bool:?}, pad1strng: {:?}, pad1fncsel: {:?}, pad1rsel: {:?}, pad2pull: {=bool:?}, pad2inpen: {=bool:?}, pad2strng: {:?}, pad2fncsel: {:?}, pad3pull: {=bool:?}, pad3inpen: {=bool:?}, pad3strng: {:?}, pad3fncsel: {:?}, pad3pwrup: {=bool:?} }}" , self . pad0pull () , self . pad0inpen () , self . pad0strng () , self . pad0fncsel () , self . pad0rsel () , self . pad1pull () , self . pad1inpen () , self . pad1strng () , self . pad1fncsel () , self . pad1rsel () , self . pad2pull () , self . pad2inpen () , self . pad2strng () , self . pad2fncsel () , self . pad3pull () , self . pad3inpen () , self . pad3strng () , self . pad3fncsel () , self . pad3pwrup ())
    }
}
#[doc = "Pad Configuration Register B (Pads 4-7)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregb(pub u32);
impl Padregb {
    #[doc = "Pad 4 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad4pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 4 pullup enable."]
    #[inline(always)]
    pub const fn set_pad4pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 4 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad4inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 4 input enable."]
    #[inline(always)]
    pub const fn set_pad4inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 4 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad4strng(&self) -> super::vals::Pad4strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad4strng::from_bits(val as u8)
    }
    #[doc = "Pad 4 drive strength."]
    #[inline(always)]
    pub const fn set_pad4strng(&mut self, val: super::vals::Pad4strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 4 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad4fncsel(&self) -> super::vals::Pad4fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad4fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 4 function select."]
    #[inline(always)]
    pub const fn set_pad4fncsel(&mut self, val: super::vals::Pad4fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 5 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad5pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 5 pullup enable."]
    #[inline(always)]
    pub const fn set_pad5pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 5 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad5inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 5 input enable."]
    #[inline(always)]
    pub const fn set_pad5inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 5 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad5strng(&self) -> super::vals::Pad5strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad5strng::from_bits(val as u8)
    }
    #[doc = "Pad 5 drive strength."]
    #[inline(always)]
    pub const fn set_pad5strng(&mut self, val: super::vals::Pad5strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 5 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad5fncsel(&self) -> super::vals::Pad5fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad5fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 5 function select."]
    #[inline(always)]
    pub const fn set_pad5fncsel(&mut self, val: super::vals::Pad5fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 5 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad5rsel(&self) -> super::vals::Pad5rsel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pad5rsel::from_bits(val as u8)
    }
    #[doc = "Pad 5 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad5rsel(&mut self, val: super::vals::Pad5rsel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Pad 6 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad6pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 6 pullup enable."]
    #[inline(always)]
    pub const fn set_pad6pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 6 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad6inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 6 input enable."]
    #[inline(always)]
    pub const fn set_pad6inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 6 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad6strng(&self) -> super::vals::Pad6strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad6strng::from_bits(val as u8)
    }
    #[doc = "Pad 6 drive strength."]
    #[inline(always)]
    pub const fn set_pad6strng(&mut self, val: super::vals::Pad6strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 6 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad6fncsel(&self) -> super::vals::Pad6fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad6fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 6 function select."]
    #[inline(always)]
    pub const fn set_pad6fncsel(&mut self, val: super::vals::Pad6fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 6 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad6rsel(&self) -> super::vals::Pad6rsel {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Pad6rsel::from_bits(val as u8)
    }
    #[doc = "Pad 6 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad6rsel(&mut self, val: super::vals::Pad6rsel) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Pad 7 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad7pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 7 pullup enable."]
    #[inline(always)]
    pub const fn set_pad7pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 7 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad7inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 7 input enable."]
    #[inline(always)]
    pub const fn set_pad7inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 7 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad7strng(&self) -> super::vals::Pad7strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad7strng::from_bits(val as u8)
    }
    #[doc = "Pad 7 drive strength."]
    #[inline(always)]
    pub const fn set_pad7strng(&mut self, val: super::vals::Pad7strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 7 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad7fncsel(&self) -> super::vals::Pad7fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad7fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 7 function select."]
    #[inline(always)]
    pub const fn set_pad7fncsel(&mut self, val: super::vals::Pad7fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
}
impl Default for Padregb {
    #[inline(always)]
    fn default() -> Padregb {
        Padregb(0)
    }
}
impl core::fmt::Debug for Padregb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregb")
            .field("pad4pull", &self.pad4pull())
            .field("pad4inpen", &self.pad4inpen())
            .field("pad4strng", &self.pad4strng())
            .field("pad4fncsel", &self.pad4fncsel())
            .field("pad5pull", &self.pad5pull())
            .field("pad5inpen", &self.pad5inpen())
            .field("pad5strng", &self.pad5strng())
            .field("pad5fncsel", &self.pad5fncsel())
            .field("pad5rsel", &self.pad5rsel())
            .field("pad6pull", &self.pad6pull())
            .field("pad6inpen", &self.pad6inpen())
            .field("pad6strng", &self.pad6strng())
            .field("pad6fncsel", &self.pad6fncsel())
            .field("pad6rsel", &self.pad6rsel())
            .field("pad7pull", &self.pad7pull())
            .field("pad7inpen", &self.pad7inpen())
            .field("pad7strng", &self.pad7strng())
            .field("pad7fncsel", &self.pad7fncsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregb {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregb {{ pad4pull: {=bool:?}, pad4inpen: {=bool:?}, pad4strng: {:?}, pad4fncsel: {:?}, pad5pull: {=bool:?}, pad5inpen: {=bool:?}, pad5strng: {:?}, pad5fncsel: {:?}, pad5rsel: {:?}, pad6pull: {=bool:?}, pad6inpen: {=bool:?}, pad6strng: {:?}, pad6fncsel: {:?}, pad6rsel: {:?}, pad7pull: {=bool:?}, pad7inpen: {=bool:?}, pad7strng: {:?}, pad7fncsel: {:?} }}" , self . pad4pull () , self . pad4inpen () , self . pad4strng () , self . pad4fncsel () , self . pad5pull () , self . pad5inpen () , self . pad5strng () , self . pad5fncsel () , self . pad5rsel () , self . pad6pull () , self . pad6inpen () , self . pad6strng () , self . pad6fncsel () , self . pad6rsel () , self . pad7pull () , self . pad7inpen () , self . pad7strng () , self . pad7fncsel ())
    }
}
#[doc = "Pad Configuration Register C (Pads 8-11)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregc(pub u32);
impl Padregc {
    #[doc = "Pad 8 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad8pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 8 pullup enable."]
    #[inline(always)]
    pub const fn set_pad8pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 8 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad8inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 8 input enable."]
    #[inline(always)]
    pub const fn set_pad8inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 8 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad8strng(&self) -> super::vals::Pad8strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad8strng::from_bits(val as u8)
    }
    #[doc = "Pad 8 drive strength."]
    #[inline(always)]
    pub const fn set_pad8strng(&mut self, val: super::vals::Pad8strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 8 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad8fncsel(&self) -> super::vals::Pad8fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad8fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 8 function select."]
    #[inline(always)]
    pub const fn set_pad8fncsel(&mut self, val: super::vals::Pad8fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 8 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad8rsel(&self) -> super::vals::Pad8rsel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pad8rsel::from_bits(val as u8)
    }
    #[doc = "Pad 8 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad8rsel(&mut self, val: super::vals::Pad8rsel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Pad 9 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad9pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 9 pullup enable."]
    #[inline(always)]
    pub const fn set_pad9pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 9 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad9inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 9 input enable."]
    #[inline(always)]
    pub const fn set_pad9inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 9 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad9strng(&self) -> super::vals::Pad9strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad9strng::from_bits(val as u8)
    }
    #[doc = "Pad 9 drive strength."]
    #[inline(always)]
    pub const fn set_pad9strng(&mut self, val: super::vals::Pad9strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 9 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad9fncsel(&self) -> super::vals::Pad9fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad9fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 9 function select."]
    #[inline(always)]
    pub const fn set_pad9fncsel(&mut self, val: super::vals::Pad9fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 9 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad9rsel(&self) -> super::vals::Pad9rsel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pad9rsel::from_bits(val as u8)
    }
    #[doc = "Pad 9 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad9rsel(&mut self, val: super::vals::Pad9rsel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Pad 10 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad10pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 10 pullup enable."]
    #[inline(always)]
    pub const fn set_pad10pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 10 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad10inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 10 input enable."]
    #[inline(always)]
    pub const fn set_pad10inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 10 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad10strng(&self) -> super::vals::Pad10strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad10strng::from_bits(val as u8)
    }
    #[doc = "Pad 10 drive strength."]
    #[inline(always)]
    pub const fn set_pad10strng(&mut self, val: super::vals::Pad10strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 10 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad10fncsel(&self) -> super::vals::Pad10fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad10fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 10 function select."]
    #[inline(always)]
    pub const fn set_pad10fncsel(&mut self, val: super::vals::Pad10fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 11 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad11pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 11 pullup enable."]
    #[inline(always)]
    pub const fn set_pad11pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 11 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad11inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 11 input enable."]
    #[inline(always)]
    pub const fn set_pad11inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 11 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad11strng(&self) -> super::vals::Pad11strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad11strng::from_bits(val as u8)
    }
    #[doc = "Pad 11 drive strength."]
    #[inline(always)]
    pub const fn set_pad11strng(&mut self, val: super::vals::Pad11strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 11 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad11fncsel(&self) -> super::vals::Pad11fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad11fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 11 function select."]
    #[inline(always)]
    pub const fn set_pad11fncsel(&mut self, val: super::vals::Pad11fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
}
impl Default for Padregc {
    #[inline(always)]
    fn default() -> Padregc {
        Padregc(0)
    }
}
impl core::fmt::Debug for Padregc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregc")
            .field("pad8pull", &self.pad8pull())
            .field("pad8inpen", &self.pad8inpen())
            .field("pad8strng", &self.pad8strng())
            .field("pad8fncsel", &self.pad8fncsel())
            .field("pad8rsel", &self.pad8rsel())
            .field("pad9pull", &self.pad9pull())
            .field("pad9inpen", &self.pad9inpen())
            .field("pad9strng", &self.pad9strng())
            .field("pad9fncsel", &self.pad9fncsel())
            .field("pad9rsel", &self.pad9rsel())
            .field("pad10pull", &self.pad10pull())
            .field("pad10inpen", &self.pad10inpen())
            .field("pad10strng", &self.pad10strng())
            .field("pad10fncsel", &self.pad10fncsel())
            .field("pad11pull", &self.pad11pull())
            .field("pad11inpen", &self.pad11inpen())
            .field("pad11strng", &self.pad11strng())
            .field("pad11fncsel", &self.pad11fncsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregc {{ pad8pull: {=bool:?}, pad8inpen: {=bool:?}, pad8strng: {:?}, pad8fncsel: {:?}, pad8rsel: {:?}, pad9pull: {=bool:?}, pad9inpen: {=bool:?}, pad9strng: {:?}, pad9fncsel: {:?}, pad9rsel: {:?}, pad10pull: {=bool:?}, pad10inpen: {=bool:?}, pad10strng: {:?}, pad10fncsel: {:?}, pad11pull: {=bool:?}, pad11inpen: {=bool:?}, pad11strng: {:?}, pad11fncsel: {:?} }}" , self . pad8pull () , self . pad8inpen () , self . pad8strng () , self . pad8fncsel () , self . pad8rsel () , self . pad9pull () , self . pad9inpen () , self . pad9strng () , self . pad9fncsel () , self . pad9rsel () , self . pad10pull () , self . pad10inpen () , self . pad10strng () , self . pad10fncsel () , self . pad11pull () , self . pad11inpen () , self . pad11strng () , self . pad11fncsel ())
    }
}
#[doc = "Pad Configuration Register D (Pads 12-15)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregd(pub u32);
impl Padregd {
    #[doc = "Pad 12 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad12pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 12 pullup enable."]
    #[inline(always)]
    pub const fn set_pad12pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 12 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad12inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 12 input enable."]
    #[inline(always)]
    pub const fn set_pad12inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 12 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad12strng(&self) -> super::vals::Pad12strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad12strng::from_bits(val as u8)
    }
    #[doc = "Pad 12 drive strength."]
    #[inline(always)]
    pub const fn set_pad12strng(&mut self, val: super::vals::Pad12strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 12 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad12fncsel(&self) -> super::vals::Pad12fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad12fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 12 function select."]
    #[inline(always)]
    pub const fn set_pad12fncsel(&mut self, val: super::vals::Pad12fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 13 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad13pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 13 pullup enable."]
    #[inline(always)]
    pub const fn set_pad13pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 13 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad13inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 13 input enable."]
    #[inline(always)]
    pub const fn set_pad13inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 13 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad13strng(&self) -> super::vals::Pad13strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad13strng::from_bits(val as u8)
    }
    #[doc = "Pad 13 drive strength."]
    #[inline(always)]
    pub const fn set_pad13strng(&mut self, val: super::vals::Pad13strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 13 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad13fncsel(&self) -> super::vals::Pad13fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad13fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 13 function select."]
    #[inline(always)]
    pub const fn set_pad13fncsel(&mut self, val: super::vals::Pad13fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 14 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad14pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 14 pullup enable."]
    #[inline(always)]
    pub const fn set_pad14pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 14 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad14inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 14 input enable."]
    #[inline(always)]
    pub const fn set_pad14inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 14 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad14strng(&self) -> super::vals::Pad14strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad14strng::from_bits(val as u8)
    }
    #[doc = "Pad 14 drive strength."]
    #[inline(always)]
    pub const fn set_pad14strng(&mut self, val: super::vals::Pad14strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 14 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad14fncsel(&self) -> super::vals::Pad14fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad14fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 14 function select."]
    #[inline(always)]
    pub const fn set_pad14fncsel(&mut self, val: super::vals::Pad14fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 15 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad15pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 15 pullup enable."]
    #[inline(always)]
    pub const fn set_pad15pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 15 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad15inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 15 input enable."]
    #[inline(always)]
    pub const fn set_pad15inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 15 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad15strng(&self) -> super::vals::Pad15strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad15strng::from_bits(val as u8)
    }
    #[doc = "Pad 15 drive strength."]
    #[inline(always)]
    pub const fn set_pad15strng(&mut self, val: super::vals::Pad15strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 15 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad15fncsel(&self) -> super::vals::Pad15fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad15fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 15 function select."]
    #[inline(always)]
    pub const fn set_pad15fncsel(&mut self, val: super::vals::Pad15fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
}
impl Default for Padregd {
    #[inline(always)]
    fn default() -> Padregd {
        Padregd(0)
    }
}
impl core::fmt::Debug for Padregd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregd")
            .field("pad12pull", &self.pad12pull())
            .field("pad12inpen", &self.pad12inpen())
            .field("pad12strng", &self.pad12strng())
            .field("pad12fncsel", &self.pad12fncsel())
            .field("pad13pull", &self.pad13pull())
            .field("pad13inpen", &self.pad13inpen())
            .field("pad13strng", &self.pad13strng())
            .field("pad13fncsel", &self.pad13fncsel())
            .field("pad14pull", &self.pad14pull())
            .field("pad14inpen", &self.pad14inpen())
            .field("pad14strng", &self.pad14strng())
            .field("pad14fncsel", &self.pad14fncsel())
            .field("pad15pull", &self.pad15pull())
            .field("pad15inpen", &self.pad15inpen())
            .field("pad15strng", &self.pad15strng())
            .field("pad15fncsel", &self.pad15fncsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregd {{ pad12pull: {=bool:?}, pad12inpen: {=bool:?}, pad12strng: {:?}, pad12fncsel: {:?}, pad13pull: {=bool:?}, pad13inpen: {=bool:?}, pad13strng: {:?}, pad13fncsel: {:?}, pad14pull: {=bool:?}, pad14inpen: {=bool:?}, pad14strng: {:?}, pad14fncsel: {:?}, pad15pull: {=bool:?}, pad15inpen: {=bool:?}, pad15strng: {:?}, pad15fncsel: {:?} }}" , self . pad12pull () , self . pad12inpen () , self . pad12strng () , self . pad12fncsel () , self . pad13pull () , self . pad13inpen () , self . pad13strng () , self . pad13fncsel () , self . pad14pull () , self . pad14inpen () , self . pad14strng () , self . pad14fncsel () , self . pad15pull () , self . pad15inpen () , self . pad15strng () , self . pad15fncsel ())
    }
}
#[doc = "Pad Configuration Register E (Pads 16-19)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padrege(pub u32);
impl Padrege {
    #[doc = "Pad 16 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad16pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 16 pullup enable."]
    #[inline(always)]
    pub const fn set_pad16pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 16 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad16inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 16 input enable."]
    #[inline(always)]
    pub const fn set_pad16inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 16 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad16strng(&self) -> super::vals::Pad16strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad16strng::from_bits(val as u8)
    }
    #[doc = "Pad 16 drive strength."]
    #[inline(always)]
    pub const fn set_pad16strng(&mut self, val: super::vals::Pad16strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 16 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad16fncsel(&self) -> super::vals::Pad16fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad16fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 16 function select."]
    #[inline(always)]
    pub const fn set_pad16fncsel(&mut self, val: super::vals::Pad16fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 17 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad17pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 17 pullup enable."]
    #[inline(always)]
    pub const fn set_pad17pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 17 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad17inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 17 input enable."]
    #[inline(always)]
    pub const fn set_pad17inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 17 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad17strng(&self) -> super::vals::Pad17strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad17strng::from_bits(val as u8)
    }
    #[doc = "Pad 17 drive strength."]
    #[inline(always)]
    pub const fn set_pad17strng(&mut self, val: super::vals::Pad17strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 17 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad17fncsel(&self) -> super::vals::Pad17fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad17fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 17 function select."]
    #[inline(always)]
    pub const fn set_pad17fncsel(&mut self, val: super::vals::Pad17fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 18 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad18pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 18 pullup enable."]
    #[inline(always)]
    pub const fn set_pad18pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 18 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad18inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 18 input enable."]
    #[inline(always)]
    pub const fn set_pad18inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 18 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad18strng(&self) -> super::vals::Pad18strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad18strng::from_bits(val as u8)
    }
    #[doc = "Pad 18 drive strength."]
    #[inline(always)]
    pub const fn set_pad18strng(&mut self, val: super::vals::Pad18strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 18 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad18fncsel(&self) -> super::vals::Pad18fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad18fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 18 function select."]
    #[inline(always)]
    pub const fn set_pad18fncsel(&mut self, val: super::vals::Pad18fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 19 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad19pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 19 pullup enable."]
    #[inline(always)]
    pub const fn set_pad19pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 19 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad19inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 19 input enable."]
    #[inline(always)]
    pub const fn set_pad19inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 19 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad19strng(&self) -> super::vals::Pad19strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad19strng::from_bits(val as u8)
    }
    #[doc = "Pad 19 drive strength."]
    #[inline(always)]
    pub const fn set_pad19strng(&mut self, val: super::vals::Pad19strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 19 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad19fncsel(&self) -> super::vals::Pad19fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad19fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 19 function select."]
    #[inline(always)]
    pub const fn set_pad19fncsel(&mut self, val: super::vals::Pad19fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
}
impl Default for Padrege {
    #[inline(always)]
    fn default() -> Padrege {
        Padrege(0)
    }
}
impl core::fmt::Debug for Padrege {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padrege")
            .field("pad16pull", &self.pad16pull())
            .field("pad16inpen", &self.pad16inpen())
            .field("pad16strng", &self.pad16strng())
            .field("pad16fncsel", &self.pad16fncsel())
            .field("pad17pull", &self.pad17pull())
            .field("pad17inpen", &self.pad17inpen())
            .field("pad17strng", &self.pad17strng())
            .field("pad17fncsel", &self.pad17fncsel())
            .field("pad18pull", &self.pad18pull())
            .field("pad18inpen", &self.pad18inpen())
            .field("pad18strng", &self.pad18strng())
            .field("pad18fncsel", &self.pad18fncsel())
            .field("pad19pull", &self.pad19pull())
            .field("pad19inpen", &self.pad19inpen())
            .field("pad19strng", &self.pad19strng())
            .field("pad19fncsel", &self.pad19fncsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padrege {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padrege {{ pad16pull: {=bool:?}, pad16inpen: {=bool:?}, pad16strng: {:?}, pad16fncsel: {:?}, pad17pull: {=bool:?}, pad17inpen: {=bool:?}, pad17strng: {:?}, pad17fncsel: {:?}, pad18pull: {=bool:?}, pad18inpen: {=bool:?}, pad18strng: {:?}, pad18fncsel: {:?}, pad19pull: {=bool:?}, pad19inpen: {=bool:?}, pad19strng: {:?}, pad19fncsel: {:?} }}" , self . pad16pull () , self . pad16inpen () , self . pad16strng () , self . pad16fncsel () , self . pad17pull () , self . pad17inpen () , self . pad17strng () , self . pad17fncsel () , self . pad18pull () , self . pad18inpen () , self . pad18strng () , self . pad18fncsel () , self . pad19pull () , self . pad19inpen () , self . pad19strng () , self . pad19fncsel ())
    }
}
#[doc = "Pad Configuration Register F (Pads 20-23)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregf(pub u32);
impl Padregf {
    #[doc = "Pad 20 pulldown enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad20pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 20 pulldown enable."]
    #[inline(always)]
    pub const fn set_pad20pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 20 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad20inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 20 input enable."]
    #[inline(always)]
    pub const fn set_pad20inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 20 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad20strng(&self) -> super::vals::Pad20strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad20strng::from_bits(val as u8)
    }
    #[doc = "Pad 20 drive strength."]
    #[inline(always)]
    pub const fn set_pad20strng(&mut self, val: super::vals::Pad20strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 20 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad20fncsel(&self) -> super::vals::Pad20fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad20fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 20 function select."]
    #[inline(always)]
    pub const fn set_pad20fncsel(&mut self, val: super::vals::Pad20fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 21 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad21pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 21 pullup enable."]
    #[inline(always)]
    pub const fn set_pad21pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 21 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad21inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 21 input enable."]
    #[inline(always)]
    pub const fn set_pad21inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 21 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad21strng(&self) -> super::vals::Pad21strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad21strng::from_bits(val as u8)
    }
    #[doc = "Pad 21 drive strength."]
    #[inline(always)]
    pub const fn set_pad21strng(&mut self, val: super::vals::Pad21strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 21 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad21fncsel(&self) -> super::vals::Pad21fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad21fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 21 function select."]
    #[inline(always)]
    pub const fn set_pad21fncsel(&mut self, val: super::vals::Pad21fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 22 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad22pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 22 pullup enable."]
    #[inline(always)]
    pub const fn set_pad22pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 22 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad22inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 22 input enable."]
    #[inline(always)]
    pub const fn set_pad22inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 22 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad22strng(&self) -> super::vals::Pad22strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad22strng::from_bits(val as u8)
    }
    #[doc = "Pad 22 drive strength."]
    #[inline(always)]
    pub const fn set_pad22strng(&mut self, val: super::vals::Pad22strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 22 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad22fncsel(&self) -> super::vals::Pad22fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad22fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 22 function select."]
    #[inline(always)]
    pub const fn set_pad22fncsel(&mut self, val: super::vals::Pad22fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 23 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad23pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 23 pullup enable."]
    #[inline(always)]
    pub const fn set_pad23pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 23 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad23inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 23 input enable."]
    #[inline(always)]
    pub const fn set_pad23inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 23 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad23strng(&self) -> super::vals::Pad23strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad23strng::from_bits(val as u8)
    }
    #[doc = "Pad 23 drive strength."]
    #[inline(always)]
    pub const fn set_pad23strng(&mut self, val: super::vals::Pad23strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 23 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad23fncsel(&self) -> super::vals::Pad23fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad23fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 23 function select."]
    #[inline(always)]
    pub const fn set_pad23fncsel(&mut self, val: super::vals::Pad23fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
}
impl Default for Padregf {
    #[inline(always)]
    fn default() -> Padregf {
        Padregf(0)
    }
}
impl core::fmt::Debug for Padregf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregf")
            .field("pad20pull", &self.pad20pull())
            .field("pad20inpen", &self.pad20inpen())
            .field("pad20strng", &self.pad20strng())
            .field("pad20fncsel", &self.pad20fncsel())
            .field("pad21pull", &self.pad21pull())
            .field("pad21inpen", &self.pad21inpen())
            .field("pad21strng", &self.pad21strng())
            .field("pad21fncsel", &self.pad21fncsel())
            .field("pad22pull", &self.pad22pull())
            .field("pad22inpen", &self.pad22inpen())
            .field("pad22strng", &self.pad22strng())
            .field("pad22fncsel", &self.pad22fncsel())
            .field("pad23pull", &self.pad23pull())
            .field("pad23inpen", &self.pad23inpen())
            .field("pad23strng", &self.pad23strng())
            .field("pad23fncsel", &self.pad23fncsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregf {{ pad20pull: {=bool:?}, pad20inpen: {=bool:?}, pad20strng: {:?}, pad20fncsel: {:?}, pad21pull: {=bool:?}, pad21inpen: {=bool:?}, pad21strng: {:?}, pad21fncsel: {:?}, pad22pull: {=bool:?}, pad22inpen: {=bool:?}, pad22strng: {:?}, pad22fncsel: {:?}, pad23pull: {=bool:?}, pad23inpen: {=bool:?}, pad23strng: {:?}, pad23fncsel: {:?} }}" , self . pad20pull () , self . pad20inpen () , self . pad20strng () , self . pad20fncsel () , self . pad21pull () , self . pad21inpen () , self . pad21strng () , self . pad21fncsel () , self . pad22pull () , self . pad22inpen () , self . pad22strng () , self . pad22fncsel () , self . pad23pull () , self . pad23inpen () , self . pad23strng () , self . pad23fncsel ())
    }
}
#[doc = "Pad Configuration Register G (Pads 24-27)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregg(pub u32);
impl Padregg {
    #[doc = "Pad 24 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad24pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 24 pullup enable."]
    #[inline(always)]
    pub const fn set_pad24pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 24 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad24inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 24 input enable."]
    #[inline(always)]
    pub const fn set_pad24inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 24 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad24strng(&self) -> super::vals::Pad24strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad24strng::from_bits(val as u8)
    }
    #[doc = "Pad 24 drive strength."]
    #[inline(always)]
    pub const fn set_pad24strng(&mut self, val: super::vals::Pad24strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 24 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad24fncsel(&self) -> super::vals::Pad24fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad24fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 24 function select."]
    #[inline(always)]
    pub const fn set_pad24fncsel(&mut self, val: super::vals::Pad24fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 25 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad25pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 25 pullup enable."]
    #[inline(always)]
    pub const fn set_pad25pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 25 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad25inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 25 input enable."]
    #[inline(always)]
    pub const fn set_pad25inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 25 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad25strng(&self) -> super::vals::Pad25strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad25strng::from_bits(val as u8)
    }
    #[doc = "Pad 25 drive strength."]
    #[inline(always)]
    pub const fn set_pad25strng(&mut self, val: super::vals::Pad25strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 25 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad25fncsel(&self) -> super::vals::Pad25fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad25fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 25 function select."]
    #[inline(always)]
    pub const fn set_pad25fncsel(&mut self, val: super::vals::Pad25fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 25 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad25rsel(&self) -> super::vals::Pad25rsel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pad25rsel::from_bits(val as u8)
    }
    #[doc = "Pad 25 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad25rsel(&mut self, val: super::vals::Pad25rsel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Pad 26 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad26pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 26 pullup enable."]
    #[inline(always)]
    pub const fn set_pad26pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 26 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad26inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 26 input enable."]
    #[inline(always)]
    pub const fn set_pad26inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 26 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad26strng(&self) -> super::vals::Pad26strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad26strng::from_bits(val as u8)
    }
    #[doc = "Pad 26 drive strength."]
    #[inline(always)]
    pub const fn set_pad26strng(&mut self, val: super::vals::Pad26strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 26 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad26fncsel(&self) -> super::vals::Pad26fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad26fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 26 function select."]
    #[inline(always)]
    pub const fn set_pad26fncsel(&mut self, val: super::vals::Pad26fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 27 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad27pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 27 pullup enable."]
    #[inline(always)]
    pub const fn set_pad27pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 27 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad27inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 27 input enable."]
    #[inline(always)]
    pub const fn set_pad27inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 27 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad27strng(&self) -> super::vals::Pad27strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad27strng::from_bits(val as u8)
    }
    #[doc = "Pad 27 drive strength."]
    #[inline(always)]
    pub const fn set_pad27strng(&mut self, val: super::vals::Pad27strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 27 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad27fncsel(&self) -> super::vals::Pad27fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad27fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 27 function select."]
    #[inline(always)]
    pub const fn set_pad27fncsel(&mut self, val: super::vals::Pad27fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Pad 27 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad27rsel(&self) -> super::vals::Pad27rsel {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Pad27rsel::from_bits(val as u8)
    }
    #[doc = "Pad 27 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad27rsel(&mut self, val: super::vals::Pad27rsel) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Padregg {
    #[inline(always)]
    fn default() -> Padregg {
        Padregg(0)
    }
}
impl core::fmt::Debug for Padregg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregg")
            .field("pad24pull", &self.pad24pull())
            .field("pad24inpen", &self.pad24inpen())
            .field("pad24strng", &self.pad24strng())
            .field("pad24fncsel", &self.pad24fncsel())
            .field("pad25pull", &self.pad25pull())
            .field("pad25inpen", &self.pad25inpen())
            .field("pad25strng", &self.pad25strng())
            .field("pad25fncsel", &self.pad25fncsel())
            .field("pad25rsel", &self.pad25rsel())
            .field("pad26pull", &self.pad26pull())
            .field("pad26inpen", &self.pad26inpen())
            .field("pad26strng", &self.pad26strng())
            .field("pad26fncsel", &self.pad26fncsel())
            .field("pad27pull", &self.pad27pull())
            .field("pad27inpen", &self.pad27inpen())
            .field("pad27strng", &self.pad27strng())
            .field("pad27fncsel", &self.pad27fncsel())
            .field("pad27rsel", &self.pad27rsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregg {{ pad24pull: {=bool:?}, pad24inpen: {=bool:?}, pad24strng: {:?}, pad24fncsel: {:?}, pad25pull: {=bool:?}, pad25inpen: {=bool:?}, pad25strng: {:?}, pad25fncsel: {:?}, pad25rsel: {:?}, pad26pull: {=bool:?}, pad26inpen: {=bool:?}, pad26strng: {:?}, pad26fncsel: {:?}, pad27pull: {=bool:?}, pad27inpen: {=bool:?}, pad27strng: {:?}, pad27fncsel: {:?}, pad27rsel: {:?} }}" , self . pad24pull () , self . pad24inpen () , self . pad24strng () , self . pad24fncsel () , self . pad25pull () , self . pad25inpen () , self . pad25strng () , self . pad25fncsel () , self . pad25rsel () , self . pad26pull () , self . pad26inpen () , self . pad26strng () , self . pad26fncsel () , self . pad27pull () , self . pad27inpen () , self . pad27strng () , self . pad27fncsel () , self . pad27rsel ())
    }
}
#[doc = "Pad Configuration Register H (Pads 28-31)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregh(pub u32);
impl Padregh {
    #[doc = "Pad 28 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad28pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 28 pullup enable."]
    #[inline(always)]
    pub const fn set_pad28pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 28 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad28inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 28 input enable."]
    #[inline(always)]
    pub const fn set_pad28inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 28 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad28strng(&self) -> super::vals::Pad28strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad28strng::from_bits(val as u8)
    }
    #[doc = "Pad 28 drive strength."]
    #[inline(always)]
    pub const fn set_pad28strng(&mut self, val: super::vals::Pad28strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 28 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad28fncsel(&self) -> super::vals::Pad28fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad28fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 28 function select."]
    #[inline(always)]
    pub const fn set_pad28fncsel(&mut self, val: super::vals::Pad28fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 29 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad29pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 29 pullup enable."]
    #[inline(always)]
    pub const fn set_pad29pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 29 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad29inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 29 input enable."]
    #[inline(always)]
    pub const fn set_pad29inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 29 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad29strng(&self) -> super::vals::Pad29strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad29strng::from_bits(val as u8)
    }
    #[doc = "Pad 29 drive strength."]
    #[inline(always)]
    pub const fn set_pad29strng(&mut self, val: super::vals::Pad29strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 29 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad29fncsel(&self) -> super::vals::Pad29fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad29fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 29 function select."]
    #[inline(always)]
    pub const fn set_pad29fncsel(&mut self, val: super::vals::Pad29fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 30 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad30pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 30 pullup enable."]
    #[inline(always)]
    pub const fn set_pad30pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 30 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad30inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 30 input enable."]
    #[inline(always)]
    pub const fn set_pad30inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 30 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad30strng(&self) -> super::vals::Pad30strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad30strng::from_bits(val as u8)
    }
    #[doc = "Pad 30 drive strength."]
    #[inline(always)]
    pub const fn set_pad30strng(&mut self, val: super::vals::Pad30strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 30 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad30fncsel(&self) -> super::vals::Pad30fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad30fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 30 function select."]
    #[inline(always)]
    pub const fn set_pad30fncsel(&mut self, val: super::vals::Pad30fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 31 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad31pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 31 pullup enable."]
    #[inline(always)]
    pub const fn set_pad31pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 31 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad31inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 31 input enable."]
    #[inline(always)]
    pub const fn set_pad31inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 31 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad31strng(&self) -> super::vals::Pad31strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad31strng::from_bits(val as u8)
    }
    #[doc = "Pad 31 drive strength."]
    #[inline(always)]
    pub const fn set_pad31strng(&mut self, val: super::vals::Pad31strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 31 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad31fncsel(&self) -> super::vals::Pad31fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad31fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 31 function select."]
    #[inline(always)]
    pub const fn set_pad31fncsel(&mut self, val: super::vals::Pad31fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
}
impl Default for Padregh {
    #[inline(always)]
    fn default() -> Padregh {
        Padregh(0)
    }
}
impl core::fmt::Debug for Padregh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregh")
            .field("pad28pull", &self.pad28pull())
            .field("pad28inpen", &self.pad28inpen())
            .field("pad28strng", &self.pad28strng())
            .field("pad28fncsel", &self.pad28fncsel())
            .field("pad29pull", &self.pad29pull())
            .field("pad29inpen", &self.pad29inpen())
            .field("pad29strng", &self.pad29strng())
            .field("pad29fncsel", &self.pad29fncsel())
            .field("pad30pull", &self.pad30pull())
            .field("pad30inpen", &self.pad30inpen())
            .field("pad30strng", &self.pad30strng())
            .field("pad30fncsel", &self.pad30fncsel())
            .field("pad31pull", &self.pad31pull())
            .field("pad31inpen", &self.pad31inpen())
            .field("pad31strng", &self.pad31strng())
            .field("pad31fncsel", &self.pad31fncsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregh {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregh {{ pad28pull: {=bool:?}, pad28inpen: {=bool:?}, pad28strng: {:?}, pad28fncsel: {:?}, pad29pull: {=bool:?}, pad29inpen: {=bool:?}, pad29strng: {:?}, pad29fncsel: {:?}, pad30pull: {=bool:?}, pad30inpen: {=bool:?}, pad30strng: {:?}, pad30fncsel: {:?}, pad31pull: {=bool:?}, pad31inpen: {=bool:?}, pad31strng: {:?}, pad31fncsel: {:?} }}" , self . pad28pull () , self . pad28inpen () , self . pad28strng () , self . pad28fncsel () , self . pad29pull () , self . pad29inpen () , self . pad29strng () , self . pad29fncsel () , self . pad30pull () , self . pad30inpen () , self . pad30strng () , self . pad30fncsel () , self . pad31pull () , self . pad31inpen () , self . pad31strng () , self . pad31fncsel ())
    }
}
#[doc = "Pad Configuration Register I (Pads 32-25)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregi(pub u32);
impl Padregi {
    #[doc = "Pad 32 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad32pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 32 pullup enable."]
    #[inline(always)]
    pub const fn set_pad32pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 32 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad32inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 32 input enable."]
    #[inline(always)]
    pub const fn set_pad32inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 32 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad32strng(&self) -> super::vals::Pad32strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad32strng::from_bits(val as u8)
    }
    #[doc = "Pad 32 drive strength."]
    #[inline(always)]
    pub const fn set_pad32strng(&mut self, val: super::vals::Pad32strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 32 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad32fncsel(&self) -> super::vals::Pad32fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad32fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 32 function select."]
    #[inline(always)]
    pub const fn set_pad32fncsel(&mut self, val: super::vals::Pad32fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 33 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad33pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 33 pullup enable."]
    #[inline(always)]
    pub const fn set_pad33pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 33 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad33inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 33 input enable."]
    #[inline(always)]
    pub const fn set_pad33inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 33 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad33strng(&self) -> super::vals::Pad33strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad33strng::from_bits(val as u8)
    }
    #[doc = "Pad 33 drive strength."]
    #[inline(always)]
    pub const fn set_pad33strng(&mut self, val: super::vals::Pad33strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 33 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad33fncsel(&self) -> super::vals::Pad33fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad33fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 33 function select."]
    #[inline(always)]
    pub const fn set_pad33fncsel(&mut self, val: super::vals::Pad33fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 34 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad34pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 34 pullup enable."]
    #[inline(always)]
    pub const fn set_pad34pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 34 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad34inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 34 input enable."]
    #[inline(always)]
    pub const fn set_pad34inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 34 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad34strng(&self) -> super::vals::Pad34strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad34strng::from_bits(val as u8)
    }
    #[doc = "Pad 34 drive strength."]
    #[inline(always)]
    pub const fn set_pad34strng(&mut self, val: super::vals::Pad34strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 34 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad34fncsel(&self) -> super::vals::Pad34fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad34fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 34 function select."]
    #[inline(always)]
    pub const fn set_pad34fncsel(&mut self, val: super::vals::Pad34fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 35 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad35pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 35 pullup enable."]
    #[inline(always)]
    pub const fn set_pad35pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 35 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad35inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 35 input enable."]
    #[inline(always)]
    pub const fn set_pad35inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 35 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad35strng(&self) -> super::vals::Pad35strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad35strng::from_bits(val as u8)
    }
    #[doc = "Pad 35 drive strength."]
    #[inline(always)]
    pub const fn set_pad35strng(&mut self, val: super::vals::Pad35strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 35 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad35fncsel(&self) -> super::vals::Pad35fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad35fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 35 function select."]
    #[inline(always)]
    pub const fn set_pad35fncsel(&mut self, val: super::vals::Pad35fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
}
impl Default for Padregi {
    #[inline(always)]
    fn default() -> Padregi {
        Padregi(0)
    }
}
impl core::fmt::Debug for Padregi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregi")
            .field("pad32pull", &self.pad32pull())
            .field("pad32inpen", &self.pad32inpen())
            .field("pad32strng", &self.pad32strng())
            .field("pad32fncsel", &self.pad32fncsel())
            .field("pad33pull", &self.pad33pull())
            .field("pad33inpen", &self.pad33inpen())
            .field("pad33strng", &self.pad33strng())
            .field("pad33fncsel", &self.pad33fncsel())
            .field("pad34pull", &self.pad34pull())
            .field("pad34inpen", &self.pad34inpen())
            .field("pad34strng", &self.pad34strng())
            .field("pad34fncsel", &self.pad34fncsel())
            .field("pad35pull", &self.pad35pull())
            .field("pad35inpen", &self.pad35inpen())
            .field("pad35strng", &self.pad35strng())
            .field("pad35fncsel", &self.pad35fncsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregi {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregi {{ pad32pull: {=bool:?}, pad32inpen: {=bool:?}, pad32strng: {:?}, pad32fncsel: {:?}, pad33pull: {=bool:?}, pad33inpen: {=bool:?}, pad33strng: {:?}, pad33fncsel: {:?}, pad34pull: {=bool:?}, pad34inpen: {=bool:?}, pad34strng: {:?}, pad34fncsel: {:?}, pad35pull: {=bool:?}, pad35inpen: {=bool:?}, pad35strng: {:?}, pad35fncsel: {:?} }}" , self . pad32pull () , self . pad32inpen () , self . pad32strng () , self . pad32fncsel () , self . pad33pull () , self . pad33inpen () , self . pad33strng () , self . pad33fncsel () , self . pad34pull () , self . pad34inpen () , self . pad34strng () , self . pad34fncsel () , self . pad35pull () , self . pad35inpen () , self . pad35strng () , self . pad35fncsel ())
    }
}
#[doc = "Pad Configuration Register J (Pads 36-39)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregj(pub u32);
impl Padregj {
    #[doc = "Pad 36 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad36pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 36 pullup enable."]
    #[inline(always)]
    pub const fn set_pad36pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 36 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad36inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 36 input enable."]
    #[inline(always)]
    pub const fn set_pad36inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 36 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad36strng(&self) -> super::vals::Pad36strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad36strng::from_bits(val as u8)
    }
    #[doc = "Pad 36 drive strength."]
    #[inline(always)]
    pub const fn set_pad36strng(&mut self, val: super::vals::Pad36strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 36 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad36fncsel(&self) -> super::vals::Pad36fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad36fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 36 function select."]
    #[inline(always)]
    pub const fn set_pad36fncsel(&mut self, val: super::vals::Pad36fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 36 VDD power switch enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad36pwrup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 36 VDD power switch enable."]
    #[inline(always)]
    pub const fn set_pad36pwrup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Pad 37 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad37pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 37 pullup enable."]
    #[inline(always)]
    pub const fn set_pad37pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 37 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad37inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 37 input enable."]
    #[inline(always)]
    pub const fn set_pad37inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 37 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad37strng(&self) -> super::vals::Pad37strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad37strng::from_bits(val as u8)
    }
    #[doc = "Pad 37 drive strength."]
    #[inline(always)]
    pub const fn set_pad37strng(&mut self, val: super::vals::Pad37strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 37 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad37fncsel(&self) -> super::vals::Pad37fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad37fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 37 function select."]
    #[inline(always)]
    pub const fn set_pad37fncsel(&mut self, val: super::vals::Pad37fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 37 VSS power switch enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad37pwrdn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 37 VSS power switch enable."]
    #[inline(always)]
    pub const fn set_pad37pwrdn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Pad 38 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad38pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 38 pullup enable."]
    #[inline(always)]
    pub const fn set_pad38pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 38 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad38inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 38 input enable."]
    #[inline(always)]
    pub const fn set_pad38inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 38 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad38strng(&self) -> super::vals::Pad38strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad38strng::from_bits(val as u8)
    }
    #[doc = "Pad 38 drive strength."]
    #[inline(always)]
    pub const fn set_pad38strng(&mut self, val: super::vals::Pad38strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 38 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad38fncsel(&self) -> super::vals::Pad38fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad38fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 38 function select."]
    #[inline(always)]
    pub const fn set_pad38fncsel(&mut self, val: super::vals::Pad38fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 39 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad39pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 39 pullup enable."]
    #[inline(always)]
    pub const fn set_pad39pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 39 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad39inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 39 input enable."]
    #[inline(always)]
    pub const fn set_pad39inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 39 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad39strng(&self) -> super::vals::Pad39strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad39strng::from_bits(val as u8)
    }
    #[doc = "Pad 39 drive strength."]
    #[inline(always)]
    pub const fn set_pad39strng(&mut self, val: super::vals::Pad39strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 39 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad39fncsel(&self) -> super::vals::Pad39fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad39fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 39 function select."]
    #[inline(always)]
    pub const fn set_pad39fncsel(&mut self, val: super::vals::Pad39fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Pad 39 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad39rsel(&self) -> super::vals::Pad39rsel {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Pad39rsel::from_bits(val as u8)
    }
    #[doc = "Pad 39 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad39rsel(&mut self, val: super::vals::Pad39rsel) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Padregj {
    #[inline(always)]
    fn default() -> Padregj {
        Padregj(0)
    }
}
impl core::fmt::Debug for Padregj {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregj")
            .field("pad36pull", &self.pad36pull())
            .field("pad36inpen", &self.pad36inpen())
            .field("pad36strng", &self.pad36strng())
            .field("pad36fncsel", &self.pad36fncsel())
            .field("pad36pwrup", &self.pad36pwrup())
            .field("pad37pull", &self.pad37pull())
            .field("pad37inpen", &self.pad37inpen())
            .field("pad37strng", &self.pad37strng())
            .field("pad37fncsel", &self.pad37fncsel())
            .field("pad37pwrdn", &self.pad37pwrdn())
            .field("pad38pull", &self.pad38pull())
            .field("pad38inpen", &self.pad38inpen())
            .field("pad38strng", &self.pad38strng())
            .field("pad38fncsel", &self.pad38fncsel())
            .field("pad39pull", &self.pad39pull())
            .field("pad39inpen", &self.pad39inpen())
            .field("pad39strng", &self.pad39strng())
            .field("pad39fncsel", &self.pad39fncsel())
            .field("pad39rsel", &self.pad39rsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregj {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregj {{ pad36pull: {=bool:?}, pad36inpen: {=bool:?}, pad36strng: {:?}, pad36fncsel: {:?}, pad36pwrup: {=bool:?}, pad37pull: {=bool:?}, pad37inpen: {=bool:?}, pad37strng: {:?}, pad37fncsel: {:?}, pad37pwrdn: {=bool:?}, pad38pull: {=bool:?}, pad38inpen: {=bool:?}, pad38strng: {:?}, pad38fncsel: {:?}, pad39pull: {=bool:?}, pad39inpen: {=bool:?}, pad39strng: {:?}, pad39fncsel: {:?}, pad39rsel: {:?} }}" , self . pad36pull () , self . pad36inpen () , self . pad36strng () , self . pad36fncsel () , self . pad36pwrup () , self . pad37pull () , self . pad37inpen () , self . pad37strng () , self . pad37fncsel () , self . pad37pwrdn () , self . pad38pull () , self . pad38inpen () , self . pad38strng () , self . pad38fncsel () , self . pad39pull () , self . pad39inpen () , self . pad39strng () , self . pad39fncsel () , self . pad39rsel ())
    }
}
#[doc = "Pad Configuration Register K (Pads 40-43)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregk(pub u32);
impl Padregk {
    #[doc = "Pad 40 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad40pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 40 pullup enable."]
    #[inline(always)]
    pub const fn set_pad40pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 40 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad40inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 40 input enable."]
    #[inline(always)]
    pub const fn set_pad40inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 40 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad40strng(&self) -> super::vals::Pad40strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad40strng::from_bits(val as u8)
    }
    #[doc = "Pad 40 drive strength."]
    #[inline(always)]
    pub const fn set_pad40strng(&mut self, val: super::vals::Pad40strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 40 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad40fncsel(&self) -> super::vals::Pad40fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad40fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 40 function select."]
    #[inline(always)]
    pub const fn set_pad40fncsel(&mut self, val: super::vals::Pad40fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 40 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad40rsel(&self) -> super::vals::Pad40rsel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pad40rsel::from_bits(val as u8)
    }
    #[doc = "Pad 40 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad40rsel(&mut self, val: super::vals::Pad40rsel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Pad 41 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad41pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 41 pullup enable."]
    #[inline(always)]
    pub const fn set_pad41pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 41 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad41inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 41 input enable."]
    #[inline(always)]
    pub const fn set_pad41inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 41 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad41strng(&self) -> super::vals::Pad41strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad41strng::from_bits(val as u8)
    }
    #[doc = "Pad 41 drive strength."]
    #[inline(always)]
    pub const fn set_pad41strng(&mut self, val: super::vals::Pad41strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 41 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad41fncsel(&self) -> super::vals::Pad41fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad41fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 41 function select."]
    #[inline(always)]
    pub const fn set_pad41fncsel(&mut self, val: super::vals::Pad41fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 41 power switch enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad41pwrdn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 41 power switch enable."]
    #[inline(always)]
    pub const fn set_pad41pwrdn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Pad 42 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad42pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 42 pullup enable."]
    #[inline(always)]
    pub const fn set_pad42pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 42 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad42inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 42 input enable."]
    #[inline(always)]
    pub const fn set_pad42inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 42 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad42strng(&self) -> super::vals::Pad42strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad42strng::from_bits(val as u8)
    }
    #[doc = "Pad 42 drive strength."]
    #[inline(always)]
    pub const fn set_pad42strng(&mut self, val: super::vals::Pad42strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 42 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad42fncsel(&self) -> super::vals::Pad42fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad42fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 42 function select."]
    #[inline(always)]
    pub const fn set_pad42fncsel(&mut self, val: super::vals::Pad42fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 42 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad42rsel(&self) -> super::vals::Pad42rsel {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Pad42rsel::from_bits(val as u8)
    }
    #[doc = "Pad 42 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad42rsel(&mut self, val: super::vals::Pad42rsel) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Pad 43 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad43pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 43 pullup enable."]
    #[inline(always)]
    pub const fn set_pad43pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 43 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad43inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 43 input enable."]
    #[inline(always)]
    pub const fn set_pad43inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 43 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad43strng(&self) -> super::vals::Pad43strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad43strng::from_bits(val as u8)
    }
    #[doc = "Pad 43 drive strength."]
    #[inline(always)]
    pub const fn set_pad43strng(&mut self, val: super::vals::Pad43strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 43 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad43fncsel(&self) -> super::vals::Pad43fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad43fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 43 function select."]
    #[inline(always)]
    pub const fn set_pad43fncsel(&mut self, val: super::vals::Pad43fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Pad 43 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad43rsel(&self) -> super::vals::Pad43rsel {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Pad43rsel::from_bits(val as u8)
    }
    #[doc = "Pad 43 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad43rsel(&mut self, val: super::vals::Pad43rsel) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Padregk {
    #[inline(always)]
    fn default() -> Padregk {
        Padregk(0)
    }
}
impl core::fmt::Debug for Padregk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregk")
            .field("pad40pull", &self.pad40pull())
            .field("pad40inpen", &self.pad40inpen())
            .field("pad40strng", &self.pad40strng())
            .field("pad40fncsel", &self.pad40fncsel())
            .field("pad40rsel", &self.pad40rsel())
            .field("pad41pull", &self.pad41pull())
            .field("pad41inpen", &self.pad41inpen())
            .field("pad41strng", &self.pad41strng())
            .field("pad41fncsel", &self.pad41fncsel())
            .field("pad41pwrdn", &self.pad41pwrdn())
            .field("pad42pull", &self.pad42pull())
            .field("pad42inpen", &self.pad42inpen())
            .field("pad42strng", &self.pad42strng())
            .field("pad42fncsel", &self.pad42fncsel())
            .field("pad42rsel", &self.pad42rsel())
            .field("pad43pull", &self.pad43pull())
            .field("pad43inpen", &self.pad43inpen())
            .field("pad43strng", &self.pad43strng())
            .field("pad43fncsel", &self.pad43fncsel())
            .field("pad43rsel", &self.pad43rsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregk {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregk {{ pad40pull: {=bool:?}, pad40inpen: {=bool:?}, pad40strng: {:?}, pad40fncsel: {:?}, pad40rsel: {:?}, pad41pull: {=bool:?}, pad41inpen: {=bool:?}, pad41strng: {:?}, pad41fncsel: {:?}, pad41pwrdn: {=bool:?}, pad42pull: {=bool:?}, pad42inpen: {=bool:?}, pad42strng: {:?}, pad42fncsel: {:?}, pad42rsel: {:?}, pad43pull: {=bool:?}, pad43inpen: {=bool:?}, pad43strng: {:?}, pad43fncsel: {:?}, pad43rsel: {:?} }}" , self . pad40pull () , self . pad40inpen () , self . pad40strng () , self . pad40fncsel () , self . pad40rsel () , self . pad41pull () , self . pad41inpen () , self . pad41strng () , self . pad41fncsel () , self . pad41pwrdn () , self . pad42pull () , self . pad42inpen () , self . pad42strng () , self . pad42fncsel () , self . pad42rsel () , self . pad43pull () , self . pad43inpen () , self . pad43strng () , self . pad43fncsel () , self . pad43rsel ())
    }
}
#[doc = "Pad Configuration Register L (Pads 44-47)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregl(pub u32);
impl Padregl {
    #[doc = "Pad 44 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad44pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 44 pullup enable."]
    #[inline(always)]
    pub const fn set_pad44pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 44 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad44inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 44 input enable."]
    #[inline(always)]
    pub const fn set_pad44inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 44 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad44strng(&self) -> super::vals::Pad44strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad44strng::from_bits(val as u8)
    }
    #[doc = "Pad 44 drive strength."]
    #[inline(always)]
    pub const fn set_pad44strng(&mut self, val: super::vals::Pad44strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 44 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad44fncsel(&self) -> super::vals::Pad44fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad44fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 44 function select."]
    #[inline(always)]
    pub const fn set_pad44fncsel(&mut self, val: super::vals::Pad44fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 45 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad45pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 45 pullup enable."]
    #[inline(always)]
    pub const fn set_pad45pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 45 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad45inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 45 input enable."]
    #[inline(always)]
    pub const fn set_pad45inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 45 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad45strng(&self) -> super::vals::Pad45strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad45strng::from_bits(val as u8)
    }
    #[doc = "Pad 45 drive strength."]
    #[inline(always)]
    pub const fn set_pad45strng(&mut self, val: super::vals::Pad45strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 45 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad45fncsel(&self) -> super::vals::Pad45fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad45fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 45 function select."]
    #[inline(always)]
    pub const fn set_pad45fncsel(&mut self, val: super::vals::Pad45fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 46 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad46pull(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 46 pullup enable."]
    #[inline(always)]
    pub const fn set_pad46pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pad 46 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad46inpen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 46 input enable."]
    #[inline(always)]
    pub const fn set_pad46inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pad 46 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad46strng(&self) -> super::vals::Pad46strng {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pad46strng::from_bits(val as u8)
    }
    #[doc = "Pad 46 drive strength."]
    #[inline(always)]
    pub const fn set_pad46strng(&mut self, val: super::vals::Pad46strng) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pad 46 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad46fncsel(&self) -> super::vals::Pad46fncsel {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Pad46fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 46 function select."]
    #[inline(always)]
    pub const fn set_pad46fncsel(&mut self, val: super::vals::Pad46fncsel) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad 47 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad47pull(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 47 pullup enable."]
    #[inline(always)]
    pub const fn set_pad47pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pad 47 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad47inpen(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 47 input enable."]
    #[inline(always)]
    pub const fn set_pad47inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pad 47 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad47strng(&self) -> super::vals::Pad47strng {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pad47strng::from_bits(val as u8)
    }
    #[doc = "Pad 47 drive strength."]
    #[inline(always)]
    pub const fn set_pad47strng(&mut self, val: super::vals::Pad47strng) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pad 47 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad47fncsel(&self) -> super::vals::Pad47fncsel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pad47fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 47 function select."]
    #[inline(always)]
    pub const fn set_pad47fncsel(&mut self, val: super::vals::Pad47fncsel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
}
impl Default for Padregl {
    #[inline(always)]
    fn default() -> Padregl {
        Padregl(0)
    }
}
impl core::fmt::Debug for Padregl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregl")
            .field("pad44pull", &self.pad44pull())
            .field("pad44inpen", &self.pad44inpen())
            .field("pad44strng", &self.pad44strng())
            .field("pad44fncsel", &self.pad44fncsel())
            .field("pad45pull", &self.pad45pull())
            .field("pad45inpen", &self.pad45inpen())
            .field("pad45strng", &self.pad45strng())
            .field("pad45fncsel", &self.pad45fncsel())
            .field("pad46pull", &self.pad46pull())
            .field("pad46inpen", &self.pad46inpen())
            .field("pad46strng", &self.pad46strng())
            .field("pad46fncsel", &self.pad46fncsel())
            .field("pad47pull", &self.pad47pull())
            .field("pad47inpen", &self.pad47inpen())
            .field("pad47strng", &self.pad47strng())
            .field("pad47fncsel", &self.pad47fncsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregl {{ pad44pull: {=bool:?}, pad44inpen: {=bool:?}, pad44strng: {:?}, pad44fncsel: {:?}, pad45pull: {=bool:?}, pad45inpen: {=bool:?}, pad45strng: {:?}, pad45fncsel: {:?}, pad46pull: {=bool:?}, pad46inpen: {=bool:?}, pad46strng: {:?}, pad46fncsel: {:?}, pad47pull: {=bool:?}, pad47inpen: {=bool:?}, pad47strng: {:?}, pad47fncsel: {:?} }}" , self . pad44pull () , self . pad44inpen () , self . pad44strng () , self . pad44fncsel () , self . pad45pull () , self . pad45inpen () , self . pad45strng () , self . pad45fncsel () , self . pad46pull () , self . pad46inpen () , self . pad46strng () , self . pad46fncsel () , self . pad47pull () , self . pad47inpen () , self . pad47strng () , self . pad47fncsel ())
    }
}
#[doc = "Pad Configuration Register M (Pads 47-48)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padregm(pub u32);
impl Padregm {
    #[doc = "Pad 48 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad48pull(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 48 pullup enable."]
    #[inline(always)]
    pub const fn set_pad48pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pad 48 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad48inpen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 48 input enable."]
    #[inline(always)]
    pub const fn set_pad48inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pad 48 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad48strng(&self) -> super::vals::Pad48strng {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pad48strng::from_bits(val as u8)
    }
    #[doc = "Pad 48 drive strength."]
    #[inline(always)]
    pub const fn set_pad48strng(&mut self, val: super::vals::Pad48strng) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pad 48 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad48fncsel(&self) -> super::vals::Pad48fncsel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Pad48fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 48 function select."]
    #[inline(always)]
    pub const fn set_pad48fncsel(&mut self, val: super::vals::Pad48fncsel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad 48 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad48rsel(&self) -> super::vals::Pad48rsel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pad48rsel::from_bits(val as u8)
    }
    #[doc = "Pad 48 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad48rsel(&mut self, val: super::vals::Pad48rsel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Pad 49 pullup enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad49pull(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 49 pullup enable."]
    #[inline(always)]
    pub const fn set_pad49pull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pad 49 input enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pad49inpen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pad 49 input enable."]
    #[inline(always)]
    pub const fn set_pad49inpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pad 49 drive strength."]
    #[must_use]
    #[inline(always)]
    pub const fn pad49strng(&self) -> super::vals::Pad49strng {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pad49strng::from_bits(val as u8)
    }
    #[doc = "Pad 49 drive strength."]
    #[inline(always)]
    pub const fn set_pad49strng(&mut self, val: super::vals::Pad49strng) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pad 49 function select."]
    #[must_use]
    #[inline(always)]
    pub const fn pad49fncsel(&self) -> super::vals::Pad49fncsel {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Pad49fncsel::from_bits(val as u8)
    }
    #[doc = "Pad 49 function select."]
    #[inline(always)]
    pub const fn set_pad49fncsel(&mut self, val: super::vals::Pad49fncsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Pad 49 pullup resistor selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pad49rsel(&self) -> super::vals::Pad49rsel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pad49rsel::from_bits(val as u8)
    }
    #[doc = "Pad 49 pullup resistor selection."]
    #[inline(always)]
    pub const fn set_pad49rsel(&mut self, val: super::vals::Pad49rsel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for Padregm {
    #[inline(always)]
    fn default() -> Padregm {
        Padregm(0)
    }
}
impl core::fmt::Debug for Padregm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padregm")
            .field("pad48pull", &self.pad48pull())
            .field("pad48inpen", &self.pad48inpen())
            .field("pad48strng", &self.pad48strng())
            .field("pad48fncsel", &self.pad48fncsel())
            .field("pad48rsel", &self.pad48rsel())
            .field("pad49pull", &self.pad49pull())
            .field("pad49inpen", &self.pad49inpen())
            .field("pad49strng", &self.pad49strng())
            .field("pad49fncsel", &self.pad49fncsel())
            .field("pad49rsel", &self.pad49rsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padregm {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padregm {{ pad48pull: {=bool:?}, pad48inpen: {=bool:?}, pad48strng: {:?}, pad48fncsel: {:?}, pad48rsel: {:?}, pad49pull: {=bool:?}, pad49inpen: {=bool:?}, pad49strng: {:?}, pad49fncsel: {:?}, pad49rsel: {:?} }}" , self . pad48pull () , self . pad48inpen () , self . pad48strng () , self . pad48fncsel () , self . pad48rsel () , self . pad49pull () , self . pad49inpen () , self . pad49strng () , self . pad49fncsel () , self . pad49rsel ())
    }
}
#[doc = "GPIO Input Register B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdb(pub u32);
impl Rdb {
    #[doc = "GPIO49-32 read data."]
    #[must_use]
    #[inline(always)]
    pub const fn rdb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "GPIO49-32 read data."]
    #[inline(always)]
    pub const fn set_rdb(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Rdb {
    #[inline(always)]
    fn default() -> Rdb {
        Rdb(0)
    }
}
impl core::fmt::Debug for Rdb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdb").field("rdb", &self.rdb()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdb {{ rdb: {=u32:?} }}", self.rdb())
    }
}
#[doc = "SCARD Card Detect select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scdet(pub u32);
impl Scdet {
    #[doc = "SCARD card detect pad select."]
    #[must_use]
    #[inline(always)]
    pub const fn scdet(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "SCARD card detect pad select."]
    #[inline(always)]
    pub const fn set_scdet(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Scdet {
    #[inline(always)]
    fn default() -> Scdet {
        Scdet(0)
    }
}
impl core::fmt::Debug for Scdet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scdet")
            .field("scdet", &self.scdet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scdet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Scdet {{ scdet: {=u8:?} }}", self.scdet())
    }
}
#[doc = "STIMER Capture Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmrcap(pub u32);
impl Stmrcap {
    #[doc = "STIMER Capture 0 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn stsel0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "STIMER Capture 0 Select."]
    #[inline(always)]
    pub const fn set_stsel0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "STIMER Capture 0 Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn stpol0(&self) -> super::vals::Stpol0 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Stpol0::from_bits(val as u8)
    }
    #[doc = "STIMER Capture 0 Polarity."]
    #[inline(always)]
    pub const fn set_stpol0(&mut self, val: super::vals::Stpol0) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "STIMER Capture 1 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn stsel1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "STIMER Capture 1 Select."]
    #[inline(always)]
    pub const fn set_stsel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "STIMER Capture 1 Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn stpol1(&self) -> super::vals::Stpol1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Stpol1::from_bits(val as u8)
    }
    #[doc = "STIMER Capture 1 Polarity."]
    #[inline(always)]
    pub const fn set_stpol1(&mut self, val: super::vals::Stpol1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "STIMER Capture 2 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn stsel2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "STIMER Capture 2 Select."]
    #[inline(always)]
    pub const fn set_stsel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "STIMER Capture 2 Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn stpol2(&self) -> super::vals::Stpol2 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Stpol2::from_bits(val as u8)
    }
    #[doc = "STIMER Capture 2 Polarity."]
    #[inline(always)]
    pub const fn set_stpol2(&mut self, val: super::vals::Stpol2) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "STIMER Capture 3 Select."]
    #[must_use]
    #[inline(always)]
    pub const fn stsel3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "STIMER Capture 3 Select."]
    #[inline(always)]
    pub const fn set_stsel3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "STIMER Capture 3 Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn stpol3(&self) -> super::vals::Stpol3 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Stpol3::from_bits(val as u8)
    }
    #[doc = "STIMER Capture 3 Polarity."]
    #[inline(always)]
    pub const fn set_stpol3(&mut self, val: super::vals::Stpol3) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Stmrcap {
    #[inline(always)]
    fn default() -> Stmrcap {
        Stmrcap(0)
    }
}
impl core::fmt::Debug for Stmrcap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stmrcap")
            .field("stsel0", &self.stsel0())
            .field("stpol0", &self.stpol0())
            .field("stsel1", &self.stsel1())
            .field("stpol1", &self.stpol1())
            .field("stsel2", &self.stsel2())
            .field("stpol2", &self.stpol2())
            .field("stsel3", &self.stsel3())
            .field("stpol3", &self.stpol3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stmrcap {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Stmrcap {{ stsel0: {=u8:?}, stpol0: {:?}, stsel1: {=u8:?}, stpol1: {:?}, stsel2: {=u8:?}, stpol2: {:?}, stsel3: {=u8:?}, stpol3: {:?} }}" , self . stsel0 () , self . stpol0 () , self . stsel1 () , self . stpol1 () , self . stsel2 () , self . stpol2 () , self . stsel3 () , self . stpol3 ())
    }
}
#[doc = "GPIO Output Register B."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wtb(pub u32);
impl Wtb {
    #[doc = "GPIO49-32 write data."]
    #[must_use]
    #[inline(always)]
    pub const fn wtb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "GPIO49-32 write data."]
    #[inline(always)]
    pub const fn set_wtb(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Wtb {
    #[inline(always)]
    fn default() -> Wtb {
        Wtb(0)
    }
}
impl core::fmt::Debug for Wtb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wtb").field("wtb", &self.wtb()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wtb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wtb {{ wtb: {=u32:?} }}", self.wtb())
    }
}
#[doc = "GPIO Output Register B Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wtcb(pub u32);
impl Wtcb {
    #[doc = "Clear the GPIO49-32 write data."]
    #[must_use]
    #[inline(always)]
    pub const fn wtcb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Clear the GPIO49-32 write data."]
    #[inline(always)]
    pub const fn set_wtcb(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Wtcb {
    #[inline(always)]
    fn default() -> Wtcb {
        Wtcb(0)
    }
}
impl core::fmt::Debug for Wtcb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wtcb").field("wtcb", &self.wtcb()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wtcb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wtcb {{ wtcb: {=u32:?} }}", self.wtcb())
    }
}
#[doc = "GPIO Output Register B Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wtsb(pub u32);
impl Wtsb {
    #[doc = "Set the GPIO49-32 write data."]
    #[must_use]
    #[inline(always)]
    pub const fn wtsb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Set the GPIO49-32 write data."]
    #[inline(always)]
    pub const fn set_wtsb(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Wtsb {
    #[inline(always)]
    fn default() -> Wtsb {
        Wtsb(0)
    }
}
impl core::fmt::Debug for Wtsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wtsb").field("wtsb", &self.wtsb()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wtsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wtsb {{ wtsb: {=u32:?} }}", self.wtsb())
    }
}

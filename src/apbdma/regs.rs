#[doc = "PIO Input Values."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bbinput(pub u32);
impl Bbinput {
    #[doc = "PIO values."]
    #[must_use]
    #[inline(always)]
    pub const fn datain(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "PIO values."]
    #[inline(always)]
    pub const fn set_datain(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Bbinput {
    #[inline(always)]
    fn default() -> Bbinput {
        Bbinput(0)
    }
}
impl core::fmt::Debug for Bbinput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bbinput")
            .field("datain", &self.datain())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bbinput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bbinput {{ datain: {=u8:?} }}", self.datain())
    }
}
#[doc = "Set/Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bbsetclear(pub u32);
impl Bbsetclear {
    #[doc = "Write 1 to Set PIO value (set hier priority than clear if both bit set)."]
    #[must_use]
    #[inline(always)]
    pub const fn set(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write 1 to Set PIO value (set hier priority than clear if both bit set)."]
    #[inline(always)]
    pub const fn set_set(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Write 1 to Clear PIO value."]
    #[must_use]
    #[inline(always)]
    pub const fn clear(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Write 1 to Clear PIO value."]
    #[inline(always)]
    pub const fn set_clear(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Bbsetclear {
    #[inline(always)]
    fn default() -> Bbsetclear {
        Bbsetclear(0)
    }
}
impl core::fmt::Debug for Bbsetclear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bbsetclear")
            .field("set", &self.set())
            .field("clear", &self.clear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bbsetclear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bbsetclear {{ set: {=u8:?}, clear: {=u8:?} }}",
            self.set(),
            self.clear()
        )
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bbvalue(pub u32);
impl Bbvalue {
    #[doc = "Data Output Values."]
    #[must_use]
    #[inline(always)]
    pub const fn dataout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Output Values."]
    #[inline(always)]
    pub const fn set_dataout(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "PIO values."]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PIO values."]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Bbvalue {
    #[inline(always)]
    fn default() -> Bbvalue {
        Bbvalue(0)
    }
}
impl core::fmt::Debug for Bbvalue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bbvalue")
            .field("dataout", &self.dataout())
            .field("pin", &self.pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bbvalue {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bbvalue {{ dataout: {=u8:?}, pin: {=u8:?} }}",
            self.dataout(),
            self.pin()
        )
    }
}
#[doc = "PIO Input Values."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug(pub u32);
impl Debug {
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debugen(&self) -> super::vals::Debugen {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Debugen::from_bits(val as u8)
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_debugen(&mut self, val: super::vals::Debugen) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Debug {
    #[inline(always)]
    fn default() -> Debug {
        Debug(0)
    }
}
impl core::fmt::Debug for Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug")
            .field("debugen", &self.debugen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Debug {{ debugen: {:?} }}", self.debugen())
    }
}

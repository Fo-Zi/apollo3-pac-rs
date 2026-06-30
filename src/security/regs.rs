#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Function Enable. Software should set the ENABLE bit to initiate a CRC operation. Hardware will clear the ENABLE bit upon completion."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Function Enable. Software should set the ENABLE bit to initiate a CRC operation. Hardware will clear the ENABLE bit upon completion."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn function(&self) -> super::vals::Function {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Function::from_bits(val as u8)
    }
    #[doc = "Function Select."]
    #[inline(always)]
    pub const fn set_function(&mut self, val: super::vals::Function) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "CRC Error Status - Set to 1 if an error occurs during a CRC operation. Cleared when CTRL register is written (with any value). Usually indicates an invalid address range."]
    #[must_use]
    #[inline(always)]
    pub const fn crcerror(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Error Status - Set to 1 if an error occurs during a CRC operation. Cleared when CTRL register is written (with any value). Usually indicates an invalid address range."]
    #[inline(always)]
    pub const fn set_crcerror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("enable", &self.enable())
            .field("function", &self.function())
            .field("crcerror", &self.crcerror())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ enable: {=bool:?}, function: {:?}, crcerror: {=bool:?} }}",
            self.enable(),
            self.function(),
            self.crcerror()
        )
    }
}
#[doc = "Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Len(pub u32);
impl Len {
    #[doc = "Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)."]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)."]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 2usize)) | (((val as u32) & 0x0003_ffff) << 2usize);
    }
}
impl Default for Len {
    #[inline(always)]
    fn default() -> Len {
        Len(0)
    }
}
impl core::fmt::Debug for Len {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Len").field("len", &self.len()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Len {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Len {{ len: {=u32:?} }}", self.len())
    }
}
#[doc = "LOCK Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lockctrl(pub u32);
impl Lockctrl {
    #[doc = "LOCK Function Select register."]
    #[must_use]
    #[inline(always)]
    pub const fn select(&self) -> super::vals::Select {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Select::from_bits(val as u8)
    }
    #[doc = "LOCK Function Select register."]
    #[inline(always)]
    pub const fn set_select(&mut self, val: super::vals::Select) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Lockctrl {
    #[inline(always)]
    fn default() -> Lockctrl {
        Lockctrl(0)
    }
}
impl core::fmt::Debug for Lockctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lockctrl")
            .field("select", &self.select())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lockctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lockctrl {{ select: {:?} }}", self.select())
    }
}
#[doc = "LOCK Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lockstat(pub u32);
impl Lockstat {
    #[doc = "LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource."]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> super::vals::Status {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Status::from_bits(val as u32)
    }
    #[doc = "LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource."]
    #[inline(always)]
    pub const fn set_status(&mut self, val: super::vals::Status) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lockstat {
    #[inline(always)]
    fn default() -> Lockstat {
        Lockstat(0)
    }
}
impl core::fmt::Debug for Lockstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lockstat")
            .field("status", &self.status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lockstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lockstat {{ status: {:?} }}", self.status())
    }
}

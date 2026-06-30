#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Function {
    #[doc = "Perform CRC32 operation value."]
    Crc32 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Function {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Function {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Function {
    #[inline(always)]
    fn from(val: u8) -> Function {
        Function::from_bits(val)
    }
}
impl From<Function> for u8 {
    #[inline(always)]
    fn from(val: Function) -> u8 {
        Function::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Select(u8);
impl Select {
    #[doc = "Lock Control should be set to NONE when not in use. value."]
    pub const None: Self = Self(0x0);
    #[doc = "Unlock Customer Key (access to top half of info0) value."]
    pub const CustomerKey: Self = Self(0x01);
}
impl Select {
    pub const fn from_bits(val: u8) -> Select {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Select {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("None"),
            0x01 => f.write_str("CustomerKey"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Select {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "None"),
            0x01 => defmt::write!(f, "CustomerKey"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Select {
    #[inline(always)]
    fn from(val: u8) -> Select {
        Select::from_bits(val)
    }
}
impl From<Select> for u8 {
    #[inline(always)]
    fn from(val: Select) -> u8 {
        Select::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Status(u32);
impl Status {
    #[doc = "No resources are unlocked value."]
    pub const None: Self = Self(0x0);
    #[doc = "Customer Key is unlocked (access is granted to top half of info0) value."]
    pub const CustomerKey: Self = Self(0x01);
}
impl Status {
    pub const fn from_bits(val: u32) -> Status {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("None"),
            0x01 => f.write_str("CustomerKey"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "None"),
            0x01 => defmt::write!(f, "CustomerKey"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Status {
    #[inline(always)]
    fn from(val: u32) -> Status {
        Status::from_bits(val)
    }
}
impl From<Status> for u32 {
    #[inline(always)]
    fn from(val: Status) -> u32 {
        Status::to_bits(val)
    }
}

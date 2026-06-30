#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg0 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A0OUT value."]
    A0out = 0x02,
    #[doc = "Output is B2OUT2. value."]
    B2out2 = 0x03,
    #[doc = "Output is A5OUT2. value."]
    A5out2 = 0x04,
    #[doc = "Output is A6OUT. value."]
    A6out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg0 {
    #[inline(always)]
    fn from(val: u8) -> Cfg0 {
        Cfg0::from_bits(val)
    }
}
impl From<Cfg0> for u8 {
    #[inline(always)]
    fn from(val: Cfg0) -> u8 {
        Cfg0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg1 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A0OUT2 value."]
    A0out2 = 0x02,
    #[doc = "Output is A0OUT. value."]
    A0out = 0x03,
    #[doc = "Output is A5OUT. value."]
    A5out = 0x04,
    #[doc = "Output is B7OUT2. value."]
    B7out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg1 {
    #[inline(always)]
    fn from(val: u8) -> Cfg1 {
        Cfg1::from_bits(val)
    }
}
impl From<Cfg1> for u8 {
    #[inline(always)]
    fn from(val: Cfg1) -> u8 {
        Cfg1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg10 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B2OUT value."]
    B2out = 0x02,
    #[doc = "Output is B3OUT2. value."]
    B3out2 = 0x03,
    #[doc = "Output is B4OUT2. value."]
    B4out2 = 0x04,
    #[doc = "Output is A6OUT. value."]
    A6out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg10 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg10 {
    #[inline(always)]
    fn from(val: u8) -> Cfg10 {
        Cfg10::from_bits(val)
    }
}
impl From<Cfg10> for u8 {
    #[inline(always)]
    fn from(val: Cfg10) -> u8 {
        Cfg10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg11 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B2OUT2 value."]
    B2out2 = 0x02,
    #[doc = "Output is B2OUT. value."]
    B2out = 0x03,
    #[doc = "Output is B4OUT. value."]
    B4out = 0x04,
    #[doc = "Output is B5OUT2. value."]
    B5out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg11 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg11 {
    #[inline(always)]
    fn from(val: u8) -> Cfg11 {
        Cfg11::from_bits(val)
    }
}
impl From<Cfg11> for u8 {
    #[inline(always)]
    fn from(val: Cfg11) -> u8 {
        Cfg11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg12 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A3OUT value."]
    A3out = 0x02,
    #[doc = "Output is B1OUT. value."]
    B1out = 0x03,
    #[doc = "Output is B0OUT2. value."]
    B0out2 = 0x04,
    #[doc = "Output is B6OUT2. value."]
    B6out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg12 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg12 {
    #[inline(always)]
    fn from(val: u8) -> Cfg12 {
        Cfg12::from_bits(val)
    }
}
impl From<Cfg12> for u8 {
    #[inline(always)]
    fn from(val: Cfg12) -> u8 {
        Cfg12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg13 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A3OUT2 value."]
    A3out2 = 0x02,
    #[doc = "Output is A3OUT. value."]
    A3out = 0x03,
    #[doc = "Output is A6OUT. value."]
    A6out = 0x04,
    #[doc = "Output is B4OUT2. value."]
    B4out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg13 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg13 {
    #[inline(always)]
    fn from(val: u8) -> Cfg13 {
        Cfg13::from_bits(val)
    }
}
impl From<Cfg13> for u8 {
    #[inline(always)]
    fn from(val: Cfg13) -> u8 {
        Cfg13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg14 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B3OUT value."]
    B3out = 0x02,
    #[doc = "Output is B1OUT. value."]
    B1out = 0x03,
    #[doc = "Output is B7OUT2. value."]
    B7out2 = 0x04,
    #[doc = "Output is A7OUT. value."]
    A7out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg14 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg14 {
    #[inline(always)]
    fn from(val: u8) -> Cfg14 {
        Cfg14::from_bits(val)
    }
}
impl From<Cfg14> for u8 {
    #[inline(always)]
    fn from(val: Cfg14) -> u8 {
        Cfg14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg15 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B3OUT2 value."]
    B3out2 = 0x02,
    #[doc = "Output is B3OUT. value."]
    B3out = 0x03,
    #[doc = "Output is A7OUT. value."]
    A7out = 0x04,
    #[doc = "Output is A4OUT2. value."]
    A4out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg15 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg15 {
    #[inline(always)]
    fn from(val: u8) -> Cfg15 {
        Cfg15::from_bits(val)
    }
}
impl From<Cfg15> for u8 {
    #[inline(always)]
    fn from(val: Cfg15) -> u8 {
        Cfg15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg16 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A4OUT value."]
    A4out = 0x02,
    #[doc = "Output is A0OUT. value."]
    A0out = 0x03,
    #[doc = "Output is A0OUT2. value."]
    A0out2 = 0x04,
    #[doc = "Output is B3OUT2. value."]
    B3out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg16 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg16 {
    #[inline(always)]
    fn from(val: u8) -> Cfg16 {
        Cfg16::from_bits(val)
    }
}
impl From<Cfg16> for u8 {
    #[inline(always)]
    fn from(val: Cfg16) -> u8 {
        Cfg16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg17 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A4OUT2 value."]
    A4out2 = 0x02,
    #[doc = "Output is B7OUT. value."]
    B7out = 0x03,
    #[doc = "Output is A4OUT. value."]
    A4out = 0x04,
    #[doc = "Output is A1OUT2. value."]
    A1out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg17 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg17 {
    #[inline(always)]
    fn from(val: u8) -> Cfg17 {
        Cfg17::from_bits(val)
    }
}
impl From<Cfg17> for u8 {
    #[inline(always)]
    fn from(val: Cfg17) -> u8 {
        Cfg17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg18 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B4OUT value."]
    B4out = 0x02,
    #[doc = "Output is B0OUT. value."]
    B0out = 0x03,
    #[doc = "Output is A0OUT. value."]
    A0out = 0x04,
    #[doc = "Output is A3OUT2. value."]
    A3out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg18 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg18 {
    #[inline(always)]
    fn from(val: u8) -> Cfg18 {
        Cfg18::from_bits(val)
    }
}
impl From<Cfg18> for u8 {
    #[inline(always)]
    fn from(val: Cfg18) -> u8 {
        Cfg18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg19 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B4OUT2 value."]
    B4out2 = 0x02,
    #[doc = "Output is A2OUT. value."]
    A2out = 0x03,
    #[doc = "Output is B4OUT. value."]
    B4out = 0x04,
    #[doc = "Output is B1OUT2. value."]
    B1out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg19 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg19 {
    #[inline(always)]
    fn from(val: u8) -> Cfg19 {
        Cfg19::from_bits(val)
    }
}
impl From<Cfg19> for u8 {
    #[inline(always)]
    fn from(val: Cfg19) -> u8 {
        Cfg19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg2 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B0OUT value."]
    B0out = 0x02,
    #[doc = "Output is B1OUT2. value."]
    B1out2 = 0x03,
    #[doc = "Output is B6OUT2. value."]
    B6out2 = 0x04,
    #[doc = "Output is A7OUT. value."]
    A7out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg2 {
    #[inline(always)]
    fn from(val: u8) -> Cfg2 {
        Cfg2::from_bits(val)
    }
}
impl From<Cfg2> for u8 {
    #[inline(always)]
    fn from(val: Cfg2) -> u8 {
        Cfg2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg20 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A5OUT value."]
    A5out = 0x02,
    #[doc = "Output is A1OUT. value."]
    A1out = 0x03,
    #[doc = "Output is A1OUT2. value."]
    A1out2 = 0x04,
    #[doc = "Output is B2OUT2. value."]
    B2out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg20 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg20 {
    #[inline(always)]
    fn from(val: u8) -> Cfg20 {
        Cfg20::from_bits(val)
    }
}
impl From<Cfg20> for u8 {
    #[inline(always)]
    fn from(val: Cfg20) -> u8 {
        Cfg20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg21 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A5OUT2 value."]
    A5out2 = 0x02,
    #[doc = "Output is A1OUT. value."]
    A1out = 0x03,
    #[doc = "Output is B5OUT. value."]
    B5out = 0x04,
    #[doc = "Output is A0OUT2. value."]
    A0out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg21 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg21 {
    #[inline(always)]
    fn from(val: u8) -> Cfg21 {
        Cfg21::from_bits(val)
    }
}
impl From<Cfg21> for u8 {
    #[inline(always)]
    fn from(val: Cfg21) -> u8 {
        Cfg21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg22 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B5OUT value."]
    B5out = 0x02,
    #[doc = "Output is A6OUT. value."]
    A6out = 0x03,
    #[doc = "Output is A1OUT. value."]
    A1out = 0x04,
    #[doc = "Output is A2OUT2. value."]
    A2out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg22 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg22 {
    #[inline(always)]
    fn from(val: u8) -> Cfg22 {
        Cfg22::from_bits(val)
    }
}
impl From<Cfg22> for u8 {
    #[inline(always)]
    fn from(val: Cfg22) -> u8 {
        Cfg22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg23 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B5OUT2 value."]
    B5out2 = 0x02,
    #[doc = "Output is A7OUT. value."]
    A7out = 0x03,
    #[doc = "Output is A5OUT. value."]
    A5out = 0x04,
    #[doc = "Output is B0OUT2. value."]
    B0out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg23 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg23 {
    #[inline(always)]
    fn from(val: u8) -> Cfg23 {
        Cfg23::from_bits(val)
    }
}
impl From<Cfg23> for u8 {
    #[inline(always)]
    fn from(val: Cfg23) -> u8 {
        Cfg23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg24 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A6OUT value."]
    A6out = 0x02,
    #[doc = "Output is A2OUT. value."]
    A2out = 0x03,
    #[doc = "Output is A1OUT. value."]
    A1out = 0x04,
    #[doc = "Output is B1OUT2. value."]
    B1out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg24 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg24 {
    #[inline(always)]
    fn from(val: u8) -> Cfg24 {
        Cfg24::from_bits(val)
    }
}
impl From<Cfg24> for u8 {
    #[inline(always)]
    fn from(val: Cfg24) -> u8 {
        Cfg24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg25 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B4OUT2 value."]
    B4out2 = 0x02,
    #[doc = "Output is B2OUT. value."]
    B2out = 0x03,
    #[doc = "Output is A6OUT. value."]
    A6out = 0x04,
    #[doc = "Output is A2OUT2. value."]
    A2out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg25 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg25 {
    #[inline(always)]
    fn from(val: u8) -> Cfg25 {
        Cfg25::from_bits(val)
    }
}
impl From<Cfg25> for u8 {
    #[inline(always)]
    fn from(val: Cfg25) -> u8 {
        Cfg25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg26 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B6OUT value."]
    B6out = 0x02,
    #[doc = "Output is B2OUT. value."]
    B2out = 0x03,
    #[doc = "Output is A5OUT. value."]
    A5out = 0x04,
    #[doc = "Output is A1OUT2. value."]
    A1out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg26 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg26 {
    #[inline(always)]
    fn from(val: u8) -> Cfg26 {
        Cfg26::from_bits(val)
    }
}
impl From<Cfg26> for u8 {
    #[inline(always)]
    fn from(val: Cfg26) -> u8 {
        Cfg26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg27 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B6OUT2 value."]
    B6out2 = 0x02,
    #[doc = "Output is A1OUT. value."]
    A1out = 0x03,
    #[doc = "Output is B6OUT. value."]
    B6out = 0x04,
    #[doc = "Output is B2OUT2. value."]
    B2out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg27 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg27 {
    #[inline(always)]
    fn from(val: u8) -> Cfg27 {
        Cfg27::from_bits(val)
    }
}
impl From<Cfg27> for u8 {
    #[inline(always)]
    fn from(val: Cfg27) -> u8 {
        Cfg27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg28 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A7OUT value."]
    A7out = 0x02,
    #[doc = "Output is A3OUT. value."]
    A3out = 0x03,
    #[doc = "Output is A5OUT2. value."]
    A5out2 = 0x04,
    #[doc = "Output is B0OUT2. value."]
    B0out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg28 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg28 {
    #[inline(always)]
    fn from(val: u8) -> Cfg28 {
        Cfg28::from_bits(val)
    }
}
impl From<Cfg28> for u8 {
    #[inline(always)]
    fn from(val: Cfg28) -> u8 {
        Cfg28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg29 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B5OUT2 value."]
    B5out2 = 0x02,
    #[doc = "Output is A1OUT. value."]
    A1out = 0x03,
    #[doc = "Output is A7OUT. value."]
    A7out = 0x04,
    #[doc = "Output is A3OUT2. value."]
    A3out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg29 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg29 {
    #[inline(always)]
    fn from(val: u8) -> Cfg29 {
        Cfg29::from_bits(val)
    }
}
impl From<Cfg29> for u8 {
    #[inline(always)]
    fn from(val: Cfg29) -> u8 {
        Cfg29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg3 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B0OUT2 value."]
    B0out2 = 0x02,
    #[doc = "Output is B0OUT. value."]
    B0out = 0x03,
    #[doc = "Output is A1OUT. value."]
    A1out = 0x04,
    #[doc = "Output is A6OUT. value."]
    A6out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg3 {
    #[inline(always)]
    fn from(val: u8) -> Cfg3 {
        Cfg3::from_bits(val)
    }
}
impl From<Cfg3> for u8 {
    #[inline(always)]
    fn from(val: Cfg3) -> u8 {
        Cfg3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg30 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B7OUT value."]
    B7out = 0x02,
    #[doc = "Output is B3OUT. value."]
    B3out = 0x03,
    #[doc = "Output is A4OUT2. value."]
    A4out2 = 0x04,
    #[doc = "Output is A0OUT2. value."]
    A0out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg30 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg30 {
    #[inline(always)]
    fn from(val: u8) -> Cfg30 {
        Cfg30::from_bits(val)
    }
}
impl From<Cfg30> for u8 {
    #[inline(always)]
    fn from(val: Cfg30) -> u8 {
        Cfg30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg31 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B7OUT2 value."]
    B7out2 = 0x02,
    #[doc = "Output is A6OUT. value."]
    A6out = 0x03,
    #[doc = "Output is B7OUT. value."]
    B7out = 0x04,
    #[doc = "Output is B3OUT2. value."]
    B3out2 = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg31 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg31 {
    #[inline(always)]
    fn from(val: u8) -> Cfg31 {
        Cfg31::from_bits(val)
    }
}
impl From<Cfg31> for u8 {
    #[inline(always)]
    fn from(val: Cfg31) -> u8 {
        Cfg31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg4 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A1OUT value."]
    A1out = 0x02,
    #[doc = "Output is A2OUT2. value."]
    A2out2 = 0x03,
    #[doc = "Output is A5OUT2. value."]
    A5out2 = 0x04,
    #[doc = "Output is B5OUT. value."]
    B5out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg4 {
    #[inline(always)]
    fn from(val: u8) -> Cfg4 {
        Cfg4::from_bits(val)
    }
}
impl From<Cfg4> for u8 {
    #[inline(always)]
    fn from(val: Cfg4) -> u8 {
        Cfg4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg5 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A1OUT2 value."]
    A1out2 = 0x02,
    #[doc = "Output is A1OUT. value."]
    A1out = 0x03,
    #[doc = "Output is A5OUT. value."]
    B6out = 0x04,
    #[doc = "Output is A7OUT. value."]
    A7out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg5 {
    #[inline(always)]
    fn from(val: u8) -> Cfg5 {
        Cfg5::from_bits(val)
    }
}
impl From<Cfg5> for u8 {
    #[inline(always)]
    fn from(val: Cfg5) -> u8 {
        Cfg5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg6 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B1OUT value."]
    B1out = 0x02,
    #[doc = "Output is A1OUT. value."]
    A1out = 0x03,
    #[doc = "Output is B5OUT2. value."]
    B5out2 = 0x04,
    #[doc = "Output is B7OUT. value."]
    B7out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg6 {
    #[inline(always)]
    fn from(val: u8) -> Cfg6 {
        Cfg6::from_bits(val)
    }
}
impl From<Cfg6> for u8 {
    #[inline(always)]
    fn from(val: Cfg6) -> u8 {
        Cfg6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg7 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is B1OUT2 value."]
    B1out2 = 0x02,
    #[doc = "Output is B1OUT. value."]
    B1out = 0x03,
    #[doc = "Output is B5OUT. value."]
    B5out = 0x04,
    #[doc = "Output is A7OUT. value."]
    A7out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg7 {
    #[inline(always)]
    fn from(val: u8) -> Cfg7 {
        Cfg7::from_bits(val)
    }
}
impl From<Cfg7> for u8 {
    #[inline(always)]
    fn from(val: Cfg7) -> u8 {
        Cfg7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg8 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A2OUT value."]
    A2out = 0x02,
    #[doc = "Output is A3OUT. value."]
    A3out2 = 0x03,
    #[doc = "Output is A4OUT2. value."]
    A4out2 = 0x04,
    #[doc = "Output is B6OUT. value."]
    B6out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg8 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg8 {
    #[inline(always)]
    fn from(val: u8) -> Cfg8 {
        Cfg8::from_bits(val)
    }
}
impl From<Cfg8> for u8 {
    #[inline(always)]
    fn from(val: Cfg8) -> u8 {
        Cfg8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg9 {
    #[doc = "Force output to 0 value."]
    Zero = 0x0,
    #[doc = "Force output to 1. value."]
    One = 0x01,
    #[doc = "Output is A2OUT2 value."]
    A2out2 = 0x02,
    #[doc = "Output is A2OUT. value."]
    A2out = 0x03,
    #[doc = "Output is A4OUT. value."]
    A4out = 0x04,
    #[doc = "Output is B0OUT. value."]
    B0out = 0x05,
    #[doc = "Output is A6OUT2. value."]
    A6out2 = 0x06,
    #[doc = "Output is A7OUT2. value."]
    A7out2 = 0x07,
}
impl Cfg9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg9 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg9 {
    #[inline(always)]
    fn from(val: u8) -> Cfg9 {
        Cfg9::from_bits(val)
    }
}
impl From<Cfg9> for u8 {
    #[inline(always)]
    fn from(val: Cfg9) -> u8 {
        Cfg9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfga0 {
    #[doc = "Input is CT0 value."]
    Ct0 = 0x0,
    #[doc = "Input is CT1 value."]
    Ct1 = 0x01,
}
impl Cfga0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfga0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfga0 {
    #[inline(always)]
    fn from(val: u8) -> Cfga0 {
        Cfga0::from_bits(val)
    }
}
impl From<Cfga0> for u8 {
    #[inline(always)]
    fn from(val: Cfga0) -> u8 {
        Cfga0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfga1 {
    #[doc = "Input is CT4 value."]
    Ct4 = 0x0,
    #[doc = "Input is CT5 value."]
    Ct5 = 0x01,
}
impl Cfga1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfga1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfga1 {
    #[inline(always)]
    fn from(val: u8) -> Cfga1 {
        Cfga1::from_bits(val)
    }
}
impl From<Cfga1> for u8 {
    #[inline(always)]
    fn from(val: Cfga1) -> u8 {
        Cfga1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfga2 {
    #[doc = "Input is CT8 value."]
    Ct8 = 0x0,
    #[doc = "Input is CT9 value."]
    Ct9 = 0x01,
}
impl Cfga2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfga2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfga2 {
    #[inline(always)]
    fn from(val: u8) -> Cfga2 {
        Cfga2::from_bits(val)
    }
}
impl From<Cfga2> for u8 {
    #[inline(always)]
    fn from(val: Cfga2) -> u8 {
        Cfga2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfga3 {
    #[doc = "Input is CT12 value."]
    Ct12 = 0x0,
    #[doc = "Input is CT13 value."]
    Ct13 = 0x01,
}
impl Cfga3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfga3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfga3 {
    #[inline(always)]
    fn from(val: u8) -> Cfga3 {
        Cfga3::from_bits(val)
    }
}
impl From<Cfga3> for u8 {
    #[inline(always)]
    fn from(val: Cfga3) -> u8 {
        Cfga3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfga4 {
    #[doc = "Input is CT16 value."]
    Ct16 = 0x0,
    #[doc = "Input is CT17 value."]
    Ct17 = 0x01,
}
impl Cfga4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfga4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfga4 {
    #[inline(always)]
    fn from(val: u8) -> Cfga4 {
        Cfga4::from_bits(val)
    }
}
impl From<Cfga4> for u8 {
    #[inline(always)]
    fn from(val: Cfga4) -> u8 {
        Cfga4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfga5 {
    #[doc = "Input is CT20 value."]
    Ct20 = 0x0,
    #[doc = "Input is CT21 value."]
    Ct21 = 0x01,
}
impl Cfga5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfga5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfga5 {
    #[inline(always)]
    fn from(val: u8) -> Cfga5 {
        Cfga5::from_bits(val)
    }
}
impl From<Cfga5> for u8 {
    #[inline(always)]
    fn from(val: Cfga5) -> u8 {
        Cfga5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfga6 {
    #[doc = "Input is CT24 value."]
    Ct24 = 0x0,
    #[doc = "Input is CT25 value."]
    Ct25 = 0x01,
}
impl Cfga6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfga6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfga6 {
    #[inline(always)]
    fn from(val: u8) -> Cfga6 {
        Cfga6::from_bits(val)
    }
}
impl From<Cfga6> for u8 {
    #[inline(always)]
    fn from(val: Cfga6) -> u8 {
        Cfga6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfga7 {
    #[doc = "Input is CT28 value."]
    Ct28 = 0x0,
    #[doc = "Input is CT29 value."]
    Ct29 = 0x01,
}
impl Cfga7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfga7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfga7 {
    #[inline(always)]
    fn from(val: u8) -> Cfga7 {
        Cfga7::from_bits(val)
    }
}
impl From<Cfga7> for u8 {
    #[inline(always)]
    fn from(val: Cfga7) -> u8 {
        Cfga7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfgb0 {
    #[doc = "Input is CT2 value."]
    Ct2 = 0x0,
    #[doc = "Input is CT3 value."]
    Ct3 = 0x01,
}
impl Cfgb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfgb0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfgb0 {
    #[inline(always)]
    fn from(val: u8) -> Cfgb0 {
        Cfgb0::from_bits(val)
    }
}
impl From<Cfgb0> for u8 {
    #[inline(always)]
    fn from(val: Cfgb0) -> u8 {
        Cfgb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfgb1 {
    #[doc = "Input is CT6 value."]
    Ct6 = 0x0,
    #[doc = "Input is CT7 value."]
    Ct7 = 0x01,
}
impl Cfgb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfgb1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfgb1 {
    #[inline(always)]
    fn from(val: u8) -> Cfgb1 {
        Cfgb1::from_bits(val)
    }
}
impl From<Cfgb1> for u8 {
    #[inline(always)]
    fn from(val: Cfgb1) -> u8 {
        Cfgb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfgb2 {
    #[doc = "Input is CT10 value."]
    Ct10 = 0x0,
    #[doc = "Input is CT11 value."]
    Ct11 = 0x01,
}
impl Cfgb2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfgb2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfgb2 {
    #[inline(always)]
    fn from(val: u8) -> Cfgb2 {
        Cfgb2::from_bits(val)
    }
}
impl From<Cfgb2> for u8 {
    #[inline(always)]
    fn from(val: Cfgb2) -> u8 {
        Cfgb2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfgb3 {
    #[doc = "Input is CT14 value."]
    Ct14 = 0x0,
    #[doc = "Input is CT15 value."]
    Ct15 = 0x01,
}
impl Cfgb3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfgb3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfgb3 {
    #[inline(always)]
    fn from(val: u8) -> Cfgb3 {
        Cfgb3::from_bits(val)
    }
}
impl From<Cfgb3> for u8 {
    #[inline(always)]
    fn from(val: Cfgb3) -> u8 {
        Cfgb3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfgb4 {
    #[doc = "Input is CT18 value."]
    Ct18 = 0x0,
    #[doc = "Input is CT19 value."]
    Ct19 = 0x01,
}
impl Cfgb4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfgb4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfgb4 {
    #[inline(always)]
    fn from(val: u8) -> Cfgb4 {
        Cfgb4::from_bits(val)
    }
}
impl From<Cfgb4> for u8 {
    #[inline(always)]
    fn from(val: Cfgb4) -> u8 {
        Cfgb4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfgb5 {
    #[doc = "Input is CT22 value."]
    Ct22 = 0x0,
    #[doc = "Input is CT23 value."]
    Ct23 = 0x01,
}
impl Cfgb5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfgb5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfgb5 {
    #[inline(always)]
    fn from(val: u8) -> Cfgb5 {
        Cfgb5::from_bits(val)
    }
}
impl From<Cfgb5> for u8 {
    #[inline(always)]
    fn from(val: Cfgb5) -> u8 {
        Cfgb5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfgb6 {
    #[doc = "Input is CT26 value."]
    Ct26 = 0x0,
    #[doc = "Input is CT27 value."]
    Ct27 = 0x01,
}
impl Cfgb6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfgb6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfgb6 {
    #[inline(always)]
    fn from(val: u8) -> Cfgb6 {
        Cfgb6::from_bits(val)
    }
}
impl From<Cfgb6> for u8 {
    #[inline(always)]
    fn from(val: Cfgb6) -> u8 {
        Cfgb6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfgb7 {
    #[doc = "Input is CT30 value."]
    Ct30 = 0x0,
    #[doc = "Input is CT31 value."]
    Ct31 = 0x01,
}
impl Cfgb7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfgb7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfgb7 {
    #[inline(always)]
    fn from(val: u8) -> Cfgb7 {
        Cfgb7::from_bits(val)
    }
}
impl From<Cfgb7> for u8 {
    #[inline(always)]
    fn from(val: Cfgb7) -> u8 {
        Cfgb7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clear {
    #[doc = "Let the COUNTER register run on its input clock. value."]
    Run = 0x0,
    #[doc = "Stop the COUNTER register for loading. value."]
    Clear = 0x01,
}
impl Clear {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clear {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clear {
    #[inline(always)]
    fn from(val: u8) -> Clear {
        Clear::from_bits(val)
    }
}
impl From<Clear> for u8 {
    #[inline(always)]
    fn from(val: Clear) -> u8 {
        Clear::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksel {
    #[doc = "No clock enabled. value."]
    Noclk = 0x0,
    #[doc = "3MHz from the HFRC clock divider. value."]
    HfrcDiv16 = 0x01,
    #[doc = "187.5KHz from the HFRC clock divider. value."]
    HfrcDiv256 = 0x02,
    #[doc = "32768Hz from the crystal oscillator. value."]
    XtalDiv1 = 0x03,
    #[doc = "16384Hz from the crystal oscillator. value."]
    XtalDiv2 = 0x04,
    #[doc = "1024Hz from the crystal oscillator. value."]
    XtalDiv32 = 0x05,
    #[doc = "Approximately 1KHz from the LFRC oscillator (uncalibrated). value."]
    LfrcDiv1 = 0x06,
    #[doc = "Use CTIMER 0 section A as a prescaler for the clock source. value."]
    Ctimer0a = 0x07,
    #[doc = "Use CTIMER 0 section B (or A and B linked together) as a prescaler for the clock source. value."]
    Ctimer0b = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Clksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksel {
    #[inline(always)]
    fn from(val: u8) -> Clksel {
        Clksel::from_bits(val)
    }
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(val: Clksel) -> u8 {
        Clksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctlink0 {
    #[doc = "Use A0/B0 timers as two independent 16-bit timers (default). value."]
    Two16bitTimers = 0x0,
    #[doc = "Link A0/B0 timers into a single 32-bit timer. value."]
    _32bitTimer = 0x01,
}
impl Ctlink0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctlink0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctlink0 {
    #[inline(always)]
    fn from(val: u8) -> Ctlink0 {
        Ctlink0::from_bits(val)
    }
}
impl From<Ctlink0> for u8 {
    #[inline(always)]
    fn from(val: Ctlink0) -> u8 {
        Ctlink0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctlink1 {
    #[doc = "Use A1/B1 timers as two independent 16-bit timers (default). value."]
    Two16bitTimers = 0x0,
    #[doc = "Link A1/B1 timers into a single 32-bit timer. value."]
    _32bitTimer = 0x01,
}
impl Ctlink1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctlink1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctlink1 {
    #[inline(always)]
    fn from(val: u8) -> Ctlink1 {
        Ctlink1::from_bits(val)
    }
}
impl From<Ctlink1> for u8 {
    #[inline(always)]
    fn from(val: Ctlink1) -> u8 {
        Ctlink1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctlink2 {
    #[doc = "Use A2/B2 timers as two independent 16-bit timers (default). value."]
    Two16bitTimers = 0x0,
    #[doc = "Link A2/B2 timers into a single 32-bit timer. value."]
    _32bitTimer = 0x01,
}
impl Ctlink2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctlink2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctlink2 {
    #[inline(always)]
    fn from(val: u8) -> Ctlink2 {
        Ctlink2::from_bits(val)
    }
}
impl From<Ctlink2> for u8 {
    #[inline(always)]
    fn from(val: Ctlink2) -> u8 {
        Ctlink2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctlink3 {
    #[doc = "Use A3/B3 timers as two independent 16-bit timers (default). value."]
    Two16bitTimers = 0x0,
    #[doc = "Link A3/B3 timers into a single 32-bit timer. value."]
    _32bitTimer = 0x01,
}
impl Ctlink3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctlink3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctlink3 {
    #[inline(always)]
    fn from(val: u8) -> Ctlink3 {
        Ctlink3::from_bits(val)
    }
}
impl From<Ctlink3> for u8 {
    #[inline(always)]
    fn from(val: Ctlink3) -> u8 {
        Ctlink3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctlink4 {
    #[doc = "Use A4/B4 timers as two independent 16-bit timers (default). value."]
    Two16bitTimers = 0x0,
    #[doc = "Link A4/B4 timers into a single 32-bit timer. value."]
    _32bitTimer = 0x01,
}
impl Ctlink4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctlink4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctlink4 {
    #[inline(always)]
    fn from(val: u8) -> Ctlink4 {
        Ctlink4::from_bits(val)
    }
}
impl From<Ctlink4> for u8 {
    #[inline(always)]
    fn from(val: Ctlink4) -> u8 {
        Ctlink4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctlink5 {
    #[doc = "Use A5/B5 timers as two independent 16-bit timers (default). value."]
    Two16bitTimers = 0x0,
    #[doc = "Link A5/B5 timers into a single 32-bit timer. value."]
    _32bitTimer = 0x01,
}
impl Ctlink5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctlink5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctlink5 {
    #[inline(always)]
    fn from(val: u8) -> Ctlink5 {
        Ctlink5::from_bits(val)
    }
}
impl From<Ctlink5> for u8 {
    #[inline(always)]
    fn from(val: Ctlink5) -> u8 {
        Ctlink5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctlink6 {
    #[doc = "Use A6/B6 timers as two independent 16-bit timers (default). value."]
    Two16bitTimers = 0x0,
    #[doc = "Link A6/B6 timers into a single 32-bit timer. value."]
    _32bitTimer = 0x01,
}
impl Ctlink6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctlink6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctlink6 {
    #[inline(always)]
    fn from(val: u8) -> Ctlink6 {
        Ctlink6::from_bits(val)
    }
}
impl From<Ctlink6> for u8 {
    #[inline(always)]
    fn from(val: Ctlink6) -> u8 {
        Ctlink6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctlink7 {
    #[doc = "Use A7/B7 timers as two independent 16-bit timers (default). value."]
    Two16bitTimers = 0x0,
    #[doc = "Link A7/B7 timers into a single 32-bit timer. value."]
    _32bitTimer = 0x01,
}
impl Ctlink7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctlink7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctlink7 {
    #[inline(always)]
    fn from(val: u8) -> Ctlink7 {
        Ctlink7::from_bits(val)
    }
}
impl From<Ctlink7> for u8 {
    #[inline(always)]
    fn from(val: Ctlink7) -> u8 {
        Ctlink7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ena0 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Ena0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena0 {
    #[inline(always)]
    fn from(val: u8) -> Ena0 {
        Ena0::from_bits(val)
    }
}
impl From<Ena0> for u8 {
    #[inline(always)]
    fn from(val: Ena0) -> u8 {
        Ena0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ena1 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Ena1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena1 {
    #[inline(always)]
    fn from(val: u8) -> Ena1 {
        Ena1::from_bits(val)
    }
}
impl From<Ena1> for u8 {
    #[inline(always)]
    fn from(val: Ena1) -> u8 {
        Ena1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ena2 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Ena2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena2 {
    #[inline(always)]
    fn from(val: u8) -> Ena2 {
        Ena2::from_bits(val)
    }
}
impl From<Ena2> for u8 {
    #[inline(always)]
    fn from(val: Ena2) -> u8 {
        Ena2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ena3 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Ena3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena3 {
    #[inline(always)]
    fn from(val: u8) -> Ena3 {
        Ena3::from_bits(val)
    }
}
impl From<Ena3> for u8 {
    #[inline(always)]
    fn from(val: Ena3) -> u8 {
        Ena3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ena4 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Ena4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena4 {
    #[inline(always)]
    fn from(val: u8) -> Ena4 {
        Ena4::from_bits(val)
    }
}
impl From<Ena4> for u8 {
    #[inline(always)]
    fn from(val: Ena4) -> u8 {
        Ena4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ena5 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Ena5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena5 {
    #[inline(always)]
    fn from(val: u8) -> Ena5 {
        Ena5::from_bits(val)
    }
}
impl From<Ena5> for u8 {
    #[inline(always)]
    fn from(val: Ena5) -> u8 {
        Ena5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ena6 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Ena6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena6 {
    #[inline(always)]
    fn from(val: u8) -> Ena6 {
        Ena6::from_bits(val)
    }
}
impl From<Ena6> for u8 {
    #[inline(always)]
    fn from(val: Ena6) -> u8 {
        Ena6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ena7 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Ena7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena7 {
    #[inline(always)]
    fn from(val: u8) -> Ena7 {
        Ena7::from_bits(val)
    }
}
impl From<Ena7> for u8 {
    #[inline(always)]
    fn from(val: Ena7) -> u8 {
        Ena7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enb0 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Enb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enb0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enb0 {
    #[inline(always)]
    fn from(val: u8) -> Enb0 {
        Enb0::from_bits(val)
    }
}
impl From<Enb0> for u8 {
    #[inline(always)]
    fn from(val: Enb0) -> u8 {
        Enb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enb1 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Enb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enb1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enb1 {
    #[inline(always)]
    fn from(val: u8) -> Enb1 {
        Enb1::from_bits(val)
    }
}
impl From<Enb1> for u8 {
    #[inline(always)]
    fn from(val: Enb1) -> u8 {
        Enb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enb2 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Enb2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enb2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enb2 {
    #[inline(always)]
    fn from(val: u8) -> Enb2 {
        Enb2::from_bits(val)
    }
}
impl From<Enb2> for u8 {
    #[inline(always)]
    fn from(val: Enb2) -> u8 {
        Enb2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enb3 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Enb3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enb3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enb3 {
    #[inline(always)]
    fn from(val: u8) -> Enb3 {
        Enb3::from_bits(val)
    }
}
impl From<Enb3> for u8 {
    #[inline(always)]
    fn from(val: Enb3) -> u8 {
        Enb3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enb4 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Enb4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enb4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enb4 {
    #[inline(always)]
    fn from(val: u8) -> Enb4 {
        Enb4::from_bits(val)
    }
}
impl From<Enb4> for u8 {
    #[inline(always)]
    fn from(val: Enb4) -> u8 {
        Enb4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enb5 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Enb5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enb5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enb5 {
    #[inline(always)]
    fn from(val: u8) -> Enb5 {
        Enb5::from_bits(val)
    }
}
impl From<Enb5> for u8 {
    #[inline(always)]
    fn from(val: Enb5) -> u8 {
        Enb5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enb6 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Enb6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enb6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enb6 {
    #[inline(always)]
    fn from(val: u8) -> Enb6 {
        Enb6::from_bits(val)
    }
}
impl From<Enb6> for u8 {
    #[inline(always)]
    fn from(val: Enb6) -> u8 {
        Enb6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enb7 {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl Enb7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enb7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enb7 {
    #[inline(always)]
    fn from(val: u8) -> Enb7 {
        Enb7::from_bits(val)
    }
}
impl From<Enb7> for u8 {
    #[inline(always)]
    fn from(val: Enb7) -> u8 {
        Enb7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Freeze {
    #[doc = "Let the COUNTER register run on its input clock. value."]
    Thaw = 0x0,
    #[doc = "Stop the COUNTER register for loading. value."]
    Freeze = 0x01,
}
impl Freeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Freeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Freeze {
    #[inline(always)]
    fn from(val: u8) -> Freeze {
        Freeze::from_bits(val)
    }
}
impl From<Freeze> for u8 {
    #[inline(always)]
    fn from(val: Freeze) -> u8 {
        Freeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra0clk {
    #[doc = "Clock source is TMRPINA. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x14,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    Ctmra1 = 0x15,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x16,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    Ctmra2 = 0x17,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x18,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x19,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x1a,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x1b,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmra0clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra0clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra0clk {
    #[inline(always)]
    fn from(val: u8) -> Tmra0clk {
        Tmra0clk::from_bits(val)
    }
}
impl From<Tmra0clk> for u8 {
    #[inline(always)]
    fn from(val: Tmra0clk) -> u8 {
        Tmra0clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra0clr {
    #[doc = "Allow counter/timer A0 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer A0 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmra0clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra0clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra0clr {
    #[inline(always)]
    fn from(val: u8) -> Tmra0clr {
        Tmra0clr::from_bits(val)
    }
}
impl From<Tmra0clr> for u8 {
    #[inline(always)]
    fn from(val: Tmra0clr) -> u8 {
        Tmra0clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra0en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmra0en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra0en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra0en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra0en23 {
        Tmra0en23::from_bits(val)
    }
}
impl From<Tmra0en23> for u8 {
    #[inline(always)]
    fn from(val: Tmra0en23) -> u8 {
        Tmra0en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra0fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A0, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A0, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A0, assert, count to CMPR1A0, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0A0, assert, count to CMPR1A0, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmra0fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra0fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra0fn {
    #[inline(always)]
    fn from(val: u8) -> Tmra0fn {
        Tmra0fn::from_bits(val)
    }
}
impl From<Tmra0fn> for u8 {
    #[inline(always)]
    fn from(val: Tmra0fn) -> u8 {
        Tmra0fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra0nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmra0nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra0nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra0nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmra0nosync {
        Tmra0nosync::from_bits(val)
    }
}
impl From<Tmra0nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmra0nosync) -> u8 {
        Tmra0nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra0pol {
    #[doc = "The polarity of the TMRPINA0 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINA0 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmra0pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra0pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra0pol {
    #[inline(always)]
    fn from(val: u8) -> Tmra0pol {
        Tmra0pol::from_bits(val)
    }
}
impl From<Tmra0pol> for u8 {
    #[inline(always)]
    fn from(val: Tmra0pol) -> u8 {
        Tmra0pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra0pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmra0pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra0pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra0pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra0pol23 {
        Tmra0pol23::from_bits(val)
    }
}
impl From<Tmra0pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmra0pol23) -> u8 {
        Tmra0pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra0trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1out = 0x04,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1out = 0x05,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5out = 0x06,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERB6 OUT2. value."]
    B6out2 = 0x0a,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4out2dual = 0x0f,
}
impl Tmra0trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra0trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra0trig {
    #[inline(always)]
    fn from(val: u8) -> Tmra0trig {
        Tmra0trig::from_bits(val)
    }
}
impl From<Tmra0trig> for u8 {
    #[inline(always)]
    fn from(val: Tmra0trig) -> u8 {
        Tmra0trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra1clk {
    #[doc = "Clock source is TMRPINA. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x14,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    Ctmra0 = 0x15,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x16,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    Ctmra2 = 0x17,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x18,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x19,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x1a,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x1b,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmra1clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra1clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra1clk {
    #[inline(always)]
    fn from(val: u8) -> Tmra1clk {
        Tmra1clk::from_bits(val)
    }
}
impl From<Tmra1clk> for u8 {
    #[inline(always)]
    fn from(val: Tmra1clk) -> u8 {
        Tmra1clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra1clr {
    #[doc = "Allow counter/timer A1 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer A1 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmra1clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra1clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra1clr {
    #[inline(always)]
    fn from(val: u8) -> Tmra1clr {
        Tmra1clr::from_bits(val)
    }
}
impl From<Tmra1clr> for u8 {
    #[inline(always)]
    fn from(val: Tmra1clr) -> u8 {
        Tmra1clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra1en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmra1en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra1en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra1en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra1en23 {
        Tmra1en23::from_bits(val)
    }
}
impl From<Tmra1en23> for u8 {
    #[inline(always)]
    fn from(val: Tmra1en23) -> u8 {
        Tmra1en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra1fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A1, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A1, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A1, assert, count to CMPR1A1, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0A1, assert, count to CMPR1A1, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmra1fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra1fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra1fn {
    #[inline(always)]
    fn from(val: u8) -> Tmra1fn {
        Tmra1fn::from_bits(val)
    }
}
impl From<Tmra1fn> for u8 {
    #[inline(always)]
    fn from(val: Tmra1fn) -> u8 {
        Tmra1fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra1nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmra1nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra1nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra1nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmra1nosync {
        Tmra1nosync::from_bits(val)
    }
}
impl From<Tmra1nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmra1nosync) -> u8 {
        Tmra1nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra1pol {
    #[doc = "The polarity of the TMRPINA1 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINA1 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmra1pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra1pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra1pol {
    #[inline(always)]
    fn from(val: u8) -> Tmra1pol {
        Tmra1pol::from_bits(val)
    }
}
impl From<Tmra1pol> for u8 {
    #[inline(always)]
    fn from(val: Tmra1pol) -> u8 {
        Tmra1pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra1pol23 {
    #[doc = "Upper output normal polarity value."]
    Normal = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmra1pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra1pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra1pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra1pol23 {
        Tmra1pol23::from_bits(val)
    }
}
impl From<Tmra1pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmra1pol23) -> u8 {
        Tmra1pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra1trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0out = 0x04,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0out = 0x05,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5out = 0x06,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA4 OUT2. value."]
    A4out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB4 OUT2. value."]
    B4out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5out2dual = 0x0f,
}
impl Tmra1trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra1trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra1trig {
    #[inline(always)]
    fn from(val: u8) -> Tmra1trig {
        Tmra1trig::from_bits(val)
    }
}
impl From<Tmra1trig> for u8 {
    #[inline(always)]
    fn from(val: Tmra1trig) -> u8 {
        Tmra1trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra2clk {
    #[doc = "Clock source is TMRPINA. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x14,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    Ctmrb3 = 0x15,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmra3 = 0x16,
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    Ctmra4 = 0x17,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x18,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x19,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x1a,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x1b,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmra2clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra2clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra2clk {
    #[inline(always)]
    fn from(val: u8) -> Tmra2clk {
        Tmra2clk::from_bits(val)
    }
}
impl From<Tmra2clk> for u8 {
    #[inline(always)]
    fn from(val: Tmra2clk) -> u8 {
        Tmra2clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra2clr {
    #[doc = "Allow counter/timer A2 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer A2 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmra2clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra2clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra2clr {
    #[inline(always)]
    fn from(val: u8) -> Tmra2clr {
        Tmra2clr::from_bits(val)
    }
}
impl From<Tmra2clr> for u8 {
    #[inline(always)]
    fn from(val: Tmra2clr) -> u8 {
        Tmra2clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra2en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmra2en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra2en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra2en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra2en23 {
        Tmra2en23::from_bits(val)
    }
}
impl From<Tmra2en23> for u8 {
    #[inline(always)]
    fn from(val: Tmra2en23) -> u8 {
        Tmra2en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra2fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A2, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A2, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A2, assert, count to CMPR1A2, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0A2, assert, count to CMPR1A2, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmra2fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra2fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra2fn {
    #[inline(always)]
    fn from(val: u8) -> Tmra2fn {
        Tmra2fn::from_bits(val)
    }
}
impl From<Tmra2fn> for u8 {
    #[inline(always)]
    fn from(val: Tmra2fn) -> u8 {
        Tmra2fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra2nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmra2nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra2nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra2nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmra2nosync {
        Tmra2nosync::from_bits(val)
    }
}
impl From<Tmra2nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmra2nosync) -> u8 {
        Tmra2nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra2pol {
    #[doc = "The polarity of the TMRPINA2 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINA2 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmra2pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra2pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra2pol {
    #[inline(always)]
    fn from(val: u8) -> Tmra2pol {
        Tmra2pol::from_bits(val)
    }
}
impl From<Tmra2pol> for u8 {
    #[inline(always)]
    fn from(val: Tmra2pol) -> u8 {
        Tmra2pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra2pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmra2pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra2pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra2pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra2pol23 {
        Tmra2pol23::from_bits(val)
    }
}
impl From<Tmra2pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmra2pol23) -> u8 {
        Tmra2pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra2trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0out = 0x04,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0out = 0x05,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4out = 0x06,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4out2dual = 0x0f,
}
impl Tmra2trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra2trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra2trig {
    #[inline(always)]
    fn from(val: u8) -> Tmra2trig {
        Tmra2trig::from_bits(val)
    }
}
impl From<Tmra2trig> for u8 {
    #[inline(always)]
    fn from(val: Tmra2trig) -> u8 {
        Tmra2trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra3clk {
    #[doc = "Clock source is TMRPINA. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x14,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    Ctmra2 = 0x15,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x16,
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    Ctmra4 = 0x17,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x18,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x19,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x1a,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x1b,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmra3clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra3clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra3clk {
    #[inline(always)]
    fn from(val: u8) -> Tmra3clk {
        Tmra3clk::from_bits(val)
    }
}
impl From<Tmra3clk> for u8 {
    #[inline(always)]
    fn from(val: Tmra3clk) -> u8 {
        Tmra3clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra3clr {
    #[doc = "Allow counter/timer A3 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer A3 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmra3clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra3clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra3clr {
    #[inline(always)]
    fn from(val: u8) -> Tmra3clr {
        Tmra3clr::from_bits(val)
    }
}
impl From<Tmra3clr> for u8 {
    #[inline(always)]
    fn from(val: Tmra3clr) -> u8 {
        Tmra3clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra3en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmra3en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra3en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra3en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra3en23 {
        Tmra3en23::from_bits(val)
    }
}
impl From<Tmra3en23> for u8 {
    #[inline(always)]
    fn from(val: Tmra3en23) -> u8 {
        Tmra3en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra3fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A3, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A3, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A3, assert, count to CMPR1A3, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0A3, assert, count to CMPR1A3, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmra3fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra3fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra3fn {
    #[inline(always)]
    fn from(val: u8) -> Tmra3fn {
        Tmra3fn::from_bits(val)
    }
}
impl From<Tmra3fn> for u8 {
    #[inline(always)]
    fn from(val: Tmra3fn) -> u8 {
        Tmra3fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra3nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmra3nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra3nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra3nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmra3nosync {
        Tmra3nosync::from_bits(val)
    }
}
impl From<Tmra3nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmra3nosync) -> u8 {
        Tmra3nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra3pol {
    #[doc = "The polarity of the TMRPINA3 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINA3 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmra3pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra3pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra3pol {
    #[inline(always)]
    fn from(val: u8) -> Tmra3pol {
        Tmra3pol::from_bits(val)
    }
}
impl From<Tmra3pol> for u8 {
    #[inline(always)]
    fn from(val: Tmra3pol) -> u8 {
        Tmra3pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra3pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmra3pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra3pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra3pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra3pol23 {
        Tmra3pol23::from_bits(val)
    }
}
impl From<Tmra3pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmra3pol23) -> u8 {
        Tmra3pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra3trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x01,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2out = 0x02,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2out = 0x03,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4out = 0x04,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4out = 0x05,
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    A7out = 0x06,
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    B7out = 0x07,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5out2 = 0x08,
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5out2 = 0x09,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    B2out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    A2out2dual = 0x0f,
}
impl Tmra3trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra3trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra3trig {
    #[inline(always)]
    fn from(val: u8) -> Tmra3trig {
        Tmra3trig::from_bits(val)
    }
}
impl From<Tmra3trig> for u8 {
    #[inline(always)]
    fn from(val: Tmra3trig) -> u8 {
        Tmra3trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra4clk {
    #[doc = "Clock source is TMRPINA. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4. (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x14,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    Ctmra1 = 0x15,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x16,
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    Ctmra5 = 0x17,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x18,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x19,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x1a,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x1b,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmra4clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra4clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra4clk {
    #[inline(always)]
    fn from(val: u8) -> Tmra4clk {
        Tmra4clk::from_bits(val)
    }
}
impl From<Tmra4clk> for u8 {
    #[inline(always)]
    fn from(val: Tmra4clk) -> u8 {
        Tmra4clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra4clr {
    #[doc = "Allow counter/timer A4 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer A4 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmra4clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra4clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra4clr {
    #[inline(always)]
    fn from(val: u8) -> Tmra4clr {
        Tmra4clr::from_bits(val)
    }
}
impl From<Tmra4clr> for u8 {
    #[inline(always)]
    fn from(val: Tmra4clr) -> u8 {
        Tmra4clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra4en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmra4en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra4en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra4en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra4en23 {
        Tmra4en23::from_bits(val)
    }
}
impl From<Tmra4en23> for u8 {
    #[inline(always)]
    fn from(val: Tmra4en23) -> u8 {
        Tmra4en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra4fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A4, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A4, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A4, assert, count to CMPR1A4, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0A4, assert, count to CMPR1A4, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmra4fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra4fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra4fn {
    #[inline(always)]
    fn from(val: u8) -> Tmra4fn {
        Tmra4fn::from_bits(val)
    }
}
impl From<Tmra4fn> for u8 {
    #[inline(always)]
    fn from(val: Tmra4fn) -> u8 {
        Tmra4fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra4nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmra4nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra4nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra4nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmra4nosync {
        Tmra4nosync::from_bits(val)
    }
}
impl From<Tmra4nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmra4nosync) -> u8 {
        Tmra4nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra4pol {
    #[doc = "The polarity of the TMRPINA4 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINA4 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmra4pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra4pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra4pol {
    #[inline(always)]
    fn from(val: u8) -> Tmra4pol {
        Tmra4pol::from_bits(val)
    }
}
impl From<Tmra4pol> for u8 {
    #[inline(always)]
    fn from(val: Tmra4pol) -> u8 {
        Tmra4pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra4pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmra4pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra4pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra4pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra4pol23 {
        Tmra4pol23::from_bits(val)
    }
}
impl From<Tmra4pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmra4pol23) -> u8 {
        Tmra4pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra4trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is STimer Interrupt. Only Active When CTLINK==1 and TMRB4TRIG!=0. TMRB4TRIG selects an STIMER interrupt value."]
    Stimer = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6out = 0x04,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6out = 0x05,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2out = 0x06,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5out2dual = 0x0f,
}
impl Tmra4trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra4trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra4trig {
    #[inline(always)]
    fn from(val: u8) -> Tmra4trig {
        Tmra4trig::from_bits(val)
    }
}
impl From<Tmra4trig> for u8 {
    #[inline(always)]
    fn from(val: Tmra4trig) -> u8 {
        Tmra4trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra5clk {
    #[doc = "Clock source is TMRPINA. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x14,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    Ctmra0 = 0x15,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x16,
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    Ctmra6 = 0x17,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x18,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x19,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x1a,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x1b,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmra5clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra5clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra5clk {
    #[inline(always)]
    fn from(val: u8) -> Tmra5clk {
        Tmra5clk::from_bits(val)
    }
}
impl From<Tmra5clk> for u8 {
    #[inline(always)]
    fn from(val: Tmra5clk) -> u8 {
        Tmra5clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra5clr {
    #[doc = "Allow counter/timer A5 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer A5 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmra5clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra5clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra5clr {
    #[inline(always)]
    fn from(val: u8) -> Tmra5clr {
        Tmra5clr::from_bits(val)
    }
}
impl From<Tmra5clr> for u8 {
    #[inline(always)]
    fn from(val: Tmra5clr) -> u8 {
        Tmra5clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra5en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmra5en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra5en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra5en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra5en23 {
        Tmra5en23::from_bits(val)
    }
}
impl From<Tmra5en23> for u8 {
    #[inline(always)]
    fn from(val: Tmra5en23) -> u8 {
        Tmra5en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra5fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A5, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A5, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A5, assert, count to CMPR1A5, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0A5, assert, count to CMPR1A5, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmra5fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra5fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra5fn {
    #[inline(always)]
    fn from(val: u8) -> Tmra5fn {
        Tmra5fn::from_bits(val)
    }
}
impl From<Tmra5fn> for u8 {
    #[inline(always)]
    fn from(val: Tmra5fn) -> u8 {
        Tmra5fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra5nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmra5nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra5nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra5nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmra5nosync {
        Tmra5nosync::from_bits(val)
    }
}
impl From<Tmra5nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmra5nosync) -> u8 {
        Tmra5nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra5pol {
    #[doc = "The polarity of the TMRPINA5 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINA5 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmra5pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra5pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra5pol {
    #[inline(always)]
    fn from(val: u8) -> Tmra5pol {
        Tmra5pol::from_bits(val)
    }
}
impl From<Tmra5pol> for u8 {
    #[inline(always)]
    fn from(val: Tmra5pol) -> u8 {
        Tmra5pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra5pol23 {
    #[doc = "Upper output normal polarity value."]
    Normal = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmra5pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra5pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra5pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra5pol23 {
        Tmra5pol23::from_bits(val)
    }
}
impl From<Tmra5pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmra5pol23) -> u8 {
        Tmra5pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra5trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is STimer Interrupt. Only Active When CTLINK==1 and TMRB5TRIG!=0. TMRB5TRIG selects an STIMER interrupt value."]
    Stimer = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4out = 0x04,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4out = 0x05,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2out = 0x06,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA0 OUT2. value."]
    A0out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB0 OUT2. value."]
    B0out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4out2dual = 0x0f,
}
impl Tmra5trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra5trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra5trig {
    #[inline(always)]
    fn from(val: u8) -> Tmra5trig {
        Tmra5trig::from_bits(val)
    }
}
impl From<Tmra5trig> for u8 {
    #[inline(always)]
    fn from(val: Tmra5trig) -> u8 {
        Tmra5trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra6clk {
    #[doc = "Clock source is TMRPINA. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x14,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    Ctmra3 = 0x15,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x16,
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    Ctmra7 = 0x17,
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    Ctmrb7 = 0x18,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x19,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x1a,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x1b,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmra6clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra6clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra6clk {
    #[inline(always)]
    fn from(val: u8) -> Tmra6clk {
        Tmra6clk::from_bits(val)
    }
}
impl From<Tmra6clk> for u8 {
    #[inline(always)]
    fn from(val: Tmra6clk) -> u8 {
        Tmra6clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra6clr {
    #[doc = "Allow counter/timer A6 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer A6 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmra6clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra6clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra6clr {
    #[inline(always)]
    fn from(val: u8) -> Tmra6clr {
        Tmra6clr::from_bits(val)
    }
}
impl From<Tmra6clr> for u8 {
    #[inline(always)]
    fn from(val: Tmra6clr) -> u8 {
        Tmra6clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra6en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmra6en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra6en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra6en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra6en23 {
        Tmra6en23::from_bits(val)
    }
}
impl From<Tmra6en23> for u8 {
    #[inline(always)]
    fn from(val: Tmra6en23) -> u8 {
        Tmra6en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra6fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A6, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A6, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A6, assert, count to CMPR1A6, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0A6, assert, count to CMPR1A6, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmra6fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra6fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra6fn {
    #[inline(always)]
    fn from(val: u8) -> Tmra6fn {
        Tmra6fn::from_bits(val)
    }
}
impl From<Tmra6fn> for u8 {
    #[inline(always)]
    fn from(val: Tmra6fn) -> u8 {
        Tmra6fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra6nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmra6nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra6nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra6nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmra6nosync {
        Tmra6nosync::from_bits(val)
    }
}
impl From<Tmra6nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmra6nosync) -> u8 {
        Tmra6nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra6pol {
    #[doc = "The polarity of the TMRPINA6 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINA6 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmra6pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra6pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra6pol {
    #[inline(always)]
    fn from(val: u8) -> Tmra6pol {
        Tmra6pol::from_bits(val)
    }
}
impl From<Tmra6pol> for u8 {
    #[inline(always)]
    fn from(val: Tmra6pol) -> u8 {
        Tmra6pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra6pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmra6pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra6pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra6pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra6pol23 {
        Tmra6pol23::from_bits(val)
    }
}
impl From<Tmra6pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmra6pol23) -> u8 {
        Tmra6pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra6trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5out = 0x04,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5out = 0x05,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1out = 0x06,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2out2 = 0x0a,
    #[doc = "Trigger source is CTIMERBb OUT2. value."]
    B2out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB0 OUT2, dual edge. value."]
    B0out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA0 OUT2, dual edge. value."]
    A0out2dual = 0x0f,
}
impl Tmra6trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra6trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra6trig {
    #[inline(always)]
    fn from(val: u8) -> Tmra6trig {
        Tmra6trig::from_bits(val)
    }
}
impl From<Tmra6trig> for u8 {
    #[inline(always)]
    fn from(val: Tmra6trig) -> u8 {
        Tmra6trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra7clk {
    #[doc = "Clock source is TMRPINA. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    Ctmrb7 = 0x14,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    Ctmra2 = 0x15,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x16,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    Ctmra0 = 0x17,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x18,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x19,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x1a,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x1b,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmra7clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra7clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra7clk {
    #[inline(always)]
    fn from(val: u8) -> Tmra7clk {
        Tmra7clk::from_bits(val)
    }
}
impl From<Tmra7clk> for u8 {
    #[inline(always)]
    fn from(val: Tmra7clk) -> u8 {
        Tmra7clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra7clr {
    #[doc = "Allow counter/timer A7 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer A7 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmra7clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra7clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra7clr {
    #[inline(always)]
    fn from(val: u8) -> Tmra7clr {
        Tmra7clr::from_bits(val)
    }
}
impl From<Tmra7clr> for u8 {
    #[inline(always)]
    fn from(val: Tmra7clr) -> u8 {
        Tmra7clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra7en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmra7en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra7en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra7en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra7en23 {
        Tmra7en23::from_bits(val)
    }
}
impl From<Tmra7en23> for u8 {
    #[inline(always)]
    fn from(val: Tmra7en23) -> u8 {
        Tmra7en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra7fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A7, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A7, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A7, assert, count to CMPR1A7, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0A7, assert, count to CMPR1A7, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmra7fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra7fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra7fn {
    #[inline(always)]
    fn from(val: u8) -> Tmra7fn {
        Tmra7fn::from_bits(val)
    }
}
impl From<Tmra7fn> for u8 {
    #[inline(always)]
    fn from(val: Tmra7fn) -> u8 {
        Tmra7fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra7nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmra7nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra7nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra7nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmra7nosync {
        Tmra7nosync::from_bits(val)
    }
}
impl From<Tmra7nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmra7nosync) -> u8 {
        Tmra7nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra7pol {
    #[doc = "The polarity of the TMRPINA7 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINA7 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmra7pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra7pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra7pol {
    #[inline(always)]
    fn from(val: u8) -> Tmra7pol {
        Tmra7pol::from_bits(val)
    }
}
impl From<Tmra7pol> for u8 {
    #[inline(always)]
    fn from(val: Tmra7pol) -> u8 {
        Tmra7pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra7pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmra7pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra7pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra7pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmra7pol23 {
        Tmra7pol23::from_bits(val)
    }
}
impl From<Tmra7pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmra7pol23) -> u8 {
        Tmra7pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmra7trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    B7out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1out = 0x04,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1out = 0x05,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4out = 0x06,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    B2out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4out2dual = 0x0f,
}
impl Tmra7trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmra7trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmra7trig {
    #[inline(always)]
    fn from(val: u8) -> Tmra7trig {
        Tmra7trig::from_bits(val)
    }
}
impl From<Tmra7trig> for u8 {
    #[inline(always)]
    fn from(val: Tmra7trig) -> u8 {
        Tmra7trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb0clk {
    #[doc = "Clock source is TMRPINB. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    Ctmra0 = 0x14,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x15,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    Ctmra1 = 0x16,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    Ctmra2 = 0x17,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x18,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x19,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x1a,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x1b,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmrb0clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb0clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb0clk {
    #[inline(always)]
    fn from(val: u8) -> Tmrb0clk {
        Tmrb0clk::from_bits(val)
    }
}
impl From<Tmrb0clk> for u8 {
    #[inline(always)]
    fn from(val: Tmrb0clk) -> u8 {
        Tmrb0clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb0clr {
    #[doc = "Allow counter/timer B0 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer B0 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmrb0clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb0clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb0clr {
    #[inline(always)]
    fn from(val: u8) -> Tmrb0clr {
        Tmrb0clr::from_bits(val)
    }
}
impl From<Tmrb0clr> for u8 {
    #[inline(always)]
    fn from(val: Tmrb0clr) -> u8 {
        Tmrb0clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb0en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmrb0en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb0en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb0en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb0en23 {
        Tmrb0en23::from_bits(val)
    }
}
impl From<Tmrb0en23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb0en23) -> u8 {
        Tmrb0en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb0fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B0, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B0, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B0, assert, count to CMPR1B0, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0B0, assert, count to CMPR1B0, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmrb0fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb0fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb0fn {
    #[inline(always)]
    fn from(val: u8) -> Tmrb0fn {
        Tmrb0fn::from_bits(val)
    }
}
impl From<Tmrb0fn> for u8 {
    #[inline(always)]
    fn from(val: Tmrb0fn) -> u8 {
        Tmrb0fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb0nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmrb0nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb0nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb0nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmrb0nosync {
        Tmrb0nosync::from_bits(val)
    }
}
impl From<Tmrb0nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmrb0nosync) -> u8 {
        Tmrb0nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb0pol {
    #[doc = "The polarity of the TMRPINB0 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINB0 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmrb0pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb0pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb0pol {
    #[inline(always)]
    fn from(val: u8) -> Tmrb0pol {
        Tmrb0pol::from_bits(val)
    }
}
impl From<Tmrb0pol> for u8 {
    #[inline(always)]
    fn from(val: Tmrb0pol) -> u8 {
        Tmrb0pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb0pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmrb0pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb0pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb0pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb0pol23 {
        Tmrb0pol23::from_bits(val)
    }
}
impl From<Tmrb0pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb0pol23) -> u8 {
        Tmrb0pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb0trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2out = 0x04,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5out = 0x05,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4out = 0x06,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERB7 OUT2. value."]
    B7out2 = 0x0a,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5out2dual = 0x0f,
}
impl Tmrb0trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb0trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb0trig {
    #[inline(always)]
    fn from(val: u8) -> Tmrb0trig {
        Tmrb0trig::from_bits(val)
    }
}
impl From<Tmrb0trig> for u8 {
    #[inline(always)]
    fn from(val: Tmrb0trig) -> u8 {
        Tmrb0trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb1clk {
    #[doc = "Clock source is TMRPINB. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    Ctmra1 = 0x14,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    Ctmra0 = 0x15,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x16,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    Ctmra2 = 0x17,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x18,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x19,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x1a,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x1b,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmrb1clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb1clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb1clk {
    #[inline(always)]
    fn from(val: u8) -> Tmrb1clk {
        Tmrb1clk::from_bits(val)
    }
}
impl From<Tmrb1clk> for u8 {
    #[inline(always)]
    fn from(val: Tmrb1clk) -> u8 {
        Tmrb1clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb1clr {
    #[doc = "Allow counter/timer B1 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer B1 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmrb1clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb1clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb1clr {
    #[inline(always)]
    fn from(val: u8) -> Tmrb1clr {
        Tmrb1clr::from_bits(val)
    }
}
impl From<Tmrb1clr> for u8 {
    #[inline(always)]
    fn from(val: Tmrb1clr) -> u8 {
        Tmrb1clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb1en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmrb1en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb1en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb1en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb1en23 {
        Tmrb1en23::from_bits(val)
    }
}
impl From<Tmrb1en23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb1en23) -> u8 {
        Tmrb1en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb1fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B1, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B1, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B1, assert, count to CMPR1B1, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0B1, assert, count to CMPR1B1, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmrb1fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb1fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb1fn {
    #[inline(always)]
    fn from(val: u8) -> Tmrb1fn {
        Tmrb1fn::from_bits(val)
    }
}
impl From<Tmrb1fn> for u8 {
    #[inline(always)]
    fn from(val: Tmrb1fn) -> u8 {
        Tmrb1fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb1nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmrb1nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb1nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb1nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmrb1nosync {
        Tmrb1nosync::from_bits(val)
    }
}
impl From<Tmrb1nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmrb1nosync) -> u8 {
        Tmrb1nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb1pol {
    #[doc = "The polarity of the TMRPINB1 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINB1 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmrb1pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb1pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb1pol {
    #[inline(always)]
    fn from(val: u8) -> Tmrb1pol {
        Tmrb1pol::from_bits(val)
    }
}
impl From<Tmrb1pol> for u8 {
    #[inline(always)]
    fn from(val: Tmrb1pol) -> u8 {
        Tmrb1pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb1pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmrb1pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb1pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb1pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb1pol23 {
        Tmrb1pol23::from_bits(val)
    }
}
impl From<Tmrb1pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb1pol23) -> u8 {
        Tmrb1pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb1trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6out = 0x04,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6out = 0x05,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0out = 0x06,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA4 OUT2. value."]
    A4out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB4 OUT2. value."]
    B4out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5out2dual = 0x0f,
}
impl Tmrb1trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb1trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb1trig {
    #[inline(always)]
    fn from(val: u8) -> Tmrb1trig {
        Tmrb1trig::from_bits(val)
    }
}
impl From<Tmrb1trig> for u8 {
    #[inline(always)]
    fn from(val: Tmrb1trig) -> u8 {
        Tmrb1trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb2clk {
    #[doc = "Clock source is TMRPINB. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    Ctmra2 = 0x14,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    Ctmrb3 = 0x15,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmra3 = 0x16,
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    Ctmra4 = 0x17,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x18,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x19,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x1a,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x1b,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmrb2clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb2clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb2clk {
    #[inline(always)]
    fn from(val: u8) -> Tmrb2clk {
        Tmrb2clk::from_bits(val)
    }
}
impl From<Tmrb2clk> for u8 {
    #[inline(always)]
    fn from(val: Tmrb2clk) -> u8 {
        Tmrb2clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb2clr {
    #[doc = "Allow counter/timer B2 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer B2 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmrb2clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb2clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb2clr {
    #[inline(always)]
    fn from(val: u8) -> Tmrb2clr {
        Tmrb2clr::from_bits(val)
    }
}
impl From<Tmrb2clr> for u8 {
    #[inline(always)]
    fn from(val: Tmrb2clr) -> u8 {
        Tmrb2clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb2en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmrb2en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb2en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb2en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb2en23 {
        Tmrb2en23::from_bits(val)
    }
}
impl From<Tmrb2en23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb2en23) -> u8 {
        Tmrb2en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb2fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B2, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B2, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B2, assert, count to CMPR1B2, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0B2, assert, count to CMPR1B2, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmrb2fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb2fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb2fn {
    #[inline(always)]
    fn from(val: u8) -> Tmrb2fn {
        Tmrb2fn::from_bits(val)
    }
}
impl From<Tmrb2fn> for u8 {
    #[inline(always)]
    fn from(val: Tmrb2fn) -> u8 {
        Tmrb2fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb2nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmrb2nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb2nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb2nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmrb2nosync {
        Tmrb2nosync::from_bits(val)
    }
}
impl From<Tmrb2nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmrb2nosync) -> u8 {
        Tmrb2nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb2pol {
    #[doc = "The polarity of the TMRPINB2 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINB2 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmrb2pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb2pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb2pol {
    #[inline(always)]
    fn from(val: u8) -> Tmrb2pol {
        Tmrb2pol::from_bits(val)
    }
}
impl From<Tmrb2pol> for u8 {
    #[inline(always)]
    fn from(val: Tmrb2pol) -> u8 {
        Tmrb2pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb2pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmrb2pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb2pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb2pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb2pol23 {
        Tmrb2pol23::from_bits(val)
    }
}
impl From<Tmrb2pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb2pol23) -> u8 {
        Tmrb2pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb2trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1out = 0x04,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1out = 0x05,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4out = 0x06,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4out2dual = 0x0f,
}
impl Tmrb2trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb2trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb2trig {
    #[inline(always)]
    fn from(val: u8) -> Tmrb2trig {
        Tmrb2trig::from_bits(val)
    }
}
impl From<Tmrb2trig> for u8 {
    #[inline(always)]
    fn from(val: Tmrb2trig) -> u8 {
        Tmrb2trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb3clk {
    #[doc = "Clock source is TMRPINB. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    Ctmra3 = 0x14,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    Ctmra2 = 0x15,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x16,
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    Ctmra4 = 0x17,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x18,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x19,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x1a,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x1b,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmrb3clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb3clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb3clk {
    #[inline(always)]
    fn from(val: u8) -> Tmrb3clk {
        Tmrb3clk::from_bits(val)
    }
}
impl From<Tmrb3clk> for u8 {
    #[inline(always)]
    fn from(val: Tmrb3clk) -> u8 {
        Tmrb3clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb3clr {
    #[doc = "Allow counter/timer B3 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer B3 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmrb3clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb3clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb3clr {
    #[inline(always)]
    fn from(val: u8) -> Tmrb3clr {
        Tmrb3clr::from_bits(val)
    }
}
impl From<Tmrb3clr> for u8 {
    #[inline(always)]
    fn from(val: Tmrb3clr) -> u8 {
        Tmrb3clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb3en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmrb3en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb3en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb3en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb3en23 {
        Tmrb3en23::from_bits(val)
    }
}
impl From<Tmrb3en23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb3en23) -> u8 {
        Tmrb3en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb3fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B3, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B3, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B3, assert, count to CMPR1B3, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0B3, assert, count to CMPR1B3, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmrb3fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb3fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb3fn {
    #[inline(always)]
    fn from(val: u8) -> Tmrb3fn {
        Tmrb3fn::from_bits(val)
    }
}
impl From<Tmrb3fn> for u8 {
    #[inline(always)]
    fn from(val: Tmrb3fn) -> u8 {
        Tmrb3fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb3nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmrb3nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb3nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb3nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmrb3nosync {
        Tmrb3nosync::from_bits(val)
    }
}
impl From<Tmrb3nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmrb3nosync) -> u8 {
        Tmrb3nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb3pol {
    #[doc = "The polarity of the TMRPINB3 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINB3 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmrb3pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb3pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb3pol {
    #[inline(always)]
    fn from(val: u8) -> Tmrb3pol {
        Tmrb3pol::from_bits(val)
    }
}
impl From<Tmrb3pol> for u8 {
    #[inline(always)]
    fn from(val: Tmrb3pol) -> u8 {
        Tmrb3pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb3pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmrb3pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb3pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb3pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb3pol23 {
        Tmrb3pol23::from_bits(val)
    }
}
impl From<Tmrb3pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb3pol23) -> u8 {
        Tmrb3pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb3trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x01,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2out = 0x02,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2out = 0x03,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4out = 0x04,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4out = 0x05,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6out = 0x06,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6out = 0x07,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5out2 = 0x08,
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5out2 = 0x09,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    B2out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    A2out2dual = 0x0f,
}
impl Tmrb3trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb3trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb3trig {
    #[inline(always)]
    fn from(val: u8) -> Tmrb3trig {
        Tmrb3trig::from_bits(val)
    }
}
impl From<Tmrb3trig> for u8 {
    #[inline(always)]
    fn from(val: Tmrb3trig) -> u8 {
        Tmrb3trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb4clk {
    #[doc = "Clock source is TMRPINB. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    Ctmra4 = 0x14,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    Ctmra1 = 0x15,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x16,
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    Ctmra5 = 0x17,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x18,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x19,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x1a,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x1b,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmrb4clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb4clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb4clk {
    #[inline(always)]
    fn from(val: u8) -> Tmrb4clk {
        Tmrb4clk::from_bits(val)
    }
}
impl From<Tmrb4clk> for u8 {
    #[inline(always)]
    fn from(val: Tmrb4clk) -> u8 {
        Tmrb4clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb4clr {
    #[doc = "Allow counter/timer B4 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer B4 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmrb4clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb4clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb4clr {
    #[inline(always)]
    fn from(val: u8) -> Tmrb4clr {
        Tmrb4clr::from_bits(val)
    }
}
impl From<Tmrb4clr> for u8 {
    #[inline(always)]
    fn from(val: Tmrb4clr) -> u8 {
        Tmrb4clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb4en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmrb4en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb4en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb4en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb4en23 {
        Tmrb4en23::from_bits(val)
    }
}
impl From<Tmrb4en23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb4en23) -> u8 {
        Tmrb4en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb4fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B4, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B4, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B4, assert, count to CMPR1B4, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0B4, assert, count to CMPR1B4, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmrb4fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb4fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb4fn {
    #[inline(always)]
    fn from(val: u8) -> Tmrb4fn {
        Tmrb4fn::from_bits(val)
    }
}
impl From<Tmrb4fn> for u8 {
    #[inline(always)]
    fn from(val: Tmrb4fn) -> u8 {
        Tmrb4fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb4nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmrb4nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb4nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb4nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmrb4nosync {
        Tmrb4nosync::from_bits(val)
    }
}
impl From<Tmrb4nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmrb4nosync) -> u8 {
        Tmrb4nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb4pol {
    #[doc = "The polarity of the TMRPINB4 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINB4 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmrb4pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb4pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb4pol {
    #[inline(always)]
    fn from(val: u8) -> Tmrb4pol {
        Tmrb4pol::from_bits(val)
    }
}
impl From<Tmrb4pol> for u8 {
    #[inline(always)]
    fn from(val: Tmrb4pol) -> u8 {
        Tmrb4pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb4pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmrb4pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb4pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb4pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb4pol23 {
        Tmrb4pol23::from_bits(val)
    }
}
impl From<Tmrb4pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb4pol23) -> u8 {
        Tmrb4pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb4trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    A7out = 0x04,
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    B7out = 0x05,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1out = 0x06,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5out2dual = 0x0f,
}
impl Tmrb4trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb4trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb4trig {
    #[inline(always)]
    fn from(val: u8) -> Tmrb4trig {
        Tmrb4trig::from_bits(val)
    }
}
impl From<Tmrb4trig> for u8 {
    #[inline(always)]
    fn from(val: Tmrb4trig) -> u8 {
        Tmrb4trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb5clk {
    #[doc = "Clock source is TMRPINB. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    Ctmra5 = 0x14,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    Ctmra0 = 0x15,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x16,
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    Ctmra6 = 0x17,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    Ctmrb6 = 0x18,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x19,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x1a,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x1b,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmrb5clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb5clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb5clk {
    #[inline(always)]
    fn from(val: u8) -> Tmrb5clk {
        Tmrb5clk::from_bits(val)
    }
}
impl From<Tmrb5clk> for u8 {
    #[inline(always)]
    fn from(val: Tmrb5clk) -> u8 {
        Tmrb5clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb5clr {
    #[doc = "Allow counter/timer B5 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer B5 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmrb5clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb5clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb5clr {
    #[inline(always)]
    fn from(val: u8) -> Tmrb5clr {
        Tmrb5clr::from_bits(val)
    }
}
impl From<Tmrb5clr> for u8 {
    #[inline(always)]
    fn from(val: Tmrb5clr) -> u8 {
        Tmrb5clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb5en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmrb5en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb5en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb5en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb5en23 {
        Tmrb5en23::from_bits(val)
    }
}
impl From<Tmrb5en23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb5en23) -> u8 {
        Tmrb5en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb5fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B5, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B5, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B5, assert, count to CMPR1B5, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0B5, assert, count to CMPR1B5, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmrb5fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb5fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb5fn {
    #[inline(always)]
    fn from(val: u8) -> Tmrb5fn {
        Tmrb5fn::from_bits(val)
    }
}
impl From<Tmrb5fn> for u8 {
    #[inline(always)]
    fn from(val: Tmrb5fn) -> u8 {
        Tmrb5fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb5nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmrb5nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb5nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb5nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmrb5nosync {
        Tmrb5nosync::from_bits(val)
    }
}
impl From<Tmrb5nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmrb5nosync) -> u8 {
        Tmrb5nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb5pol {
    #[doc = "The polarity of the TMRPINB5 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINB5 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmrb5pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb5pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb5pol {
    #[inline(always)]
    fn from(val: u8) -> Tmrb5pol {
        Tmrb5pol::from_bits(val)
    }
}
impl From<Tmrb5pol> for u8 {
    #[inline(always)]
    fn from(val: Tmrb5pol) -> u8 {
        Tmrb5pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb5pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmrb5pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb5pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb5pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb5pol23 {
        Tmrb5pol23::from_bits(val)
    }
}
impl From<Tmrb5pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb5pol23) -> u8 {
        Tmrb5pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb5trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6out = 0x04,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6out = 0x05,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1out = 0x06,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA0 OUT2. value."]
    A0out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB0 OUT2. value."]
    B0out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4out2dual = 0x0f,
}
impl Tmrb5trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb5trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb5trig {
    #[inline(always)]
    fn from(val: u8) -> Tmrb5trig {
        Tmrb5trig::from_bits(val)
    }
}
impl From<Tmrb5trig> for u8 {
    #[inline(always)]
    fn from(val: Tmrb5trig) -> u8 {
        Tmrb5trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb6clk {
    #[doc = "Clock source is TMRPINB. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    Ctmra6 = 0x14,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    Ctmra3 = 0x15,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x16,
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    Ctmra7 = 0x17,
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    Ctmrb7 = 0x18,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x19,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x1a,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x1b,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmrb6clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb6clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb6clk {
    #[inline(always)]
    fn from(val: u8) -> Tmrb6clk {
        Tmrb6clk::from_bits(val)
    }
}
impl From<Tmrb6clk> for u8 {
    #[inline(always)]
    fn from(val: Tmrb6clk) -> u8 {
        Tmrb6clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb6clr {
    #[doc = "Allow counter/timer B6 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer B6 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmrb6clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb6clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb6clr {
    #[inline(always)]
    fn from(val: u8) -> Tmrb6clr {
        Tmrb6clr::from_bits(val)
    }
}
impl From<Tmrb6clr> for u8 {
    #[inline(always)]
    fn from(val: Tmrb6clr) -> u8 {
        Tmrb6clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb6en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmrb6en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb6en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb6en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb6en23 {
        Tmrb6en23::from_bits(val)
    }
}
impl From<Tmrb6en23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb6en23) -> u8 {
        Tmrb6en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb6fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B6, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B6, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B6, assert, count to CMPR1B6, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0B6, assert, count to CMPR1B6, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmrb6fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb6fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb6fn {
    #[inline(always)]
    fn from(val: u8) -> Tmrb6fn {
        Tmrb6fn::from_bits(val)
    }
}
impl From<Tmrb6fn> for u8 {
    #[inline(always)]
    fn from(val: Tmrb6fn) -> u8 {
        Tmrb6fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb6nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmrb6nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb6nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb6nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmrb6nosync {
        Tmrb6nosync::from_bits(val)
    }
}
impl From<Tmrb6nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmrb6nosync) -> u8 {
        Tmrb6nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb6pol {
    #[doc = "The polarity of the TMRPINB6 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINB6 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmrb6pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb6pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb6pol {
    #[inline(always)]
    fn from(val: u8) -> Tmrb6pol {
        Tmrb6pol::from_bits(val)
    }
}
impl From<Tmrb6pol> for u8 {
    #[inline(always)]
    fn from(val: Tmrb6pol) -> u8 {
        Tmrb6pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb6pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmrb6pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb6pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb6pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb6pol23 {
        Tmrb6pol23::from_bits(val)
    }
}
impl From<Tmrb6pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb6pol23) -> u8 {
        Tmrb6pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb6trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4out = 0x04,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4out = 0x05,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1out = 0x06,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    B2out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB0 OUT2, dual edge. value."]
    B0out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA0 OUT2, dual edge. value."]
    A0out2dual = 0x0f,
}
impl Tmrb6trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb6trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb6trig {
    #[inline(always)]
    fn from(val: u8) -> Tmrb6trig {
        Tmrb6trig::from_bits(val)
    }
}
impl From<Tmrb6trig> for u8 {
    #[inline(always)]
    fn from(val: Tmrb6trig) -> u8 {
        Tmrb6trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb7clk {
    #[doc = "Clock source is TMRPINB. value."]
    Tmrpin = 0x0,
    #[doc = "Clock source is the HFRC / 4 value."]
    HfrcDiv4 = 0x01,
    #[doc = "Clock source is HFRC / 16 value."]
    HfrcDiv16 = 0x02,
    #[doc = "Clock source is HFRC / 256 value."]
    HfrcDiv256 = 0x03,
    #[doc = "Clock source is HFRC / 1024 value."]
    HfrcDiv1024 = 0x04,
    #[doc = "Clock source is HFRC / 4096 value."]
    HfrcDiv4k = 0x05,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    Xt = 0x06,
    #[doc = "Clock source is XT / 2 value."]
    XtDiv2 = 0x07,
    #[doc = "Clock source is XT / 16 value."]
    XtDiv16 = 0x08,
    #[doc = "Clock source is XT / 128 value."]
    XtDiv128 = 0x09,
    #[doc = "Clock source is LFRC / 2 value."]
    LfrcDiv2 = 0x0a,
    #[doc = "Clock source is LFRC / 32 value."]
    LfrcDiv32 = 0x0b,
    #[doc = "Clock source is LFRC / 1024 value."]
    LfrcDiv1k = 0x0c,
    #[doc = "Clock source is LFRC value."]
    Lfrc = 0x0d,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    Rtc100hz = 0x0e,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HclkDiv4 = 0x0f,
    #[doc = "Clock source is XT / 4 value."]
    XtDiv4 = 0x10,
    #[doc = "Clock source is XT / 8 value."]
    XtDiv8 = 0x11,
    #[doc = "Clock source is XT / 32 value."]
    XtDiv32 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    Ctmra7 = 0x14,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    Ctmra2 = 0x15,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    Ctmrb2 = 0x16,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    Ctmra0 = 0x17,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    Ctmrb0 = 0x18,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    Ctmrb1 = 0x19,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    Ctmrb3 = 0x1a,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    Ctmrb4 = 0x1b,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    Ctmrb5 = 0x1c,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    Buckble = 0x1d,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    Buckb = 0x1e,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    Bucka = 0x1f,
}
impl Tmrb7clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb7clk {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb7clk {
    #[inline(always)]
    fn from(val: u8) -> Tmrb7clk {
        Tmrb7clk::from_bits(val)
    }
}
impl From<Tmrb7clk> for u8 {
    #[inline(always)]
    fn from(val: Tmrb7clk) -> u8 {
        Tmrb7clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb7clr {
    #[doc = "Allow counter/timer B7 to run value."]
    Run = 0x0,
    #[doc = "Holds counter/timer B7 at 0x0000. value."]
    Clear = 0x01,
}
impl Tmrb7clr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb7clr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb7clr {
    #[inline(always)]
    fn from(val: u8) -> Tmrb7clr {
        Tmrb7clr::from_bits(val)
    }
}
impl From<Tmrb7clr> for u8 {
    #[inline(always)]
    fn from(val: Tmrb7clr) -> u8 {
        Tmrb7clr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb7en23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmrb7en23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb7en23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb7en23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb7en23 {
        Tmrb7en23::from_bits(val)
    }
}
impl From<Tmrb7en23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb7en23) -> u8 {
        Tmrb7en23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb7fn {
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B7, stop. value."]
    Singlecount = 0x0,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B7, restart. value."]
    Repeatedcount = 0x01,
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B7, assert, count to CMPR1B7, deassert, stop. value."]
    PulseOnce = 0x02,
    #[doc = "Pulse continously. Count to CMPR0B7, assert, count to CMPR1B7, deassert, restart. value."]
    PulseCont = 0x03,
    #[doc = "Single pattern. value."]
    Singlepattern = 0x04,
    #[doc = "Repeated pattern. value."]
    Repeatpattern = 0x05,
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    Continuous = 0x06,
    #[doc = "Alternate PWM value."]
    Altpwn = 0x07,
}
impl Tmrb7fn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb7fn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb7fn {
    #[inline(always)]
    fn from(val: u8) -> Tmrb7fn {
        Tmrb7fn::from_bits(val)
    }
}
impl From<Tmrb7fn> for u8 {
    #[inline(always)]
    fn from(val: Tmrb7fn) -> u8 {
        Tmrb7fn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb7nosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmrb7nosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb7nosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb7nosync {
    #[inline(always)]
    fn from(val: u8) -> Tmrb7nosync {
        Tmrb7nosync::from_bits(val)
    }
}
impl From<Tmrb7nosync> for u8 {
    #[inline(always)]
    fn from(val: Tmrb7nosync) -> u8 {
        Tmrb7nosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb7pol {
    #[doc = "The polarity of the TMRPINB7 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINB7 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmrb7pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb7pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb7pol {
    #[inline(always)]
    fn from(val: u8) -> Tmrb7pol {
        Tmrb7pol::from_bits(val)
    }
}
impl From<Tmrb7pol> for u8 {
    #[inline(always)]
    fn from(val: Tmrb7pol) -> u8 {
        Tmrb7pol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb7pol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmrb7pol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb7pol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb7pol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrb7pol23 {
        Tmrb7pol23::from_bits(val)
    }
}
impl From<Tmrb7pol23> for u8 {
    #[inline(always)]
    fn from(val: Tmrb7pol23) -> u8 {
        Tmrb7pol23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrb7trig {
    #[doc = "Trigger source is disabled. value."]
    Dis = 0x0,
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    A7out = 0x01,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3out = 0x02,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3out = 0x03,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5out = 0x04,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5out = 0x05,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2out = 0x06,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2out = 0x07,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3out2 = 0x08,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3out2 = 0x09,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2out2 = 0x0a,
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    B2out2 = 0x0b,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6out2dual = 0x0c,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7out2dual = 0x0d,
    #[doc = "Trigger source is CTIMERB1 OUT2, dual edge. value."]
    B1out2dual = 0x0e,
    #[doc = "Trigger source is CTIMERA1 OUT2, dual edge. value."]
    A1out2dual = 0x0f,
}
impl Tmrb7trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrb7trig {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrb7trig {
    #[inline(always)]
    fn from(val: u8) -> Tmrb7trig {
        Tmrb7trig::from_bits(val)
    }
}
impl From<Tmrb7trig> for u8 {
    #[inline(always)]
    fn from(val: Tmrb7trig) -> u8 {
        Tmrb7trig::to_bits(val)
    }
}

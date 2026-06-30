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
pub enum Ctlink {
    #[doc = "Use A0/B0 timers as two independent 16-bit timers (default). value."]
    Two16bitTimers = 0x0,
    #[doc = "Link A0/B0 timers into a single 32-bit timer. value."]
    _32bitTimer = 0x01,
}
impl Ctlink {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctlink {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctlink {
    #[inline(always)]
    fn from(val: u8) -> Ctlink {
        Ctlink::from_bits(val)
    }
}
impl From<Ctlink> for u8 {
    #[inline(always)]
    fn from(val: Ctlink) -> u8 {
        Ctlink::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En {
    #[doc = "Disable CTIMER. value."]
    Dis = 0x0,
    #[doc = "Use local enable. value."]
    Lco = 0x01,
}
impl En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En {
    #[inline(always)]
    fn from(val: u8) -> En {
        En::from_bits(val)
    }
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(val: En) -> u8 {
        En::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmren23 {
    #[doc = "Enable enhanced functions. value."]
    En = 0x0,
    #[doc = "Disable enhanced functions. value."]
    Dis = 0x01,
}
impl Tmren23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmren23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmren23 {
    #[inline(always)]
    fn from(val: u8) -> Tmren23 {
        Tmren23::from_bits(val)
    }
}
impl From<Tmren23> for u8 {
    #[inline(always)]
    fn from(val: Tmren23) -> u8 {
        Tmren23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrfn {
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
impl Tmrfn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrfn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrfn {
    #[inline(always)]
    fn from(val: u8) -> Tmrfn {
        Tmrfn::from_bits(val)
    }
}
impl From<Tmrfn> for u8 {
    #[inline(always)]
    fn from(val: Tmrfn) -> u8 {
        Tmrfn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrnosync {
    #[doc = "Synchronization on source clock value."]
    Dis = 0x0,
    #[doc = "No synchronization on source clock value."]
    Nosync = 0x01,
}
impl Tmrnosync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrnosync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrnosync {
    #[inline(always)]
    fn from(val: u8) -> Tmrnosync {
        Tmrnosync::from_bits(val)
    }
}
impl From<Tmrnosync> for u8 {
    #[inline(always)]
    fn from(val: Tmrnosync) -> u8 {
        Tmrnosync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrpol {
    #[doc = "The polarity of the TMRPINA0 pin is the same as the timer output. value."]
    Normal = 0x0,
    #[doc = "The polarity of the TMRPINA0 pin is the inverse of the timer output. value."]
    Inverted = 0x01,
}
impl Tmrpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrpol {
    #[inline(always)]
    fn from(val: u8) -> Tmrpol {
        Tmrpol::from_bits(val)
    }
}
impl From<Tmrpol> for u8 {
    #[inline(always)]
    fn from(val: Tmrpol) -> u8 {
        Tmrpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrpol23 {
    #[doc = "Upper output normal polarity value."]
    Norm = 0x0,
    #[doc = "Upper output inverted polarity. value."]
    Inv = 0x01,
}
impl Tmrpol23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrpol23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrpol23 {
    #[inline(always)]
    fn from(val: u8) -> Tmrpol23 {
        Tmrpol23::from_bits(val)
    }
}
impl From<Tmrpol23> for u8 {
    #[inline(always)]
    fn from(val: Tmrpol23) -> u8 {
        Tmrpol23::to_bits(val)
    }
}

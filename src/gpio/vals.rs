#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En0 {
    #[doc = "Enable CT0 for output value."]
    En = 0x0,
    #[doc = "Disable CT0 for output value."]
    Dis = 0x01,
}
impl En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En0 {
    #[inline(always)]
    fn from(val: u8) -> En0 {
        En0::from_bits(val)
    }
}
impl From<En0> for u8 {
    #[inline(always)]
    fn from(val: En0) -> u8 {
        En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En1 {
    #[doc = "Enable CT1 for output value."]
    En = 0x0,
    #[doc = "Disable CT1 for output value."]
    Dis = 0x01,
}
impl En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En1 {
    #[inline(always)]
    fn from(val: u8) -> En1 {
        En1::from_bits(val)
    }
}
impl From<En1> for u8 {
    #[inline(always)]
    fn from(val: En1) -> u8 {
        En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En10 {
    #[doc = "Enable CT10 for output value."]
    En = 0x0,
    #[doc = "Disable CT10 for output value."]
    Dis = 0x01,
}
impl En10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En10 {
    #[inline(always)]
    fn from(val: u8) -> En10 {
        En10::from_bits(val)
    }
}
impl From<En10> for u8 {
    #[inline(always)]
    fn from(val: En10) -> u8 {
        En10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En11 {
    #[doc = "Enable CT11 for output value."]
    En = 0x0,
    #[doc = "Disable CT11 for output value."]
    Dis = 0x01,
}
impl En11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En11 {
    #[inline(always)]
    fn from(val: u8) -> En11 {
        En11::from_bits(val)
    }
}
impl From<En11> for u8 {
    #[inline(always)]
    fn from(val: En11) -> u8 {
        En11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En12 {
    #[doc = "Enable CT12 for output value."]
    En = 0x0,
    #[doc = "Disable CT12 for output value."]
    Dis = 0x01,
}
impl En12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En12 {
    #[inline(always)]
    fn from(val: u8) -> En12 {
        En12::from_bits(val)
    }
}
impl From<En12> for u8 {
    #[inline(always)]
    fn from(val: En12) -> u8 {
        En12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En13 {
    #[doc = "Enable CT13 for output value."]
    En = 0x0,
    #[doc = "Disable CT13 for output value."]
    Dis = 0x01,
}
impl En13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En13 {
    #[inline(always)]
    fn from(val: u8) -> En13 {
        En13::from_bits(val)
    }
}
impl From<En13> for u8 {
    #[inline(always)]
    fn from(val: En13) -> u8 {
        En13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En14 {
    #[doc = "Enable CT14 for output value."]
    En = 0x0,
    #[doc = "Disable CT14 for output value."]
    Dis = 0x01,
}
impl En14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En14 {
    #[inline(always)]
    fn from(val: u8) -> En14 {
        En14::from_bits(val)
    }
}
impl From<En14> for u8 {
    #[inline(always)]
    fn from(val: En14) -> u8 {
        En14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En15 {
    #[doc = "Enable CT15 for output value."]
    En = 0x0,
    #[doc = "Disable CT15 for output value."]
    Dis = 0x01,
}
impl En15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En15 {
    #[inline(always)]
    fn from(val: u8) -> En15 {
        En15::from_bits(val)
    }
}
impl From<En15> for u8 {
    #[inline(always)]
    fn from(val: En15) -> u8 {
        En15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En16 {
    #[doc = "Enable CT16 for output value."]
    En = 0x0,
    #[doc = "Disable CT16 for output value."]
    Dis = 0x01,
}
impl En16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En16 {
    #[inline(always)]
    fn from(val: u8) -> En16 {
        En16::from_bits(val)
    }
}
impl From<En16> for u8 {
    #[inline(always)]
    fn from(val: En16) -> u8 {
        En16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En17 {
    #[doc = "Enable CT17 for output value."]
    En = 0x0,
    #[doc = "Disable CT17 for output value."]
    Dis = 0x01,
}
impl En17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En17 {
    #[inline(always)]
    fn from(val: u8) -> En17 {
        En17::from_bits(val)
    }
}
impl From<En17> for u8 {
    #[inline(always)]
    fn from(val: En17) -> u8 {
        En17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En18 {
    #[doc = "Enable CT18 for output value."]
    En = 0x0,
    #[doc = "Disable CT18 for output value."]
    Dis = 0x01,
}
impl En18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En18 {
    #[inline(always)]
    fn from(val: u8) -> En18 {
        En18::from_bits(val)
    }
}
impl From<En18> for u8 {
    #[inline(always)]
    fn from(val: En18) -> u8 {
        En18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En19 {
    #[doc = "Enable CT19 for output value."]
    En = 0x0,
    #[doc = "Disable CT19 for output value."]
    Dis = 0x01,
}
impl En19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En19 {
    #[inline(always)]
    fn from(val: u8) -> En19 {
        En19::from_bits(val)
    }
}
impl From<En19> for u8 {
    #[inline(always)]
    fn from(val: En19) -> u8 {
        En19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En2 {
    #[doc = "Enable CT2 for output value."]
    En = 0x0,
    #[doc = "Disable CT2 for output value."]
    Dis = 0x01,
}
impl En2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En2 {
    #[inline(always)]
    fn from(val: u8) -> En2 {
        En2::from_bits(val)
    }
}
impl From<En2> for u8 {
    #[inline(always)]
    fn from(val: En2) -> u8 {
        En2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En20 {
    #[doc = "Enable CT20 for output value."]
    En = 0x0,
    #[doc = "Disable CT20 for output value."]
    Dis = 0x01,
}
impl En20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En20 {
    #[inline(always)]
    fn from(val: u8) -> En20 {
        En20::from_bits(val)
    }
}
impl From<En20> for u8 {
    #[inline(always)]
    fn from(val: En20) -> u8 {
        En20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En21 {
    #[doc = "Enable CT21 for output value."]
    En = 0x0,
    #[doc = "Disable CT21 for output value."]
    Dis = 0x01,
}
impl En21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En21 {
    #[inline(always)]
    fn from(val: u8) -> En21 {
        En21::from_bits(val)
    }
}
impl From<En21> for u8 {
    #[inline(always)]
    fn from(val: En21) -> u8 {
        En21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En22 {
    #[doc = "Enable CT22 for output value."]
    En = 0x0,
    #[doc = "Disable CT22 for output value."]
    Dis = 0x01,
}
impl En22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En22 {
    #[inline(always)]
    fn from(val: u8) -> En22 {
        En22::from_bits(val)
    }
}
impl From<En22> for u8 {
    #[inline(always)]
    fn from(val: En22) -> u8 {
        En22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En23 {
    #[doc = "Enable CT23 for output value."]
    En = 0x0,
    #[doc = "Disable CT23 for output value."]
    Dis = 0x01,
}
impl En23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En23 {
    #[inline(always)]
    fn from(val: u8) -> En23 {
        En23::from_bits(val)
    }
}
impl From<En23> for u8 {
    #[inline(always)]
    fn from(val: En23) -> u8 {
        En23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En24 {
    #[doc = "Enable CT24 for output value."]
    En = 0x0,
    #[doc = "Disable CT24 for output value."]
    Dis = 0x01,
}
impl En24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En24 {
    #[inline(always)]
    fn from(val: u8) -> En24 {
        En24::from_bits(val)
    }
}
impl From<En24> for u8 {
    #[inline(always)]
    fn from(val: En24) -> u8 {
        En24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En25 {
    #[doc = "Enable CT25 for output value."]
    En = 0x0,
    #[doc = "Disable CT25 for output value."]
    Dis = 0x01,
}
impl En25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En25 {
    #[inline(always)]
    fn from(val: u8) -> En25 {
        En25::from_bits(val)
    }
}
impl From<En25> for u8 {
    #[inline(always)]
    fn from(val: En25) -> u8 {
        En25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En26 {
    #[doc = "Enable CT26 for output value."]
    En = 0x0,
    #[doc = "Disable CT26 for output value."]
    Dis = 0x01,
}
impl En26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En26 {
    #[inline(always)]
    fn from(val: u8) -> En26 {
        En26::from_bits(val)
    }
}
impl From<En26> for u8 {
    #[inline(always)]
    fn from(val: En26) -> u8 {
        En26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En27 {
    #[doc = "Enable CT27 for output value."]
    En = 0x0,
    #[doc = "Disable CT27 for output value."]
    Dis = 0x01,
}
impl En27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En27 {
    #[inline(always)]
    fn from(val: u8) -> En27 {
        En27::from_bits(val)
    }
}
impl From<En27> for u8 {
    #[inline(always)]
    fn from(val: En27) -> u8 {
        En27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En28 {
    #[doc = "Enable CT28 for output value."]
    En = 0x0,
    #[doc = "Disable CT28 for output value."]
    Dis = 0x01,
}
impl En28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En28 {
    #[inline(always)]
    fn from(val: u8) -> En28 {
        En28::from_bits(val)
    }
}
impl From<En28> for u8 {
    #[inline(always)]
    fn from(val: En28) -> u8 {
        En28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En29 {
    #[doc = "Enable CT29 for output value."]
    En = 0x0,
    #[doc = "Disable CT29 for output value."]
    Dis = 0x01,
}
impl En29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En29 {
    #[inline(always)]
    fn from(val: u8) -> En29 {
        En29::from_bits(val)
    }
}
impl From<En29> for u8 {
    #[inline(always)]
    fn from(val: En29) -> u8 {
        En29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En3 {
    #[doc = "Enable CT3 for output value."]
    En = 0x0,
    #[doc = "Disable CT3 for output value."]
    Dis = 0x01,
}
impl En3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En3 {
    #[inline(always)]
    fn from(val: u8) -> En3 {
        En3::from_bits(val)
    }
}
impl From<En3> for u8 {
    #[inline(always)]
    fn from(val: En3) -> u8 {
        En3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En30 {
    #[doc = "Enable CT30 for output value."]
    En = 0x0,
    #[doc = "Disable CT30 for output value."]
    Dis = 0x01,
}
impl En30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En30 {
    #[inline(always)]
    fn from(val: u8) -> En30 {
        En30::from_bits(val)
    }
}
impl From<En30> for u8 {
    #[inline(always)]
    fn from(val: En30) -> u8 {
        En30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En31 {
    #[doc = "Enable CT31 for output value."]
    En = 0x0,
    #[doc = "Disable CT31 for output value."]
    Dis = 0x01,
}
impl En31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En31 {
    #[inline(always)]
    fn from(val: u8) -> En31 {
        En31::from_bits(val)
    }
}
impl From<En31> for u8 {
    #[inline(always)]
    fn from(val: En31) -> u8 {
        En31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En4 {
    #[doc = "Enable CT4 for output value."]
    En = 0x0,
    #[doc = "Disable CT4 for output value."]
    Dis = 0x01,
}
impl En4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En4 {
    #[inline(always)]
    fn from(val: u8) -> En4 {
        En4::from_bits(val)
    }
}
impl From<En4> for u8 {
    #[inline(always)]
    fn from(val: En4) -> u8 {
        En4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En5 {
    #[doc = "Enable CT5 for output value."]
    En = 0x0,
    #[doc = "Disable CT5 for output value."]
    Dis = 0x01,
}
impl En5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En5 {
    #[inline(always)]
    fn from(val: u8) -> En5 {
        En5::from_bits(val)
    }
}
impl From<En5> for u8 {
    #[inline(always)]
    fn from(val: En5) -> u8 {
        En5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En6 {
    #[doc = "Enable CT6 for output value."]
    En = 0x0,
    #[doc = "Disable CT6 for output value."]
    Dis = 0x01,
}
impl En6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En6 {
    #[inline(always)]
    fn from(val: u8) -> En6 {
        En6::from_bits(val)
    }
}
impl From<En6> for u8 {
    #[inline(always)]
    fn from(val: En6) -> u8 {
        En6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En7 {
    #[doc = "Enable CT7 for output value."]
    En = 0x0,
    #[doc = "Disable CT7 for output value."]
    Dis = 0x01,
}
impl En7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En7 {
    #[inline(always)]
    fn from(val: u8) -> En7 {
        En7::from_bits(val)
    }
}
impl From<En7> for u8 {
    #[inline(always)]
    fn from(val: En7) -> u8 {
        En7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En8 {
    #[doc = "Enable CT8 for output value."]
    En = 0x0,
    #[doc = "Disable CT8 for output value."]
    Dis = 0x01,
}
impl En8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En8 {
    #[inline(always)]
    fn from(val: u8) -> En8 {
        En8::from_bits(val)
    }
}
impl From<En8> for u8 {
    #[inline(always)]
    fn from(val: En8) -> u8 {
        En8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio0incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio0incfg {
        Gpio0incfg::from_bits(val)
    }
}
impl From<Gpio0incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio0incfg) -> u8 {
        Gpio0incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0intd {
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio0intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio0intd {
        Gpio0intd::from_bits(val)
    }
}
impl From<Gpio0intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio0intd) -> u8 {
        Gpio0intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio0outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio0outcfg {
        Gpio0outcfg::from_bits(val)
    }
}
impl From<Gpio0outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio0outcfg) -> u8 {
        Gpio0outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio10incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio10incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio10incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio10incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio10incfg {
        Gpio10incfg::from_bits(val)
    }
}
impl From<Gpio10incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio10incfg) -> u8 {
        Gpio10incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio10intd {
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio10intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio10intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio10intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio10intd {
        Gpio10intd::from_bits(val)
    }
}
impl From<Gpio10intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio10intd) -> u8 {
        Gpio10intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio10outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio10outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio10outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio10outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio10outcfg {
        Gpio10outcfg::from_bits(val)
    }
}
impl From<Gpio10outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio10outcfg) -> u8 {
        Gpio10outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio11incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio11incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio11incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio11incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio11incfg {
        Gpio11incfg::from_bits(val)
    }
}
impl From<Gpio11incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio11incfg) -> u8 {
        Gpio11incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio11intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio11intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio11intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio11intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio11intd {
        Gpio11intd::from_bits(val)
    }
}
impl From<Gpio11intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio11intd) -> u8 {
        Gpio11intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio11outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio11outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio11outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio11outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio11outcfg {
        Gpio11outcfg::from_bits(val)
    }
}
impl From<Gpio11outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio11outcfg) -> u8 {
        Gpio11outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio12incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio12incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio12incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio12incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio12incfg {
        Gpio12incfg::from_bits(val)
    }
}
impl From<Gpio12incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio12incfg) -> u8 {
        Gpio12incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio12intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio12intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio12intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio12intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio12intd {
        Gpio12intd::from_bits(val)
    }
}
impl From<Gpio12intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio12intd) -> u8 {
        Gpio12intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio12outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio12outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio12outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio12outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio12outcfg {
        Gpio12outcfg::from_bits(val)
    }
}
impl From<Gpio12outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio12outcfg) -> u8 {
        Gpio12outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio13incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio13incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio13incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio13incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio13incfg {
        Gpio13incfg::from_bits(val)
    }
}
impl From<Gpio13incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio13incfg) -> u8 {
        Gpio13incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio13intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio13intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio13intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio13intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio13intd {
        Gpio13intd::from_bits(val)
    }
}
impl From<Gpio13intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio13intd) -> u8 {
        Gpio13intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio13outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio13outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio13outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio13outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio13outcfg {
        Gpio13outcfg::from_bits(val)
    }
}
impl From<Gpio13outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio13outcfg) -> u8 {
        Gpio13outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio14incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio14incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio14incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio14incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio14incfg {
        Gpio14incfg::from_bits(val)
    }
}
impl From<Gpio14incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio14incfg) -> u8 {
        Gpio14incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio14intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio14intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio14intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio14intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio14intd {
        Gpio14intd::from_bits(val)
    }
}
impl From<Gpio14intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio14intd) -> u8 {
        Gpio14intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio14outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio14outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio14outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio14outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio14outcfg {
        Gpio14outcfg::from_bits(val)
    }
}
impl From<Gpio14outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio14outcfg) -> u8 {
        Gpio14outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio15incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio15incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio15incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio15incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio15incfg {
        Gpio15incfg::from_bits(val)
    }
}
impl From<Gpio15incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio15incfg) -> u8 {
        Gpio15incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio15intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio15intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio15intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio15intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio15intd {
        Gpio15intd::from_bits(val)
    }
}
impl From<Gpio15intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio15intd) -> u8 {
        Gpio15intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio15outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio15outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio15outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio15outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio15outcfg {
        Gpio15outcfg::from_bits(val)
    }
}
impl From<Gpio15outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio15outcfg) -> u8 {
        Gpio15outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio16incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio16incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio16incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio16incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio16incfg {
        Gpio16incfg::from_bits(val)
    }
}
impl From<Gpio16incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio16incfg) -> u8 {
        Gpio16incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio16intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio16intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio16intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio16intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio16intd {
        Gpio16intd::from_bits(val)
    }
}
impl From<Gpio16intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio16intd) -> u8 {
        Gpio16intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio16outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio16outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio16outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio16outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio16outcfg {
        Gpio16outcfg::from_bits(val)
    }
}
impl From<Gpio16outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio16outcfg) -> u8 {
        Gpio16outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio17incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio17incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio17incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio17incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio17incfg {
        Gpio17incfg::from_bits(val)
    }
}
impl From<Gpio17incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio17incfg) -> u8 {
        Gpio17incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio17intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio17intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio17intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio17intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio17intd {
        Gpio17intd::from_bits(val)
    }
}
impl From<Gpio17intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio17intd) -> u8 {
        Gpio17intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio17outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio17outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio17outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio17outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio17outcfg {
        Gpio17outcfg::from_bits(val)
    }
}
impl From<Gpio17outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio17outcfg) -> u8 {
        Gpio17outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio18incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio18incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio18incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio18incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio18incfg {
        Gpio18incfg::from_bits(val)
    }
}
impl From<Gpio18incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio18incfg) -> u8 {
        Gpio18incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio18intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio18intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio18intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio18intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio18intd {
        Gpio18intd::from_bits(val)
    }
}
impl From<Gpio18intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio18intd) -> u8 {
        Gpio18intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio18outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio18outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio18outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio18outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio18outcfg {
        Gpio18outcfg::from_bits(val)
    }
}
impl From<Gpio18outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio18outcfg) -> u8 {
        Gpio18outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio19incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio19incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio19incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio19incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio19incfg {
        Gpio19incfg::from_bits(val)
    }
}
impl From<Gpio19incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio19incfg) -> u8 {
        Gpio19incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio19intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio19intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio19intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio19intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio19intd {
        Gpio19intd::from_bits(val)
    }
}
impl From<Gpio19intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio19intd) -> u8 {
        Gpio19intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio19outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio19outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio19outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio19outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio19outcfg {
        Gpio19outcfg::from_bits(val)
    }
}
impl From<Gpio19outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio19outcfg) -> u8 {
        Gpio19outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio1incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio1incfg {
        Gpio1incfg::from_bits(val)
    }
}
impl From<Gpio1incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio1incfg) -> u8 {
        Gpio1incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1intd {
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio1intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio1intd {
        Gpio1intd::from_bits(val)
    }
}
impl From<Gpio1intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio1intd) -> u8 {
        Gpio1intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio1outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio1outcfg {
        Gpio1outcfg::from_bits(val)
    }
}
impl From<Gpio1outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio1outcfg) -> u8 {
        Gpio1outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio20incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio20incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio20incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio20incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio20incfg {
        Gpio20incfg::from_bits(val)
    }
}
impl From<Gpio20incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio20incfg) -> u8 {
        Gpio20incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio20intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio20intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio20intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio20intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio20intd {
        Gpio20intd::from_bits(val)
    }
}
impl From<Gpio20intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio20intd) -> u8 {
        Gpio20intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio20outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio20outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio20outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio20outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio20outcfg {
        Gpio20outcfg::from_bits(val)
    }
}
impl From<Gpio20outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio20outcfg) -> u8 {
        Gpio20outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio21incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio21incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio21incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio21incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio21incfg {
        Gpio21incfg::from_bits(val)
    }
}
impl From<Gpio21incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio21incfg) -> u8 {
        Gpio21incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio21intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio21intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio21intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio21intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio21intd {
        Gpio21intd::from_bits(val)
    }
}
impl From<Gpio21intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio21intd) -> u8 {
        Gpio21intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio21outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio21outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio21outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio21outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio21outcfg {
        Gpio21outcfg::from_bits(val)
    }
}
impl From<Gpio21outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio21outcfg) -> u8 {
        Gpio21outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio22incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio22incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio22incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio22incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio22incfg {
        Gpio22incfg::from_bits(val)
    }
}
impl From<Gpio22incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio22incfg) -> u8 {
        Gpio22incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio22intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio22intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio22intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio22intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio22intd {
        Gpio22intd::from_bits(val)
    }
}
impl From<Gpio22intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio22intd) -> u8 {
        Gpio22intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio22outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio22outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio22outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio22outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio22outcfg {
        Gpio22outcfg::from_bits(val)
    }
}
impl From<Gpio22outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio22outcfg) -> u8 {
        Gpio22outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio23incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio23incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio23incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio23incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio23incfg {
        Gpio23incfg::from_bits(val)
    }
}
impl From<Gpio23incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio23incfg) -> u8 {
        Gpio23incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio23intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio23intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio23intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio23intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio23intd {
        Gpio23intd::from_bits(val)
    }
}
impl From<Gpio23intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio23intd) -> u8 {
        Gpio23intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio23outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio23outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio23outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio23outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio23outcfg {
        Gpio23outcfg::from_bits(val)
    }
}
impl From<Gpio23outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio23outcfg) -> u8 {
        Gpio23outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio24incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio24incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio24incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio24incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio24incfg {
        Gpio24incfg::from_bits(val)
    }
}
impl From<Gpio24incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio24incfg) -> u8 {
        Gpio24incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio24intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio24intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio24intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio24intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio24intd {
        Gpio24intd::from_bits(val)
    }
}
impl From<Gpio24intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio24intd) -> u8 {
        Gpio24intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio24outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio24outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio24outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio24outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio24outcfg {
        Gpio24outcfg::from_bits(val)
    }
}
impl From<Gpio24outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio24outcfg) -> u8 {
        Gpio24outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio25incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio25incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio25incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio25incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio25incfg {
        Gpio25incfg::from_bits(val)
    }
}
impl From<Gpio25incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio25incfg) -> u8 {
        Gpio25incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio25intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio25intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio25intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio25intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio25intd {
        Gpio25intd::from_bits(val)
    }
}
impl From<Gpio25intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio25intd) -> u8 {
        Gpio25intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio25outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio25outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio25outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio25outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio25outcfg {
        Gpio25outcfg::from_bits(val)
    }
}
impl From<Gpio25outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio25outcfg) -> u8 {
        Gpio25outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio26incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio26incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio26incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio26incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio26incfg {
        Gpio26incfg::from_bits(val)
    }
}
impl From<Gpio26incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio26incfg) -> u8 {
        Gpio26incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio26intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio26intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio26intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio26intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio26intd {
        Gpio26intd::from_bits(val)
    }
}
impl From<Gpio26intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio26intd) -> u8 {
        Gpio26intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio26outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio26outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio26outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio26outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio26outcfg {
        Gpio26outcfg::from_bits(val)
    }
}
impl From<Gpio26outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio26outcfg) -> u8 {
        Gpio26outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio27incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio27incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio27incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio27incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio27incfg {
        Gpio27incfg::from_bits(val)
    }
}
impl From<Gpio27incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio27incfg) -> u8 {
        Gpio27incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio27intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio27intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio27intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio27intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio27intd {
        Gpio27intd::from_bits(val)
    }
}
impl From<Gpio27intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio27intd) -> u8 {
        Gpio27intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio27outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio27outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio27outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio27outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio27outcfg {
        Gpio27outcfg::from_bits(val)
    }
}
impl From<Gpio27outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio27outcfg) -> u8 {
        Gpio27outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio28incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio28incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio28incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio28incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio28incfg {
        Gpio28incfg::from_bits(val)
    }
}
impl From<Gpio28incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio28incfg) -> u8 {
        Gpio28incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio28intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio28intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio28intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio28intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio28intd {
        Gpio28intd::from_bits(val)
    }
}
impl From<Gpio28intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio28intd) -> u8 {
        Gpio28intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio28outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio28outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio28outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio28outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio28outcfg {
        Gpio28outcfg::from_bits(val)
    }
}
impl From<Gpio28outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio28outcfg) -> u8 {
        Gpio28outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio29incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio29incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio29incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio29incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio29incfg {
        Gpio29incfg::from_bits(val)
    }
}
impl From<Gpio29incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio29incfg) -> u8 {
        Gpio29incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio29intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio29intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio29intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio29intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio29intd {
        Gpio29intd::from_bits(val)
    }
}
impl From<Gpio29intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio29intd) -> u8 {
        Gpio29intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio29outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio29outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio29outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio29outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio29outcfg {
        Gpio29outcfg::from_bits(val)
    }
}
impl From<Gpio29outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio29outcfg) -> u8 {
        Gpio29outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio2incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio2incfg {
        Gpio2incfg::from_bits(val)
    }
}
impl From<Gpio2incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio2incfg) -> u8 {
        Gpio2incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2intd {
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio2intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio2intd {
        Gpio2intd::from_bits(val)
    }
}
impl From<Gpio2intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio2intd) -> u8 {
        Gpio2intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio2outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio2outcfg {
        Gpio2outcfg::from_bits(val)
    }
}
impl From<Gpio2outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio2outcfg) -> u8 {
        Gpio2outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio30incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio30incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio30incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio30incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio30incfg {
        Gpio30incfg::from_bits(val)
    }
}
impl From<Gpio30incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio30incfg) -> u8 {
        Gpio30incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio30intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio30intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio30intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio30intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio30intd {
        Gpio30intd::from_bits(val)
    }
}
impl From<Gpio30intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio30intd) -> u8 {
        Gpio30intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio30outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio30outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio30outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio30outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio30outcfg {
        Gpio30outcfg::from_bits(val)
    }
}
impl From<Gpio30outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio30outcfg) -> u8 {
        Gpio30outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio31incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio31incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio31incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio31incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio31incfg {
        Gpio31incfg::from_bits(val)
    }
}
impl From<Gpio31incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio31incfg) -> u8 {
        Gpio31incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio31intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio31intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio31intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio31intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio31intd {
        Gpio31intd::from_bits(val)
    }
}
impl From<Gpio31intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio31intd) -> u8 {
        Gpio31intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio31outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio31outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio31outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio31outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio31outcfg {
        Gpio31outcfg::from_bits(val)
    }
}
impl From<Gpio31outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio31outcfg) -> u8 {
        Gpio31outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio32incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio32incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio32incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio32incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio32incfg {
        Gpio32incfg::from_bits(val)
    }
}
impl From<Gpio32incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio32incfg) -> u8 {
        Gpio32incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio32intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio32intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio32intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio32intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio32intd {
        Gpio32intd::from_bits(val)
    }
}
impl From<Gpio32intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio32intd) -> u8 {
        Gpio32intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio32outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio32outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio32outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio32outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio32outcfg {
        Gpio32outcfg::from_bits(val)
    }
}
impl From<Gpio32outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio32outcfg) -> u8 {
        Gpio32outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio33incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio33incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio33incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio33incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio33incfg {
        Gpio33incfg::from_bits(val)
    }
}
impl From<Gpio33incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio33incfg) -> u8 {
        Gpio33incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio33intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio33intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio33intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio33intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio33intd {
        Gpio33intd::from_bits(val)
    }
}
impl From<Gpio33intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio33intd) -> u8 {
        Gpio33intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio33outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio33outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio33outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio33outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio33outcfg {
        Gpio33outcfg::from_bits(val)
    }
}
impl From<Gpio33outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio33outcfg) -> u8 {
        Gpio33outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio34incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio34incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio34incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio34incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio34incfg {
        Gpio34incfg::from_bits(val)
    }
}
impl From<Gpio34incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio34incfg) -> u8 {
        Gpio34incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio34intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio34intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio34intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio34intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio34intd {
        Gpio34intd::from_bits(val)
    }
}
impl From<Gpio34intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio34intd) -> u8 {
        Gpio34intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio34outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio34outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio34outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio34outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio34outcfg {
        Gpio34outcfg::from_bits(val)
    }
}
impl From<Gpio34outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio34outcfg) -> u8 {
        Gpio34outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio35incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio35incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio35incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio35incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio35incfg {
        Gpio35incfg::from_bits(val)
    }
}
impl From<Gpio35incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio35incfg) -> u8 {
        Gpio35incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio35intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio35intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio35intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio35intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio35intd {
        Gpio35intd::from_bits(val)
    }
}
impl From<Gpio35intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio35intd) -> u8 {
        Gpio35intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio35outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio35outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio35outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio35outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio35outcfg {
        Gpio35outcfg::from_bits(val)
    }
}
impl From<Gpio35outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio35outcfg) -> u8 {
        Gpio35outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio36incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio36incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio36incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio36incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio36incfg {
        Gpio36incfg::from_bits(val)
    }
}
impl From<Gpio36incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio36incfg) -> u8 {
        Gpio36incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio36intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio36intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio36intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio36intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio36intd {
        Gpio36intd::from_bits(val)
    }
}
impl From<Gpio36intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio36intd) -> u8 {
        Gpio36intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio36outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio36outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio36outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio36outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio36outcfg {
        Gpio36outcfg::from_bits(val)
    }
}
impl From<Gpio36outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio36outcfg) -> u8 {
        Gpio36outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio37incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio37incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio37incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio37incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio37incfg {
        Gpio37incfg::from_bits(val)
    }
}
impl From<Gpio37incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio37incfg) -> u8 {
        Gpio37incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio37intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio37intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio37intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio37intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio37intd {
        Gpio37intd::from_bits(val)
    }
}
impl From<Gpio37intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio37intd) -> u8 {
        Gpio37intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio37outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio37outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio37outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio37outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio37outcfg {
        Gpio37outcfg::from_bits(val)
    }
}
impl From<Gpio37outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio37outcfg) -> u8 {
        Gpio37outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio38incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio38incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio38incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio38incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio38incfg {
        Gpio38incfg::from_bits(val)
    }
}
impl From<Gpio38incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio38incfg) -> u8 {
        Gpio38incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio38intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio38intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio38intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio38intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio38intd {
        Gpio38intd::from_bits(val)
    }
}
impl From<Gpio38intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio38intd) -> u8 {
        Gpio38intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio38outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio38outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio38outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio38outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio38outcfg {
        Gpio38outcfg::from_bits(val)
    }
}
impl From<Gpio38outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio38outcfg) -> u8 {
        Gpio38outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio39incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio39incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio39incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio39incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio39incfg {
        Gpio39incfg::from_bits(val)
    }
}
impl From<Gpio39incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio39incfg) -> u8 {
        Gpio39incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio39intd {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    Intdis = 0x0,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    Intboth = 0x01,
}
impl Gpio39intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio39intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio39intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio39intd {
        Gpio39intd::from_bits(val)
    }
}
impl From<Gpio39intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio39intd) -> u8 {
        Gpio39intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio39outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio39outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio39outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio39outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio39outcfg {
        Gpio39outcfg::from_bits(val)
    }
}
impl From<Gpio39outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio39outcfg) -> u8 {
        Gpio39outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio3incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio3incfg {
        Gpio3incfg::from_bits(val)
    }
}
impl From<Gpio3incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio3incfg) -> u8 {
        Gpio3incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3intd {
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio3intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio3intd {
        Gpio3intd::from_bits(val)
    }
}
impl From<Gpio3intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio3intd) -> u8 {
        Gpio3intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio3outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio3outcfg {
        Gpio3outcfg::from_bits(val)
    }
}
impl From<Gpio3outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio3outcfg) -> u8 {
        Gpio3outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio40incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio40incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio40incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio40incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio40incfg {
        Gpio40incfg::from_bits(val)
    }
}
impl From<Gpio40incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio40incfg) -> u8 {
        Gpio40incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio40intd {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    Intdis = 0x0,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    Intboth = 0x01,
}
impl Gpio40intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio40intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio40intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio40intd {
        Gpio40intd::from_bits(val)
    }
}
impl From<Gpio40intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio40intd) -> u8 {
        Gpio40intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio40outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio40outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio40outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio40outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio40outcfg {
        Gpio40outcfg::from_bits(val)
    }
}
impl From<Gpio40outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio40outcfg) -> u8 {
        Gpio40outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio41incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio41incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio41incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio41incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio41incfg {
        Gpio41incfg::from_bits(val)
    }
}
impl From<Gpio41incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio41incfg) -> u8 {
        Gpio41incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio41intd {
    #[doc = "FNCSEL = 0x0 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x0 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio41intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio41intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio41intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio41intd {
        Gpio41intd::from_bits(val)
    }
}
impl From<Gpio41intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio41intd) -> u8 {
        Gpio41intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio41outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio41outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio41outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio41outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio41outcfg {
        Gpio41outcfg::from_bits(val)
    }
}
impl From<Gpio41outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio41outcfg) -> u8 {
        Gpio41outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio42incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio42incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio42incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio42incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio42incfg {
        Gpio42incfg::from_bits(val)
    }
}
impl From<Gpio42incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio42incfg) -> u8 {
        Gpio42incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio42intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio42intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio42intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio42intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio42intd {
        Gpio42intd::from_bits(val)
    }
}
impl From<Gpio42intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio42intd) -> u8 {
        Gpio42intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio42outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio42outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio42outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio42outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio42outcfg {
        Gpio42outcfg::from_bits(val)
    }
}
impl From<Gpio42outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio42outcfg) -> u8 {
        Gpio42outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio43incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio43incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio43incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio43incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio43incfg {
        Gpio43incfg::from_bits(val)
    }
}
impl From<Gpio43incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio43incfg) -> u8 {
        Gpio43incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio43intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio43intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio43intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio43intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio43intd {
        Gpio43intd::from_bits(val)
    }
}
impl From<Gpio43intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio43intd) -> u8 {
        Gpio43intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio43outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio43outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio43outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio43outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio43outcfg {
        Gpio43outcfg::from_bits(val)
    }
}
impl From<Gpio43outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio43outcfg) -> u8 {
        Gpio43outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio44incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio44incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio44incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio44incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio44incfg {
        Gpio44incfg::from_bits(val)
    }
}
impl From<Gpio44incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio44incfg) -> u8 {
        Gpio44incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio44intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio44intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio44intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio44intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio44intd {
        Gpio44intd::from_bits(val)
    }
}
impl From<Gpio44intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio44intd) -> u8 {
        Gpio44intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio44outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio44outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio44outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio44outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio44outcfg {
        Gpio44outcfg::from_bits(val)
    }
}
impl From<Gpio44outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio44outcfg) -> u8 {
        Gpio44outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio45incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio45incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio45incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio45incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio45incfg {
        Gpio45incfg::from_bits(val)
    }
}
impl From<Gpio45incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio45incfg) -> u8 {
        Gpio45incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio45intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio45intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio45intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio45intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio45intd {
        Gpio45intd::from_bits(val)
    }
}
impl From<Gpio45intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio45intd) -> u8 {
        Gpio45intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio45outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio45outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio45outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio45outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio45outcfg {
        Gpio45outcfg::from_bits(val)
    }
}
impl From<Gpio45outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio45outcfg) -> u8 {
        Gpio45outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio46incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio46incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio46incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio46incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio46incfg {
        Gpio46incfg::from_bits(val)
    }
}
impl From<Gpio46incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio46incfg) -> u8 {
        Gpio46incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio46intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio46intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio46intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio46intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio46intd {
        Gpio46intd::from_bits(val)
    }
}
impl From<Gpio46intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio46intd) -> u8 {
        Gpio46intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio46outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio46outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio46outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio46outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio46outcfg {
        Gpio46outcfg::from_bits(val)
    }
}
impl From<Gpio46outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio46outcfg) -> u8 {
        Gpio46outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio47incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio47incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio47incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio47incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio47incfg {
        Gpio47incfg::from_bits(val)
    }
}
impl From<Gpio47incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio47incfg) -> u8 {
        Gpio47incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio47intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio47intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio47intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio47intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio47intd {
        Gpio47intd::from_bits(val)
    }
}
impl From<Gpio47intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio47intd) -> u8 {
        Gpio47intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio47outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio47outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio47outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio47outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio47outcfg {
        Gpio47outcfg::from_bits(val)
    }
}
impl From<Gpio47outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio47outcfg) -> u8 {
        Gpio47outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio48incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio48incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio48incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio48incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio48incfg {
        Gpio48incfg::from_bits(val)
    }
}
impl From<Gpio48incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio48incfg) -> u8 {
        Gpio48incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio48intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio48intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio48intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio48intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio48intd {
        Gpio48intd::from_bits(val)
    }
}
impl From<Gpio48intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio48intd) -> u8 {
        Gpio48intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio48outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio48outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio48outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio48outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio48outcfg {
        Gpio48outcfg::from_bits(val)
    }
}
impl From<Gpio48outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio48outcfg) -> u8 {
        Gpio48outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio49incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio49incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio49incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio49incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio49incfg {
        Gpio49incfg::from_bits(val)
    }
}
impl From<Gpio49incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio49incfg) -> u8 {
        Gpio49incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio49intd {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio49intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio49intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio49intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio49intd {
        Gpio49intd::from_bits(val)
    }
}
impl From<Gpio49intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio49intd) -> u8 {
        Gpio49intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio49outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio49outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio49outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio49outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio49outcfg {
        Gpio49outcfg::from_bits(val)
    }
}
impl From<Gpio49outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio49outcfg) -> u8 {
        Gpio49outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio4incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio4incfg {
        Gpio4incfg::from_bits(val)
    }
}
impl From<Gpio4incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio4incfg) -> u8 {
        Gpio4incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4intd {
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio4intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio4intd {
        Gpio4intd::from_bits(val)
    }
}
impl From<Gpio4intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio4intd) -> u8 {
        Gpio4intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio4outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio4outcfg {
        Gpio4outcfg::from_bits(val)
    }
}
impl From<Gpio4outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio4outcfg) -> u8 {
        Gpio4outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio5incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio5incfg {
        Gpio5incfg::from_bits(val)
    }
}
impl From<Gpio5incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio5incfg) -> u8 {
        Gpio5incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5intd {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    Intdis = 0x0,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    Intboth = 0x01,
}
impl Gpio5intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio5intd {
        Gpio5intd::from_bits(val)
    }
}
impl From<Gpio5intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio5intd) -> u8 {
        Gpio5intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio5outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio5outcfg {
        Gpio5outcfg::from_bits(val)
    }
}
impl From<Gpio5outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio5outcfg) -> u8 {
        Gpio5outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio6incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio6incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio6incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio6incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio6incfg {
        Gpio6incfg::from_bits(val)
    }
}
impl From<Gpio6incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio6incfg) -> u8 {
        Gpio6incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio6intd {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    Intdis = 0x0,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    Intboth = 0x01,
}
impl Gpio6intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio6intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio6intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio6intd {
        Gpio6intd::from_bits(val)
    }
}
impl From<Gpio6intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio6intd) -> u8 {
        Gpio6intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio6outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio6outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio6outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio6outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio6outcfg {
        Gpio6outcfg::from_bits(val)
    }
}
impl From<Gpio6outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio6outcfg) -> u8 {
        Gpio6outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio7incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio7incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio7incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio7incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio7incfg {
        Gpio7incfg::from_bits(val)
    }
}
impl From<Gpio7incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio7incfg) -> u8 {
        Gpio7incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio7intd {
    #[doc = "FNCSEL = 0x0 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x0 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio7intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio7intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio7intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio7intd {
        Gpio7intd::from_bits(val)
    }
}
impl From<Gpio7intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio7intd) -> u8 {
        Gpio7intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio7outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio7outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio7outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio7outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio7outcfg {
        Gpio7outcfg::from_bits(val)
    }
}
impl From<Gpio7outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio7outcfg) -> u8 {
        Gpio7outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio8incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio8incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio8incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio8incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio8incfg {
        Gpio8incfg::from_bits(val)
    }
}
impl From<Gpio8incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio8incfg) -> u8 {
        Gpio8incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio8intd {
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio8intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio8intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio8intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio8intd {
        Gpio8intd::from_bits(val)
    }
}
impl From<Gpio8intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio8intd) -> u8 {
        Gpio8intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio8outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio8outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio8outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio8outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio8outcfg {
        Gpio8outcfg::from_bits(val)
    }
}
impl From<Gpio8outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio8outcfg) -> u8 {
        Gpio8outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio9incfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpio9incfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio9incfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio9incfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio9incfg {
        Gpio9incfg::from_bits(val)
    }
}
impl From<Gpio9incfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio9incfg) -> u8 {
        Gpio9incfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio9intd {
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl Gpio9intd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio9intd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio9intd {
    #[inline(always)]
    fn from(val: u8) -> Gpio9intd {
        Gpio9intd::from_bits(val)
    }
}
impl From<Gpio9intd> for u8 {
    #[inline(always)]
    fn from(val: Gpio9intd) -> u8 {
        Gpio9intd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio9outcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpio9outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio9outcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio9outcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpio9outcfg {
        Gpio9outcfg::from_bits(val)
    }
}
impl From<Gpio9outcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpio9outcfg) -> u8 {
        Gpio9outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad0fncsel {
    #[doc = "Configure as the IOSLAVE I2C SCL signal value."]
    Slscl = 0x0,
    #[doc = "Configure as the IOSLAVE SPI SCK signal value."]
    Slsck = 0x01,
    #[doc = "Configure as the CLKOUT signal value."]
    Clkout = 0x02,
    #[doc = "Configure as GPIO0 value."]
    Gpio0 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "MSPI data connection 4 value."]
    Mspi4 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "IOM/MSPI nCE group 0 value."]
    Nce0 = 0x07,
}
impl Pad0fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad0fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad0fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad0fncsel {
        Pad0fncsel::from_bits(val)
    }
}
impl From<Pad0fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad0fncsel) -> u8 {
        Pad0fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad0rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad0rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad0rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad0rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad0rsel {
        Pad0rsel::from_bits(val)
    }
}
impl From<Pad0rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad0rsel) -> u8 {
        Pad0rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad0strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad0strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad0strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad0strng {
    #[inline(always)]
    fn from(val: u8) -> Pad0strng {
        Pad0strng::from_bits(val)
    }
}
impl From<Pad0strng> for u8 {
    #[inline(always)]
    fn from(val: Pad0strng) -> u8 {
        Pad0strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad10fncsel {
    _RESERVED_0 = 0x0,
    #[doc = "Configure as the IOMSTR1 SPI MOSI signal value."]
    M1mosi = 0x01,
    #[doc = "IOM/MSPI nCE group 10 value."]
    Nce10 = 0x02,
    #[doc = "Configure as GPIO10 value."]
    Gpio10 = 0x03,
    #[doc = "PDM serial clock out value."]
    Pdmclk = 0x04,
    #[doc = "Configure as the UART1 RTS output signal value."]
    Ua1rts = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad10fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad10fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad10fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad10fncsel {
        Pad10fncsel::from_bits(val)
    }
}
impl From<Pad10fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad10fncsel) -> u8 {
        Pad10fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad10strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad10strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad10strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad10strng {
    #[inline(always)]
    fn from(val: u8) -> Pad10strng {
        Pad10strng::from_bits(val)
    }
}
impl From<Pad10strng> for u8 {
    #[inline(always)]
    fn from(val: Pad10strng) -> u8 {
        Pad10strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad11fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 2 value."]
    Adcse2 = 0x0,
    #[doc = "IOM/MSPI nCE group 11 value."]
    Nce11 = 0x01,
    #[doc = "CTIMER connection 31 value."]
    Ct31 = 0x02,
    #[doc = "Configure as GPIO11 value."]
    Gpio11 = 0x03,
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    Slint = 0x04,
    #[doc = "Configure as the UART1 CTS input signal value."]
    Ua1cts = 0x05,
    #[doc = "Configure as the UART0 RX input signal value."]
    Uart0rx = 0x06,
    #[doc = "Configure as the PDM Data input signal value."]
    PdmData = 0x07,
}
impl Pad11fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad11fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad11fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad11fncsel {
        Pad11fncsel::from_bits(val)
    }
}
impl From<Pad11fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad11fncsel) -> u8 {
        Pad11fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad11strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad11strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad11strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad11strng {
    #[inline(always)]
    fn from(val: u8) -> Pad11strng {
        Pad11strng::from_bits(val)
    }
}
impl From<Pad11strng> for u8 {
    #[inline(always)]
    fn from(val: Pad11strng) -> u8 {
        Pad11strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad12fncsel {
    #[doc = "Configure as the ADC Differential pair 0 N, or Single Ended input 9 analog input signal. Determination of the D0N vs SE9 usage is done when the particular channel is selected within the ADC module value."]
    Adcd0nse9 = 0x0,
    #[doc = "IOM/MSPI nCE group 12 value."]
    Nce12 = 0x01,
    #[doc = "CTIMER connection 0 value."]
    Ct0 = 0x02,
    #[doc = "Configure as GPIO12 value."]
    Gpio12 = 0x03,
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    SLnCe = 0x04,
    #[doc = "PDM serial clock output value."]
    Pdmclk = 0x05,
    #[doc = "Configure as the UART0 CTS input signal value."]
    Ua0cts = 0x06,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x07,
}
impl Pad12fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad12fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad12fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad12fncsel {
        Pad12fncsel::from_bits(val)
    }
}
impl From<Pad12fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad12fncsel) -> u8 {
        Pad12fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad12strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad12strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad12strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad12strng {
    #[inline(always)]
    fn from(val: u8) -> Pad12strng {
        Pad12strng::from_bits(val)
    }
}
impl From<Pad12strng> for u8 {
    #[inline(always)]
    fn from(val: Pad12strng) -> u8 {
        Pad12strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad13fncsel {
    #[doc = "Configure as the ADC Differential pair 0 P, or Single Ended input 8 analog input signal. Determination of the D0P vs SE8 usage is done when the particular channel is selected within the ADC module value."]
    Adcd0pse8 = 0x0,
    #[doc = "IOM/MSPI nCE group 13 value."]
    Nce13 = 0x01,
    #[doc = "CTIMER connection 2 value."]
    Ct2 = 0x02,
    #[doc = "Configure as GPIO13 value."]
    Gpio13 = 0x03,
    #[doc = "I2C interface bit clock value."]
    I2sbclk = 0x04,
    #[doc = "Configure as the external HFRC oscillator input value."]
    Exthfb = 0x05,
    #[doc = "Configure as the UART0 RTS signal output value."]
    Ua0rts = 0x06,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x07,
}
impl Pad13fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad13fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad13fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad13fncsel {
        Pad13fncsel::from_bits(val)
    }
}
impl From<Pad13fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad13fncsel) -> u8 {
        Pad13fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad13strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad13strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad13strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad13strng {
    #[inline(always)]
    fn from(val: u8) -> Pad13strng {
        Pad13strng::from_bits(val)
    }
}
impl From<Pad13strng> for u8 {
    #[inline(always)]
    fn from(val: Pad13strng) -> u8 {
        Pad13strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad14fncsel {
    #[doc = "Configure as the analog ADC differential pair 1 P input signal value."]
    Adcd1p = 0x0,
    #[doc = "IOM/MSPI nCE group 14 value."]
    Nce14 = 0x01,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x02,
    #[doc = "Configure as GPIO14 value."]
    Gpio14 = 0x03,
    #[doc = "PDM serial clock output value."]
    Pdmclk = 0x04,
    #[doc = "Configure as the External HFRC oscillator input select value."]
    Exthfs = 0x05,
    #[doc = "Configure as the alternate input for the SWDCK input signal value."]
    Swdck = 0x06,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32kHzXt = 0x07,
}
impl Pad14fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad14fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad14fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad14fncsel {
        Pad14fncsel::from_bits(val)
    }
}
impl From<Pad14fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad14fncsel) -> u8 {
        Pad14fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad14strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad14strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad14strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad14strng {
    #[inline(always)]
    fn from(val: u8) -> Pad14strng {
        Pad14strng::from_bits(val)
    }
}
impl From<Pad14strng> for u8 {
    #[inline(always)]
    fn from(val: Pad14strng) -> u8 {
        Pad14strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad15fncsel {
    #[doc = "Configure as the analog ADC differential pair 1 N input signal value."]
    Adcd1n = 0x0,
    #[doc = "IOM/MSPI nCE group 15 value."]
    Nce15 = 0x01,
    #[doc = "Configure as the UART1 RX signal value."]
    Uart1rx = 0x02,
    #[doc = "Configure as GPIO15 value."]
    Gpio15 = 0x03,
    #[doc = "PDM serial data input value."]
    Pdmdata = 0x04,
    #[doc = "Configure as the external XTAL oscillator input value."]
    Extxt = 0x05,
    #[doc = "Configure as an alternate port for the SWDIO I/O signal value."]
    Swdio = 0x06,
    #[doc = "Configure as an SWO (Serial Wire Trace output) value."]
    Swo = 0x07,
}
impl Pad15fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad15fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad15fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad15fncsel {
        Pad15fncsel::from_bits(val)
    }
}
impl From<Pad15fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad15fncsel) -> u8 {
        Pad15fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad15strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad15strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad15strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad15strng {
    #[inline(always)]
    fn from(val: u8) -> Pad15strng {
        Pad15strng::from_bits(val)
    }
}
impl From<Pad15strng> for u8 {
    #[inline(always)]
    fn from(val: Pad15strng) -> u8 {
        Pad15strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad16fncsel {
    #[doc = "Configure as the analog ADC single ended port 0 input signal value."]
    Adcse0 = 0x0,
    #[doc = "IOM/MSPI nCE group 16 value."]
    Nce16 = 0x01,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    Trig0 = 0x02,
    #[doc = "Configure as GPIO16 value."]
    Gpio16 = 0x03,
    #[doc = "SCARD reset output value."]
    Sccrst = 0x04,
    #[doc = "Configure as comparator input 0 signal value."]
    Cmpin0 = 0x05,
    #[doc = "Configure as UART0 TX output signal value."]
    Uart0tx = 0x06,
    #[doc = "Configure as UART1 RTS output signal value."]
    Ua1rts = 0x07,
}
impl Pad16fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad16fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad16fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad16fncsel {
        Pad16fncsel::from_bits(val)
    }
}
impl From<Pad16fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad16fncsel) -> u8 {
        Pad16fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad16strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad16strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad16strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad16strng {
    #[inline(always)]
    fn from(val: u8) -> Pad16strng {
        Pad16strng::from_bits(val)
    }
}
impl From<Pad16strng> for u8 {
    #[inline(always)]
    fn from(val: Pad16strng) -> u8 {
        Pad16strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad17fncsel {
    #[doc = "Configure as the analog comparator reference signal 1 input signal value."]
    Cmprf1 = 0x0,
    #[doc = "IOM/MSPI nCE group 17 value."]
    Nce17 = 0x01,
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    Trig1 = 0x02,
    #[doc = "Configure as GPIO17 value."]
    Gpio17 = 0x03,
    #[doc = "SCARD serial clock output value."]
    Sccclk = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Configure as UART0 RX input signal value."]
    Uart0rx = 0x06,
    #[doc = "Configure as UART1 CTS input signal value."]
    Ua1cts = 0x07,
}
impl Pad17fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad17fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad17fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad17fncsel {
        Pad17fncsel::from_bits(val)
    }
}
impl From<Pad17fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad17fncsel) -> u8 {
        Pad17fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad17strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad17strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad17strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad17strng {
    #[inline(always)]
    fn from(val: u8) -> Pad17strng {
        Pad17strng::from_bits(val)
    }
}
impl From<Pad17strng> for u8 {
    #[inline(always)]
    fn from(val: Pad17strng) -> u8 {
        Pad17strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad18fncsel {
    #[doc = "Configure as the analog comparator input 1 signal value."]
    Cmpin1 = 0x0,
    #[doc = "IOM/MSPI nCE group 18 value."]
    Nce18 = 0x01,
    #[doc = "CTIMER connection 4 value."]
    Ct4 = 0x02,
    #[doc = "Configure as GPIO18 value."]
    Gpio18 = 0x03,
    #[doc = "Configure as UART0 RTS output signal value."]
    Ua0rts = 0x04,
    #[doc = "Configure as ANATEST2 I/O signal value."]
    Anatest2 = 0x05,
    #[doc = "Configure as UART1 TX output signal value."]
    Uart1tx = 0x06,
    #[doc = "SCARD data input/output connectin value."]
    Sccio = 0x07,
}
impl Pad18fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad18fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad18fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad18fncsel {
        Pad18fncsel::from_bits(val)
    }
}
impl From<Pad18fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad18fncsel) -> u8 {
        Pad18fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad18strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad18strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad18strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad18strng {
    #[inline(always)]
    fn from(val: u8) -> Pad18strng {
        Pad18strng::from_bits(val)
    }
}
impl From<Pad18strng> for u8 {
    #[inline(always)]
    fn from(val: Pad18strng) -> u8 {
        Pad18strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad19fncsel {
    #[doc = "Configure as the analog comparator reference 0 signal value."]
    Cmprf0 = 0x0,
    #[doc = "IOM/MSPI nCE group 19 value."]
    Nce19 = 0x01,
    #[doc = "CTIMER conenction 6 value."]
    Ct6 = 0x02,
    #[doc = "Configure as GPIO19 value."]
    Gpio19 = 0x03,
    #[doc = "SCARD serial clock value."]
    Scclk = 0x04,
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    Anatest1 = 0x05,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x06,
    #[doc = "Configure as the PDM I2S bit clock input signal value."]
    I2sbclk = 0x07,
}
impl Pad19fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad19fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad19fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad19fncsel {
        Pad19fncsel::from_bits(val)
    }
}
impl From<Pad19fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad19fncsel) -> u8 {
        Pad19fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad19strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad19strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad19strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad19strng {
    #[inline(always)]
    fn from(val: u8) -> Pad19strng {
        Pad19strng::from_bits(val)
    }
}
impl From<Pad19strng> for u8 {
    #[inline(always)]
    fn from(val: Pad19strng) -> u8 {
        Pad19strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad1fncsel {
    #[doc = "Configure as the IOSLAVE I2C SDA or SPI WIR3 signal value."]
    Slsdawir3 = 0x0,
    #[doc = "Configure as the IOSLAVE SPI MOSI signal value."]
    Slmosi = 0x01,
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x02,
    #[doc = "Configure as GPIO1 value."]
    Gpio1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "MSPI data connection 5 value."]
    Mspi5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "IOM/MSPI nCE group 1 value."]
    Nce1 = 0x07,
}
impl Pad1fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad1fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad1fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad1fncsel {
        Pad1fncsel::from_bits(val)
    }
}
impl From<Pad1fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad1fncsel) -> u8 {
        Pad1fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad1rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad1rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad1rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad1rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad1rsel {
        Pad1rsel::from_bits(val)
    }
}
impl From<Pad1rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad1rsel) -> u8 {
        Pad1rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad1strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad1strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad1strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad1strng {
    #[inline(always)]
    fn from(val: u8) -> Pad1strng {
        Pad1strng::from_bits(val)
    }
}
impl From<Pad1strng> for u8 {
    #[inline(always)]
    fn from(val: Pad1strng) -> u8 {
        Pad1strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad20fncsel {
    #[doc = "Configure as the serial wire debug clock signal value."]
    Swdck = 0x0,
    #[doc = "IOM/MSPI nCE group 20 value."]
    Nce20 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Configure as GPIO20 value."]
    Gpio20 = 0x03,
    #[doc = "Configure as UART0 TX output signal value."]
    Uart0tx = 0x04,
    #[doc = "Configure as UART1 TX output signal value."]
    Uart1tx = 0x05,
    #[doc = "I2S byte clock input value."]
    I2sbclk = 0x06,
    #[doc = "Configure as UART1 RTS output signal value."]
    Ua1rts = 0x07,
}
impl Pad20fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad20fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad20fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad20fncsel {
        Pad20fncsel::from_bits(val)
    }
}
impl From<Pad20fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad20fncsel) -> u8 {
        Pad20fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad20strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad20strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad20strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad20strng {
    #[inline(always)]
    fn from(val: u8) -> Pad20strng {
        Pad20strng::from_bits(val)
    }
}
impl From<Pad20strng> for u8 {
    #[inline(always)]
    fn from(val: Pad20strng) -> u8 {
        Pad20strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad21fncsel {
    #[doc = "Configure as the serial wire debug data signal value."]
    Swdio = 0x0,
    #[doc = "IOM/MSPI nCE group 21 value."]
    Nce21 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Configure as GPIO21 value."]
    Gpio21 = 0x03,
    #[doc = "Configure as UART0 RX input signal value."]
    Uart0rx = 0x04,
    #[doc = "Configure as UART1 RX input signal value."]
    Uart1rx = 0x05,
    #[doc = "I2S byte clock input value."]
    I2sbclk = 0x06,
    #[doc = "Configure as UART1 CTS input signal value."]
    Ua1cts = 0x07,
}
impl Pad21fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad21fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad21fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad21fncsel {
        Pad21fncsel::from_bits(val)
    }
}
impl From<Pad21fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad21fncsel) -> u8 {
        Pad21fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad21strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad21strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad21strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad21strng {
    #[inline(always)]
    fn from(val: u8) -> Pad21strng {
        Pad21strng::from_bits(val)
    }
}
impl From<Pad21strng> for u8 {
    #[inline(always)]
    fn from(val: Pad21strng) -> u8 {
        Pad21strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad22fncsel {
    #[doc = "Configure as the UART0 TX signal value."]
    Uart0tx = 0x0,
    #[doc = "IOM/MSPI nCE group 22 value."]
    Nce22 = 0x01,
    #[doc = "CTIMER connection 12 value."]
    Ct12 = 0x02,
    #[doc = "Configure as GPIO22 value."]
    Gpio22 = 0x03,
    #[doc = "Configure as the PDM CLK output value."]
    PdmClk = 0x04,
    #[doc = "External LFRC input value."]
    Extlf = 0x05,
    #[doc = "MSPI data connection 0 value."]
    Mspi0 = 0x06,
    #[doc = "Configure as the serial trace data output signal value."]
    Swo = 0x07,
}
impl Pad22fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad22fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad22fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad22fncsel {
        Pad22fncsel::from_bits(val)
    }
}
impl From<Pad22fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad22fncsel) -> u8 {
        Pad22fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad22strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad22strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad22strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad22strng {
    #[inline(always)]
    fn from(val: u8) -> Pad22strng {
        Pad22strng::from_bits(val)
    }
}
impl From<Pad22strng> for u8 {
    #[inline(always)]
    fn from(val: Pad22strng) -> u8 {
        Pad22strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad23fncsel {
    #[doc = "Configure as the UART0 RX signal value."]
    Uart0rx = 0x0,
    #[doc = "IOM/MSPI nCE group 23 value."]
    Nce23 = 0x01,
    #[doc = "CTIMER connection 14 value."]
    Ct14 = 0x02,
    #[doc = "Configure as GPIO23 value."]
    Gpio23 = 0x03,
    #[doc = "I2S word clock input value."]
    I2swclk = 0x04,
    #[doc = "Configure as voltage comparitor output value."]
    Cmpout = 0x05,
    #[doc = "MSPI data connection 3 value."]
    Mspi3 = 0x06,
    #[doc = "External XTAL osacillatgor input value."]
    Extxt = 0x07,
}
impl Pad23fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad23fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad23fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad23fncsel {
        Pad23fncsel::from_bits(val)
    }
}
impl From<Pad23fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad23fncsel) -> u8 {
        Pad23fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad23strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad23strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad23strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad23strng {
    #[inline(always)]
    fn from(val: u8) -> Pad23strng {
        Pad23strng::from_bits(val)
    }
}
impl From<Pad23strng> for u8 {
    #[inline(always)]
    fn from(val: Pad23strng) -> u8 {
        Pad23strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad24fncsel {
    #[doc = "Configure as UART1 TX output signal value."]
    Uart1tx = 0x0,
    #[doc = "IOM/MSPI nCE group 24 value."]
    Nce24 = 0x01,
    #[doc = "MSPI data connection 8 value."]
    Mspi8 = 0x02,
    #[doc = "Configure as GPIO24 value."]
    Gpio24 = 0x03,
    #[doc = "Configure as UART0 CTS input signal value."]
    Ua0cts = 0x04,
    #[doc = "CTIMER connection 21 value."]
    Ct21 = 0x05,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32kHzXt = 0x06,
    #[doc = "Configure as the serial trace data output signal value."]
    Swo = 0x07,
}
impl Pad24fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad24fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad24fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad24fncsel {
        Pad24fncsel::from_bits(val)
    }
}
impl From<Pad24fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad24fncsel) -> u8 {
        Pad24fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad24strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad24strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad24strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad24strng {
    #[inline(always)]
    fn from(val: u8) -> Pad24strng {
        Pad24strng::from_bits(val)
    }
}
impl From<Pad24strng> for u8 {
    #[inline(always)]
    fn from(val: Pad24strng) -> u8 {
        Pad24strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad25fncsel {
    #[doc = "Configure as UART1 RX input signal value."]
    Uart1rx = 0x0,
    #[doc = "IOM/MSPI nCE group 25 value."]
    Nce25 = 0x01,
    #[doc = "CTIMER connection 1 value."]
    Ct1 = 0x02,
    #[doc = "Configure as GPIO25 value."]
    Gpio25 = 0x03,
    #[doc = "Configure as the IOMSTR2 I2C SDA or SPI WIR3 signal value."]
    M2sdawir3 = 0x04,
    #[doc = "Configure as the IOMSTR2 SPI MISO input signal value."]
    M2miso = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad25fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad25fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad25fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad25fncsel {
        Pad25fncsel::from_bits(val)
    }
}
impl From<Pad25fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad25fncsel) -> u8 {
        Pad25fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad25rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad25rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad25rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad25rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad25rsel {
        Pad25rsel::from_bits(val)
    }
}
impl From<Pad25rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad25rsel) -> u8 {
        Pad25rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad25strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad25strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad25strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad25strng {
    #[inline(always)]
    fn from(val: u8) -> Pad25strng {
        Pad25strng::from_bits(val)
    }
}
impl From<Pad25strng> for u8 {
    #[inline(always)]
    fn from(val: Pad25strng) -> u8 {
        Pad25strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad26fncsel {
    #[doc = "Configure as the external HFRC oscillator input value."]
    Exthf = 0x0,
    #[doc = "IOM/MSPI nCE group 26 value."]
    Nce26 = 0x01,
    #[doc = "CTIMER connection 3 value."]
    Ct3 = 0x02,
    #[doc = "Configure as GPIO26 value."]
    Gpio26 = 0x03,
    #[doc = "SCARD reset output value."]
    Sccrst = 0x04,
    #[doc = "MSPI data connection 1 value."]
    Mspi1 = 0x05,
    #[doc = "Configure as UART0 TX output signal value."]
    Uart0tx = 0x06,
    #[doc = "Configure as UART1 CTS input signal value."]
    Ua1cts = 0x07,
}
impl Pad26fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad26fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad26fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad26fncsel {
        Pad26fncsel::from_bits(val)
    }
}
impl From<Pad26fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad26fncsel) -> u8 {
        Pad26fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad26strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad26strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad26strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad26strng {
    #[inline(always)]
    fn from(val: u8) -> Pad26strng {
        Pad26strng::from_bits(val)
    }
}
impl From<Pad26strng> for u8 {
    #[inline(always)]
    fn from(val: Pad26strng) -> u8 {
        Pad26strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad27fncsel {
    #[doc = "Configure as UART0 RX input signal value."]
    Uart0rx = 0x0,
    #[doc = "IOM/MSPI nCE group 27 value."]
    Nce27 = 0x01,
    #[doc = "CTIMER connection 5 value."]
    Ct5 = 0x02,
    #[doc = "Configure as GPIO27 value."]
    Gpio27 = 0x03,
    #[doc = "Configure as I2C clock I/O signal from IOMSTR2 value."]
    M2scl = 0x04,
    #[doc = "Configure as SPI clock output signal from IOMSTR2 value."]
    M2sck = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad27fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad27fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad27fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad27fncsel {
        Pad27fncsel::from_bits(val)
    }
}
impl From<Pad27fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad27fncsel) -> u8 {
        Pad27fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad27rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad27rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad27rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad27rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad27rsel {
        Pad27rsel::from_bits(val)
    }
}
impl From<Pad27rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad27rsel) -> u8 {
        Pad27rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad27strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad27strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad27strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad27strng {
    #[inline(always)]
    fn from(val: u8) -> Pad27strng {
        Pad27strng::from_bits(val)
    }
}
impl From<Pad27strng> for u8 {
    #[inline(always)]
    fn from(val: Pad27strng) -> u8 {
        Pad27strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad28fncsel {
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    I2sWclk = 0x0,
    #[doc = "IOM/MSPI nCE group 28 value."]
    Nce28 = 0x01,
    #[doc = "CTIMER connection 7 value."]
    Ct7 = 0x02,
    #[doc = "Configure as GPIO28 value."]
    Gpio28 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the IOMSTR2 SPI MOSI output signal value."]
    M2mosi = 0x05,
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad28fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad28fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad28fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad28fncsel {
        Pad28fncsel::from_bits(val)
    }
}
impl From<Pad28fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad28fncsel) -> u8 {
        Pad28fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad28strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad28strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad28strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad28strng {
    #[inline(always)]
    fn from(val: u8) -> Pad28strng {
        Pad28strng::from_bits(val)
    }
}
impl From<Pad28strng> for u8 {
    #[inline(always)]
    fn from(val: Pad28strng) -> u8 {
        Pad28strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad29fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 1 value."]
    Adcse1 = 0x0,
    #[doc = "IOM/MSPI nCE group 29 value."]
    Nce29 = 0x01,
    #[doc = "CTIMER connection 9 value."]
    Ct9 = 0x02,
    #[doc = "Configure as GPIO29 value."]
    Gpio29 = 0x03,
    #[doc = "Configure as the UART0 CTS input signal value."]
    Ua0cts = 0x04,
    #[doc = "Configure as the UART1 CTS input signal value."]
    Ua1cts = 0x05,
    #[doc = "Configure as the UART0 RX input signal value."]
    Uart0rx = 0x06,
    #[doc = "Configure as PDM DATA input value."]
    PdmData = 0x07,
}
impl Pad29fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad29fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad29fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad29fncsel {
        Pad29fncsel::from_bits(val)
    }
}
impl From<Pad29fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad29fncsel) -> u8 {
        Pad29fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad29strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad29strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad29strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad29strng {
    #[inline(always)]
    fn from(val: u8) -> Pad29strng {
        Pad29strng::from_bits(val)
    }
}
impl From<Pad29strng> for u8 {
    #[inline(always)]
    fn from(val: Pad29strng) -> u8 {
        Pad29strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad2fncsel {
    _RESERVED_0 = 0x0,
    #[doc = "Configure as the IOSLAVE SPI MISO signal value."]
    Slmiso = 0x01,
    #[doc = "Configure as the UART0 RX input value."]
    Uart0rx = 0x02,
    #[doc = "Configure as GPIO2 value."]
    Gpio2 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CMSPI data connection 6 value."]
    Mspi6 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "IOM/MSPI nCE group 2 value."]
    Nce2 = 0x07,
}
impl Pad2fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad2fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad2fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad2fncsel {
        Pad2fncsel::from_bits(val)
    }
}
impl From<Pad2fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad2fncsel) -> u8 {
        Pad2fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad2strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad2strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad2strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad2strng {
    #[inline(always)]
    fn from(val: u8) -> Pad2strng {
        Pad2strng::from_bits(val)
    }
}
impl From<Pad2strng> for u8 {
    #[inline(always)]
    fn from(val: Pad2strng) -> u8 {
        Pad2strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad30fncsel {
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    Anatest1 = 0x0,
    #[doc = "IOM/MSPI nCE group 30 value."]
    Nce30 = 0x01,
    #[doc = "CTIMER connection 11 value."]
    Ct11 = 0x02,
    #[doc = "Configure as GPIO30 value."]
    Gpio30 = 0x03,
    #[doc = "Configure as UART0 TX output signal value."]
    Uart0tx = 0x04,
    #[doc = "Configure as UART1 RTS output signal value."]
    Ua1rts = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Configure as the PDM I2S Data output signal value."]
    I2sDat = 0x07,
}
impl Pad30fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad30fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad30fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad30fncsel {
        Pad30fncsel::from_bits(val)
    }
}
impl From<Pad30fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad30fncsel) -> u8 {
        Pad30fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad30strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad30strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad30strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad30strng {
    #[inline(always)]
    fn from(val: u8) -> Pad30strng {
        Pad30strng::from_bits(val)
    }
}
impl From<Pad30strng> for u8 {
    #[inline(always)]
    fn from(val: Pad30strng) -> u8 {
        Pad30strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad31fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 3 value."]
    Adcse3 = 0x0,
    #[doc = "IOM/MSPI nCE group 31 value."]
    Nce31 = 0x01,
    #[doc = "CTIMER connection 13 value."]
    Ct13 = 0x02,
    #[doc = "Configure as GPIO31 value."]
    Gpio31 = 0x03,
    #[doc = "Configure as the UART0 RX input signal value."]
    Uart0rx = 0x04,
    #[doc = "SCARD serial clock output value."]
    Sccclk = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Configure as UART1 RTS output signal value."]
    Ua1rts = 0x07,
}
impl Pad31fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad31fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad31fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad31fncsel {
        Pad31fncsel::from_bits(val)
    }
}
impl From<Pad31fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad31fncsel) -> u8 {
        Pad31fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad31strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad31strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad31strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad31strng {
    #[inline(always)]
    fn from(val: u8) -> Pad31strng {
        Pad31strng::from_bits(val)
    }
}
impl From<Pad31strng> for u8 {
    #[inline(always)]
    fn from(val: Pad31strng) -> u8 {
        Pad31strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad32fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 4 value."]
    Adcse4 = 0x0,
    #[doc = "IOM/MSPI nCE group 32 value."]
    Nce32 = 0x01,
    #[doc = "CTIMER connection 15 value."]
    Ct15 = 0x02,
    #[doc = "Configure as GPIO32 value."]
    Gpio32 = 0x03,
    #[doc = "SCARD serial data input/output value."]
    Sccio = 0x04,
    #[doc = "External input to the LFRC oscillator value."]
    Extlf = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Configure as the UART1 CTS input value."]
    Ua1cts = 0x07,
}
impl Pad32fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad32fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad32fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad32fncsel {
        Pad32fncsel::from_bits(val)
    }
}
impl From<Pad32fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad32fncsel) -> u8 {
        Pad32fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad32strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad32strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad32strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad32strng {
    #[inline(always)]
    fn from(val: u8) -> Pad32strng {
        Pad32strng::from_bits(val)
    }
}
impl From<Pad32strng> for u8 {
    #[inline(always)]
    fn from(val: Pad32strng) -> u8 {
        Pad32strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad33fncsel {
    #[doc = "Configure as the analog ADC single ended port 5 input signal value."]
    Adcse5 = 0x0,
    #[doc = "IOM/MSPI nCE group 33 value."]
    Nce33 = 0x01,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32kHzXt = 0x02,
    #[doc = "Configure as GPIO33 value."]
    Gpio33 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the UART0 CTS input value."]
    Ua0cts = 0x05,
    #[doc = "CTIMER connection 23 value."]
    Ct23 = 0x06,
    #[doc = "Configure as the serial trace data output signal value."]
    Swo = 0x07,
}
impl Pad33fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad33fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad33fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad33fncsel {
        Pad33fncsel::from_bits(val)
    }
}
impl From<Pad33fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad33fncsel) -> u8 {
        Pad33fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad33strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad33strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad33strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad33strng {
    #[inline(always)]
    fn from(val: u8) -> Pad33strng {
        Pad33strng::from_bits(val)
    }
}
impl From<Pad33strng> for u8 {
    #[inline(always)]
    fn from(val: Pad33strng) -> u8 {
        Pad33strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad34fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 6 value."]
    Adcse6 = 0x0,
    #[doc = "IOM/MSPI nCE group 34 value."]
    Nce34 = 0x01,
    #[doc = "Configure as the UART1 RTS output value."]
    Ua1rts = 0x02,
    #[doc = "Configure as GPIO34 value."]
    Gpio34 = 0x03,
    #[doc = "Configure as the analog comparator reference 2 signal value."]
    Cmprf2 = 0x04,
    #[doc = "Configure as the UART0 RTS output value."]
    Ua0rts = 0x05,
    #[doc = "Configure as the UART0 RX input value."]
    Uart0rx = 0x06,
    #[doc = "PDM serial data input value."]
    Pdmdata = 0x07,
}
impl Pad34fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad34fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad34fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad34fncsel {
        Pad34fncsel::from_bits(val)
    }
}
impl From<Pad34fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad34fncsel) -> u8 {
        Pad34fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad34strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad34strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad34strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad34strng {
    #[inline(always)]
    fn from(val: u8) -> Pad34strng {
        Pad34strng::from_bits(val)
    }
}
impl From<Pad34strng> for u8 {
    #[inline(always)]
    fn from(val: Pad34strng) -> u8 {
        Pad34strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad35fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 7 value."]
    Adcse7 = 0x0,
    #[doc = "IOM/MSPI nCE group 35 value."]
    Nce35 = 0x01,
    #[doc = "Configure as the UART1 TX signal value."]
    Uart1tx = 0x02,
    #[doc = "Configure as GPIO35 value."]
    Gpio35 = 0x03,
    #[doc = "I2S serial data output value."]
    I2sdat = 0x04,
    #[doc = "CTIMER connection 27 value."]
    Ct27 = 0x05,
    #[doc = "Configure as the UART0 RTS output value."]
    Ua0rts = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad35fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad35fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad35fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad35fncsel {
        Pad35fncsel::from_bits(val)
    }
}
impl From<Pad35fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad35fncsel) -> u8 {
        Pad35fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad35strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad35strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad35strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad35strng {
    #[inline(always)]
    fn from(val: u8) -> Pad35strng {
        Pad35strng::from_bits(val)
    }
}
impl From<Pad35strng> for u8 {
    #[inline(always)]
    fn from(val: Pad35strng) -> u8 {
        Pad35strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad36fncsel {
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    Trig1 = 0x0,
    #[doc = "IOM/MSPI nCE group 36 value."]
    Nce36 = 0x01,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x02,
    #[doc = "Configure as GPIO36 value."]
    Gpio36 = 0x03,
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32kHzXt = 0x04,
    #[doc = "Configure as the UART1 CTS input signal value."]
    Ua1cts = 0x05,
    #[doc = "Configure as the UART0 CTS input signal value."]
    Ua0cts = 0x06,
    #[doc = "PDM serial data input value."]
    Pdmdata = 0x07,
}
impl Pad36fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad36fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad36fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad36fncsel {
        Pad36fncsel::from_bits(val)
    }
}
impl From<Pad36fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad36fncsel) -> u8 {
        Pad36fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad36strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad36strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad36strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad36strng {
    #[inline(always)]
    fn from(val: u8) -> Pad36strng {
        Pad36strng::from_bits(val)
    }
}
impl From<Pad36strng> for u8 {
    #[inline(always)]
    fn from(val: Pad36strng) -> u8 {
        Pad36strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad37fncsel {
    #[doc = "Configure as the ADC Trigger 2 signal value."]
    Trig2 = 0x0,
    #[doc = "IOM/MSPI nCE group 37 value."]
    Nce37 = 0x01,
    #[doc = "Configure as the UART0 RTS output signal value."]
    Ua0rts = 0x02,
    #[doc = "Configure as GPIO37 value."]
    Gpio37 = 0x03,
    #[doc = "SCARD serial data input/output value."]
    Sccio = 0x04,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x05,
    #[doc = "Configure as the PDM CLK output signal value."]
    Pdmclk = 0x06,
    #[doc = "CTIMER connection 29 value."]
    Ct29 = 0x07,
}
impl Pad37fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad37fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad37fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad37fncsel {
        Pad37fncsel::from_bits(val)
    }
}
impl From<Pad37fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad37fncsel) -> u8 {
        Pad37fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad37strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad37strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad37strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad37strng {
    #[inline(always)]
    fn from(val: u8) -> Pad37strng {
        Pad37strng::from_bits(val)
    }
}
impl From<Pad37strng> for u8 {
    #[inline(always)]
    fn from(val: Pad37strng) -> u8 {
        Pad37strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad38fncsel {
    #[doc = "Configure as the ADC Trigger 3 signal value."]
    Trig3 = 0x0,
    #[doc = "IOM/MSPI nCE group 38 value."]
    Nce38 = 0x01,
    #[doc = "Configure as the UART0 CTS signal value."]
    Ua0cts = 0x02,
    #[doc = "Configure as GPIO38 value."]
    Gpio38 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the IOMSTR3 SPI MOSI output signal value."]
    M3mosi = 0x05,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad38fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad38fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad38fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad38fncsel {
        Pad38fncsel::from_bits(val)
    }
}
impl From<Pad38fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad38fncsel) -> u8 {
        Pad38fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad38strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad38strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad38strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad38strng {
    #[inline(always)]
    fn from(val: u8) -> Pad38strng {
        Pad38strng::from_bits(val)
    }
}
impl From<Pad38strng> for u8 {
    #[inline(always)]
    fn from(val: Pad38strng) -> u8 {
        Pad38strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad39fncsel {
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x0,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x01,
    #[doc = "CTIMER connection 25 value."]
    Ct25 = 0x02,
    #[doc = "Configure as GPIO39 value."]
    Gpio39 = 0x03,
    #[doc = "Configure as the IOMSTR4 I2C SCL signal value."]
    M4scl = 0x04,
    #[doc = "Configure as the IOMSTR4 SPI SCK signal value."]
    M4sck = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad39fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad39fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad39fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad39fncsel {
        Pad39fncsel::from_bits(val)
    }
}
impl From<Pad39fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad39fncsel) -> u8 {
        Pad39fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad39rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad39rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad39rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad39rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad39rsel {
        Pad39rsel::from_bits(val)
    }
}
impl From<Pad39rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad39rsel) -> u8 {
        Pad39rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad39strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad39strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad39strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad39strng {
    #[inline(always)]
    fn from(val: u8) -> Pad39strng {
        Pad39strng::from_bits(val)
    }
}
impl From<Pad39strng> for u8 {
    #[inline(always)]
    fn from(val: Pad39strng) -> u8 {
        Pad39strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad3fncsel {
    #[doc = "Configure as the UART0 RTS output value."]
    Ua0rts = 0x0,
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    SLnCe = 0x01,
    #[doc = "IOM/MSPI nCE group 3 value."]
    Nce3 = 0x02,
    #[doc = "Configure as GPIO3 value."]
    Gpio3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "MSPI data connection 7 value."]
    Mspi7 = 0x05,
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    Trig1 = 0x06,
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    I2sWclk = 0x07,
}
impl Pad3fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad3fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad3fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad3fncsel {
        Pad3fncsel::from_bits(val)
    }
}
impl From<Pad3fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad3fncsel) -> u8 {
        Pad3fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad3strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad3strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad3strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad3strng {
    #[inline(always)]
    fn from(val: u8) -> Pad3strng {
        Pad3strng::from_bits(val)
    }
}
impl From<Pad3strng> for u8 {
    #[inline(always)]
    fn from(val: Pad3strng) -> u8 {
        Pad3strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad40fncsel {
    #[doc = "Configure as the UART0 RX input signal value."]
    Uart0rx = 0x0,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x01,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    Trig0 = 0x02,
    #[doc = "Configure as GPIO40 value."]
    Gpio40 = 0x03,
    #[doc = "Configure as the IOMSTR4 I2C SDA or SPI WIR3 signal value."]
    M4sdawir3 = 0x04,
    #[doc = "Configure as the IOMSTR4 SPI MISO input signal value."]
    M4miso = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad40fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad40fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad40fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad40fncsel {
        Pad40fncsel::from_bits(val)
    }
}
impl From<Pad40fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad40fncsel) -> u8 {
        Pad40fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad40rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad40rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad40rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad40rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad40rsel {
        Pad40rsel::from_bits(val)
    }
}
impl From<Pad40rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad40rsel) -> u8 {
        Pad40rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad40strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad40strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad40strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad40strng {
    #[inline(always)]
    fn from(val: u8) -> Pad40strng {
        Pad40strng::from_bits(val)
    }
}
impl From<Pad40strng> for u8 {
    #[inline(always)]
    fn from(val: Pad40strng) -> u8 {
        Pad40strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad41fncsel {
    #[doc = "IOM/MSPI nCE group 41 value."]
    Nce41 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    Swo = 0x02,
    #[doc = "Configure as GPIO41 value."]
    Gpio41 = 0x03,
    #[doc = "I2S word clock input value."]
    I2swclk = 0x04,
    #[doc = "Configure as the UART1 RTS output signal value."]
    Ua1rts = 0x05,
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x06,
    #[doc = "Configure as the UART0 RTS output signal value."]
    Ua0rts = 0x07,
}
impl Pad41fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad41fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad41fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad41fncsel {
        Pad41fncsel::from_bits(val)
    }
}
impl From<Pad41fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad41fncsel) -> u8 {
        Pad41fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad41strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad41strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad41strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad41strng {
    #[inline(always)]
    fn from(val: u8) -> Pad41strng {
        Pad41strng::from_bits(val)
    }
}
impl From<Pad41strng> for u8 {
    #[inline(always)]
    fn from(val: Pad41strng) -> u8 {
        Pad41strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad42fncsel {
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x0,
    #[doc = "IOM/MSPI nCE group 42 value."]
    Nce42 = 0x01,
    #[doc = "CTIMER connection 16 value."]
    Ct16 = 0x02,
    #[doc = "Configure as GPIO42 value."]
    Gpio42 = 0x03,
    #[doc = "Configure as the IOMSTR3 I2C SCL clock I/O signal value."]
    M3scl = 0x04,
    #[doc = "Configure as the IOMSTR3 SPI SCK output value."]
    M3sck = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad42fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad42fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad42fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad42fncsel {
        Pad42fncsel::from_bits(val)
    }
}
impl From<Pad42fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad42fncsel) -> u8 {
        Pad42fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad42rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad42rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad42rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad42rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad42rsel {
        Pad42rsel::from_bits(val)
    }
}
impl From<Pad42rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad42rsel) -> u8 {
        Pad42rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad42strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad42strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad42strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad42strng {
    #[inline(always)]
    fn from(val: u8) -> Pad42strng {
        Pad42strng::from_bits(val)
    }
}
impl From<Pad42strng> for u8 {
    #[inline(always)]
    fn from(val: Pad42strng) -> u8 {
        Pad42strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad43fncsel {
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x0,
    #[doc = "IOM/MSPI nCE group 43 value."]
    Nce43 = 0x01,
    #[doc = "CTIMER connection 18 value."]
    Ct18 = 0x02,
    #[doc = "Configure as GPIO43 value."]
    Gpio43 = 0x03,
    #[doc = "Configure as the IOMSTR3 I2C SDA or SPI WIR3 signal value."]
    M3sdawir3 = 0x04,
    #[doc = "Configure as the IOMSTR3 SPI MISO signal value."]
    M3miso = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad43fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad43fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad43fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad43fncsel {
        Pad43fncsel::from_bits(val)
    }
}
impl From<Pad43fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad43fncsel) -> u8 {
        Pad43fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad43rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad43rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad43rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad43rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad43rsel {
        Pad43rsel::from_bits(val)
    }
}
impl From<Pad43rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad43rsel) -> u8 {
        Pad43rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad43strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad43strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad43strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad43strng {
    #[inline(always)]
    fn from(val: u8) -> Pad43strng {
        Pad43strng::from_bits(val)
    }
}
impl From<Pad43strng> for u8 {
    #[inline(always)]
    fn from(val: Pad43strng) -> u8 {
        Pad43strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad44fncsel {
    #[doc = "Configure as the UART1 RTS output signal value."]
    Ua1rts = 0x0,
    #[doc = "IOM/MSPI nCE group 44 value."]
    Nce44 = 0x01,
    #[doc = "CTIMER connection 20 value."]
    Ct20 = 0x02,
    #[doc = "Configure as GPIO44 value."]
    Gpio44 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the IOMSTR4 SPI MOSI signal value."]
    M4mosi = 0x05,
    #[doc = "Configure as the SPI channel 6 nCE signal from IOMSTR5 value."]
    M5nCe6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad44fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad44fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad44fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad44fncsel {
        Pad44fncsel::from_bits(val)
    }
}
impl From<Pad44fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad44fncsel) -> u8 {
        Pad44fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad44strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad44strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad44strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad44strng {
    #[inline(always)]
    fn from(val: u8) -> Pad44strng {
        Pad44strng::from_bits(val)
    }
}
impl From<Pad44strng> for u8 {
    #[inline(always)]
    fn from(val: Pad44strng) -> u8 {
        Pad44strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad45fncsel {
    #[doc = "Configure as the UART1 CTS input signal value."]
    Ua1cts = 0x0,
    #[doc = "IOM/MSPI nCE group 45 value."]
    Nce45 = 0x01,
    #[doc = "CTIMER connection 22 value."]
    Ct22 = 0x02,
    #[doc = "Configure as GPIO45 value."]
    Gpio45 = 0x03,
    #[doc = "I2S serial data output value."]
    I2sdat = 0x04,
    #[doc = "PDM serial data input value."]
    Pdmdata = 0x05,
    #[doc = "Configure as the SPI channel 5 nCE signal from IOMSTR5 value."]
    Uart0rx = 0x06,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    Swo = 0x07,
}
impl Pad45fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad45fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad45fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad45fncsel {
        Pad45fncsel::from_bits(val)
    }
}
impl From<Pad45fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad45fncsel) -> u8 {
        Pad45fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad45strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad45strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad45strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad45strng {
    #[inline(always)]
    fn from(val: u8) -> Pad45strng {
        Pad45strng::from_bits(val)
    }
}
impl From<Pad45strng> for u8 {
    #[inline(always)]
    fn from(val: Pad45strng) -> u8 {
        Pad45strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad46fncsel {
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32khzXt = 0x0,
    #[doc = "IOM/MSPI nCE group 46 value."]
    Nce46 = 0x01,
    #[doc = "CTIMER connection 24 value."]
    Ct24 = 0x02,
    #[doc = "Configure as GPIO46 value."]
    Gpio46 = 0x03,
    #[doc = "SCARD reset output value."]
    Sccrst = 0x04,
    #[doc = "PDM serial clock output value."]
    Pdmclk = 0x05,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x06,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    Swo = 0x07,
}
impl Pad46fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad46fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad46fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad46fncsel {
        Pad46fncsel::from_bits(val)
    }
}
impl From<Pad46fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad46fncsel) -> u8 {
        Pad46fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad46strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad46strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad46strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad46strng {
    #[inline(always)]
    fn from(val: u8) -> Pad46strng {
        Pad46strng::from_bits(val)
    }
}
impl From<Pad46strng> for u8 {
    #[inline(always)]
    fn from(val: Pad46strng) -> u8 {
        Pad46strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad47fncsel {
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32kHzXt = 0x0,
    #[doc = "IOM/MSPI nCE group 47 value."]
    Nce47 = 0x01,
    #[doc = "CTIMER connection 26 value."]
    Ct26 = 0x02,
    #[doc = "Configure as GPIO47 value."]
    Gpio47 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the IOMSTR5 SPI MOSI output signal value."]
    M5mosi = 0x05,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad47fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad47fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad47fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad47fncsel {
        Pad47fncsel::from_bits(val)
    }
}
impl From<Pad47fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad47fncsel) -> u8 {
        Pad47fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad47strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad47strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad47strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad47strng {
    #[inline(always)]
    fn from(val: u8) -> Pad47strng {
        Pad47strng::from_bits(val)
    }
}
impl From<Pad47strng> for u8 {
    #[inline(always)]
    fn from(val: Pad47strng) -> u8 {
        Pad47strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad48fncsel {
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x0,
    #[doc = "IOM/MSPI nCE group 48 value."]
    Nce48 = 0x01,
    #[doc = "CTIMER conenction 28 value."]
    Ct28 = 0x02,
    #[doc = "Configure as GPIO48 value."]
    Gpio48 = 0x03,
    #[doc = "Configure as the IOMSTR5 I2C SCL clock I/O signal value."]
    M5scl = 0x04,
    #[doc = "Configure as the IOMSTR5 SPI SCK output value."]
    M5sck = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad48fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad48fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad48fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad48fncsel {
        Pad48fncsel::from_bits(val)
    }
}
impl From<Pad48fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad48fncsel) -> u8 {
        Pad48fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad48rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad48rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad48rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad48rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad48rsel {
        Pad48rsel::from_bits(val)
    }
}
impl From<Pad48rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad48rsel) -> u8 {
        Pad48rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad48strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad48strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad48strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad48strng {
    #[inline(always)]
    fn from(val: u8) -> Pad48strng {
        Pad48strng::from_bits(val)
    }
}
impl From<Pad48strng> for u8 {
    #[inline(always)]
    fn from(val: Pad48strng) -> u8 {
        Pad48strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad49fncsel {
    #[doc = "Configure as the UART0 RX input signal value."]
    Uart0rx = 0x0,
    #[doc = "IOM/MSPPI nCE group 49 value."]
    Nce49 = 0x01,
    #[doc = "CTIMER connection 30 value."]
    Ct30 = 0x02,
    #[doc = "Configure as GPIO49 value."]
    Gpio49 = 0x03,
    #[doc = "Configure as the IOMSTR5 I2C SDA or SPI WIR3 signal value."]
    M5sdawir3 = 0x04,
    #[doc = "Configure as the IOMSTR5 SPI MISO input signal value."]
    M5miso = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad49fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad49fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad49fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad49fncsel {
        Pad49fncsel::from_bits(val)
    }
}
impl From<Pad49fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad49fncsel) -> u8 {
        Pad49fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad49rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad49rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad49rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad49rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad49rsel {
        Pad49rsel::from_bits(val)
    }
}
impl From<Pad49rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad49rsel) -> u8 {
        Pad49rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad49strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad49strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad49strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad49strng {
    #[inline(always)]
    fn from(val: u8) -> Pad49strng {
        Pad49strng::from_bits(val)
    }
}
impl From<Pad49strng> for u8 {
    #[inline(always)]
    fn from(val: Pad49strng) -> u8 {
        Pad49strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad4fncsel {
    #[doc = "Configure as the UART0 CTS input signal value."]
    Ua0cts = 0x0,
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    Slint = 0x01,
    #[doc = "IOM/SPI nCE group 4 value."]
    Nce4 = 0x02,
    #[doc = "Configure as GPIO4 value."]
    Gpio4 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the UART0 RX input value."]
    Uart0rx = 0x05,
    #[doc = "CTIMER connection 17 value."]
    Ct17 = 0x06,
    #[doc = "MSPI data connection 2 value."]
    Mspi2 = 0x07,
}
impl Pad4fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad4fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad4fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad4fncsel {
        Pad4fncsel::from_bits(val)
    }
}
impl From<Pad4fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad4fncsel) -> u8 {
        Pad4fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad4strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad4strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad4strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad4strng {
    #[inline(always)]
    fn from(val: u8) -> Pad4strng {
        Pad4strng::from_bits(val)
    }
}
impl From<Pad4strng> for u8 {
    #[inline(always)]
    fn from(val: Pad4strng) -> u8 {
        Pad4strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad5fncsel {
    #[doc = "Configure as the IOMSTR0 I2C SCL signal value."]
    M0scl = 0x0,
    #[doc = "Configure as the IOMSTR0 SPI SCK signal value."]
    M0sck = 0x01,
    #[doc = "Configure as the UART0 RTS signal output value."]
    Ua0rts = 0x02,
    #[doc = "Configure as GPIO5 value."]
    Gpio5 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the External HFA input clock value."]
    Exthfa = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "CTIMER connection 8 value."]
    Ct8 = 0x07,
}
impl Pad5fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad5fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad5fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad5fncsel {
        Pad5fncsel::from_bits(val)
    }
}
impl From<Pad5fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad5fncsel) -> u8 {
        Pad5fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad5rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad5rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad5rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad5rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad5rsel {
        Pad5rsel::from_bits(val)
    }
}
impl From<Pad5rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad5rsel) -> u8 {
        Pad5rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad5strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad5strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad5strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad5strng {
    #[inline(always)]
    fn from(val: u8) -> Pad5strng {
        Pad5strng::from_bits(val)
    }
}
impl From<Pad5strng> for u8 {
    #[inline(always)]
    fn from(val: Pad5strng) -> u8 {
        Pad5strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad6fncsel {
    #[doc = "Configure as the IOMSTR0 I2C SDA or SPI WIR3 signal value."]
    M0sdawir3 = 0x0,
    #[doc = "Configure as the IOMSTR0 SPI MISO signal value."]
    M0miso = 0x01,
    #[doc = "Configure as the UART0 CTS input signal value."]
    Ua0cts = 0x02,
    #[doc = "Configure as GPIO6 value."]
    Gpio6 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CTIMER connection 10 value."]
    Ct10 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Configure as the PDM I2S Data output signal value."]
    I2sDat = 0x07,
}
impl Pad6fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad6fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad6fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad6fncsel {
        Pad6fncsel::from_bits(val)
    }
}
impl From<Pad6fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad6fncsel) -> u8 {
        Pad6fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad6rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad6rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad6rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad6rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad6rsel {
        Pad6rsel::from_bits(val)
    }
}
impl From<Pad6rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad6rsel) -> u8 {
        Pad6rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad6strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad6strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad6strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad6strng {
    #[inline(always)]
    fn from(val: u8) -> Pad6strng {
        Pad6strng::from_bits(val)
    }
}
impl From<Pad6strng> for u8 {
    #[inline(always)]
    fn from(val: Pad6strng) -> u8 {
        Pad6strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad7fncsel {
    #[doc = "IOM/MSPI nCE group 7 value."]
    Nce7 = 0x0,
    #[doc = "Configure as the IOMSTR0 SPI MOSI signal value."]
    M0mosi = 0x01,
    #[doc = "Configure as the CLKOUT signal value."]
    Clkout = 0x02,
    #[doc = "Configure as GPIO7 value."]
    Gpio7 = 0x03,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    Trig0 = 0x04,
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "CTIMER connection 19 value."]
    Ct19 = 0x07,
}
impl Pad7fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad7fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad7fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad7fncsel {
        Pad7fncsel::from_bits(val)
    }
}
impl From<Pad7fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad7fncsel) -> u8 {
        Pad7fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad7strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad7strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad7strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad7strng {
    #[inline(always)]
    fn from(val: u8) -> Pad7strng {
        Pad7strng::from_bits(val)
    }
}
impl From<Pad7strng> for u8 {
    #[inline(always)]
    fn from(val: Pad7strng) -> u8 {
        Pad7strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad8fncsel {
    #[doc = "Configure as the IOMSTR1 I2C SCL signal value."]
    M1scl = 0x0,
    #[doc = "Configure as the IOMSTR1 SPI SCK signal value."]
    M1sck = 0x01,
    #[doc = "IOM/MSPI nCE group 8 value."]
    Nce8 = 0x02,
    #[doc = "Configure as GPIO8 value."]
    Gpio8 = 0x03,
    #[doc = "SCARD serial clock output value."]
    Scclk = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad8fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad8fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad8fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad8fncsel {
        Pad8fncsel::from_bits(val)
    }
}
impl From<Pad8fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad8fncsel) -> u8 {
        Pad8fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad8rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad8rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad8rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad8rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad8rsel {
        Pad8rsel::from_bits(val)
    }
}
impl From<Pad8rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad8rsel) -> u8 {
        Pad8rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad8strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad8strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad8strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad8strng {
    #[inline(always)]
    fn from(val: u8) -> Pad8strng {
        Pad8strng::from_bits(val)
    }
}
impl From<Pad8strng> for u8 {
    #[inline(always)]
    fn from(val: Pad8strng) -> u8 {
        Pad8strng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad9fncsel {
    #[doc = "Configure as the IOMSTR1 I2C SDA or SPI WIR3 signal value."]
    M1sdawir3 = 0x0,
    #[doc = "Configure as the IOMSTR1 SPI MISO signal value."]
    M1miso = 0x01,
    #[doc = "IOM/MSPI nCE group 9 value."]
    Nce9 = 0x02,
    #[doc = "Configure as GPIO9 value."]
    Gpio9 = 0x03,
    #[doc = "SCARD data I/O connection value."]
    Sccio = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Configure as UART1 RX input signal value."]
    Uart1rx = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad9fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad9fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad9fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad9fncsel {
        Pad9fncsel::from_bits(val)
    }
}
impl From<Pad9fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad9fncsel) -> u8 {
        Pad9fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad9rsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Pad9rsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad9rsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad9rsel {
    #[inline(always)]
    fn from(val: u8) -> Pad9rsel {
        Pad9rsel::from_bits(val)
    }
}
impl From<Pad9rsel> for u8 {
    #[inline(always)]
    fn from(val: Pad9rsel) -> u8 {
        Pad9rsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad9strng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Pad9strng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad9strng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad9strng {
    #[inline(always)]
    fn from(val: u8) -> Pad9strng {
        Pad9strng::from_bits(val)
    }
}
impl From<Pad9strng> for u8 {
    #[inline(always)]
    fn from(val: Pad9strng) -> u8 {
        Pad9strng::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Padkey(u32);
impl Padkey {
    #[doc = "Key value."]
    pub const Key: Self = Self(0x73);
}
impl Padkey {
    pub const fn from_bits(val: u32) -> Padkey {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Padkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x73 => f.write_str("Key"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x73 => defmt::write!(f, "Key"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Padkey {
    #[inline(always)]
    fn from(val: u32) -> Padkey {
        Padkey::from_bits(val)
    }
}
impl From<Padkey> for u32 {
    #[inline(always)]
    fn from(val: Padkey) -> u32 {
        Padkey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stpol0 {
    #[doc = "Capture on low to high GPIO transition value."]
    Caplh = 0x0,
    #[doc = "Capture on high to low GPIO transition value."]
    Caphl = 0x01,
}
impl Stpol0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stpol0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stpol0 {
    #[inline(always)]
    fn from(val: u8) -> Stpol0 {
        Stpol0::from_bits(val)
    }
}
impl From<Stpol0> for u8 {
    #[inline(always)]
    fn from(val: Stpol0) -> u8 {
        Stpol0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stpol1 {
    #[doc = "Capture on low to high GPIO transition value."]
    Caplh = 0x0,
    #[doc = "Capture on high to low GPIO transition value."]
    Caphl = 0x01,
}
impl Stpol1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stpol1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stpol1 {
    #[inline(always)]
    fn from(val: u8) -> Stpol1 {
        Stpol1::from_bits(val)
    }
}
impl From<Stpol1> for u8 {
    #[inline(always)]
    fn from(val: Stpol1) -> u8 {
        Stpol1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stpol2 {
    #[doc = "Capture on low to high GPIO transition value."]
    Caplh = 0x0,
    #[doc = "Capture on high to low GPIO transition value."]
    Caphl = 0x01,
}
impl Stpol2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stpol2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stpol2 {
    #[inline(always)]
    fn from(val: u8) -> Stpol2 {
        Stpol2::from_bits(val)
    }
}
impl From<Stpol2> for u8 {
    #[inline(always)]
    fn from(val: Stpol2) -> u8 {
        Stpol2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stpol3 {
    #[doc = "Capture on low to high GPIO transition value."]
    Caplh = 0x0,
    #[doc = "Capture on high to low GPIO transition value."]
    Caphl = 0x01,
}
impl Stpol3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stpol3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stpol3 {
    #[inline(always)]
    fn from(val: u8) -> Stpol3 {
        Stpol3::from_bits(val)
    }
}
impl From<Stpol3> for u8 {
    #[inline(always)]
    fn from(val: Stpol3) -> u8 {
        Stpol3::to_bits(val)
    }
}

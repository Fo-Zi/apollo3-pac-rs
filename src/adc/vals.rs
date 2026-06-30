#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adsel {
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot. value."]
    Avg1Msrmt = 0x0,
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot. value."]
    Avg2Msrmts = 0x01,
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot. value."]
    Avg4Msrmts = 0x02,
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot. value."]
    Avg8Msrmt = 0x03,
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot. value."]
    Avg16Msrmts = 0x04,
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot. value."]
    Avg32Msrmts = 0x05,
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot. value."]
    Avg64Msrmts = 0x06,
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot. value."]
    Avg128Msrmts = 0x07,
}
impl Adsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adsel {
    #[inline(always)]
    fn from(val: u8) -> Adsel {
        Adsel::from_bits(val)
    }
}
impl From<Adsel> for u8 {
    #[inline(always)]
    fn from(val: Adsel) -> u8 {
        Adsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chsel {
    #[doc = "single ended external GPIO connection to pad16. value."]
    Se0 = 0x0,
    #[doc = "single ended external GPIO connection to pad29. value."]
    Se1 = 0x01,
    #[doc = "single ended external GPIO connection to pad11. value."]
    Se2 = 0x02,
    #[doc = "single ended external GPIO connection to pad31. value."]
    Se3 = 0x03,
    #[doc = "single ended external GPIO connection to pad32. value."]
    Se4 = 0x04,
    #[doc = "single ended external GPIO connection to pad33. value."]
    Se5 = 0x05,
    #[doc = "single ended external GPIO connection to pad34. value."]
    Se6 = 0x06,
    #[doc = "single ended external GPIO connection to pad35. value."]
    Se7 = 0x07,
    #[doc = "single ended external GPIO connection to pad13. value."]
    Se8 = 0x08,
    #[doc = "single ended external GPIO connection to pad12. value."]
    Se9 = 0x09,
    #[doc = "differential external GPIO connections to pad12(N) and pad13(P). value."]
    Df0 = 0x0a,
    #[doc = "differential external GPIO connections to pad15(N) and pad14(P). value."]
    Df1 = 0x0b,
    #[doc = "internal temperature sensor. value."]
    Temp = 0x0c,
    #[doc = "internal voltage divide-by-3 connection. value."]
    Batt = 0x0d,
    #[doc = "Input VSS value."]
    Vss = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Chsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chsel {
    #[inline(always)]
    fn from(val: u8) -> Chsel {
        Chsel::from_bits(val)
    }
}
impl From<Chsel> for u8 {
    #[inline(always)]
    fn from(val: Chsel) -> u8 {
        Chsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ckmode {
    #[doc = "Disable the clock between scans for LPMODE0. Set LPCKMODE to 0x1 while configuring the ADC. value."]
    Lpckmode = 0x0,
    #[doc = "Low Latency Clock Mode. When set, HFRC and the adc_clk will remain on while in functioning in LPMODE0. value."]
    Llckmode = 0x01,
}
impl Ckmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ckmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ckmode {
    #[inline(always)]
    fn from(val: u8) -> Ckmode {
        Ckmode::from_bits(val)
    }
}
impl From<Ckmode> for u8 {
    #[inline(always)]
    fn from(val: Ckmode) -> u8 {
        Ckmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksel {
    #[doc = "Off mode. The HFRC or HFRC_DIV2 clock must be selected for the ADC to function. The ADC controller automatically shuts off the clock in it's low power modes. When setting ADCEN to '0', the CLKSEL should remain set to one of the two clock selects for proper power down sequencing. value."]
    Off = 0x0,
    #[doc = "HFRC Core Clock divided by (CORESEL+1) value."]
    Hfrc = 0x01,
    #[doc = "HFRC Core Clock / 2 further divided by (CORESEL+1) value."]
    HfrcDiv2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Clksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksel {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum Dmapri {
    #[doc = "Low Priority (service as best effort) value."]
    Low = 0x0,
    #[doc = "High Priority (service immediately) value."]
    High = 0x01,
}
impl Dmapri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmapri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmapri {
    #[inline(always)]
    fn from(val: u8) -> Dmapri {
        Dmapri::from_bits(val)
    }
}
impl From<Dmapri> for u8 {
    #[inline(always)]
    fn from(val: Dmapri) -> u8 {
        Dmapri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpmode {
    #[doc = "Low Power Mode 0. Leaves the ADC fully powered between scans with minimum latency between a trigger event and sample data collection. value."]
    Mode0 = 0x0,
    #[doc = "Low Power Mode 1. Powers down all circuity and clocks associated with the ADC until the next trigger event. Between scans, the reference buffer requires up to 50us of delay from a scan trigger event before the conversion will commence while operating in this mode. value."]
    Mode1 = 0x01,
}
impl Lpmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpmode {
    #[inline(always)]
    fn from(val: u8) -> Lpmode {
        Lpmode::from_bits(val)
    }
}
impl From<Lpmode> for u8 {
    #[inline(always)]
    fn from(val: Lpmode) -> u8 {
        Lpmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prmode {
    #[doc = "14-bit precision mode value."]
    P14b = 0x0,
    #[doc = "12-bit precision mode value."]
    P12b = 0x01,
    #[doc = "10-bit precision mode value."]
    P10b = 0x02,
    #[doc = "8-bit precision mode value."]
    P8b = 0x03,
}
impl Prmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prmode {
    #[inline(always)]
    fn from(val: u8) -> Prmode {
        Prmode::from_bits(val)
    }
}
impl From<Prmode> for u8 {
    #[inline(always)]
    fn from(val: Prmode) -> u8 {
        Prmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwdstat {
    #[doc = "Powered on. value."]
    On = 0x0,
    #[doc = "ADC Low Power Mode 1. value."]
    PoweredDown = 0x01,
}
impl Pwdstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwdstat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwdstat {
    #[inline(always)]
    fn from(val: u8) -> Pwdstat {
        Pwdstat::from_bits(val)
    }
}
impl From<Pwdstat> for u8 {
    #[inline(always)]
    fn from(val: Pwdstat) -> u8 {
        Pwdstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refsel {
    #[doc = "Internal 2.0V Bandgap Reference Voltage value."]
    Int2p0 = 0x0,
    #[doc = "Internal 1.5V Bandgap Reference Voltage value."]
    Int1p5 = 0x01,
    #[doc = "Off Chip 2.0V Reference value."]
    Ext2p0 = 0x02,
    #[doc = "Off Chip 1.5V Reference value."]
    Ext1p5 = 0x03,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rpten {
    #[doc = "In Single Scan Mode, the ADC will complete a single scan upon each trigger event. value."]
    SingleScan = 0x0,
    #[doc = "In Repeating Scan Mode, the ADC will complete it's first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 internal timer until the timer is disabled or the ADC is disabled. When disabling the ADC (setting ADCEN to '0'), the RPTEN bit should be cleared. value."]
    RepeatingScan = 0x01,
}
impl Rpten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpten {
    #[inline(always)]
    fn from(val: u8) -> Rpten {
        Rpten::from_bits(val)
    }
}
impl From<Rpten> for u8 {
    #[inline(always)]
    fn from(val: Rpten) -> u8 {
        Rpten::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Swt(u8);
impl Swt {
    #[doc = "Writing this value generates a software trigger. value."]
    pub const GenSwTrigger: Self = Self(0x37);
}
impl Swt {
    pub const fn from_bits(val: u8) -> Swt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Swt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x37 => f.write_str("GenSwTrigger"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x37 => defmt::write!(f, "GenSwTrigger"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Swt {
    #[inline(always)]
    fn from(val: u8) -> Swt {
        Swt::from_bits(val)
    }
}
impl From<Swt> for u8 {
    #[inline(always)]
    fn from(val: Swt) -> u8 {
        Swt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trigpol {
    #[doc = "Trigger on rising edge. value."]
    RisingEdge = 0x0,
    #[doc = "Trigger on falling edge. value."]
    FallingEdge = 0x01,
}
impl Trigpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigpol {
    #[inline(always)]
    fn from(val: u8) -> Trigpol {
        Trigpol::from_bits(val)
    }
}
impl From<Trigpol> for u8 {
    #[inline(always)]
    fn from(val: Trigpol) -> u8 {
        Trigpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trigsel {
    #[doc = "Off chip External Trigger0 (ADC_ET0) value."]
    Ext0 = 0x0,
    #[doc = "Off chip External Trigger1 (ADC_ET1) value."]
    Ext1 = 0x01,
    #[doc = "Off chip External Trigger2 (ADC_ET2) value."]
    Ext2 = 0x02,
    #[doc = "Off chip External Trigger3 (ADC_ET3) value."]
    Ext3 = 0x03,
    #[doc = "Voltage Comparator Output value."]
    Vcomp = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Software Trigger value."]
    Swt = 0x07,
}
impl Trigsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigsel {
    #[inline(always)]
    fn from(val: u8) -> Trigsel {
        Trigsel::from_bits(val)
    }
}
impl From<Trigsel> for u8 {
    #[inline(always)]
    fn from(val: Trigsel) -> u8 {
        Trigsel::to_bits(val)
    }
}

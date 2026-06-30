#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acal {
    #[doc = "Disable Autocalibration value."]
    Dis = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Autocalibrate every 1024 seconds. Once autocalibration is done, an interrupt will be triggered at the end of 1024 seconds. value."]
    _1024sec = 0x02,
    #[doc = "Autocalibrate every 512 seconds. Once autocalibration is done, an interrupt will be trigged at the end of 512 seconds. value."]
    _512sec = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Frequency measurement using XT. The XT clock is normally considered much more accurate than the LFRC clock source. value."]
    Xtfreq = 0x06,
    #[doc = "Frequency measurement using external clock. value."]
    Extfreq = 0x07,
}
impl Acal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acal {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acal {
    #[inline(always)]
    fn from(val: u8) -> Acal {
        Acal::from_bits(val)
    }
}
impl From<Acal> for u8 {
    #[inline(always)]
    fn from(val: Acal) -> u8 {
        Acal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cksel {
    #[doc = "LFRC value."]
    Lfrc = 0x0,
    #[doc = "XT / 2 value."]
    XtDiv2 = 0x01,
    #[doc = "XT / 4 value."]
    XtDiv4 = 0x02,
    #[doc = "XT / 8 value."]
    XtDiv8 = 0x03,
    #[doc = "XT / 16 value."]
    XtDiv16 = 0x04,
    #[doc = "XT / 32 value."]
    XtDiv32 = 0x05,
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
    #[doc = "1 Hz as selected in RTC value."]
    Rtc1hz = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    #[doc = "XT / 2^21 value."]
    XtDiv2m = 0x16,
    #[doc = "XT value."]
    Xt = 0x17,
    #[doc = "100 Hz as selected in CLKGEN value."]
    Cg100hz = 0x18,
    #[doc = "HFRC value."]
    Hfrc = 0x19,
    #[doc = "HFRC / 4 value."]
    HfrcDiv4 = 0x1a,
    #[doc = "HFRC / 8 value."]
    HfrcDiv8 = 0x1b,
    #[doc = "HFRC / 16 value."]
    HfrcDiv16 = 0x1c,
    #[doc = "HFRC / 64 value."]
    HfrcDiv64 = 0x1d,
    #[doc = "HFRC / 128 value."]
    HfrcDiv128 = 0x1e,
    #[doc = "HFRC / 256 value."]
    HfrcDiv256 = 0x1f,
    #[doc = "HFRC / 512 value."]
    HfrcDiv512 = 0x20,
    _RESERVED_21 = 0x21,
    #[doc = "Flash Clock value."]
    FlashClk = 0x22,
    #[doc = "LFRC / 2 value."]
    LfrcDiv2 = 0x23,
    #[doc = "LFRC / 32 value."]
    LfrcDiv32 = 0x24,
    #[doc = "LFRC / 512 value."]
    LfrcDiv512 = 0x25,
    #[doc = "LFRC / 32768 value."]
    LfrcDiv32k = 0x26,
    #[doc = "XT / 256 value."]
    XtDiv256 = 0x27,
    #[doc = "XT / 8192 value."]
    XtDiv8k = 0x28,
    #[doc = "XT / 2^16 value."]
    XtDiv64k = 0x29,
    #[doc = "Uncal LFRC / 16 value."]
    UlfrcDiv16 = 0x2a,
    #[doc = "Uncal LFRC / 128 value."]
    UlfrcDiv128 = 0x2b,
    #[doc = "Uncal LFRC / 1024 value."]
    Ulfrc1hz = 0x2c,
    #[doc = "Uncal LFRC / 4096 value."]
    UlfrcDiv4k = 0x2d,
    #[doc = "Uncal LFRC / 2^20 value."]
    UlfrcDiv1m = 0x2e,
    #[doc = "HFRC / 2^16 value."]
    HfrcDiv64k = 0x2f,
    #[doc = "HFRC / 2^24 value."]
    HfrcDiv16m = 0x30,
    #[doc = "LFRC / 2^20 value."]
    LfrcDiv1m = 0x31,
    #[doc = "HFRC (not autoenabled) value."]
    Hfrcne = 0x32,
    #[doc = "HFRC / 8 (not autoenabled) value."]
    HfrcneDiv8 = 0x33,
    _RESERVED_34 = 0x34,
    #[doc = "XT (not autoenabled) value."]
    Xtne = 0x35,
    #[doc = "XT / 16 (not autoenabled) value."]
    XtneDiv16 = 0x36,
    #[doc = "LFRC / 32 (not autoenabled) value."]
    LfrcneDiv32 = 0x37,
    _RESERVED_38 = 0x38,
    #[doc = "LFRC (not autoenabled) - Default for undefined values value."]
    Lfrcne = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Cksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cksel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cksel {
    #[inline(always)]
    fn from(val: u8) -> Cksel {
        Cksel::from_bits(val)
    }
}
impl From<Cksel> for u8 {
    #[inline(always)]
    fn from(val: Cksel) -> u8 {
        Cksel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clkkey(u32);
impl Clkkey {
    #[doc = "Key value."]
    pub const Key: Self = Self(0x47);
}
impl Clkkey {
    pub const fn from_bits(val: u32) -> Clkkey {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Clkkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x47 => f.write_str("Key"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x47 => defmt::write!(f, "Key"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Clkkey {
    #[inline(always)]
    fn from(val: u32) -> Clkkey {
        Clkkey::from_bits(val)
    }
}
impl From<Clkkey> for u32 {
    #[inline(always)]
    fn from(val: Clkkey) -> u32 {
        Clkkey::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clocken2stat(u32);
impl Clocken2stat {
    #[doc = "Clock enable for the IO MASTER 1 IFC INTERFACE value."]
    pub const Iomstrifc1Clken: Self = Self(0x01);
    #[doc = "Clock enable for the IO MASTER 2 IFC INTERFACE value."]
    pub const Iomstrifc2Clken: Self = Self(0x02);
    #[doc = "Clock enable for the IO MASTER 3 IFC INTERFACE value."]
    pub const Iomstrifc3Clken: Self = Self(0x04);
    #[doc = "Clock enable for the IO MASTER 4 IFC INTERFACE value."]
    pub const Iomstrifc4Clken: Self = Self(0x08);
    #[doc = "Clock enable for the IO MASTER 5 IFC INTERFACE value."]
    pub const Iomstrifc5Clken: Self = Self(0x10);
    #[doc = "Clock enable for the PDM value."]
    pub const PdmClken: Self = Self(0x20);
    #[doc = "Clock enable for the PDM INTERFACE value."]
    pub const PdmifcClken: Self = Self(0x40);
    #[doc = "Clock enable for the PWRCTRL value."]
    pub const PwrctrlClken: Self = Self(0x80);
    #[doc = "Clock enable for the PWRCTRL counter value."]
    pub const PwrctrlCountClken: Self = Self(0x0100);
    #[doc = "Clock enable for the RSTGEN value."]
    pub const RstgenClken: Self = Self(0x0200);
    #[doc = "Clock enable for the SCARD value."]
    pub const ScardClken: Self = Self(0x0400);
    #[doc = "Clock enable for the SCARD ALTAPB value."]
    pub const ScardAltapbClken: Self = Self(0x0800);
    #[doc = "Clock enable for the STIMER_CNT_CLKEN value."]
    pub const StimerCntClken: Self = Self(0x1000);
    #[doc = "Clock enable for the TPIU_CLKEN value."]
    pub const TpiuClken: Self = Self(0x2000);
    #[doc = "Clock enable for the UART0 HF value."]
    pub const Uart0hfClken: Self = Self(0x4000);
    #[doc = "Clock enable for the UART1 HF value."]
    pub const Uart1hfClken: Self = Self(0x8000);
    #[doc = "Clock enable for the XT 32KHZ value."]
    pub const Xt32khzEn: Self = Self(0x4000_0000);
    #[doc = "HFRC is forced on Status. value."]
    pub const Forcehfrc: Self = Self(0x8000_0000);
}
impl Clocken2stat {
    pub const fn from_bits(val: u32) -> Clocken2stat {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Clocken2stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Iomstrifc1Clken"),
            0x02 => f.write_str("Iomstrifc2Clken"),
            0x04 => f.write_str("Iomstrifc3Clken"),
            0x08 => f.write_str("Iomstrifc4Clken"),
            0x10 => f.write_str("Iomstrifc5Clken"),
            0x20 => f.write_str("PdmClken"),
            0x40 => f.write_str("PdmifcClken"),
            0x80 => f.write_str("PwrctrlClken"),
            0x0100 => f.write_str("PwrctrlCountClken"),
            0x0200 => f.write_str("RstgenClken"),
            0x0400 => f.write_str("ScardClken"),
            0x0800 => f.write_str("ScardAltapbClken"),
            0x1000 => f.write_str("StimerCntClken"),
            0x2000 => f.write_str("TpiuClken"),
            0x4000 => f.write_str("Uart0hfClken"),
            0x8000 => f.write_str("Uart1hfClken"),
            0x4000_0000 => f.write_str("Xt32khzEn"),
            0x8000_0000 => f.write_str("Forcehfrc"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clocken2stat {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Iomstrifc1Clken"),
            0x02 => defmt::write!(f, "Iomstrifc2Clken"),
            0x04 => defmt::write!(f, "Iomstrifc3Clken"),
            0x08 => defmt::write!(f, "Iomstrifc4Clken"),
            0x10 => defmt::write!(f, "Iomstrifc5Clken"),
            0x20 => defmt::write!(f, "PdmClken"),
            0x40 => defmt::write!(f, "PdmifcClken"),
            0x80 => defmt::write!(f, "PwrctrlClken"),
            0x0100 => defmt::write!(f, "PwrctrlCountClken"),
            0x0200 => defmt::write!(f, "RstgenClken"),
            0x0400 => defmt::write!(f, "ScardClken"),
            0x0800 => defmt::write!(f, "ScardAltapbClken"),
            0x1000 => defmt::write!(f, "StimerCntClken"),
            0x2000 => defmt::write!(f, "TpiuClken"),
            0x4000 => defmt::write!(f, "Uart0hfClken"),
            0x8000 => defmt::write!(f, "Uart1hfClken"),
            0x4000_0000 => defmt::write!(f, "Xt32khzEn"),
            0x8000_0000 => defmt::write!(f, "Forcehfrc"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Clocken2stat {
    #[inline(always)]
    fn from(val: u32) -> Clocken2stat {
        Clocken2stat::from_bits(val)
    }
}
impl From<Clocken2stat> for u32 {
    #[inline(always)]
    fn from(val: Clocken2stat) -> u32 {
        Clocken2stat::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clocken3stat(u32);
impl Clocken3stat {
    #[doc = "DAP clock is enabled \\[17\\] value."]
    pub const DapEnabled: Self = Self(0x0002_0000);
    #[doc = "VCOMP powerdown indicator \\[18\\] value."]
    pub const VcompEnabled: Self = Self(0x0004_0000);
    #[doc = "XTAL is enabled \\[24\\] value."]
    pub const XtalEnabled: Self = Self(0x0100_0000);
    #[doc = "HFRC is enabled \\[25\\] value."]
    pub const HfrcEnabled: Self = Self(0x0200_0000);
    #[doc = "HFRC Adjust enabled \\[26\\] value."]
    pub const Hfadjen: Self = Self(0x0400_0000);
    #[doc = "HFRC Enabled out \\[27\\] value."]
    pub const HfrcEnOut: Self = Self(0x0800_0000);
    #[doc = "RTC use XT \\[28\\] value."]
    pub const RtcXt: Self = Self(0x1000_0000);
    #[doc = "XTAL clkout enabled \\[29\\] value."]
    pub const ClkoutXtalEn: Self = Self(0x2000_0000);
    #[doc = "HFRC clkout enabled \\[30\\] value."]
    pub const ClkoutHfrcEn: Self = Self(0x4000_0000);
    #[doc = "Flash clk is enabled \\[31\\] value."]
    pub const FlashclkEn: Self = Self(0x8000_0000);
}
impl Clocken3stat {
    pub const fn from_bits(val: u32) -> Clocken3stat {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Clocken3stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0002_0000 => f.write_str("DapEnabled"),
            0x0004_0000 => f.write_str("VcompEnabled"),
            0x0100_0000 => f.write_str("XtalEnabled"),
            0x0200_0000 => f.write_str("HfrcEnabled"),
            0x0400_0000 => f.write_str("Hfadjen"),
            0x0800_0000 => f.write_str("HfrcEnOut"),
            0x1000_0000 => f.write_str("RtcXt"),
            0x2000_0000 => f.write_str("ClkoutXtalEn"),
            0x4000_0000 => f.write_str("ClkoutHfrcEn"),
            0x8000_0000 => f.write_str("FlashclkEn"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clocken3stat {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0002_0000 => defmt::write!(f, "DapEnabled"),
            0x0004_0000 => defmt::write!(f, "VcompEnabled"),
            0x0100_0000 => defmt::write!(f, "XtalEnabled"),
            0x0200_0000 => defmt::write!(f, "HfrcEnabled"),
            0x0400_0000 => defmt::write!(f, "Hfadjen"),
            0x0800_0000 => defmt::write!(f, "HfrcEnOut"),
            0x1000_0000 => defmt::write!(f, "RtcXt"),
            0x2000_0000 => defmt::write!(f, "ClkoutXtalEn"),
            0x4000_0000 => defmt::write!(f, "ClkoutHfrcEn"),
            0x8000_0000 => defmt::write!(f, "FlashclkEn"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Clocken3stat {
    #[inline(always)]
    fn from(val: u32) -> Clocken3stat {
        Clocken3stat::from_bits(val)
    }
}
impl From<Clocken3stat> for u32 {
    #[inline(always)]
    fn from(val: Clocken3stat) -> u32 {
        Clocken3stat::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clockenstat(u32);
impl Clockenstat {
    #[doc = "Clock enable for the ADC. value."]
    pub const AdcClken: Self = Self(0x01);
    #[doc = "Clock enable for the APBDMA ACTIVITY value."]
    pub const ApbdmaActivityClken: Self = Self(0x02);
    #[doc = "Clock enable for the APBDMA AOH DOMAIN value."]
    pub const ApbdmaAohClken: Self = Self(0x04);
    #[doc = "Clock enable for the APBDMA AOL DOMAIN value."]
    pub const ApbdmaAolClken: Self = Self(0x08);
    #[doc = "Clock enable for the APBDMA_APB value."]
    pub const ApbdmaApbClken: Self = Self(0x10);
    #[doc = "Clock enable for the APBDMA_BLEL value."]
    pub const ApbdmaBlelClken: Self = Self(0x20);
    #[doc = "Clock enable for the APBDMA_HCPA value."]
    pub const ApbdmaHcpaClken: Self = Self(0x40);
    #[doc = "Clock enable for the APBDMA_HCPB value."]
    pub const ApbdmaHcpbClken: Self = Self(0x80);
    #[doc = "Clock enable for the APBDMA_HCPC value."]
    pub const ApbdmaHcpcClken: Self = Self(0x0100);
    #[doc = "Clock enable for the APBDMA_MSPI value."]
    pub const ApbdmaMspiClken: Self = Self(0x0200);
    #[doc = "Clock enable for the APBDMA_PDM value."]
    pub const ApbdmaPdmClken: Self = Self(0x0400);
    #[doc = "Clock enable for the BLEIF value."]
    pub const BleifClkClken: Self = Self(0x0800);
    #[doc = "Clock enable for the BLEIF 32khZ CLOCK value."]
    pub const BleifClk32kClken: Self = Self(0x1000);
    #[doc = "Clock enable for the CTIMER BLOCK value."]
    pub const CtimerClken: Self = Self(0x2000);
    #[doc = "Clock enable for the CTIMER0A value."]
    pub const Ctimer0aClken: Self = Self(0x4000);
    #[doc = "Clock enable for the CTIMER0B value."]
    pub const Ctimer0bClken: Self = Self(0x8000);
    #[doc = "Clock enable for the CTIMER1A value."]
    pub const Ctimer1aClken: Self = Self(0x0001_0000);
    #[doc = "Clock enable for the CTIMER1B value."]
    pub const Ctimer1bClken: Self = Self(0x0002_0000);
    #[doc = "Clock enable for the CTIMER2A value."]
    pub const Ctimer2aClken: Self = Self(0x0004_0000);
    #[doc = "Clock enable for the CTIMER2B value."]
    pub const Ctimer2bClken: Self = Self(0x0008_0000);
    #[doc = "Clock enable for the CTIMER3A value."]
    pub const Ctimer3aClken: Self = Self(0x0010_0000);
    #[doc = "Clock enable for the CTIMER3B value."]
    pub const Ctimer3bClken: Self = Self(0x0020_0000);
    #[doc = "Clock enable for the CTIMER4A value."]
    pub const Ctimer4aClken: Self = Self(0x0040_0000);
    #[doc = "Clock enable for the CTIMER4B value."]
    pub const Ctimer4bClken: Self = Self(0x0080_0000);
    #[doc = "Clock enable for the CTIMER5A value."]
    pub const Ctimer5aClken: Self = Self(0x0100_0000);
    #[doc = "Clock enable for the CTIMER5B value."]
    pub const Ctimer5bClken: Self = Self(0x0200_0000);
    #[doc = "Clock enable for the CTIMER6A value."]
    pub const Ctimer6aClken: Self = Self(0x0400_0000);
    #[doc = "Clock enable for the CTIMER6B value."]
    pub const Ctimer6bClken: Self = Self(0x0800_0000);
    #[doc = "Clock enable for the CTIMER7A value."]
    pub const Ctimer7aClken: Self = Self(0x1000_0000);
    #[doc = "Clock enable for the CTIMER7B value."]
    pub const Ctimer7bClken: Self = Self(0x2000_0000);
    #[doc = "Clock enable for the DAP value."]
    pub const DapClken: Self = Self(0x4000_0000);
    #[doc = "Clock enable for the IOMSTRIFC0 value."]
    pub const Iomstrifc0Clken: Self = Self(0x8000_0000);
}
impl Clockenstat {
    pub const fn from_bits(val: u32) -> Clockenstat {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Clockenstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("AdcClken"),
            0x02 => f.write_str("ApbdmaActivityClken"),
            0x04 => f.write_str("ApbdmaAohClken"),
            0x08 => f.write_str("ApbdmaAolClken"),
            0x10 => f.write_str("ApbdmaApbClken"),
            0x20 => f.write_str("ApbdmaBlelClken"),
            0x40 => f.write_str("ApbdmaHcpaClken"),
            0x80 => f.write_str("ApbdmaHcpbClken"),
            0x0100 => f.write_str("ApbdmaHcpcClken"),
            0x0200 => f.write_str("ApbdmaMspiClken"),
            0x0400 => f.write_str("ApbdmaPdmClken"),
            0x0800 => f.write_str("BleifClkClken"),
            0x1000 => f.write_str("BleifClk32kClken"),
            0x2000 => f.write_str("CtimerClken"),
            0x4000 => f.write_str("Ctimer0aClken"),
            0x8000 => f.write_str("Ctimer0bClken"),
            0x0001_0000 => f.write_str("Ctimer1aClken"),
            0x0002_0000 => f.write_str("Ctimer1bClken"),
            0x0004_0000 => f.write_str("Ctimer2aClken"),
            0x0008_0000 => f.write_str("Ctimer2bClken"),
            0x0010_0000 => f.write_str("Ctimer3aClken"),
            0x0020_0000 => f.write_str("Ctimer3bClken"),
            0x0040_0000 => f.write_str("Ctimer4aClken"),
            0x0080_0000 => f.write_str("Ctimer4bClken"),
            0x0100_0000 => f.write_str("Ctimer5aClken"),
            0x0200_0000 => f.write_str("Ctimer5bClken"),
            0x0400_0000 => f.write_str("Ctimer6aClken"),
            0x0800_0000 => f.write_str("Ctimer6bClken"),
            0x1000_0000 => f.write_str("Ctimer7aClken"),
            0x2000_0000 => f.write_str("Ctimer7bClken"),
            0x4000_0000 => f.write_str("DapClken"),
            0x8000_0000 => f.write_str("Iomstrifc0Clken"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clockenstat {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "AdcClken"),
            0x02 => defmt::write!(f, "ApbdmaActivityClken"),
            0x04 => defmt::write!(f, "ApbdmaAohClken"),
            0x08 => defmt::write!(f, "ApbdmaAolClken"),
            0x10 => defmt::write!(f, "ApbdmaApbClken"),
            0x20 => defmt::write!(f, "ApbdmaBlelClken"),
            0x40 => defmt::write!(f, "ApbdmaHcpaClken"),
            0x80 => defmt::write!(f, "ApbdmaHcpbClken"),
            0x0100 => defmt::write!(f, "ApbdmaHcpcClken"),
            0x0200 => defmt::write!(f, "ApbdmaMspiClken"),
            0x0400 => defmt::write!(f, "ApbdmaPdmClken"),
            0x0800 => defmt::write!(f, "BleifClkClken"),
            0x1000 => defmt::write!(f, "BleifClk32kClken"),
            0x2000 => defmt::write!(f, "CtimerClken"),
            0x4000 => defmt::write!(f, "Ctimer0aClken"),
            0x8000 => defmt::write!(f, "Ctimer0bClken"),
            0x0001_0000 => defmt::write!(f, "Ctimer1aClken"),
            0x0002_0000 => defmt::write!(f, "Ctimer1bClken"),
            0x0004_0000 => defmt::write!(f, "Ctimer2aClken"),
            0x0008_0000 => defmt::write!(f, "Ctimer2bClken"),
            0x0010_0000 => defmt::write!(f, "Ctimer3aClken"),
            0x0020_0000 => defmt::write!(f, "Ctimer3bClken"),
            0x0040_0000 => defmt::write!(f, "Ctimer4aClken"),
            0x0080_0000 => defmt::write!(f, "Ctimer4bClken"),
            0x0100_0000 => defmt::write!(f, "Ctimer5aClken"),
            0x0200_0000 => defmt::write!(f, "Ctimer5bClken"),
            0x0400_0000 => defmt::write!(f, "Ctimer6aClken"),
            0x0800_0000 => defmt::write!(f, "Ctimer6bClken"),
            0x1000_0000 => defmt::write!(f, "Ctimer7aClken"),
            0x2000_0000 => defmt::write!(f, "Ctimer7bClken"),
            0x4000_0000 => defmt::write!(f, "DapClken"),
            0x8000_0000 => defmt::write!(f, "Iomstrifc0Clken"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Clockenstat {
    #[inline(always)]
    fn from(val: u32) -> Clockenstat {
        Clockenstat::from_bits(val)
    }
}
impl From<Clockenstat> for u32 {
    #[inline(always)]
    fn from(val: Clockenstat) -> u32 {
        Clockenstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coresel {
    #[doc = "Core Clock is HFRC value."]
    Hfrc = 0x0,
    #[doc = "Core Clock is HFRC / 2 value."]
    HfrcDiv2 = 0x01,
}
impl Coresel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Coresel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Coresel {
    #[inline(always)]
    fn from(val: u8) -> Coresel {
        Coresel::from_bits(val)
    }
}
impl From<Coresel> for u8 {
    #[inline(always)]
    fn from(val: Coresel) -> u8 {
        Coresel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hfadjck {
    #[doc = "Autoadjust repeat period = 4 seconds value."]
    _4sec = 0x0,
    #[doc = "Autoadjust repeat period = 16 seconds value."]
    _16sec = 0x01,
    #[doc = "Autoadjust repeat period = 32 seconds value."]
    _32sec = 0x02,
    #[doc = "Autoadjust repeat period = 64 seconds value."]
    _64sec = 0x03,
    #[doc = "Autoadjust repeat period = 128 seconds value."]
    _128sec = 0x04,
    #[doc = "Autoadjust repeat period = 256 seconds value."]
    _256sec = 0x05,
    #[doc = "Autoadjust repeat period = 512 seconds value."]
    _512sec = 0x06,
    #[doc = "Autoadjust repeat period = 1024 seconds value."]
    _1024sec = 0x07,
}
impl Hfadjck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfadjck {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfadjck {
    #[inline(always)]
    fn from(val: u8) -> Hfadjck {
        Hfadjck::from_bits(val)
    }
}
impl From<Hfadjck> for u8 {
    #[inline(always)]
    fn from(val: Hfadjck) -> u8 {
        Hfadjck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hfadjgain {
    #[doc = "HF Adjust with Gain of 1 value."]
    GainOf1 = 0x0,
    #[doc = "HF Adjust with Gain of 0.5 value."]
    GainOf1In2 = 0x01,
    #[doc = "HF Adjust with Gain of 0.25 value."]
    GainOf1In4 = 0x02,
    #[doc = "HF Adjust with Gain of 0.125 value."]
    GainOf1In8 = 0x03,
    #[doc = "HF Adjust with Gain of 0.0625 value."]
    GainOf1In16 = 0x04,
    #[doc = "HF Adjust with Gain of 0.03125 value."]
    GainOf1In32 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Hfadjgain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfadjgain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfadjgain {
    #[inline(always)]
    fn from(val: u8) -> Hfadjgain {
        Hfadjgain::from_bits(val)
    }
}
impl From<Hfadjgain> for u8 {
    #[inline(always)]
    fn from(val: Hfadjgain) -> u8 {
        Hfadjgain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hfwarmup {
    #[doc = "Autoadjust XT warmup period = 1-2 seconds value."]
    _1sec = 0x0,
    #[doc = "Autoadjust XT warmup period = 2-4 seconds value."]
    _2sec = 0x01,
}
impl Hfwarmup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfwarmup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfwarmup {
    #[inline(always)]
    fn from(val: u8) -> Hfwarmup {
        Hfwarmup::from_bits(val)
    }
}
impl From<Hfwarmup> for u8 {
    #[inline(always)]
    fn from(val: Hfwarmup) -> u8 {
        Hfwarmup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osel {
    #[doc = "RTC uses the XT value."]
    RtcXt = 0x0,
    #[doc = "RTC uses the LFRC value."]
    RtcLfrc = 0x01,
}
impl Osel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osel {
    #[inline(always)]
    fn from(val: u8) -> Osel {
        Osel::from_bits(val)
    }
}
impl From<Osel> for u8 {
    #[inline(always)]
    fn from(val: Osel) -> u8 {
        Osel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stoprc {
    #[doc = "Enable the LFRC Oscillator to drive the RTC value."]
    En = 0x0,
    #[doc = "Stop the LFRC Oscillator when driving the RTC value."]
    Stop = 0x01,
}
impl Stoprc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stoprc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stoprc {
    #[inline(always)]
    fn from(val: u8) -> Stoprc {
        Stoprc::from_bits(val)
    }
}
impl From<Stoprc> for u8 {
    #[inline(always)]
    fn from(val: Stoprc) -> u8 {
        Stoprc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stopxt {
    #[doc = "Enable the XT Oscillator to drive the RTC value."]
    En = 0x0,
    #[doc = "Stop the XT Oscillator when driving the RTC value."]
    Stop = 0x01,
}
impl Stopxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stopxt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stopxt {
    #[inline(always)]
    fn from(val: u8) -> Stopxt {
        Stopxt::from_bits(val)
    }
}
impl From<Stopxt> for u8 {
    #[inline(always)]
    fn from(val: Stopxt) -> u8 {
        Stopxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tonadjustperiod {
    #[doc = "Adjust done for every 1 94KHz period value."]
    Hfrc94kHz = 0x0,
    #[doc = "Adjust done for every 1 47KHz period value."]
    Hfrc47kHz = 0x01,
    #[doc = "Adjust done for every 1 12KHz period value."]
    Hfrc12kHz = 0x02,
    #[doc = "Adjust done for every 1 3KHz period value."]
    Hfrc3kHz = 0x03,
}
impl Tonadjustperiod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tonadjustperiod {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tonadjustperiod {
    #[inline(always)]
    fn from(val: u8) -> Tonadjustperiod {
        Tonadjustperiod::from_bits(val)
    }
}
impl From<Tonadjustperiod> for u8 {
    #[inline(always)]
    fn from(val: Tonadjustperiod) -> u8 {
        Tonadjustperiod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zerolendetecttrim {
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 2.0us (10 percent margin of error) or more value."]
    Set0 = 0x0,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 5.4us (10 percent margin of error) or more value."]
    Set1 = 0x01,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 10.8us (10 percent margin of error) or more value."]
    Set2 = 0x02,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 16.2us (10 percent margin of error) or more value."]
    Set3 = 0x03,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 21.6us (10 percent margin of error) or more value."]
    Set4 = 0x04,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 27.0us (10 percent margin of error) or more value."]
    Set5 = 0x05,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 32.4us (10 percent margin of error) or more value."]
    Set6 = 0x06,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 37.8us (10 percent margin of error) or more value."]
    Set7 = 0x07,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 43.2us (10 percent margin of error) or more value."]
    Set8 = 0x08,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 48.6us (10 percent margin of error) or more value."]
    Set9 = 0x09,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 54.0us (10 percent margin of error) or more value."]
    SetA = 0x0a,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 59.4us (10 percent margin of error) or more value."]
    SetB = 0x0b,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 64.8us (10 percent margin of error) or more value."]
    SetC = 0x0c,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 70.2us (10 percent margin of error) or more value."]
    SetD = 0x0d,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 75.6us (10 percent margin of error) or more value."]
    SetE = 0x0e,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 81us (10 percent margin of error) or more value."]
    SetF = 0x0f,
}
impl Zerolendetecttrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zerolendetecttrim {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zerolendetecttrim {
    #[inline(always)]
    fn from(val: u8) -> Zerolendetecttrim {
        Zerolendetecttrim::from_bits(val)
    }
}
impl From<Zerolendetecttrim> for u8 {
    #[inline(always)]
    fn from(val: Zerolendetecttrim) -> u8 {
        Zerolendetecttrim::to_bits(val)
    }
}

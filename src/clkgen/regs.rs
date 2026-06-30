#[doc = "Autocalibration Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acalctr(pub u32);
impl Acalctr {
    #[doc = "Autocalibration Counter result. Bits 17 down to 0 of this is feed directly to the CALRC register if ACAL register in OCTRL register is set to 1024SEC or 512SEC."]
    #[must_use]
    #[inline(always)]
    pub const fn acalctr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Autocalibration Counter result. Bits 17 down to 0 of this is feed directly to the CALRC register if ACAL register in OCTRL register is set to 1024SEC or 512SEC."]
    #[inline(always)]
    pub const fn set_acalctr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Acalctr {
    #[inline(always)]
    fn default() -> Acalctr {
        Acalctr(0)
    }
}
impl core::fmt::Debug for Acalctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Acalctr")
            .field("acalctr", &self.acalctr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Acalctr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Acalctr {{ acalctr: {=u32:?} }}", self.acalctr())
    }
}
#[doc = "BLE BUCK TON ADJUST."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blebucktonadj(pub u32);
impl Blebucktonadj {
    #[doc = "TON ADJUST LOW THRESHOLD. Suggested values are #A(94KHz) #15(47KHz) #53(12Khz) #14D(3Khz)."]
    #[must_use]
    #[inline(always)]
    pub const fn tonlowthreshold(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "TON ADJUST LOW THRESHOLD. Suggested values are #A(94KHz) #15(47KHz) #53(12Khz) #14D(3Khz)."]
    #[inline(always)]
    pub const fn set_tonlowthreshold(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "TON ADJUST HIGH THRESHOLD. Suggested values are #15(94KHz) #2A(47Khz) #A6(12Khz) #29A(3Khz)."]
    #[must_use]
    #[inline(always)]
    pub const fn tonhighthreshold(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[doc = "TON ADJUST HIGH THRESHOLD. Suggested values are #15(94KHz) #2A(47Khz) #A6(12Khz) #29A(3Khz)."]
    #[inline(always)]
    pub const fn set_tonhighthreshold(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[doc = "TON ADJUST PERIOD."]
    #[must_use]
    #[inline(always)]
    pub const fn tonadjustperiod(&self) -> super::vals::Tonadjustperiod {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Tonadjustperiod::from_bits(val as u8)
    }
    #[doc = "TON ADJUST PERIOD."]
    #[inline(always)]
    pub const fn set_tonadjustperiod(&mut self, val: super::vals::Tonadjustperiod) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "TON ADJUST ENABLE."]
    #[must_use]
    #[inline(always)]
    pub const fn tonadjusten(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TON ADJUST ENABLE."]
    #[inline(always)]
    pub const fn set_tonadjusten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "BLEBUCK ZERO LENGTH DETECT TRIM."]
    #[must_use]
    #[inline(always)]
    pub const fn zerolendetecttrim(&self) -> super::vals::Zerolendetecttrim {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::Zerolendetecttrim::from_bits(val as u8)
    }
    #[doc = "BLEBUCK ZERO LENGTH DETECT TRIM."]
    #[inline(always)]
    pub const fn set_zerolendetecttrim(&mut self, val: super::vals::Zerolendetecttrim) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "BLEBUCK ZERO LENGTH DETECT ENABLE."]
    #[must_use]
    #[inline(always)]
    pub const fn zerolendetecten(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "BLEBUCK ZERO LENGTH DETECT ENABLE."]
    #[inline(always)]
    pub const fn set_zerolendetecten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Blebucktonadj {
    #[inline(always)]
    fn default() -> Blebucktonadj {
        Blebucktonadj(0)
    }
}
impl core::fmt::Debug for Blebucktonadj {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blebucktonadj")
            .field("tonlowthreshold", &self.tonlowthreshold())
            .field("tonhighthreshold", &self.tonhighthreshold())
            .field("tonadjustperiod", &self.tonadjustperiod())
            .field("tonadjusten", &self.tonadjusten())
            .field("zerolendetecttrim", &self.zerolendetecttrim())
            .field("zerolendetecten", &self.zerolendetecten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blebucktonadj {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blebucktonadj {{ tonlowthreshold: {=u16:?}, tonhighthreshold: {=u16:?}, tonadjustperiod: {:?}, tonadjusten: {=bool:?}, zerolendetecttrim: {:?}, zerolendetecten: {=bool:?} }}" , self . tonlowthreshold () , self . tonhighthreshold () , self . tonadjustperiod () , self . tonadjusten () , self . zerolendetecttrim () , self . zerolendetecten ())
    }
}
#[doc = "RC Oscillator Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calrc(pub u32);
impl Calrc {
    #[doc = "LFRC Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 512 Hz clock derived from the original 1024 version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The range is from -131072 (decimal) to 131071 (decimal). This register is normally used in conjuction with ACALCTR register. The CALRC register will load the ACALCTR register (bits 17:0) if the ACALCTR register is set to measure the LFRC with the XT clock."]
    #[must_use]
    #[inline(always)]
    pub const fn calrc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "LFRC Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 512 Hz clock derived from the original 1024 version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The range is from -131072 (decimal) to 131071 (decimal). This register is normally used in conjuction with ACALCTR register. The CALRC register will load the ACALCTR register (bits 17:0) if the ACALCTR register is set to measure the LFRC with the XT clock."]
    #[inline(always)]
    pub const fn set_calrc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Calrc {
    #[inline(always)]
    fn default() -> Calrc {
        Calrc(0)
    }
}
impl core::fmt::Debug for Calrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calrc")
            .field("calrc", &self.calrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Calrc {{ calrc: {=u32:?} }}", self.calrc())
    }
}
#[doc = "XT Oscillator Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calxt(pub u32);
impl Calxt {
    #[doc = "XT Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 16KHz clock derived from the original 32KHz version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The maximum value that is effective is from -1024 to 1023."]
    #[must_use]
    #[inline(always)]
    pub const fn calxt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "XT Oscillator calibration value. This register will enable the hardware to increase or decrease the number of cycles in a 16KHz clock derived from the original 32KHz version. The most significant bit is the sign. A '1' is a reduction, and a '0' is an addition. This calibration value will add or reduce the number of cycles programmed here across a 32 second interval. The maximum value that is effective is from -1024 to 1023."]
    #[inline(always)]
    pub const fn set_calxt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for Calxt {
    #[inline(always)]
    fn default() -> Calxt {
        Calxt(0)
    }
}
impl core::fmt::Debug for Calxt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calxt")
            .field("calxt", &self.calxt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calxt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Calxt {{ calxt: {=u16:?} }}", self.calxt())
    }
}
#[doc = "HFRC Clock Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cctrl(pub u32);
impl Cctrl {
    #[doc = "Core Clock divisor."]
    #[must_use]
    #[inline(always)]
    pub const fn coresel(&self) -> super::vals::Coresel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Coresel::from_bits(val as u8)
    }
    #[doc = "Core Clock divisor."]
    #[inline(always)]
    pub const fn set_coresel(&mut self, val: super::vals::Coresel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Cctrl {
    #[inline(always)]
    fn default() -> Cctrl {
        Cctrl(0)
    }
}
impl core::fmt::Debug for Cctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cctrl")
            .field("coresel", &self.coresel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cctrl {{ coresel: {:?} }}", self.coresel())
    }
}
#[doc = "Key Register for Clock Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkkey(pub u32);
impl Clkkey {
    #[doc = "Key register value."]
    #[must_use]
    #[inline(always)]
    pub const fn clkkey(&self) -> super::vals::Clkkey {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Clkkey::from_bits(val as u32)
    }
    #[doc = "Key register value."]
    #[inline(always)]
    pub const fn set_clkkey(&mut self, val: super::vals::Clkkey) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Clkkey {
    #[inline(always)]
    fn default() -> Clkkey {
        Clkkey(0)
    }
}
impl core::fmt::Debug for Clkkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkkey")
            .field("clkkey", &self.clkkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkkey {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkkey {{ clkkey: {:?} }}", self.clkkey())
    }
}
#[doc = "CLKOUT Frequency Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkout(pub u32);
impl Clkout {
    #[doc = "CLKOUT signal select."]
    #[must_use]
    #[inline(always)]
    pub const fn cksel(&self) -> super::vals::Cksel {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cksel::from_bits(val as u8)
    }
    #[doc = "CLKOUT signal select."]
    #[inline(always)]
    pub const fn set_cksel(&mut self, val: super::vals::Cksel) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Enable the CLKOUT signal."]
    #[must_use]
    #[inline(always)]
    pub const fn cken(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the CLKOUT signal."]
    #[inline(always)]
    pub const fn set_cken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Clkout {
    #[inline(always)]
    fn default() -> Clkout {
        Clkout(0)
    }
}
impl core::fmt::Debug for Clkout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkout")
            .field("cksel", &self.cksel())
            .field("cken", &self.cken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkout {{ cksel: {:?}, cken: {=bool:?} }}",
            self.cksel(),
            self.cken()
        )
    }
}
#[doc = "Clock Enable Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clocken2stat(pub u32);
impl Clocken2stat {
    #[doc = "Clock enable status 2."]
    #[must_use]
    #[inline(always)]
    pub const fn clocken2stat(&self) -> super::vals::Clocken2stat {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Clocken2stat::from_bits(val as u32)
    }
    #[doc = "Clock enable status 2."]
    #[inline(always)]
    pub const fn set_clocken2stat(&mut self, val: super::vals::Clocken2stat) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Clocken2stat {
    #[inline(always)]
    fn default() -> Clocken2stat {
        Clocken2stat(0)
    }
}
impl core::fmt::Debug for Clocken2stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clocken2stat")
            .field("clocken2stat", &self.clocken2stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clocken2stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clocken2stat {{ clocken2stat: {:?} }}",
            self.clocken2stat()
        )
    }
}
#[doc = "Clock Enable Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clocken3stat(pub u32);
impl Clocken3stat {
    #[doc = "Clock enable status 3."]
    #[must_use]
    #[inline(always)]
    pub const fn clocken3stat(&self) -> super::vals::Clocken3stat {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Clocken3stat::from_bits(val as u32)
    }
    #[doc = "Clock enable status 3."]
    #[inline(always)]
    pub const fn set_clocken3stat(&mut self, val: super::vals::Clocken3stat) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Clocken3stat {
    #[inline(always)]
    fn default() -> Clocken3stat {
        Clocken3stat(0)
    }
}
impl core::fmt::Debug for Clocken3stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clocken3stat")
            .field("clocken3stat", &self.clocken3stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clocken3stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clocken3stat {{ clocken3stat: {:?} }}",
            self.clocken3stat()
        )
    }
}
#[doc = "Clock Enable Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clockenstat(pub u32);
impl Clockenstat {
    #[doc = "Clock enable status."]
    #[must_use]
    #[inline(always)]
    pub const fn clockenstat(&self) -> super::vals::Clockenstat {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Clockenstat::from_bits(val as u32)
    }
    #[doc = "Clock enable status."]
    #[inline(always)]
    pub const fn set_clockenstat(&mut self, val: super::vals::Clockenstat) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Clockenstat {
    #[inline(always)]
    fn default() -> Clockenstat {
        Clockenstat(0)
    }
}
impl core::fmt::Debug for Clockenstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clockenstat")
            .field("clockenstat", &self.clockenstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clockenstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clockenstat {{ clockenstat: {:?} }}", self.clockenstat())
    }
}
#[doc = "HFRC Frequency Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Freqctrl(pub u32);
impl Freqctrl {
    #[doc = "Frequency Burst Enable Request."]
    #[must_use]
    #[inline(always)]
    pub const fn burstreq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Burst Enable Request."]
    #[inline(always)]
    pub const fn set_burstreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Frequency Burst Request Acknowledge. Frequency burst requested is always acknowledged whether burst is granted or not depending on feature enable."]
    #[must_use]
    #[inline(always)]
    pub const fn burstack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Burst Request Acknowledge. Frequency burst requested is always acknowledged whether burst is granted or not depending on feature enable."]
    #[inline(always)]
    pub const fn set_burstack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This represents frequency burst status."]
    #[must_use]
    #[inline(always)]
    pub const fn burststatus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This represents frequency burst status."]
    #[inline(always)]
    pub const fn set_burststatus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Freqctrl {
    #[inline(always)]
    fn default() -> Freqctrl {
        Freqctrl(0)
    }
}
impl core::fmt::Debug for Freqctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Freqctrl")
            .field("burstreq", &self.burstreq())
            .field("burstack", &self.burstack())
            .field("burststatus", &self.burststatus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Freqctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Freqctrl {{ burstreq: {=bool:?}, burstack: {=bool:?}, burststatus: {=bool:?} }}",
            self.burstreq(),
            self.burstack(),
            self.burststatus()
        )
    }
}
#[doc = "HFRC Adjustment."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfadj(pub u32);
impl Hfadj {
    #[doc = "HFRC adjustment control."]
    #[must_use]
    #[inline(always)]
    pub const fn hfadjen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HFRC adjustment control."]
    #[inline(always)]
    pub const fn set_hfadjen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Repeat period for HFRC adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn hfadjck(&self) -> super::vals::Hfadjck {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Hfadjck::from_bits(val as u8)
    }
    #[doc = "Repeat period for HFRC adjustment."]
    #[inline(always)]
    pub const fn set_hfadjck(&mut self, val: super::vals::Hfadjck) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "Target HFRC adjustment value."]
    #[must_use]
    #[inline(always)]
    pub const fn hfxtadj(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Target HFRC adjustment value."]
    #[inline(always)]
    pub const fn set_hfxtadj(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "XT warmup period for HFRC adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn hfwarmup(&self) -> super::vals::Hfwarmup {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Hfwarmup::from_bits(val as u8)
    }
    #[doc = "XT warmup period for HFRC adjustment."]
    #[inline(always)]
    pub const fn set_hfwarmup(&mut self, val: super::vals::Hfwarmup) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Gain control for HFRC adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn hfadjgain(&self) -> super::vals::Hfadjgain {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::Hfadjgain::from_bits(val as u8)
    }
    #[doc = "Gain control for HFRC adjustment."]
    #[inline(always)]
    pub const fn set_hfadjgain(&mut self, val: super::vals::Hfadjgain) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
}
impl Default for Hfadj {
    #[inline(always)]
    fn default() -> Hfadj {
        Hfadj(0)
    }
}
impl core::fmt::Debug for Hfadj {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hfadj")
            .field("hfadjen", &self.hfadjen())
            .field("hfadjck", &self.hfadjck())
            .field("hfxtadj", &self.hfxtadj())
            .field("hfwarmup", &self.hfwarmup())
            .field("hfadjgain", &self.hfadjgain())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hfadj {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hfadj {{ hfadjen: {=bool:?}, hfadjck: {:?}, hfxtadj: {=u16:?}, hfwarmup: {:?}, hfadjgain: {:?} }}" , self . hfadjen () , self . hfadjck () , self . hfxtadj () , self . hfwarmup () , self . hfadjgain ())
    }
}
#[doc = "CLKGEN Interrupt Register: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intrptclr(pub u32);
impl Intrptclr {
    #[doc = "Autocalibration Fail interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn acf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Autocalibration Fail interrupt."]
    #[inline(always)]
    pub const fn set_acf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Autocalibration Complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn acc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Autocalibration Complete interrupt."]
    #[inline(always)]
    pub const fn set_acc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "XT Oscillator Fail interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "XT Oscillator Fail interrupt."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Intrptclr {
    #[inline(always)]
    fn default() -> Intrptclr {
        Intrptclr(0)
    }
}
impl core::fmt::Debug for Intrptclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intrptclr")
            .field("acf", &self.acf())
            .field("acc", &self.acc())
            .field("of", &self.of())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intrptclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intrptclr {{ acf: {=bool:?}, acc: {=bool:?}, of: {=bool:?} }}",
            self.acf(),
            self.acc(),
            self.of()
        )
    }
}
#[doc = "CLKGEN Interrupt Register: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intrpten(pub u32);
impl Intrpten {
    #[doc = "Autocalibration Fail interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn acf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Autocalibration Fail interrupt."]
    #[inline(always)]
    pub const fn set_acf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Autocalibration Complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn acc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Autocalibration Complete interrupt."]
    #[inline(always)]
    pub const fn set_acc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "XT Oscillator Fail interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "XT Oscillator Fail interrupt."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Intrpten {
    #[inline(always)]
    fn default() -> Intrpten {
        Intrpten(0)
    }
}
impl core::fmt::Debug for Intrpten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intrpten")
            .field("acf", &self.acf())
            .field("acc", &self.acc())
            .field("of", &self.of())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intrpten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intrpten {{ acf: {=bool:?}, acc: {=bool:?}, of: {=bool:?} }}",
            self.acf(),
            self.acc(),
            self.of()
        )
    }
}
#[doc = "CLKGEN Interrupt Register: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intrptset(pub u32);
impl Intrptset {
    #[doc = "Autocalibration Fail interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn acf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Autocalibration Fail interrupt."]
    #[inline(always)]
    pub const fn set_acf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Autocalibration Complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn acc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Autocalibration Complete interrupt."]
    #[inline(always)]
    pub const fn set_acc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "XT Oscillator Fail interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "XT Oscillator Fail interrupt."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Intrptset {
    #[inline(always)]
    fn default() -> Intrptset {
        Intrptset(0)
    }
}
impl core::fmt::Debug for Intrptset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intrptset")
            .field("acf", &self.acf())
            .field("acc", &self.acc())
            .field("of", &self.of())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intrptset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intrptset {{ acf: {=bool:?}, acc: {=bool:?}, of: {=bool:?} }}",
            self.acf(),
            self.acc(),
            self.of()
        )
    }
}
#[doc = "CLKGEN Interrupt Register: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intrptstat(pub u32);
impl Intrptstat {
    #[doc = "Autocalibration Fail interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn acf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Autocalibration Fail interrupt."]
    #[inline(always)]
    pub const fn set_acf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Autocalibration Complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn acc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Autocalibration Complete interrupt."]
    #[inline(always)]
    pub const fn set_acc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "XT Oscillator Fail interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "XT Oscillator Fail interrupt."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Intrptstat {
    #[inline(always)]
    fn default() -> Intrptstat {
        Intrptstat(0)
    }
}
impl core::fmt::Debug for Intrptstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intrptstat")
            .field("acf", &self.acf())
            .field("acc", &self.acc())
            .field("of", &self.of())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intrptstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intrptstat {{ acf: {=bool:?}, acc: {=bool:?}, of: {=bool:?} }}",
            self.acf(),
            self.acc(),
            self.of()
        )
    }
}
#[doc = "Oscillator Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Octrl(pub u32);
impl Octrl {
    #[doc = "Stop the XT Oscillator to the RTC."]
    #[must_use]
    #[inline(always)]
    pub const fn stopxt(&self) -> super::vals::Stoposc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Stoposc::from_bits(val as u8)
    }
    #[doc = "Stop the XT Oscillator to the RTC."]
    #[inline(always)]
    pub const fn set_stopxt(&mut self, val: super::vals::Stoposc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Stop the LFRC Oscillator to the RTC."]
    #[must_use]
    #[inline(always)]
    pub const fn stoprc(&self) -> super::vals::Stoposc {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Stoposc::from_bits(val as u8)
    }
    #[doc = "Stop the LFRC Oscillator to the RTC."]
    #[inline(always)]
    pub const fn set_stoprc(&mut self, val: super::vals::Stoposc) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Oscillator switch on failure function. If this is set, then LFRC clock source will switch from XT to RC."]
    #[must_use]
    #[inline(always)]
    pub const fn fos(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator switch on failure function. If this is set, then LFRC clock source will switch from XT to RC."]
    #[inline(always)]
    pub const fn set_fos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Selects the RTC oscillator (1 => LFRC, 0 => XT)."]
    #[must_use]
    #[inline(always)]
    pub const fn osel(&self) -> super::vals::Osel {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Osel::from_bits(val as u8)
    }
    #[doc = "Selects the RTC oscillator (1 => LFRC, 0 => XT)."]
    #[inline(always)]
    pub const fn set_osel(&mut self, val: super::vals::Osel) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Autocalibration control. This selects the source to be used in the autocalibration flow. This flow can also be used to measure an internal clock against an external clock source, with the external clock normally used as the reference."]
    #[must_use]
    #[inline(always)]
    pub const fn acal(&self) -> super::vals::Acal {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Acal::from_bits(val as u8)
    }
    #[doc = "Autocalibration control. This selects the source to be used in the autocalibration flow. This flow can also be used to measure an internal clock against an external clock source, with the external clock normally used as the reference."]
    #[inline(always)]
    pub const fn set_acal(&mut self, val: super::vals::Acal) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
}
impl Default for Octrl {
    #[inline(always)]
    fn default() -> Octrl {
        Octrl(0)
    }
}
impl core::fmt::Debug for Octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Octrl")
            .field("stopxt", &self.stopxt())
            .field("stoprc", &self.stoprc())
            .field("fos", &self.fos())
            .field("osel", &self.osel())
            .field("acal", &self.acal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Octrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Octrl {{ stopxt: {:?}, stoprc: {:?}, fos: {=bool:?}, osel: {:?}, acal: {:?} }}",
            self.stopxt(),
            self.stoprc(),
            self.fos(),
            self.osel(),
            self.acal()
        )
    }
}
#[doc = "Clock Generator Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Current RTC oscillator (1 => LFRC, 0 => XT). After an RTC oscillator change, it may take up to 2 seconds for this field to reflect the new oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn omode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current RTC oscillator (1 => LFRC, 0 => XT). After an RTC oscillator change, it may take up to 2 seconds for this field to reflect the new oscillator."]
    #[inline(always)]
    pub const fn set_omode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "XT Oscillator is enabled but not oscillating."]
    #[must_use]
    #[inline(always)]
    pub const fn oscf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "XT Oscillator is enabled but not oscillating."]
    #[inline(always)]
    pub const fn set_oscf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("omode", &self.omode())
            .field("oscf", &self.oscf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ omode: {=bool:?}, oscf: {=bool:?} }}",
            self.omode(),
            self.oscf()
        )
    }
}

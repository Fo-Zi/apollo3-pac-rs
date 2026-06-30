#[doc = "ADC Battery Load Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcbattload(pub u32);
impl Adcbattload {
    #[doc = "Enable the ADC battery load resistor."]
    #[must_use]
    #[inline(always)]
    pub const fn battload(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the ADC battery load resistor."]
    #[inline(always)]
    pub const fn set_battload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Adcbattload {
    #[inline(always)]
    fn default() -> Adcbattload {
        Adcbattload(0)
    }
}
impl core::fmt::Debug for Adcbattload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcbattload")
            .field("battload", &self.battload())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcbattload {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Adcbattload {{ battload: {=bool:?} }}", self.battload())
    }
}
#[doc = "ADC Calibration Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adccal(pub u32);
impl Adccal {
    #[doc = "Run ADC Calibration on initial power up sequence."]
    #[must_use]
    #[inline(always)]
    pub const fn calonpwrup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Run ADC Calibration on initial power up sequence."]
    #[inline(always)]
    pub const fn set_calonpwrup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Status for ADC Calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn adccalibrated(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status for ADC Calibration."]
    #[inline(always)]
    pub const fn set_adccalibrated(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Adccal {
    #[inline(always)]
    fn default() -> Adccal {
        Adccal(0)
    }
}
impl core::fmt::Debug for Adccal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adccal")
            .field("calonpwrup", &self.calonpwrup())
            .field("adccalibrated", &self.adccalibrated())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adccal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adccal {{ calonpwrup: {=bool:?}, adccalibrated: {=bool:?} }}",
            self.calonpwrup(),
            self.adccalibrated()
        )
    }
}
#[doc = "ADC Power Up Delay Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcpwrdly(pub u32);
impl Adcpwrdly {
    #[doc = "ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[must_use]
    #[inline(always)]
    pub const fn adcpwr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub const fn set_adcpwr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[must_use]
    #[inline(always)]
    pub const fn adcpwr1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub const fn set_adcpwr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Adcpwrdly {
    #[inline(always)]
    fn default() -> Adcpwrdly {
        Adcpwrdly(0)
    }
}
impl core::fmt::Debug for Adcpwrdly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcpwrdly")
            .field("adcpwr0", &self.adcpwr0())
            .field("adcpwr1", &self.adcpwr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcpwrdly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcpwrdly {{ adcpwr0: {=u8:?}, adcpwr1: {=u8:?} }}",
            self.adcpwr0(),
            self.adcpwr1()
        )
    }
}
#[doc = "ADC Referece Keeper and Comparator Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcrefcomp(pub u32);
impl Adcrefcomp {
    #[doc = "Output of the ADC reference comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_refcomp_out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output of the ADC reference comparator."]
    #[inline(always)]
    pub const fn set_adc_refcomp_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC Reference Keeper Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn adcrefkeeptrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC Reference Keeper Trim."]
    #[inline(always)]
    pub const fn set_adcrefkeeptrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "ADC Reference comparator power down."]
    #[must_use]
    #[inline(always)]
    pub const fn adcrfcmpen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Reference comparator power down."]
    #[inline(always)]
    pub const fn set_adcrfcmpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Adcrefcomp {
    #[inline(always)]
    fn default() -> Adcrefcomp {
        Adcrefcomp(0)
    }
}
impl core::fmt::Debug for Adcrefcomp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcrefcomp")
            .field("adc_refcomp_out", &self.adc_refcomp_out())
            .field("adcrefkeeptrim", &self.adcrefkeeptrim())
            .field("adcrfcmpen", &self.adcrfcmpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcrefcomp {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Adcrefcomp {{ adc_refcomp_out: {=bool:?}, adcrefkeeptrim: {=u8:?}, adcrfcmpen: {=bool:?} }}" , self . adc_refcomp_out () , self . adcrefkeeptrim () , self . adcrfcmpen ())
    }
}
#[doc = "ADC Trims."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adctrim(pub u32);
impl Adctrim {
    #[doc = "ADC Reference Ibias trim."]
    #[must_use]
    #[inline(always)]
    pub const fn adcrefkeepibtrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ADC Reference Ibias trim."]
    #[inline(always)]
    pub const fn set_adcrefkeepibtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "ADC Reference buffer trim."]
    #[must_use]
    #[inline(always)]
    pub const fn adcrefbuftrim(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC Reference buffer trim."]
    #[inline(always)]
    pub const fn set_adcrefbuftrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "ADC reference buffer input bias trim."]
    #[must_use]
    #[inline(always)]
    pub const fn adcrfbufibtrim(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "ADC reference buffer input bias trim."]
    #[inline(always)]
    pub const fn set_adcrfbufibtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
}
impl Default for Adctrim {
    #[inline(always)]
    fn default() -> Adctrim {
        Adctrim(0)
    }
}
impl core::fmt::Debug for Adctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adctrim")
            .field("adcrefkeepibtrim", &self.adcrefkeepibtrim())
            .field("adcrefbuftrim", &self.adcrefbuftrim())
            .field("adcrfbufibtrim", &self.adcrfbufibtrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Adctrim {{ adcrefkeepibtrim: {=u8:?}, adcrefbuftrim: {=u8:?}, adcrfbufibtrim: {=u8:?} }}" , self . adcrefkeepibtrim () , self . adcrefbuftrim () , self . adcrfbufibtrim ())
    }
}
#[doc = "DMA Control Register. Determines misc settings for DMA operation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apbdmactrl(pub u32);
impl Apbdmactrl {
    #[doc = "Enable the DMA controller. When disabled, DMA requests will be ignored by the controller."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the DMA controller. When disabled, DMA requests will be ignored by the controller."]
    #[inline(always)]
    pub const fn set_dma_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
    #[must_use]
    #[inline(always)]
    pub const fn decodeabort(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
    #[inline(always)]
    pub const fn set_decodeabort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This field determines how long the DMA will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms."]
    #[must_use]
    #[inline(always)]
    pub const fn hysteresis(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "This field determines how long the DMA will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms."]
    #[inline(always)]
    pub const fn set_hysteresis(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Apbdmactrl {
    #[inline(always)]
    fn default() -> Apbdmactrl {
        Apbdmactrl(0)
    }
}
impl core::fmt::Debug for Apbdmactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apbdmactrl")
            .field("dma_enable", &self.dma_enable())
            .field("decodeabort", &self.decodeabort())
            .field("hysteresis", &self.hysteresis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apbdmactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apbdmactrl {{ dma_enable: {=bool:?}, decodeabort: {=bool:?}, hysteresis: {=u8:?} }}",
            self.dma_enable(),
            self.decodeabort(),
            self.hysteresis()
        )
    }
}
#[doc = "BLEBUCK2 Control Reg."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blebuck2(pub u32);
impl Blebuck2 {
    #[doc = "blebuck_ton_low_trim."]
    #[must_use]
    #[inline(always)]
    pub const fn blebucktonlowtrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "blebuck_ton_low_trim."]
    #[inline(always)]
    pub const fn set_blebucktonlowtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "blebuck_ton_hi_trim."]
    #[must_use]
    #[inline(always)]
    pub const fn blebucktonhitrim(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "blebuck_ton_hi_trim."]
    #[inline(always)]
    pub const fn set_blebucktonhitrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "blebuck_ton_trim."]
    #[must_use]
    #[inline(always)]
    pub const fn blebucktond2atrim(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "blebuck_ton_trim."]
    #[inline(always)]
    pub const fn set_blebucktond2atrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
}
impl Default for Blebuck2 {
    #[inline(always)]
    fn default() -> Blebuck2 {
        Blebuck2(0)
    }
}
impl core::fmt::Debug for Blebuck2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blebuck2")
            .field("blebucktonlowtrim", &self.blebucktonlowtrim())
            .field("blebucktonhitrim", &self.blebucktonhitrim())
            .field("blebucktond2atrim", &self.blebucktond2atrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blebuck2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blebuck2 {{ blebucktonlowtrim: {=u8:?}, blebucktonhitrim: {=u8:?}, blebucktond2atrim: {=u8:?} }}" , self . blebucktonlowtrim () , self . blebucktonhitrim () , self . blebucktond2atrim ())
    }
}
#[doc = "BOD control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bodctrl(pub u32);
impl Bodctrl {
    #[doc = "BODL Power Down."]
    #[must_use]
    #[inline(always)]
    pub const fn bodlpwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BODL Power Down."]
    #[inline(always)]
    pub const fn set_bodlpwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BODH Power Down."]
    #[must_use]
    #[inline(always)]
    pub const fn bodhpwd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BODH Power Down."]
    #[inline(always)]
    pub const fn set_bodhpwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BODC Power Down."]
    #[must_use]
    #[inline(always)]
    pub const fn bodcpwd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BODC Power Down."]
    #[inline(always)]
    pub const fn set_bodcpwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BODF Power Down."]
    #[must_use]
    #[inline(always)]
    pub const fn bodfpwd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BODF Power Down."]
    #[inline(always)]
    pub const fn set_bodfpwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[must_use]
    #[inline(always)]
    pub const fn bodlvrefsel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "BODL External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub const fn set_bodlvrefsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[must_use]
    #[inline(always)]
    pub const fn bodhvrefsel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BODH External Reference Select. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub const fn set_bodhvrefsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Bodctrl {
    #[inline(always)]
    fn default() -> Bodctrl {
        Bodctrl(0)
    }
}
impl core::fmt::Debug for Bodctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bodctrl")
            .field("bodlpwd", &self.bodlpwd())
            .field("bodhpwd", &self.bodhpwd())
            .field("bodcpwd", &self.bodcpwd())
            .field("bodfpwd", &self.bodfpwd())
            .field("bodlvrefsel", &self.bodlvrefsel())
            .field("bodhvrefsel", &self.bodhvrefsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bodctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bodctrl {{ bodlpwd: {=bool:?}, bodhpwd: {=bool:?}, bodcpwd: {=bool:?}, bodfpwd: {=bool:?}, bodlvrefsel: {=bool:?}, bodhvrefsel: {=bool:?} }}" , self . bodlpwd () , self . bodhpwd () , self . bodcpwd () , self . bodfpwd () , self . bodlvrefsel () , self . bodhvrefsel ())
    }
}
#[doc = "Bootloader and secure boot functions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootloader(pub u32);
impl Bootloader {
    #[doc = "Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn bootloaderlow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear."]
    #[inline(always)]
    pub const fn set_bootloaderlow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure boot lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
    #[must_use]
    #[inline(always)]
    pub const fn sblock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure boot lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
    #[inline(always)]
    pub const fn set_sblock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Flash protection lock. Always resets to 1, write 1 to clear. Enables writes to flash protection register set."]
    #[must_use]
    #[inline(always)]
    pub const fn protlock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Flash protection lock. Always resets to 1, write 1 to clear. Enables writes to flash protection register set."]
    #[inline(always)]
    pub const fn set_protlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates whether the secure boot feature is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn secbootfeature(&self) -> super::vals::Secbootfeature {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Secbootfeature::from_bits(val as u8)
    }
    #[doc = "Indicates whether the secure boot feature is enabled."]
    #[inline(always)]
    pub const fn set_secbootfeature(&mut self, val: super::vals::Secbootfeature) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Indicates whether the secure boot on cold reset is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn secboot(&self) -> super::vals::Secboot {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Secboot::from_bits(val as u8)
    }
    #[doc = "Indicates whether the secure boot on cold reset is enabled."]
    #[inline(always)]
    pub const fn set_secboot(&mut self, val: super::vals::Secboot) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Indicates whether the secure boot on warm reset is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn secbootonrst(&self) -> super::vals::Secbootonrst {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Secbootonrst::from_bits(val as u8)
    }
    #[doc = "Indicates whether the secure boot on warm reset is enabled."]
    #[inline(always)]
    pub const fn set_secbootonrst(&mut self, val: super::vals::Secbootonrst) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Bootloader {
    #[inline(always)]
    fn default() -> Bootloader {
        Bootloader(0)
    }
}
impl core::fmt::Debug for Bootloader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bootloader")
            .field("bootloaderlow", &self.bootloaderlow())
            .field("sblock", &self.sblock())
            .field("protlock", &self.protlock())
            .field("secbootfeature", &self.secbootfeature())
            .field("secboot", &self.secboot())
            .field("secbootonrst", &self.secbootonrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bootloader {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bootloader {{ bootloaderlow: {=bool:?}, sblock: {=bool:?}, protlock: {=bool:?}, secbootfeature: {:?}, secboot: {:?}, secbootonrst: {:?} }}" , self . bootloaderlow () , self . sblock () , self . protlock () , self . secbootfeature () , self . secboot () , self . secbootonrst ())
    }
}
#[doc = "Unique Chip ID 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid0(pub u32);
impl Chipid0 {
    #[doc = "Unique chip ID 0."]
    #[must_use]
    #[inline(always)]
    pub const fn chipid0(&self) -> super::vals::Chipid0 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Chipid0::from_bits(val as u32)
    }
    #[doc = "Unique chip ID 0."]
    #[inline(always)]
    pub const fn set_chipid0(&mut self, val: super::vals::Chipid0) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Chipid0 {
    #[inline(always)]
    fn default() -> Chipid0 {
        Chipid0(0)
    }
}
impl core::fmt::Debug for Chipid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chipid0")
            .field("chipid0", &self.chipid0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chipid0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Chipid0 {{ chipid0: {:?} }}", self.chipid0())
    }
}
#[doc = "Unique Chip ID 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid1(pub u32);
impl Chipid1 {
    #[doc = "Unique chip ID 1."]
    #[must_use]
    #[inline(always)]
    pub const fn chipid1(&self) -> super::vals::Chipid1 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Chipid1::from_bits(val as u32)
    }
    #[doc = "Unique chip ID 1."]
    #[inline(always)]
    pub const fn set_chipid1(&mut self, val: super::vals::Chipid1) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Chipid1 {
    #[inline(always)]
    fn default() -> Chipid1 {
        Chipid1(0)
    }
}
impl core::fmt::Debug for Chipid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chipid1")
            .field("chipid1", &self.chipid1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chipid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Chipid1 {{ chipid1: {:?} }}", self.chipid1())
    }
}
#[doc = "Chip Information Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chippn(pub u32);
impl Chippn {
    #[doc = "BCD part number."]
    #[must_use]
    #[inline(always)]
    pub const fn partnum(&self) -> super::vals::Partnum {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Partnum::from_bits(val as u32)
    }
    #[doc = "BCD part number."]
    #[inline(always)]
    pub const fn set_partnum(&mut self, val: super::vals::Partnum) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Chippn {
    #[inline(always)]
    fn default() -> Chippn {
        Chippn(0)
    }
}
impl core::fmt::Debug for Chippn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chippn")
            .field("partnum", &self.partnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chippn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Chippn {{ partnum: {:?} }}", self.partnum())
    }
}
#[doc = "Chip Revision."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chiprev(pub u32);
impl Chiprev {
    #[doc = "Minor Revision ID."]
    #[must_use]
    #[inline(always)]
    pub const fn revmin(&self) -> super::vals::Revmin {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Revmin::from_bits(val as u8)
    }
    #[doc = "Minor Revision ID."]
    #[inline(always)]
    pub const fn set_revmin(&mut self, val: super::vals::Revmin) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Major Revision ID."]
    #[must_use]
    #[inline(always)]
    pub const fn revmaj(&self) -> super::vals::Revmaj {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Revmaj::from_bits(val as u8)
    }
    #[doc = "Major Revision ID."]
    #[inline(always)]
    pub const fn set_revmaj(&mut self, val: super::vals::Revmaj) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Silicon Part ID."]
    #[must_use]
    #[inline(always)]
    pub const fn sipart(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Silicon Part ID."]
    #[inline(always)]
    pub const fn set_sipart(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
}
impl Default for Chiprev {
    #[inline(always)]
    fn default() -> Chiprev {
        Chiprev(0)
    }
}
impl core::fmt::Debug for Chiprev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chiprev")
            .field("revmin", &self.revmin())
            .field("revmaj", &self.revmaj())
            .field("sipart", &self.sipart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chiprev {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Chiprev {{ revmin: {:?}, revmaj: {:?}, sipart: {=u16:?} }}",
            self.revmin(),
            self.revmaj(),
            self.sipart()
        )
    }
}
#[doc = "Debugger Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debugger(pub u32);
impl Debugger {
    #[doc = "Lockout of debugger (SWD)."]
    #[must_use]
    #[inline(always)]
    pub const fn lockout(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lockout of debugger (SWD)."]
    #[inline(always)]
    pub const fn set_lockout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Debugger {
    #[inline(always)]
    fn default() -> Debugger {
        Debugger(0)
    }
}
impl core::fmt::Debug for Debugger {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debugger")
            .field("lockout", &self.lockout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debugger {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Debugger {{ lockout: {=bool:?} }}", self.lockout())
    }
}
#[doc = "SRAM read-protection bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmasramreadprotect1(pub u32);
impl Dmasramreadprotect1 {
    #[doc = "Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_rprot1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub const fn set_dma_rprot1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dmasramreadprotect1 {
    #[inline(always)]
    fn default() -> Dmasramreadprotect1 {
        Dmasramreadprotect1(0)
    }
}
impl core::fmt::Debug for Dmasramreadprotect1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmasramreadprotect1")
            .field("dma_rprot1", &self.dma_rprot1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmasramreadprotect1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmasramreadprotect1 {{ dma_rprot1: {=u16:?} }}",
            self.dma_rprot1()
        )
    }
}
#[doc = "SRAM write-protection bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmasramwriteprotect1(pub u32);
impl Dmasramwriteprotect1 {
    #[doc = "Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_wprot1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA writes, when set to 0, DMA may write the region."]
    #[inline(always)]
    pub const fn set_dma_wprot1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dmasramwriteprotect1 {
    #[inline(always)]
    fn default() -> Dmasramwriteprotect1 {
        Dmasramwriteprotect1(0)
    }
}
impl core::fmt::Debug for Dmasramwriteprotect1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmasramwriteprotect1")
            .field("dma_wprot1", &self.dma_wprot1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmasramwriteprotect1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmasramwriteprotect1 {{ dma_wprot1: {=u16:?} }}",
            self.dma_wprot1()
        )
    }
}
#[doc = "Enable the fault capture registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Faultcaptureen(pub u32);
impl Faultcaptureen {
    #[doc = "Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
    #[must_use]
    #[inline(always)]
    pub const fn faultcaptureen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
    #[inline(always)]
    pub const fn set_faultcaptureen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Faultcaptureen {
    #[inline(always)]
    fn default() -> Faultcaptureen {
        Faultcaptureen(0)
    }
}
impl core::fmt::Debug for Faultcaptureen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Faultcaptureen")
            .field("faultcaptureen", &self.faultcaptureen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Faultcaptureen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Faultcaptureen {{ faultcaptureen: {=bool:?} }}",
            self.faultcaptureen()
        )
    }
}
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Faultstatus(pub u32);
impl Faultstatus {
    #[doc = "The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[must_use]
    #[inline(always)]
    pub const fn icodefault(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub const fn set_icodefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[must_use]
    #[inline(always)]
    pub const fn dcodefault(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub const fn set_dcodefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[must_use]
    #[inline(always)]
    pub const fn sysfault(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub const fn set_sysfault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Faultstatus {
    #[inline(always)]
    fn default() -> Faultstatus {
        Faultstatus(0)
    }
}
impl core::fmt::Debug for Faultstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Faultstatus")
            .field("icodefault", &self.icodefault())
            .field("dcodefault", &self.dcodefault())
            .field("sysfault", &self.sysfault())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Faultstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Faultstatus {{ icodefault: {=bool:?}, dcodefault: {=bool:?}, sysfault: {=bool:?} }}",
            self.icodefault(),
            self.dcodefault(),
            self.sysfault()
        )
    }
}
#[doc = "Feature Enable on Burst and BLE."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Featureenable(pub u32);
impl Featureenable {
    #[doc = "Controls the BLE functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn blereq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the BLE functionality."]
    #[inline(always)]
    pub const fn set_blereq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ACK for BLEREQ."]
    #[must_use]
    #[inline(always)]
    pub const fn bleack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ACK for BLEREQ."]
    #[inline(always)]
    pub const fn set_bleack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AVAILABILITY of the BLE functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn bleavail(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AVAILABILITY of the BLE functionality."]
    #[inline(always)]
    pub const fn set_bleavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Controls the Burst functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn burstreq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the Burst functionality."]
    #[inline(always)]
    pub const fn set_burstreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ACK for BURSTREQ."]
    #[must_use]
    #[inline(always)]
    pub const fn burstack(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ACK for BURSTREQ."]
    #[inline(always)]
    pub const fn set_burstack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Availability of Burst functionality."]
    #[must_use]
    #[inline(always)]
    pub const fn burstavail(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Availability of Burst functionality."]
    #[inline(always)]
    pub const fn set_burstavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Featureenable {
    #[inline(always)]
    fn default() -> Featureenable {
        Featureenable(0)
    }
}
impl core::fmt::Debug for Featureenable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Featureenable")
            .field("blereq", &self.blereq())
            .field("bleack", &self.bleack())
            .field("bleavail", &self.bleavail())
            .field("burstreq", &self.burstreq())
            .field("burstack", &self.burstack())
            .field("burstavail", &self.burstavail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Featureenable {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Featureenable {{ blereq: {=bool:?}, bleack: {=bool:?}, bleavail: {=bool:?}, burstreq: {=bool:?}, burstack: {=bool:?}, burstavail: {=bool:?} }}" , self . blereq () , self . bleack () , self . bleavail () , self . burstreq () , self . burstack () , self . burstavail ())
    }
}
#[doc = "Key Register to enable the use of external clock selects via the EXTCLKSEL reg."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kextclksel(pub u32);
impl Kextclksel {
    #[doc = "Key register value."]
    #[must_use]
    #[inline(always)]
    pub const fn kextclksel(&self) -> super::vals::Kextclksel {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Kextclksel::from_bits(val as u32)
    }
    #[doc = "Key register value."]
    #[inline(always)]
    pub const fn set_kextclksel(&mut self, val: super::vals::Kextclksel) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Kextclksel {
    #[inline(always)]
    fn default() -> Kextclksel {
        Kextclksel(0)
    }
}
impl core::fmt::Debug for Kextclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kextclksel")
            .field("kextclksel", &self.kextclksel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kextclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Kextclksel {{ kextclksel: {:?} }}", self.kextclksel())
    }
}
#[doc = "Miscellaneous control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miscctrl(pub u32);
impl Miscctrl {
    #[doc = "Reserved bits, always leave unchanged. The MISCCTRL register must be modified via atomic RMW, leaving this bitfield completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved_rw_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved bits, always leave unchanged. The MISCCTRL register must be modified via atomic RMW, leaving this bitfield completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub const fn set_reserved_rw_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "BLE reset signal."]
    #[must_use]
    #[inline(always)]
    pub const fn ble_resetn(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BLE reset signal."]
    #[inline(always)]
    pub const fn set_ble_resetn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Miscctrl {
    #[inline(always)]
    fn default() -> Miscctrl {
        Miscctrl(0)
    }
}
impl core::fmt::Debug for Miscctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Miscctrl")
            .field("reserved_rw_0", &self.reserved_rw_0())
            .field("ble_resetn", &self.ble_resetn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Miscctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Miscctrl {{ reserved_rw_0: {=u8:?}, ble_resetn: {=bool:?} }}",
            self.reserved_rw_0(),
            self.ble_resetn()
        )
    }
}
#[doc = "OTA (Over the Air) Update Pointer/Status. Reset only by POA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otapointer(pub u32);
impl Otapointer {
    #[doc = "Indicates that an OTA update is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn otavalid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that an OTA update is valid."]
    #[inline(always)]
    pub const fn set_otavalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates that the sbl_init has been updated."]
    #[must_use]
    #[inline(always)]
    pub const fn otasblupdate(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the sbl_init has been updated."]
    #[inline(always)]
    pub const fn set_otasblupdate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Flash page pointer with updated OTA image."]
    #[must_use]
    #[inline(always)]
    pub const fn otapointer(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Flash page pointer with updated OTA image."]
    #[inline(always)]
    pub const fn set_otapointer(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Otapointer {
    #[inline(always)]
    fn default() -> Otapointer {
        Otapointer(0)
    }
}
impl core::fmt::Debug for Otapointer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Otapointer")
            .field("otavalid", &self.otavalid())
            .field("otasblupdate", &self.otasblupdate())
            .field("otapointer", &self.otapointer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Otapointer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Otapointer {{ otavalid: {=bool:?}, otasblupdate: {=bool:?}, otapointer: {=u32:?} }}",
            self.otavalid(),
            self.otasblupdate(),
            self.otapointer()
        )
    }
}
#[doc = "Control bit to enable/disable the PMU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmuenable(pub u32);
impl Pmuenable {
    #[doc = "PMU Enable Control bit. When set, the MCU's PMU will place the MCU into the lowest power consuming Deep Sleep mode upon execution of a WFI instruction (dependent on the setting of the SLEEPDEEP bit in the ARM SCR register). When cleared, regardless of the requested sleep mode, the PMU will not enter the lowest power Deep Sleep mode, instead entering the Sleep mode."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PMU Enable Control bit. When set, the MCU's PMU will place the MCU into the lowest power consuming Deep Sleep mode upon execution of a WFI instruction (dependent on the setting of the SLEEPDEEP bit in the ARM SCR register). When cleared, regardless of the requested sleep mode, the PMU will not enter the lowest power Deep Sleep mode, instead entering the Sleep mode."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Pmuenable {
    #[inline(always)]
    fn default() -> Pmuenable {
        Pmuenable(0)
    }
}
impl core::fmt::Debug for Pmuenable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmuenable")
            .field("enable", &self.enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmuenable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmuenable {{ enable: {=bool:?} }}", self.enable())
    }
}
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shadowvalid(pub u32);
impl Shadowvalid {
    #[doc = "Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[must_use]
    #[inline(always)]
    pub const fn bldsleep(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline(always)]
    pub const fn set_bldsleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates whether info0 contains valid data."]
    #[must_use]
    #[inline(always)]
    pub const fn info0_valid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether info0 contains valid data."]
    #[inline(always)]
    pub const fn set_info0_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Shadowvalid {
    #[inline(always)]
    fn default() -> Shadowvalid {
        Shadowvalid(0)
    }
}
impl core::fmt::Debug for Shadowvalid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shadowvalid")
            .field("valid", &self.valid())
            .field("bldsleep", &self.bldsleep())
            .field("info0_valid", &self.info0_valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shadowvalid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shadowvalid {{ valid: {=bool:?}, bldsleep: {=bool:?}, info0_valid: {=bool:?} }}",
            self.valid(),
            self.bldsleep(),
            self.info0_valid()
        )
    }
}
#[doc = "SIMO Buck Control Reg1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simobuck4(pub u32);
impl Simobuck4 {
    #[doc = "simobuck_clkdiv_sel."]
    #[must_use]
    #[inline(always)]
    pub const fn simobuckclkdivsel(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "simobuck_clkdiv_sel."]
    #[inline(always)]
    pub const fn set_simobuckclkdivsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
}
impl Default for Simobuck4 {
    #[inline(always)]
    fn default() -> Simobuck4 {
        Simobuck4(0)
    }
}
impl core::fmt::Debug for Simobuck4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Simobuck4")
            .field("simobuckclkdivsel", &self.simobuckclkdivsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Simobuck4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Simobuck4 {{ simobuckclkdivsel: {=u8:?} }}",
            self.simobuckclkdivsel()
        )
    }
}
#[doc = "Unique Chip SKU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sku(pub u32);
impl Sku {
    #[doc = "Allow Burst feature."]
    #[must_use]
    #[inline(always)]
    pub const fn allowburst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Burst feature."]
    #[inline(always)]
    pub const fn set_allowburst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Allow BLE feature."]
    #[must_use]
    #[inline(always)]
    pub const fn allowble(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Allow BLE feature."]
    #[inline(always)]
    pub const fn set_allowble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure boot feature allowed."]
    #[must_use]
    #[inline(always)]
    pub const fn secboot(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Secure boot feature allowed."]
    #[inline(always)]
    pub const fn set_secboot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Sku {
    #[inline(always)]
    fn default() -> Sku {
        Sku(0)
    }
}
impl core::fmt::Debug for Sku {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sku")
            .field("allowburst", &self.allowburst())
            .field("allowble", &self.allowble())
            .field("secboot", &self.secboot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sku {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sku {{ allowburst: {=bool:?}, allowble: {=bool:?}, secboot: {=bool:?} }}",
            self.allowburst(),
            self.allowble(),
            self.secboot()
        )
    }
}
#[doc = "SRAM Controller mode bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srammode(pub u32);
impl Srammode {
    #[doc = "When set, instruction accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Generally, this mode bit should be set for improved performance when executing instructions from SRAM."]
    #[must_use]
    #[inline(always)]
    pub const fn iprefetch(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When set, instruction accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Generally, this mode bit should be set for improved performance when executing instructions from SRAM."]
    #[inline(always)]
    pub const fn set_iprefetch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secondary prefetch feature that will cache prefetched data across bus waitstates (requires IPREFETCH to be set)."]
    #[must_use]
    #[inline(always)]
    pub const fn iprefetch_cache(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary prefetch feature that will cache prefetched data across bus waitstates (requires IPREFETCH to be set)."]
    #[inline(always)]
    pub const fn set_iprefetch_cache(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When set, data bus accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Use of this mode bit is only recommended if the work flow has a large number of sequential accesses."]
    #[must_use]
    #[inline(always)]
    pub const fn dprefetch(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "When set, data bus accesses to the SRAM banks will be prefetched (normally 2 cycle read access). Use of this mode bit is only recommended if the work flow has a large number of sequential accesses."]
    #[inline(always)]
    pub const fn set_dprefetch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Secondary prefetch feature that will cache prefetched data across bus waitstates (requires DPREFETCH to be set)."]
    #[must_use]
    #[inline(always)]
    pub const fn dprefetch_cache(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary prefetch feature that will cache prefetched data across bus waitstates (requires DPREFETCH to be set)."]
    #[inline(always)]
    pub const fn set_dprefetch_cache(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Srammode {
    #[inline(always)]
    fn default() -> Srammode {
        Srammode(0)
    }
}
impl core::fmt::Debug for Srammode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srammode")
            .field("iprefetch", &self.iprefetch())
            .field("iprefetch_cache", &self.iprefetch_cache())
            .field("dprefetch", &self.dprefetch())
            .field("dprefetch_cache", &self.dprefetch_cache())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srammode {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Srammode {{ iprefetch: {=bool:?}, iprefetch_cache: {=bool:?}, dprefetch: {=bool:?}, dprefetch_cache: {=bool:?} }}" , self . iprefetch () , self . iprefetch_cache () , self . dprefetch () , self . dprefetch_cache ())
    }
}
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpiuctrl(pub u32);
impl Tpiuctrl {
    #[doc = "TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This field selects the frequency of the ARM M4 TPIU port."]
    #[must_use]
    #[inline(always)]
    pub const fn clksel(&self) -> super::vals::Clksel {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Clksel::from_bits(val as u8)
    }
    #[doc = "This field selects the frequency of the ARM M4 TPIU port."]
    #[inline(always)]
    pub const fn set_clksel(&mut self, val: super::vals::Clksel) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
}
impl Default for Tpiuctrl {
    #[inline(always)]
    fn default() -> Tpiuctrl {
        Tpiuctrl(0)
    }
}
impl core::fmt::Debug for Tpiuctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tpiuctrl")
            .field("enable", &self.enable())
            .field("clksel", &self.clksel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tpiuctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tpiuctrl {{ enable: {=bool:?}, clksel: {:?} }}",
            self.enable(),
            self.clksel()
        )
    }
}
#[doc = "Unique Vendor ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vendorid(pub u32);
impl Vendorid {
    #[doc = "Unique Vendor ID."]
    #[must_use]
    #[inline(always)]
    pub const fn vendorid(&self) -> super::vals::Vendorid {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Vendorid::from_bits(val as u32)
    }
    #[doc = "Unique Vendor ID."]
    #[inline(always)]
    pub const fn set_vendorid(&mut self, val: super::vals::Vendorid) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Vendorid {
    #[inline(always)]
    fn default() -> Vendorid {
        Vendorid(0)
    }
}
impl core::fmt::Debug for Vendorid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vendorid")
            .field("vendorid", &self.vendorid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vendorid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Vendorid {{ vendorid: {:?} }}", self.vendorid())
    }
}
#[doc = "XTAL Oscillator Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtalctrl(pub u32);
impl Xtalctrl {
    #[doc = "XTAL Software Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn xtalswe(&self) -> super::vals::Xtalswe {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xtalswe::from_bits(val as u8)
    }
    #[doc = "XTAL Software Override Enable."]
    #[inline(always)]
    pub const fn set_xtalswe(&mut self, val: super::vals::Xtalswe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "XTAL Oscillator Disable Feedback."]
    #[must_use]
    #[inline(always)]
    pub const fn fdbkdsblxtal(&self) -> super::vals::Fdbkdsblxtal {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fdbkdsblxtal::from_bits(val as u8)
    }
    #[doc = "XTAL Oscillator Disable Feedback."]
    #[inline(always)]
    pub const fn set_fdbkdsblxtal(&mut self, val: super::vals::Fdbkdsblxtal) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "XTAL Oscillator Bypass Comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn bypcmprxtal(&self) -> super::vals::Bypcmprxtal {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Bypcmprxtal::from_bits(val as u8)
    }
    #[doc = "XTAL Oscillator Bypass Comparator."]
    #[inline(always)]
    pub const fn set_bypcmprxtal(&mut self, val: super::vals::Bypcmprxtal) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "XTAL Oscillator Power Down Core."]
    #[must_use]
    #[inline(always)]
    pub const fn pdnbcorextal(&self) -> super::vals::Pdnbcorextal {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdnbcorextal::from_bits(val as u8)
    }
    #[doc = "XTAL Oscillator Power Down Core."]
    #[inline(always)]
    pub const fn set_pdnbcorextal(&mut self, val: super::vals::Pdnbcorextal) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "XTAL Oscillator Power Down Comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn pdnbcmprxtal(&self) -> super::vals::Pdnbcmprxtal {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdnbcmprxtal::from_bits(val as u8)
    }
    #[doc = "XTAL Oscillator Power Down Comparator."]
    #[inline(always)]
    pub const fn set_pdnbcmprxtal(&mut self, val: super::vals::Pdnbcmprxtal) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "XTAL Power down on brown out."]
    #[must_use]
    #[inline(always)]
    pub const fn pwdbodxtal(&self) -> super::vals::Pwdbodxtal {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pwdbodxtal::from_bits(val as u8)
    }
    #[doc = "XTAL Power down on brown out."]
    #[inline(always)]
    pub const fn set_pwdbodxtal(&mut self, val: super::vals::Pwdbodxtal) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "XTAL IBUFF trim."]
    #[must_use]
    #[inline(always)]
    pub const fn xtalibuftrim(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "XTAL IBUFF trim."]
    #[inline(always)]
    pub const fn set_xtalibuftrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "XTAL ICOMP trim."]
    #[must_use]
    #[inline(always)]
    pub const fn xtalicomptrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "XTAL ICOMP trim."]
    #[inline(always)]
    pub const fn set_xtalicomptrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for Xtalctrl {
    #[inline(always)]
    fn default() -> Xtalctrl {
        Xtalctrl(0)
    }
}
impl core::fmt::Debug for Xtalctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xtalctrl")
            .field("xtalswe", &self.xtalswe())
            .field("fdbkdsblxtal", &self.fdbkdsblxtal())
            .field("bypcmprxtal", &self.bypcmprxtal())
            .field("pdnbcorextal", &self.pdnbcorextal())
            .field("pdnbcmprxtal", &self.pdnbcmprxtal())
            .field("pwdbodxtal", &self.pwdbodxtal())
            .field("xtalibuftrim", &self.xtalibuftrim())
            .field("xtalicomptrim", &self.xtalicomptrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xtalctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Xtalctrl {{ xtalswe: {:?}, fdbkdsblxtal: {:?}, bypcmprxtal: {:?}, pdnbcorextal: {:?}, pdnbcmprxtal: {:?}, pwdbodxtal: {:?}, xtalibuftrim: {=u8:?}, xtalicomptrim: {=u8:?} }}" , self . xtalswe () , self . fdbkdsblxtal () , self . bypcmprxtal () , self . pdnbcorextal () , self . pdnbcmprxtal () , self . pwdbodxtal () , self . xtalibuftrim () , self . xtalicomptrim ())
    }
}
#[doc = "XTAL Oscillator General Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtalgenctrl(pub u32);
impl Xtalgenctrl {
    #[doc = "Auto-calibration delay control."]
    #[must_use]
    #[inline(always)]
    pub const fn acwarmup(&self) -> super::vals::Acwarmup {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Acwarmup::from_bits(val as u8)
    }
    #[doc = "Auto-calibration delay control."]
    #[inline(always)]
    pub const fn set_acwarmup(&mut self, val: super::vals::Acwarmup) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "XTAL BIAS trim."]
    #[must_use]
    #[inline(always)]
    pub const fn xtalbiastrim(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "XTAL BIAS trim."]
    #[inline(always)]
    pub const fn set_xtalbiastrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
    #[must_use]
    #[inline(always)]
    pub const fn xtalksbiastrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
    #[inline(always)]
    pub const fn set_xtalksbiastrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Xtalgenctrl {
    #[inline(always)]
    fn default() -> Xtalgenctrl {
        Xtalgenctrl(0)
    }
}
impl core::fmt::Debug for Xtalgenctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xtalgenctrl")
            .field("acwarmup", &self.acwarmup())
            .field("xtalbiastrim", &self.xtalbiastrim())
            .field("xtalksbiastrim", &self.xtalksbiastrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xtalgenctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Xtalgenctrl {{ acwarmup: {:?}, xtalbiastrim: {=u8:?}, xtalksbiastrim: {=u8:?} }}",
            self.acwarmup(),
            self.xtalbiastrim(),
            self.xtalksbiastrim()
        )
    }
}

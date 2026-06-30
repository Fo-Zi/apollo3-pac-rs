#[doc = "Power Status Register for ADC Block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcstatus(pub u32);
impl Adcstatus {
    #[doc = "This bit indicates that the ADC is powered down."]
    #[must_use]
    #[inline(always)]
    pub const fn adcpwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the ADC is powered down."]
    #[inline(always)]
    pub const fn set_adcpwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit indicates that the ADC Band Gap is powered down."]
    #[must_use]
    #[inline(always)]
    pub const fn bgtpwd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the ADC Band Gap is powered down."]
    #[inline(always)]
    pub const fn set_bgtpwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the ADC temperature sensor input buffer is powered down."]
    #[must_use]
    #[inline(always)]
    pub const fn vptatpwd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the ADC temperature sensor input buffer is powered down."]
    #[inline(always)]
    pub const fn set_vptatpwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that the ADC VBAT resistor divider is powered down."]
    #[must_use]
    #[inline(always)]
    pub const fn vbatpwd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the ADC VBAT resistor divider is powered down."]
    #[inline(always)]
    pub const fn set_vbatpwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that the ADC REFKEEP is powered down."]
    #[must_use]
    #[inline(always)]
    pub const fn refkeeppwd(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the ADC REFKEEP is powered down."]
    #[inline(always)]
    pub const fn set_refkeeppwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that the ADC REFBUF is powered down."]
    #[must_use]
    #[inline(always)]
    pub const fn refbufpwd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the ADC REFBUF is powered down."]
    #[inline(always)]
    pub const fn set_refbufpwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Adcstatus {
    #[inline(always)]
    fn default() -> Adcstatus {
        Adcstatus(0)
    }
}
impl core::fmt::Debug for Adcstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcstatus")
            .field("adcpwd", &self.adcpwd())
            .field("bgtpwd", &self.bgtpwd())
            .field("vptatpwd", &self.vptatpwd())
            .field("vbatpwd", &self.vbatpwd())
            .field("refkeeppwd", &self.refkeeppwd())
            .field("refbufpwd", &self.refbufpwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Adcstatus {{ adcpwd: {=bool:?}, bgtpwd: {=bool:?}, vptatpwd: {=bool:?}, vbatpwd: {=bool:?}, refkeeppwd: {=bool:?}, refbufpwd: {=bool:?} }}" , self . adcpwd () , self . bgtpwd () , self . vptatpwd () , self . vbatpwd () , self . refkeeppwd () , self . refbufpwd ())
    }
}
#[doc = "Device Power Enables."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devpwren(pub u32);
impl Devpwren {
    #[doc = "Power up IO Slave."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrios(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power up IO Slave."]
    #[inline(always)]
    pub const fn set_pwrios(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power up IO Master 0."]
    #[must_use]
    #[inline(always)]
    pub const fn pwriom0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power up IO Master 0."]
    #[inline(always)]
    pub const fn set_pwriom0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Power up IO Master 1."]
    #[must_use]
    #[inline(always)]
    pub const fn pwriom1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Power up IO Master 1."]
    #[inline(always)]
    pub const fn set_pwriom1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Power up IO Master 2."]
    #[must_use]
    #[inline(always)]
    pub const fn pwriom2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Power up IO Master 2."]
    #[inline(always)]
    pub const fn set_pwriom2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Power up IO Master 3."]
    #[must_use]
    #[inline(always)]
    pub const fn pwriom3(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Power up IO Master 3."]
    #[inline(always)]
    pub const fn set_pwriom3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Power up IO Master 4."]
    #[must_use]
    #[inline(always)]
    pub const fn pwriom4(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Power up IO Master 4."]
    #[inline(always)]
    pub const fn set_pwriom4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Power up IO Master 5."]
    #[must_use]
    #[inline(always)]
    pub const fn pwriom5(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Power up IO Master 5."]
    #[inline(always)]
    pub const fn set_pwriom5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Power up UART Controller 0."]
    #[must_use]
    #[inline(always)]
    pub const fn pwruart0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Power up UART Controller 0."]
    #[inline(always)]
    pub const fn set_pwruart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Power up UART Controller 1."]
    #[must_use]
    #[inline(always)]
    pub const fn pwruart1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Power up UART Controller 1."]
    #[inline(always)]
    pub const fn set_pwruart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Power up ADC Digital Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn pwradc(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Power up ADC Digital Controller."]
    #[inline(always)]
    pub const fn set_pwradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Power up SCARD Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrscard(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power up SCARD Controller."]
    #[inline(always)]
    pub const fn set_pwrscard(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power up MSPI Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrmspi(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power up MSPI Controller."]
    #[inline(always)]
    pub const fn set_pwrmspi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Power up PDM block."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrpdm(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power up PDM block."]
    #[inline(always)]
    pub const fn set_pwrpdm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Power up BLE controller."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrblel(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Power up BLE controller."]
    #[inline(always)]
    pub const fn set_pwrblel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Devpwren {
    #[inline(always)]
    fn default() -> Devpwren {
        Devpwren(0)
    }
}
impl core::fmt::Debug for Devpwren {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Devpwren")
            .field("pwrios", &self.pwrios())
            .field("pwriom0", &self.pwriom0())
            .field("pwriom1", &self.pwriom1())
            .field("pwriom2", &self.pwriom2())
            .field("pwriom3", &self.pwriom3())
            .field("pwriom4", &self.pwriom4())
            .field("pwriom5", &self.pwriom5())
            .field("pwruart0", &self.pwruart0())
            .field("pwruart1", &self.pwruart1())
            .field("pwradc", &self.pwradc())
            .field("pwrscard", &self.pwrscard())
            .field("pwrmspi", &self.pwrmspi())
            .field("pwrpdm", &self.pwrpdm())
            .field("pwrblel", &self.pwrblel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Devpwren {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Devpwren {{ pwrios: {=bool:?}, pwriom0: {=bool:?}, pwriom1: {=bool:?}, pwriom2: {=bool:?}, pwriom3: {=bool:?}, pwriom4: {=bool:?}, pwriom5: {=bool:?}, pwruart0: {=bool:?}, pwruart1: {=bool:?}, pwradc: {=bool:?}, pwrscard: {=bool:?}, pwrmspi: {=bool:?}, pwrpdm: {=bool:?}, pwrblel: {=bool:?} }}" , self . pwrios () , self . pwriom0 () , self . pwriom1 () , self . pwriom2 () , self . pwriom3 () , self . pwriom4 () , self . pwriom5 () , self . pwruart0 () , self . pwruart1 () , self . pwradc () , self . pwrscard () , self . pwrmspi () , self . pwrpdm () , self . pwrblel ())
    }
}
#[doc = "Event enable register to control which DEVPWRSTATUS bits are routed to event input of CPU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devpwreventen(pub u32);
impl Devpwreventen {
    #[doc = "Control MCUL power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn mculeven(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control MCUL power-on status event."]
    #[inline(always)]
    pub const fn set_mculeven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control MCUH power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn mcuheven(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control MCUH power-on status event."]
    #[inline(always)]
    pub const fn set_mcuheven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control HCPA power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn hcpaeven(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control HCPA power-on status event."]
    #[inline(always)]
    pub const fn set_hcpaeven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control HCPB power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn hcpbeven(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control HCPB power-on status event."]
    #[inline(always)]
    pub const fn set_hcpbeven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control HCPC power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn hcpceven(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Control HCPC power-on status event."]
    #[inline(always)]
    pub const fn set_hcpceven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Control ADC power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn adceven(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Control ADC power-on status event."]
    #[inline(always)]
    pub const fn set_adceven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Control MSPI power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn mspieven(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Control MSPI power-on status event."]
    #[inline(always)]
    pub const fn set_mspieven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Control PDM power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn pdmeven(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Control PDM power-on status event."]
    #[inline(always)]
    pub const fn set_pdmeven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Control BLE power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn bleleven(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Control BLE power-on status event."]
    #[inline(always)]
    pub const fn set_bleleven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Control BLEFEATURE status event."]
    #[must_use]
    #[inline(always)]
    pub const fn blefeatureeven(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Control BLEFEATURE status event."]
    #[inline(always)]
    pub const fn set_blefeatureeven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Control BURSTFEATURE status event."]
    #[must_use]
    #[inline(always)]
    pub const fn burstfeatureeven(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Control BURSTFEATURE status event."]
    #[inline(always)]
    pub const fn set_burstfeatureeven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Control BURST status event."]
    #[must_use]
    #[inline(always)]
    pub const fn bursteven(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Control BURST status event."]
    #[inline(always)]
    pub const fn set_bursteven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Devpwreventen {
    #[inline(always)]
    fn default() -> Devpwreventen {
        Devpwreventen(0)
    }
}
impl core::fmt::Debug for Devpwreventen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Devpwreventen")
            .field("mculeven", &self.mculeven())
            .field("mcuheven", &self.mcuheven())
            .field("hcpaeven", &self.hcpaeven())
            .field("hcpbeven", &self.hcpbeven())
            .field("hcpceven", &self.hcpceven())
            .field("adceven", &self.adceven())
            .field("mspieven", &self.mspieven())
            .field("pdmeven", &self.pdmeven())
            .field("bleleven", &self.bleleven())
            .field("blefeatureeven", &self.blefeatureeven())
            .field("burstfeatureeven", &self.burstfeatureeven())
            .field("bursteven", &self.bursteven())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Devpwreventen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Devpwreventen {{ mculeven: {=bool:?}, mcuheven: {=bool:?}, hcpaeven: {=bool:?}, hcpbeven: {=bool:?}, hcpceven: {=bool:?}, adceven: {=bool:?}, mspieven: {=bool:?}, pdmeven: {=bool:?}, bleleven: {=bool:?}, blefeatureeven: {=bool:?}, burstfeatureeven: {=bool:?}, bursteven: {=bool:?} }}" , self . mculeven () , self . mcuheven () , self . hcpaeven () , self . hcpbeven () , self . hcpceven () , self . adceven () , self . mspieven () , self . pdmeven () , self . bleleven () , self . blefeatureeven () , self . burstfeatureeven () , self . bursteven ())
    }
}
#[doc = "Device Power ON Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devpwrstatus(pub u32);
impl Devpwrstatus {
    #[doc = "This bit is 1 if power is supplied to MCUL."]
    #[must_use]
    #[inline(always)]
    pub const fn mcul(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to MCUL."]
    #[inline(always)]
    pub const fn set_mcul(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit is 1 if power is supplied to MCUH."]
    #[must_use]
    #[inline(always)]
    pub const fn mcuh(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to MCUH."]
    #[inline(always)]
    pub const fn set_mcuh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit is 1 if power is supplied to HCPA domain (IO SLAVE, UART0, UART1, SCARD)."]
    #[must_use]
    #[inline(always)]
    pub const fn hcpa(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to HCPA domain (IO SLAVE, UART0, UART1, SCARD)."]
    #[inline(always)]
    pub const fn set_hcpa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit is 1 if power is supplied to HCPB domain (IO MASTER 0, 1, 2)."]
    #[must_use]
    #[inline(always)]
    pub const fn hcpb(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to HCPB domain (IO MASTER 0, 1, 2)."]
    #[inline(always)]
    pub const fn set_hcpb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit is 1 if power is supplied to HCPC domain (IO MASTER4, 5, 6)."]
    #[must_use]
    #[inline(always)]
    pub const fn hcpc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to HCPC domain (IO MASTER4, 5, 6)."]
    #[inline(always)]
    pub const fn set_hcpc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit is 1 if power is supplied to ADC."]
    #[must_use]
    #[inline(always)]
    pub const fn pwradc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to ADC."]
    #[inline(always)]
    pub const fn set_pwradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit is 1 if power is supplied to MSPI."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrmspi(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to MSPI."]
    #[inline(always)]
    pub const fn set_pwrmspi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit is 1 if power is supplied to PDM."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrpdm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to PDM."]
    #[inline(always)]
    pub const fn set_pwrpdm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit is 1 if power is supplied to BLEL."]
    #[must_use]
    #[inline(always)]
    pub const fn blel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to BLEL."]
    #[inline(always)]
    pub const fn set_blel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit is 1 if power is supplied to BLEH."]
    #[must_use]
    #[inline(always)]
    pub const fn bleh(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to BLEH."]
    #[inline(always)]
    pub const fn set_bleh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Devpwrstatus {
    #[inline(always)]
    fn default() -> Devpwrstatus {
        Devpwrstatus(0)
    }
}
impl core::fmt::Debug for Devpwrstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Devpwrstatus")
            .field("mcul", &self.mcul())
            .field("mcuh", &self.mcuh())
            .field("hcpa", &self.hcpa())
            .field("hcpb", &self.hcpb())
            .field("hcpc", &self.hcpc())
            .field("pwradc", &self.pwradc())
            .field("pwrmspi", &self.pwrmspi())
            .field("pwrpdm", &self.pwrpdm())
            .field("blel", &self.blel())
            .field("bleh", &self.bleh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Devpwrstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Devpwrstatus {{ mcul: {=bool:?}, mcuh: {=bool:?}, hcpa: {=bool:?}, hcpb: {=bool:?}, hcpc: {=bool:?}, pwradc: {=bool:?}, pwrmspi: {=bool:?}, pwrpdm: {=bool:?}, blel: {=bool:?}, bleh: {=bool:?} }}" , self . mcul () , self . mcuh () , self . hcpa () , self . hcpb () , self . hcpc () , self . pwradc () , self . pwrmspi () , self . pwrpdm () , self . blel () , self . bleh ())
    }
}
#[doc = "Powerdown SRAM banks in Deep Sleep mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mempwdinsleep(pub u32);
impl Mempwdinsleep {
    #[doc = "power down DTCM in deep sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcmpwdslp(&self) -> super::vals::Dtcmpwdslp {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Dtcmpwdslp::from_bits(val as u8)
    }
    #[doc = "power down DTCM in deep sleep."]
    #[inline(always)]
    pub const fn set_dtcmpwdslp(&mut self, val: super::vals::Dtcmpwdslp) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[must_use]
    #[inline(always)]
    pub const fn srampwdslp(&self) -> super::vals::Srampwdslp {
        let val = (self.0 >> 3usize) & 0x03ff;
        super::vals::Srampwdslp::from_bits(val as u16)
    }
    #[doc = "Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub const fn set_srampwdslp(&mut self, val: super::vals::Srampwdslp) {
        self.0 = (self.0 & !(0x03ff << 3usize)) | (((val.to_bits() as u32) & 0x03ff) << 3usize);
    }
    #[doc = "Powerdown flash0 in deep sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn flash0pwdslp(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Powerdown flash0 in deep sleep."]
    #[inline(always)]
    pub const fn set_flash0pwdslp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Powerdown flash1 in deep sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn flash1pwdslp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Powerdown flash1 in deep sleep."]
    #[inline(always)]
    pub const fn set_flash1pwdslp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "power down cache in deep sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn cachepwdslp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "power down cache in deep sleep."]
    #[inline(always)]
    pub const fn set_cachepwdslp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mempwdinsleep {
    #[inline(always)]
    fn default() -> Mempwdinsleep {
        Mempwdinsleep(0)
    }
}
impl core::fmt::Debug for Mempwdinsleep {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mempwdinsleep")
            .field("dtcmpwdslp", &self.dtcmpwdslp())
            .field("srampwdslp", &self.srampwdslp())
            .field("flash0pwdslp", &self.flash0pwdslp())
            .field("flash1pwdslp", &self.flash1pwdslp())
            .field("cachepwdslp", &self.cachepwdslp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mempwdinsleep {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mempwdinsleep {{ dtcmpwdslp: {:?}, srampwdslp: {:?}, flash0pwdslp: {=bool:?}, flash1pwdslp: {=bool:?}, cachepwdslp: {=bool:?} }}" , self . dtcmpwdslp () , self . srampwdslp () , self . flash0pwdslp () , self . flash1pwdslp () , self . cachepwdslp ())
    }
}
#[doc = "Enables individual banks of the MEMORY array."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mempwren(pub u32);
impl Mempwren {
    #[doc = "Power up DTCM."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcm(&self) -> super::vals::Dtcm {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Dtcm::from_bits(val as u8)
    }
    #[doc = "Power up DTCM."]
    #[inline(always)]
    pub const fn set_dtcm(&mut self, val: super::vals::Dtcm) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Power up SRAM groups."]
    #[must_use]
    #[inline(always)]
    pub const fn sram(&self) -> super::vals::Sram {
        let val = (self.0 >> 3usize) & 0x03ff;
        super::vals::Sram::from_bits(val as u16)
    }
    #[doc = "Power up SRAM groups."]
    #[inline(always)]
    pub const fn set_sram(&mut self, val: super::vals::Sram) {
        self.0 = (self.0 & !(0x03ff << 3usize)) | (((val.to_bits() as u32) & 0x03ff) << 3usize);
    }
    #[doc = "Power up Flash0."]
    #[must_use]
    #[inline(always)]
    pub const fn flash0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Power up Flash0."]
    #[inline(always)]
    pub const fn set_flash0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Power up Flash1."]
    #[must_use]
    #[inline(always)]
    pub const fn flash1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Power up Flash1."]
    #[inline(always)]
    pub const fn set_flash1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
    #[must_use]
    #[inline(always)]
    pub const fn cacheb0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub const fn set_cacheb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
    #[must_use]
    #[inline(always)]
    pub const fn cacheb2(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub const fn set_cacheb2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mempwren {
    #[inline(always)]
    fn default() -> Mempwren {
        Mempwren(0)
    }
}
impl core::fmt::Debug for Mempwren {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mempwren")
            .field("dtcm", &self.dtcm())
            .field("sram", &self.sram())
            .field("flash0", &self.flash0())
            .field("flash1", &self.flash1())
            .field("cacheb0", &self.cacheb0())
            .field("cacheb2", &self.cacheb2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mempwren {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mempwren {{ dtcm: {:?}, sram: {:?}, flash0: {=bool:?}, flash1: {=bool:?}, cacheb0: {=bool:?}, cacheb2: {=bool:?} }}" , self . dtcm () , self . sram () , self . flash0 () , self . flash1 () , self . cacheb0 () , self . cacheb2 ())
    }
}
#[doc = "Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mempwreventen(pub u32);
impl Mempwreventen {
    #[doc = "Enable DTCM power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcmen(&self) -> super::vals::Dtcmen {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Dtcmen::from_bits(val as u8)
    }
    #[doc = "Enable DTCM power-on status event."]
    #[inline(always)]
    pub const fn set_dtcmen(&mut self, val: super::vals::Dtcmen) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Control SRAM power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn sramen(&self) -> super::vals::Sramen {
        let val = (self.0 >> 3usize) & 0x03ff;
        super::vals::Sramen::from_bits(val as u16)
    }
    #[doc = "Control SRAM power-on status event."]
    #[inline(always)]
    pub const fn set_sramen(&mut self, val: super::vals::Sramen) {
        self.0 = (self.0 & !(0x03ff << 3usize)) | (((val.to_bits() as u32) & 0x03ff) << 3usize);
    }
    #[doc = "Control Flash power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn flash0en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Control Flash power-on status event."]
    #[inline(always)]
    pub const fn set_flash0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Control Flash power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn flash1en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Control Flash power-on status event."]
    #[inline(always)]
    pub const fn set_flash1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Control CACHE BANK 0 power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn cacheb0en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Control CACHE BANK 0 power-on status event."]
    #[inline(always)]
    pub const fn set_cacheb0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Control CACHEB2 power-on status event."]
    #[must_use]
    #[inline(always)]
    pub const fn cacheb2en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Control CACHEB2 power-on status event."]
    #[inline(always)]
    pub const fn set_cacheb2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mempwreventen {
    #[inline(always)]
    fn default() -> Mempwreventen {
        Mempwreventen(0)
    }
}
impl core::fmt::Debug for Mempwreventen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mempwreventen")
            .field("dtcmen", &self.dtcmen())
            .field("sramen", &self.sramen())
            .field("flash0en", &self.flash0en())
            .field("flash1en", &self.flash1en())
            .field("cacheb0en", &self.cacheb0en())
            .field("cacheb2en", &self.cacheb2en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mempwreventen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mempwreventen {{ dtcmen: {:?}, sramen: {:?}, flash0en: {=bool:?}, flash1en: {=bool:?}, cacheb0en: {=bool:?}, cacheb2en: {=bool:?} }}" , self . dtcmen () , self . sramen () , self . flash0en () , self . flash1en () , self . cacheb0en () , self . cacheb2en ())
    }
}
#[doc = "Mem Power ON Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mempwrstatus(pub u32);
impl Mempwrstatus {
    #[doc = "This bit is 1 if power is supplied to DTCM GROUP0_0."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcm00(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to DTCM GROUP0_0."]
    #[inline(always)]
    pub const fn set_dtcm00(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit is 1 if power is supplied to DTCM GROUP0_1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcm01(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to DTCM GROUP0_1."]
    #[inline(always)]
    pub const fn set_dtcm01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit is 1 if power is supplied to DTCM GROUP1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcm1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to DTCM GROUP1."]
    #[inline(always)]
    pub const fn set_dtcm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP0."]
    #[must_use]
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP0."]
    #[inline(always)]
    pub const fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP1."]
    #[must_use]
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP1."]
    #[inline(always)]
    pub const fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP2."]
    #[must_use]
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP2."]
    #[inline(always)]
    pub const fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP3."]
    #[must_use]
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP3."]
    #[inline(always)]
    pub const fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP4."]
    #[must_use]
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP4."]
    #[inline(always)]
    pub const fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP5."]
    #[must_use]
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP5."]
    #[inline(always)]
    pub const fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP6."]
    #[must_use]
    #[inline(always)]
    pub const fn sram6(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP6."]
    #[inline(always)]
    pub const fn set_sram6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP7."]
    #[must_use]
    #[inline(always)]
    pub const fn sram7(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP7."]
    #[inline(always)]
    pub const fn set_sram7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP8."]
    #[must_use]
    #[inline(always)]
    pub const fn sram8(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP8."]
    #[inline(always)]
    pub const fn set_sram8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP9."]
    #[must_use]
    #[inline(always)]
    pub const fn sram9(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to SRAM GROUP9."]
    #[inline(always)]
    pub const fn set_sram9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is 1 if power is supplied to FLASH 0."]
    #[must_use]
    #[inline(always)]
    pub const fn flash0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to FLASH 0."]
    #[inline(always)]
    pub const fn set_flash0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit is 1 if power is supplied to FLASH 1."]
    #[must_use]
    #[inline(always)]
    pub const fn flash1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to FLASH 1."]
    #[inline(always)]
    pub const fn set_flash1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit is 1 if power is supplied to Cache Bank 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cacheb0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to Cache Bank 0."]
    #[inline(always)]
    pub const fn set_cacheb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bit is 1 if power is supplied to Cache Bank 2."]
    #[must_use]
    #[inline(always)]
    pub const fn cacheb2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if power is supplied to Cache Bank 2."]
    #[inline(always)]
    pub const fn set_cacheb2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Mempwrstatus {
    #[inline(always)]
    fn default() -> Mempwrstatus {
        Mempwrstatus(0)
    }
}
impl core::fmt::Debug for Mempwrstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mempwrstatus")
            .field("dtcm00", &self.dtcm00())
            .field("dtcm01", &self.dtcm01())
            .field("dtcm1", &self.dtcm1())
            .field("sram0", &self.sram0())
            .field("sram1", &self.sram1())
            .field("sram2", &self.sram2())
            .field("sram3", &self.sram3())
            .field("sram4", &self.sram4())
            .field("sram5", &self.sram5())
            .field("sram6", &self.sram6())
            .field("sram7", &self.sram7())
            .field("sram8", &self.sram8())
            .field("sram9", &self.sram9())
            .field("flash0", &self.flash0())
            .field("flash1", &self.flash1())
            .field("cacheb0", &self.cacheb0())
            .field("cacheb2", &self.cacheb2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mempwrstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mempwrstatus {{ dtcm00: {=bool:?}, dtcm01: {=bool:?}, dtcm1: {=bool:?}, sram0: {=bool:?}, sram1: {=bool:?}, sram2: {=bool:?}, sram3: {=bool:?}, sram4: {=bool:?}, sram5: {=bool:?}, sram6: {=bool:?}, sram7: {=bool:?}, sram8: {=bool:?}, sram9: {=bool:?}, flash0: {=bool:?}, flash1: {=bool:?}, cacheb0: {=bool:?}, cacheb2: {=bool:?} }}" , self . dtcm00 () , self . dtcm01 () , self . dtcm1 () , self . sram0 () , self . sram1 () , self . sram2 () , self . sram3 () , self . sram4 () , self . sram5 () , self . sram6 () , self . sram7 () , self . sram8 () , self . sram9 () , self . flash0 () , self . flash1 () , self . cacheb0 () , self . cacheb2 ())
    }
}
#[doc = "Power Optimization Control Bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc(pub u32);
impl Misc {
    #[doc = "Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[must_use]
    #[inline(always)]
    pub const fn forcememvrlptimers(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline(always)]
    pub const fn set_forcememvrlptimers(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
    #[must_use]
    #[inline(always)]
    pub const fn memvrlpble(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
    #[inline(always)]
    pub const fn set_memvrlpble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Misc {
    #[inline(always)]
    fn default() -> Misc {
        Misc(0)
    }
}
impl core::fmt::Debug for Misc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc")
            .field("forcememvrlptimers", &self.forcememvrlptimers())
            .field("memvrlpble", &self.memvrlpble())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc {{ forcememvrlptimers: {=bool:?}, memvrlpble: {=bool:?} }}",
            self.forcememvrlptimers(),
            self.memvrlpble()
        )
    }
}
#[doc = "SRAM Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramctrl(pub u32);
impl Sramctrl {
    #[doc = "This bit is 1 if clock gating is allowed for individual system SRAMs."]
    #[must_use]
    #[inline(always)]
    pub const fn sramclkgate(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 if clock gating is allowed for individual system SRAMs."]
    #[inline(always)]
    pub const fn set_sramclkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)."]
    #[must_use]
    #[inline(always)]
    pub const fn srammasterclkgate(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)."]
    #[inline(always)]
    pub const fn set_srammasterclkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
    #[must_use]
    #[inline(always)]
    pub const fn sramlightsleep(&self) -> super::vals::Sramlightsleep {
        let val = (self.0 >> 8usize) & 0x0fff;
        super::vals::Sramlightsleep::from_bits(val as u16)
    }
    #[doc = "Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
    #[inline(always)]
    pub const fn set_sramlightsleep(&mut self, val: super::vals::Sramlightsleep) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val.to_bits() as u32) & 0x0fff) << 8usize);
    }
}
impl Default for Sramctrl {
    #[inline(always)]
    fn default() -> Sramctrl {
        Sramctrl(0)
    }
}
impl core::fmt::Debug for Sramctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramctrl")
            .field("sramclkgate", &self.sramclkgate())
            .field("srammasterclkgate", &self.srammasterclkgate())
            .field("sramlightsleep", &self.sramlightsleep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sramctrl {{ sramclkgate: {=bool:?}, srammasterclkgate: {=bool:?}, sramlightsleep: {:?} }}" , self . sramclkgate () , self . srammasterclkgate () , self . sramlightsleep ())
    }
}
#[doc = "Voltage Regulator Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Supplysrc(pub u32);
impl Supplysrc {
    #[doc = "Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed."]
    #[must_use]
    #[inline(always)]
    pub const fn blebucken(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed."]
    #[inline(always)]
    pub const fn set_blebucken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Supplysrc {
    #[inline(always)]
    fn default() -> Supplysrc {
        Supplysrc(0)
    }
}
impl core::fmt::Debug for Supplysrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Supplysrc")
            .field("blebucken", &self.blebucken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Supplysrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Supplysrc {{ blebucken: {=bool:?} }}", self.blebucken())
    }
}
#[doc = "Voltage Regulators status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Supplystatus(pub u32);
impl Supplystatus {
    #[doc = "Indicates whether the Core/Mem low-voltage domains are supplied from the LDO or the Buck."]
    #[must_use]
    #[inline(always)]
    pub const fn simobuckon(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the Core/Mem low-voltage domains are supplied from the LDO or the Buck."]
    #[inline(always)]
    pub const fn set_simobuckon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates whether the BLE (if supported) domain and burst (if supported) domain is supplied from the LDO or the Buck. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate reature is allowed."]
    #[must_use]
    #[inline(always)]
    pub const fn blebuckon(&self) -> super::vals::Blebuckon {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Blebuckon::from_bits(val as u8)
    }
    #[doc = "Indicates whether the BLE (if supported) domain and burst (if supported) domain is supplied from the LDO or the Buck. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate reature is allowed."]
    #[inline(always)]
    pub const fn set_blebuckon(&mut self, val: super::vals::Blebuckon) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Supplystatus {
    #[inline(always)]
    fn default() -> Supplystatus {
        Supplystatus(0)
    }
}
impl core::fmt::Debug for Supplystatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Supplystatus")
            .field("simobuckon", &self.simobuckon())
            .field("blebuckon", &self.blebuckon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Supplystatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Supplystatus {{ simobuckon: {=bool:?}, blebuckon: {:?} }}",
            self.simobuckon(),
            self.blebuckon()
        )
    }
}

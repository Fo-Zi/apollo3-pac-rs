#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "This bit is the UART enable."]
    #[must_use]
    #[inline(always)]
    pub const fn uarten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the UART enable."]
    #[inline(always)]
    pub const fn set_uarten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit is the SIR ENDEC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn siren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the SIR ENDEC enable."]
    #[inline(always)]
    pub const fn set_siren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit is the SIR low power select."]
    #[must_use]
    #[inline(always)]
    pub const fn sirlp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the SIR low power select."]
    #[inline(always)]
    pub const fn set_sirlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit is the UART clock enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clken(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the UART clock enable."]
    #[inline(always)]
    pub const fn set_clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bitfield is the UART clock select."]
    #[must_use]
    #[inline(always)]
    pub const fn clksel(&self) -> super::vals::Clksel {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Clksel::from_bits(val as u8)
    }
    #[doc = "This bitfield is the UART clock select."]
    #[inline(always)]
    pub const fn set_clksel(&mut self, val: super::vals::Clksel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This bit is the loopback enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lbe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the loopback enable."]
    #[inline(always)]
    pub const fn set_lbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit is the transmit enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the transmit enable."]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit is the receive enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the receive enable."]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit enables data transmit ready."]
    #[must_use]
    #[inline(always)]
    pub const fn dtr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables data transmit ready."]
    #[inline(always)]
    pub const fn set_dtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit enables request to send."]
    #[must_use]
    #[inline(always)]
    pub const fn rts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables request to send."]
    #[inline(always)]
    pub const fn set_rts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit holds modem Out1."]
    #[must_use]
    #[inline(always)]
    pub const fn out1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds modem Out1."]
    #[inline(always)]
    pub const fn set_out1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit holds modem Out2."]
    #[must_use]
    #[inline(always)]
    pub const fn out2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds modem Out2."]
    #[inline(always)]
    pub const fn set_out2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit enables RTS hardware flow control."]
    #[must_use]
    #[inline(always)]
    pub const fn rtsen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables RTS hardware flow control."]
    #[inline(always)]
    pub const fn set_rtsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit enables CTS hardware flow control."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables CTS hardware flow control."]
    #[inline(always)]
    pub const fn set_ctsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("uarten", &self.uarten())
            .field("siren", &self.siren())
            .field("sirlp", &self.sirlp())
            .field("clken", &self.clken())
            .field("clksel", &self.clksel())
            .field("lbe", &self.lbe())
            .field("txe", &self.txe())
            .field("rxe", &self.rxe())
            .field("dtr", &self.dtr())
            .field("rts", &self.rts())
            .field("out1", &self.out1())
            .field("out2", &self.out2())
            .field("rtsen", &self.rtsen())
            .field("ctsen", &self.ctsen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cr {{ uarten: {=bool:?}, siren: {=bool:?}, sirlp: {=bool:?}, clken: {=bool:?}, clksel: {:?}, lbe: {=bool:?}, txe: {=bool:?}, rxe: {=bool:?}, dtr: {=bool:?}, rts: {=bool:?}, out1: {=bool:?}, out2: {=bool:?}, rtsen: {=bool:?}, ctsen: {=bool:?} }}" , self . uarten () , self . siren () , self . sirlp () , self . clken () , self . clksel () , self . lbe () , self . txe () , self . rxe () , self . dtr () , self . rts () , self . out1 () , self . out2 () , self . rtsen () , self . ctsen ())
    }
}
#[doc = "UART Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "This is the UART data port."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "This is the UART data port."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This is the framing error indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn fedata(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This is the framing error indicator."]
    #[inline(always)]
    pub const fn set_fedata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This is the parity error indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn pedata(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This is the parity error indicator."]
    #[inline(always)]
    pub const fn set_pedata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This is the break error indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn bedata(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This is the break error indicator."]
    #[inline(always)]
    pub const fn set_bedata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This is the overrun error indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn oedata(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This is the overrun error indicator."]
    #[inline(always)]
    pub const fn set_oedata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
impl core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr")
            .field("data", &self.data())
            .field("fedata", &self.fedata())
            .field("pedata", &self.pedata())
            .field("bedata", &self.bedata())
            .field("oedata", &self.oedata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dr {{ data: {=u8:?}, fedata: {=bool:?}, pedata: {=bool:?}, bedata: {=bool:?}, oedata: {=bool:?} }}" , self . data () , self . fedata () , self . pedata () , self . bedata () , self . oedata ())
    }
}
#[doc = "Fractional Baud Rate Divisor."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbrd(pub u32);
impl Fbrd {
    #[doc = "These bits hold the baud fractional divisor."]
    #[must_use]
    #[inline(always)]
    pub const fn divfrac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits hold the baud fractional divisor."]
    #[inline(always)]
    pub const fn set_divfrac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Fbrd {
    #[inline(always)]
    fn default() -> Fbrd {
        Fbrd(0)
    }
}
impl core::fmt::Debug for Fbrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fbrd")
            .field("divfrac", &self.divfrac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fbrd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fbrd {{ divfrac: {=u8:?} }}", self.divfrac())
    }
}
#[doc = "Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fr(pub u32);
impl Fr {
    #[doc = "This bit holds the clear to send indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn cts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the clear to send indicator."]
    #[inline(always)]
    pub const fn set_cts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit holds the data set ready indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn dsr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the data set ready indicator."]
    #[inline(always)]
    pub const fn set_dsr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit holds the data carrier detect indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn dcd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the data carrier detect indicator."]
    #[inline(always)]
    pub const fn set_dcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit holds the busy indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the busy indicator."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit holds the receive FIFO empty indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the receive FIFO empty indicator."]
    #[inline(always)]
    pub const fn set_rxfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit holds the transmit FIFO full indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn txff(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the transmit FIFO full indicator."]
    #[inline(always)]
    pub const fn set_txff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit holds the receive FIFO full indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn rxff(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the receive FIFO full indicator."]
    #[inline(always)]
    pub const fn set_rxff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit holds the transmit FIFO empty indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn txfe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the transmit FIFO empty indicator."]
    #[inline(always)]
    pub const fn set_txfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit holds the transmit BUSY indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn txbusy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the transmit BUSY indicator."]
    #[inline(always)]
    pub const fn set_txbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Fr {
    #[inline(always)]
    fn default() -> Fr {
        Fr(0)
    }
}
impl core::fmt::Debug for Fr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fr")
            .field("cts", &self.cts())
            .field("dsr", &self.dsr())
            .field("dcd", &self.dcd())
            .field("busy", &self.busy())
            .field("rxfe", &self.rxfe())
            .field("txff", &self.txff())
            .field("rxff", &self.rxff())
            .field("txfe", &self.txfe())
            .field("txbusy", &self.txbusy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Fr {{ cts: {=bool:?}, dsr: {=bool:?}, dcd: {=bool:?}, busy: {=bool:?}, rxfe: {=bool:?}, txff: {=bool:?}, rxff: {=bool:?}, txfe: {=bool:?}, txbusy: {=bool:?} }}" , self . cts () , self . dsr () , self . dcd () , self . busy () , self . rxfe () , self . txff () , self . rxff () , self . txfe () , self . txbusy ())
    }
}
#[doc = "Integer Baud Rate Divisor."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibrd(pub u32);
impl Ibrd {
    #[doc = "These bits hold the baud integer divisor."]
    #[must_use]
    #[inline(always)]
    pub const fn divint(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "These bits hold the baud integer divisor."]
    #[inline(always)]
    pub const fn set_divint(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ibrd {
    #[inline(always)]
    fn default() -> Ibrd {
        Ibrd(0)
    }
}
impl core::fmt::Debug for Ibrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ibrd")
            .field("divint", &self.divint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ibrd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ibrd {{ divint: {=u16:?} }}", self.divint())
    }
}
#[doc = "Interrupt Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iec(pub u32);
impl Iec {
    #[doc = "This bit holds the modem TXCMP interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn txcmpmic(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem TXCMP interrupt clear."]
    #[inline(always)]
    pub const fn set_txcmpmic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit holds the modem CTS interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsmic(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem CTS interrupt clear."]
    #[inline(always)]
    pub const fn set_ctsmic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit holds the modem DCD interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdmic(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem DCD interrupt clear."]
    #[inline(always)]
    pub const fn set_dcdmic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit holds the modem DSR interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dsrmic(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem DSR interrupt clear."]
    #[inline(always)]
    pub const fn set_dsrmic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit holds the receive interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rxic(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the receive interrupt clear."]
    #[inline(always)]
    pub const fn set_rxic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit holds the transmit interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn txic(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the transmit interrupt clear."]
    #[inline(always)]
    pub const fn set_txic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit holds the receive timeout interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rtic(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the receive timeout interrupt clear."]
    #[inline(always)]
    pub const fn set_rtic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit holds the framing error interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn feic(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the framing error interrupt clear."]
    #[inline(always)]
    pub const fn set_feic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit holds the parity error interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn peic(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the parity error interrupt clear."]
    #[inline(always)]
    pub const fn set_peic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit holds the break error interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn beic(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the break error interrupt clear."]
    #[inline(always)]
    pub const fn set_beic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit holds the overflow interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn oeic(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the overflow interrupt clear."]
    #[inline(always)]
    pub const fn set_oeic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Iec {
    #[inline(always)]
    fn default() -> Iec {
        Iec(0)
    }
}
impl core::fmt::Debug for Iec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iec")
            .field("txcmpmic", &self.txcmpmic())
            .field("ctsmic", &self.ctsmic())
            .field("dcdmic", &self.dcdmic())
            .field("dsrmic", &self.dsrmic())
            .field("rxic", &self.rxic())
            .field("txic", &self.txic())
            .field("rtic", &self.rtic())
            .field("feic", &self.feic())
            .field("peic", &self.peic())
            .field("beic", &self.beic())
            .field("oeic", &self.oeic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iec {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Iec {{ txcmpmic: {=bool:?}, ctsmic: {=bool:?}, dcdmic: {=bool:?}, dsrmic: {=bool:?}, rxic: {=bool:?}, txic: {=bool:?}, rtic: {=bool:?}, feic: {=bool:?}, peic: {=bool:?}, beic: {=bool:?}, oeic: {=bool:?} }}" , self . txcmpmic () , self . ctsmic () , self . dcdmic () , self . dsrmic () , self . rxic () , self . txic () , self . rtic () , self . feic () , self . peic () , self . beic () , self . oeic ())
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "This bit holds the modem TXCMP interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txcmpmim(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem TXCMP interrupt enable."]
    #[inline(always)]
    pub const fn set_txcmpmim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit holds the modem CTS interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsmim(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem CTS interrupt enable."]
    #[inline(always)]
    pub const fn set_ctsmim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit holds the modem DCD interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdmim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem DCD interrupt enable."]
    #[inline(always)]
    pub const fn set_dcdmim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit holds the modem DSR interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dsrmim(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem DSR interrupt enable."]
    #[inline(always)]
    pub const fn set_dsrmim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit holds the receive interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxim(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the receive interrupt enable."]
    #[inline(always)]
    pub const fn set_rxim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit holds the transmit interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txim(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the transmit interrupt enable."]
    #[inline(always)]
    pub const fn set_txim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit holds the receive timeout interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rtim(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the receive timeout interrupt enable."]
    #[inline(always)]
    pub const fn set_rtim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit holds the framing error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn feim(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the framing error interrupt enable."]
    #[inline(always)]
    pub const fn set_feim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit holds the parity error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn peim(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the parity error interrupt enable."]
    #[inline(always)]
    pub const fn set_peim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit holds the break error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn beim(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the break error interrupt enable."]
    #[inline(always)]
    pub const fn set_beim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit holds the overflow interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn oeim(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the overflow interrupt enable."]
    #[inline(always)]
    pub const fn set_oeim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("txcmpmim", &self.txcmpmim())
            .field("ctsmim", &self.ctsmim())
            .field("dcdmim", &self.dcdmim())
            .field("dsrmim", &self.dsrmim())
            .field("rxim", &self.rxim())
            .field("txim", &self.txim())
            .field("rtim", &self.rtim())
            .field("feim", &self.feim())
            .field("peim", &self.peim())
            .field("beim", &self.beim())
            .field("oeim", &self.oeim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ier {{ txcmpmim: {=bool:?}, ctsmim: {=bool:?}, dcdmim: {=bool:?}, dsrmim: {=bool:?}, rxim: {=bool:?}, txim: {=bool:?}, rtim: {=bool:?}, feim: {=bool:?}, peim: {=bool:?}, beim: {=bool:?}, oeim: {=bool:?} }}" , self . txcmpmim () , self . ctsmim () , self . dcdmim () , self . dsrmim () , self . rxim () , self . txim () , self . rtim () , self . feim () , self . peim () , self . beim () , self . oeim ())
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ies(pub u32);
impl Ies {
    #[doc = "This bit holds the modem TXCMP interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn txcmpmris(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem TXCMP interrupt status."]
    #[inline(always)]
    pub const fn set_txcmpmris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit holds the modem CTS interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsmris(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem CTS interrupt status."]
    #[inline(always)]
    pub const fn set_ctsmris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit holds the modem DCD interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdmris(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem DCD interrupt status."]
    #[inline(always)]
    pub const fn set_dcdmris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit holds the modem DSR interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn dsrmris(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem DSR interrupt status."]
    #[inline(always)]
    pub const fn set_dsrmris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit holds the receive interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn rxris(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the receive interrupt status."]
    #[inline(always)]
    pub const fn set_rxris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit holds the transmit interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn txris(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the transmit interrupt status."]
    #[inline(always)]
    pub const fn set_txris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit holds the receive timeout interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn rtris(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the receive timeout interrupt status."]
    #[inline(always)]
    pub const fn set_rtris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit holds the framing error interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn feris(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the framing error interrupt status."]
    #[inline(always)]
    pub const fn set_feris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit holds the parity error interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn peris(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the parity error interrupt status."]
    #[inline(always)]
    pub const fn set_peris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit holds the break error interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn beris(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the break error interrupt status."]
    #[inline(always)]
    pub const fn set_beris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit holds the overflow interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn oeris(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the overflow interrupt status."]
    #[inline(always)]
    pub const fn set_oeris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Ies {
    #[inline(always)]
    fn default() -> Ies {
        Ies(0)
    }
}
impl core::fmt::Debug for Ies {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ies")
            .field("txcmpmris", &self.txcmpmris())
            .field("ctsmris", &self.ctsmris())
            .field("dcdmris", &self.dcdmris())
            .field("dsrmris", &self.dsrmris())
            .field("rxris", &self.rxris())
            .field("txris", &self.txris())
            .field("rtris", &self.rtris())
            .field("feris", &self.feris())
            .field("peris", &self.peris())
            .field("beris", &self.beris())
            .field("oeris", &self.oeris())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ies {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ies {{ txcmpmris: {=bool:?}, ctsmris: {=bool:?}, dcdmris: {=bool:?}, dsrmris: {=bool:?}, rxris: {=bool:?}, txris: {=bool:?}, rtris: {=bool:?}, feris: {=bool:?}, peris: {=bool:?}, beris: {=bool:?}, oeris: {=bool:?} }}" , self . txcmpmris () , self . ctsmris () , self . dcdmris () , self . dsrmris () , self . rxris () , self . txris () , self . rtris () , self . feris () , self . peris () , self . beris () , self . oeris ())
    }
}
#[doc = "FIFO Interrupt Level Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifls(pub u32);
impl Ifls {
    #[doc = "These bits hold the transmit FIFO interrupt level."]
    #[must_use]
    #[inline(always)]
    pub const fn txiflsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits hold the transmit FIFO interrupt level."]
    #[inline(always)]
    pub const fn set_txiflsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "These bits hold the receive FIFO interrupt level."]
    #[must_use]
    #[inline(always)]
    pub const fn rxiflsel(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "These bits hold the receive FIFO interrupt level."]
    #[inline(always)]
    pub const fn set_rxiflsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
}
impl Default for Ifls {
    #[inline(always)]
    fn default() -> Ifls {
        Ifls(0)
    }
}
impl core::fmt::Debug for Ifls {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ifls")
            .field("txiflsel", &self.txiflsel())
            .field("rxiflsel", &self.rxiflsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifls {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ifls {{ txiflsel: {=u8:?}, rxiflsel: {=u8:?} }}",
            self.txiflsel(),
            self.rxiflsel()
        )
    }
}
#[doc = "IrDA Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ilpr(pub u32);
impl Ilpr {
    #[doc = "These bits hold the IrDA counter divisor."]
    #[must_use]
    #[inline(always)]
    pub const fn ilpdvsr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits hold the IrDA counter divisor."]
    #[inline(always)]
    pub const fn set_ilpdvsr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ilpr {
    #[inline(always)]
    fn default() -> Ilpr {
        Ilpr(0)
    }
}
impl core::fmt::Debug for Ilpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ilpr")
            .field("ilpdvsr", &self.ilpdvsr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ilpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ilpr {{ ilpdvsr: {=u8:?} }}", self.ilpdvsr())
    }
}
#[doc = "Line Control High."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcrh(pub u32);
impl Lcrh {
    #[doc = "This bit holds the break set."]
    #[must_use]
    #[inline(always)]
    pub const fn brk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the break set."]
    #[inline(always)]
    pub const fn set_brk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit holds the parity enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the parity enable."]
    #[inline(always)]
    pub const fn set_pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit holds the even parity select."]
    #[must_use]
    #[inline(always)]
    pub const fn eps(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the even parity select."]
    #[inline(always)]
    pub const fn set_eps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit holds the two stop bits select."]
    #[must_use]
    #[inline(always)]
    pub const fn stp2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the two stop bits select."]
    #[inline(always)]
    pub const fn set_stp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit holds the FIFO enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the FIFO enable."]
    #[inline(always)]
    pub const fn set_fen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "These bits hold the write length."]
    #[must_use]
    #[inline(always)]
    pub const fn wlen(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "These bits hold the write length."]
    #[inline(always)]
    pub const fn set_wlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "This bit holds the stick parity select."]
    #[must_use]
    #[inline(always)]
    pub const fn sps(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the stick parity select."]
    #[inline(always)]
    pub const fn set_sps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Lcrh {
    #[inline(always)]
    fn default() -> Lcrh {
        Lcrh(0)
    }
}
impl core::fmt::Debug for Lcrh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lcrh")
            .field("brk", &self.brk())
            .field("pen", &self.pen())
            .field("eps", &self.eps())
            .field("stp2", &self.stp2())
            .field("fen", &self.fen())
            .field("wlen", &self.wlen())
            .field("sps", &self.sps())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lcrh {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Lcrh {{ brk: {=bool:?}, pen: {=bool:?}, eps: {=bool:?}, stp2: {=bool:?}, fen: {=bool:?}, wlen: {=u8:?}, sps: {=bool:?} }}" , self . brk () , self . pen () , self . eps () , self . stp2 () , self . fen () , self . wlen () , self . sps ())
    }
}
#[doc = "Masked Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mis(pub u32);
impl Mis {
    #[doc = "This bit holds the modem TXCMP interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn txcmpmmis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem TXCMP interrupt status masked."]
    #[inline(always)]
    pub const fn set_txcmpmmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit holds the modem CTS interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsmmis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem CTS interrupt status masked."]
    #[inline(always)]
    pub const fn set_ctsmmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit holds the modem DCD interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdmmis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem DCD interrupt status masked."]
    #[inline(always)]
    pub const fn set_dcdmmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit holds the modem DSR interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn dsrmmis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the modem DSR interrupt status masked."]
    #[inline(always)]
    pub const fn set_dsrmmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit holds the receive interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn rxmis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the receive interrupt status masked."]
    #[inline(always)]
    pub const fn set_rxmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit holds the transmit interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn txmis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the transmit interrupt status masked."]
    #[inline(always)]
    pub const fn set_txmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit holds the receive timeout interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn rtmis(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the receive timeout interrupt status masked."]
    #[inline(always)]
    pub const fn set_rtmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit holds the framing error interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn femis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the framing error interrupt status masked."]
    #[inline(always)]
    pub const fn set_femis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit holds the parity error interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn pemis(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the parity error interrupt status masked."]
    #[inline(always)]
    pub const fn set_pemis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit holds the break error interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn bemis(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the break error interrupt status masked."]
    #[inline(always)]
    pub const fn set_bemis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit holds the overflow interrupt status masked."]
    #[must_use]
    #[inline(always)]
    pub const fn oemis(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit holds the overflow interrupt status masked."]
    #[inline(always)]
    pub const fn set_oemis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Mis {
    #[inline(always)]
    fn default() -> Mis {
        Mis(0)
    }
}
impl core::fmt::Debug for Mis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mis")
            .field("txcmpmmis", &self.txcmpmmis())
            .field("ctsmmis", &self.ctsmmis())
            .field("dcdmmis", &self.dcdmmis())
            .field("dsrmmis", &self.dsrmmis())
            .field("rxmis", &self.rxmis())
            .field("txmis", &self.txmis())
            .field("rtmis", &self.rtmis())
            .field("femis", &self.femis())
            .field("pemis", &self.pemis())
            .field("bemis", &self.bemis())
            .field("oemis", &self.oemis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mis {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mis {{ txcmpmmis: {=bool:?}, ctsmmis: {=bool:?}, dcdmmis: {=bool:?}, dsrmmis: {=bool:?}, rxmis: {=bool:?}, txmis: {=bool:?}, rtmis: {=bool:?}, femis: {=bool:?}, pemis: {=bool:?}, bemis: {=bool:?}, oemis: {=bool:?} }}" , self . txcmpmmis () , self . ctsmmis () , self . dcdmmis () , self . dsrmmis () , self . rxmis () , self . txmis () , self . rtmis () , self . femis () , self . pemis () , self . bemis () , self . oemis ())
    }
}
#[doc = "UART Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsr(pub u32);
impl Rsr {
    #[doc = "This is the framing error indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn festat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This is the framing error indicator."]
    #[inline(always)]
    pub const fn set_festat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This is the parity error indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn pestat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This is the parity error indicator."]
    #[inline(always)]
    pub const fn set_pestat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This is the break error indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn bestat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This is the break error indicator."]
    #[inline(always)]
    pub const fn set_bestat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This is the overrun error indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn oestat(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This is the overrun error indicator."]
    #[inline(always)]
    pub const fn set_oestat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Rsr {
    #[inline(always)]
    fn default() -> Rsr {
        Rsr(0)
    }
}
impl core::fmt::Debug for Rsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsr")
            .field("festat", &self.festat())
            .field("pestat", &self.pestat())
            .field("bestat", &self.bestat())
            .field("oestat", &self.oestat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rsr {{ festat: {=bool:?}, pestat: {=bool:?}, bestat: {=bool:?}, oestat: {=bool:?} }}",
            self.festat(),
            self.pestat(),
            self.bestat(),
            self.oestat()
        )
    }
}

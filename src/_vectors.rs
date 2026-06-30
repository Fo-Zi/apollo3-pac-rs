unsafe extern "C" {
    fn BROWNOUT();
    fn WDT();
    fn RTC();
    fn VCOMP();
    fn IOSLAVE();
    fn IOSLAVEACC();
    fn IOMSTR0();
    fn IOMSTR1();
    fn IOMSTR2();
    fn IOMSTR3();
    fn IOMSTR4();
    fn IOMSTR5();
    fn BLE();
    fn GPIO();
    fn CTIMER();
    fn UART0();
    fn UART1();
    fn SCARD();
    fn ADC();
    fn PDM();
    fn MSPI();
    fn STIMER();
    fn STIMER_CMPR0();
    fn STIMER_CMPR1();
    fn STIMER_CMPR2();
    fn STIMER_CMPR3();
    fn STIMER_CMPR4();
    fn STIMER_CMPR5();
    fn STIMER_CMPR6();
    fn STIMER_CMPR7();
    fn CLKGEN();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: BROWNOUT },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector { _handler: VCOMP },
    Vector { _handler: IOSLAVE },
    Vector {
        _handler: IOSLAVEACC,
    },
    Vector { _handler: IOMSTR0 },
    Vector { _handler: IOMSTR1 },
    Vector { _handler: IOMSTR2 },
    Vector { _handler: IOMSTR3 },
    Vector { _handler: IOMSTR4 },
    Vector { _handler: IOMSTR5 },
    Vector { _handler: BLE },
    Vector { _handler: GPIO },
    Vector { _handler: CTIMER },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: SCARD },
    Vector { _handler: ADC },
    Vector { _handler: PDM },
    Vector { _handler: MSPI },
    Vector { _reserved: 0 },
    Vector { _handler: STIMER },
    Vector {
        _handler: STIMER_CMPR0,
    },
    Vector {
        _handler: STIMER_CMPR1,
    },
    Vector {
        _handler: STIMER_CMPR2,
    },
    Vector {
        _handler: STIMER_CMPR3,
    },
    Vector {
        _handler: STIMER_CMPR4,
    },
    Vector {
        _handler: STIMER_CMPR5,
    },
    Vector {
        _handler: STIMER_CMPR6,
    },
    Vector {
        _handler: STIMER_CMPR7,
    },
    Vector { _handler: CLKGEN },
];

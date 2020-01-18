use crate::board::{
    led::Led,
    Board,
};
use crate::target::Peripherals;
use crate::hal::{
    delay::Delay,
    flash::FlashExt,
    rcc::RccExt,
    time::U32Ext,
    gpio::{
        GpioExt,
        Output,
        PushPull,
        gpioc::{
            Parts,
            PC13,
        },
    }
};

impl Board<PC13<Output<PushPull>>> {
    pub fn new() -> Self {
        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();

        let mut flash = p.FLASH.constrain();
        let mut rcc = p.RCC.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(48.mhz())
            .pclk1(24.mhz())
            .freeze(&mut flash.acr);

        assert!(clocks.usbclk_valid());

        let mut gpioc: Parts = p.GPIOC.split(&mut rcc.apb2);
        let led0 = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        Board {
            status_led: Led::new(led0),
            delay: Delay::new(cp.SYST, clocks),
        }
    }
}

use crate::board::{
    led::Led,
    Board,
};
use crate::target::Peripherals;
use crate::hal::{
    delay::Delay,
    flash::FlashExt,
    rcc::RccExt,
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

        let mut rcc = p.RCC.constrain();
        let mut flash = p.FLASH.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpioc: Parts = p.GPIOC.split(&mut rcc.apb2);
        let led0 = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        Board {
            status_led: Led::new(led0),
            delay: Delay::new(cp.SYST, clocks),
        }
    }
}

use crate::board::Board;
use stm32f1::stm32f103::Peripherals;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::flash::FlashExt;
use stm32f1xx_hal::gpio::{
    GpioExt,
    Output,
    PushPull,
    gpioc::{
        Parts,
        PC13,
    },
};
use crate::board::led::Led;
use stm32f1xx_hal::delay::Delay;

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

use crate::board::{
    led::Led,
    Board,
};
use crate::target::Peripherals;
use crate::hal::{
    Delay,
    gpio::{
        Output,
        PushPull,
        GpioExt,
        Level,
        p0::P0_24,
    },
};

impl Board<P0_24<Output<PushPull>>> {
    pub fn new() -> Self {
        let cp = cortex_m::Peripherals::take().unwrap();
        let p = Peripherals::take().unwrap();
        let port0 = p.P0.split();
        let led0 = port0.p0_24.into_push_pull_output(Level::Low);

        Board {
            status_led: Led::new(led0),
            delay: Delay::new(cp.SYST),
        }
    }
}

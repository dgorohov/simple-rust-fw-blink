use crate::target::Peripherals;
use crate::board::{
    Board,
    ToggleStatusLed,
};

impl<T> Board<T> where T: ToggleStatusLed {
    pub fn new() -> Self {
        let p = Peripherals::take().unwrap();
        let port0 = p.P0.split();
        let led0 = port0.p0_24.into_push_pull_output().degrade();
        Board {
            status_led: led0,
        }
    }
}

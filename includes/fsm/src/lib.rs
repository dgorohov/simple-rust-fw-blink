#![no_std]

use alloc::vec::Vec;

pub trait StateMachine {
    type Error;
    fn switch_to(&mut self) -> Result<(), Self::Error>;
    fn invalidate(&mut self);
}

pub trait StateMachineRegister {
    type Error;
    fn register<S: StateMachine>(&mut self) -> Result<S, Self::Error>;
}


pub struct FsmExecutor<S: StateMachine> {
    state_machines: Vec<S>,
}

use common::IO_BASE;
use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

/// The base address for the ARM system timer registers.
const TIMER_REG_BASE: usize = IO_BASE + 0x3000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    CS: Volatile<u32>,
    CLO: ReadVolatile<u32>,
    CHI: ReadVolatile<u32>,
    COMPARE: [Volatile<u32>; 4]
}

/// The Raspberry Pi ARM system timer.
pub struct Timer {
    registers: &'static mut Registers
}

impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut Registers) },
        }
    }

    /// Reads the system timer's counter and returns the 64-bit counter value.
    /// The returned value is the number of elapsed microseconds.
    pub fn read(&self) -> u64 {
        let lo = self.registers.CLO.read() as u64;
        let hi = self.registers.CHI.read() as u64;
        (hi << 32) + lo
    }

    pub fn tick_in(&mut self, us: u32) {
        let current_low = self.registers.CLO.read();
        let compare = current_low.wrapping_add(us);
        self.registers.COMPARE[1].write(compare); // timer 1
        self.registers.CS.or_mask(0b0010); // clear timer 1 interrupt
    }
}

/// Returns the current time in microseconds.
pub fn current_time() -> u64 {
    Timer::new().read()
}

/// Spins until `us` microseconds have passed.
pub fn spin_sleep_us(us: u64) {
    let timer = Timer::new();
    for _ in 0..us {
        let lo = timer.registers.CLO.read();
        timer.registers.COMPARE[1].write(lo + 1);
        while !timer.registers.CS.has_mask(0b10u32) {}
    }
}

/// Spins until `ms` milliseconds have passed.
pub fn spin_sleep_ms(ms: u64) {
    spin_sleep_us(ms * 1000);
}

pub fn tick_in(us: u32) {
    let mut timer = Timer::new();
    timer.tick_in(us);
}
use core::cell::RefCell;
use core::mem;

use alloc::sync::Arc;
use hal::clocks::Internal;
use hal::clocks::LfOscStarted;
use hal::clocks::LfOscStopped;
use hal::rtc;
use hal::Clocks;
use hal::Rtc;
use pac::Peripherals;

// Clock resolution = 30517 nanoseconds = 30.517 microseconds
const MILLISECOND: u32 = 32; // rounded down from 32.768
const SECOND: u32 = 32768; // rounded down from 32768.62

// fn sleep(mut rtc: Rtc<RTC0, hal::rtc::Stopped>, amt: u32) -> Rtc<RTC0, hal::rtc::Stopped> {
//     rprintln!("Sleeping...");
//     rtc.clear_counter();
//     rtc.set_compare(hal::rtc::RtcCompareReg::Compare0, rtc.get_counter() + amt);
//     rtc.enable_event(hal::rtc::RtcInterrupt::Compare0);
//     let mut rtc = rtc.enable_counter();
//     while !rtc.get_event_triggered(hal::rtc::RtcInterrupt::Compare0, true) {}
//     rtc.disable_event(hal::rtc::RtcInterrupt::Compare0);
//     rtc.disable_counter()
// }

struct DynRTC {
    ptr: *const pac::rtc0::RegisterBlock,
    started: bool,
}

// Managed by ClockManager, so we're cool
unsafe impl Send for DynRTC {}
unsafe impl Sync for DynRTC {}

impl From<*const pac::rtc0::RegisterBlock> for DynRTC {
    fn from(ptr: *const pac::rtc0::RegisterBlock) -> Self {
        Self {
            ptr,
            started: false,
        }
    }
}

impl DynRTC {
    fn is_on(&self) -> bool {
        self.started
    }

    fn start(&mut self) {
        todo!();
        self.started = true;
    }

    fn stop(&mut self) {
        todo!();
        self.started = false;
    }
}

pub struct ClockManager<C, const N: usize> {
    clocks: Clocks<Internal, Internal, C>,
    rtcs: [RefCell<DynRTC>; N],
}

impl<const N: usize> ClockManager<LfOscStopped, N> {
    pub fn new(c: pac::CLOCK, ptrs: [*const pac::rtc0::RegisterBlock; N]) -> Self {
        let clocks = hal::clocks::Clocks::new(c);
        let rtcs: [RefCell<DynRTC>; N] = ptrs.map(|ptr| RefCell::new(DynRTC::from(ptr)));
        Self { clocks, rtcs }
    }

    pub fn start(self) -> ClockManager<LfOscStarted, N> {
        ClockManager {
            clocks: self.clocks.start_lfclk(),
            rtcs: self.rtcs,
        }
    }
}

impl<C, const N: usize> ClockManager<C, N> {
    pub(crate) fn on_rtc0(&self) {
        todo!()
    }

    pub(crate) fn on_rtc1(&self) {
        todo!()
    }

    pub(crate) fn on_rtc2(&self) {
        todo!()
    }
}

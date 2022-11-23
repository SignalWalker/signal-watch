#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
#![feature(min_const_generics)]
#![feature(array_map)]

extern crate alloc;

extern crate cortex_m as cortex;
extern crate cortex_m_rt as rt;
extern crate embedded_hal as ehal;
extern crate nrf52840_hal as hal;
extern crate nrf52840_pac as pac;

#[global_allocator]
static ALLOCATOR: alloc_cortex_m::CortexMHeap = alloc_cortex_m::CortexMHeap::empty();

use hal::gpio::Output;
use hal::gpio::Pin;
use hal::gpio::PushPull;
use hal::prelude::OutputPin;
use panic_halt as _;

mod periph;
// mod gfx;
// use gfx::*;

// Notes available at [https://wiki.ashwalker.net/Signal_Watch]

pub struct DOut {
    state: bool,
    pin: Pin<Output<PushPull>>,
}

impl DOut {
    pub fn new(pin: Pin<Output<PushPull>>, state: bool) -> Self {
        DOut { state, pin }
    }

    pub fn toggle(&mut self) -> bool {
        match self.state {
            true => self.set_off(),
            false => self.set_on(),
        }
        self.state
    }

    pub fn set_on(&mut self) {
        self.pin.set_high().unwrap();
        self.state = true;
    }

    pub fn set_off(&mut self) {
        self.pin.set_low().unwrap();
        self.state = false;
    }

    pub fn is_on(&self) -> bool {
        self.state
    }
}

#[rtic::app(device = pac, peripherals = true)]
mod app {
    use crate::periph::*;
    use cortex::asm;
    use hal::clocks::LfOscStarted;
    use hal::gpio::Input;
    use hal::gpio::Level;
    use hal::gpio::Pin;
    use hal::gpio::PullUp;
    use hal::gpiote::Gpiote;
    use hal::prelude::*;
    use rtic_core::prelude::*;

    use rtt_target::{rprintln, rtt_init_print};

    use crate::DOut;

    #[shared]
    struct SharedResources {}

    #[local]
    struct LocalResources {
        clock: ClockManager<LfOscStarted, 3>,
        gpiote: Gpiote,
        user_sw: Pin<Input<PullUp>>,
        reset: Pin<Input<PullUp>>,
        led_red: DOut,
        led_blue: DOut,
    }

    #[init]
    fn init(cx: init::Context) -> (SharedResources, LocalResources, init::Monotonics) {
        rtt_init_print!();
        rprintln!("[Init]");
        unsafe {
            let start = rt::heap_start() as usize;
            // TODO :: Figure out how to decide heap size
            let size = 199 * 1024; // 199KB / 796KB
            rprintln!("Heap Start: {}, Size: {} B", start, size);
            crate::ALLOCATOR.init(start, size);
        }
        rprintln!("Initializing clock and RTCs...");
        let clock = ClockManager::new(
            cx.device.CLOCK,
            [pac::RTC0::ptr(), pac::RTC1::ptr(), pac::RTC2::ptr()],
        )
        .start();
        rprintln!("Initializing GPIO...");
        let p0 = hal::gpio::p0::Parts::new(cx.device.P0);
        let p1 = hal::gpio::p1::Parts::new(cx.device.P1);
        let user_sw = p1.p1_02.into_pullup_input().degrade();
        let reset = p0.p0_18.into_pullup_input().degrade();
        let led_red = DOut::new(p1.p1_15.into_push_pull_output(Level::Low).degrade(), false);
        let led_blue = DOut::new(p1.p1_10.into_push_pull_output(Level::Low).degrade(), false);

        let gpiote = hal::gpiote::Gpiote::new(cx.device.GPIOTE);
        gpiote.port().input_pin(&reset).low();
        gpiote.port().input_pin(&user_sw).low();
        gpiote.port().enable_interrupt();

        (
            SharedResources {},
            LocalResources {
                clock,
                gpiote,
                user_sw,
                reset,
                led_red,
                led_blue,
            },
            init::Monotonics {},
        )
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("[Idle]");
        loop {
            asm::wfi();
        }
    }
}

// #[entry]
// fn main() -> ! {
//     rtt_init_print!();
//     unsafe {
//         // TODO :: Figure out how to determine optimal heap size
//         ALLOCATOR.init(rt::heap_start() as usize, 1024);
//     }

//     // DC  -> p1.11
//     // RST -> p1.12
//     // TCS -> p1.15
//     let periph = pac::Peripherals::take().unwrap();
//     // let port0 = hal::gpio::p0::Parts::new(periph.P0);
//     let port1 = hal::gpio::p1::Parts::new(periph.P1);

//     // DC  :: Toggle between data and command input (I assume low:data and high:command)
//     // RST :: Reset display
//     // TCS :: Chip select
//     // let mut dc = port1.p1_11.into_push_pull_output(Level::Low);
//     // let mut rst = port1.p1_12.into_push_pull_output(Level::Low);
//     // let mut tcs = port1.p1_15.into_push_pull_output(Level::High).degrade();

//     // let clk  = port0.p0_14.into_push_pull_output(Level::Low).degrade();
//     // let miso = port0.p0_15.into_floating_input().degrade();
//     // let mosi = port0.p0_13.into_push_pull_output(Level::Low).degrade();

//     let mut red = port1.p1_15.into_push_pull_output(Level::Low).degrade();
//     let mut blue = port1.p1_10.into_push_pull_output(Level::Low).degrade();
//     // let neo  = port1.p0_16.into_push_pull_output(Level::Low).degrade();

//     // let mut spi = Spim::new(
//     //     periph.SPIM0, // which spim do i use...?
//     //     spim::Pins {
//     //         sck: clk,
//     //         miso: Some(miso),
//     //         mosi: Some(mosi),
//     //     },
//     //     spim::Frequency::M32, // 32MHz spi clock
//     //     spim::MODE_0,
//     //     0, // wtf is orc
//     // );

//     // i'm just gonna write a 240*240 buffer to the tft because idk what else to do
//     // let mut tft_buf = PixelBuf::default();
//     // make it all white
//     // tft_buf.clear(0xff);

//     // Reset tft
//     // rst.set_high();
//     // rst.set_low();

//     loop {
//         // Write to tft
//         // spi.write(&mut tcs, tft_buf.as_u8()).unwrap();
//         rprintln!("Red");
//         red.set_high();
//         blue.set_low();
//         // rtc = sleep(rtc, SECOND);
//         rprintln!("Blue");
//         red.set_low();
//         blue.set_high();
//         // rtc = sleep(rtc, SECOND);
//     }
// }

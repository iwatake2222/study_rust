#![no_std]
#![no_main]

use wio_terminal as wio;
use embedded_graphics as eg;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use core::panic::PanicInfo;
use cortex_m::interrupt::{self, Mutex};
use wio::hal::clock::GenericClockController;
use wio::hal::gpio::*;
use wio::hal::sercom::*;
use wio::pac::CorePeripherals;
use wio::pac::Peripherals;
use wio::prelude::*;
use wio::{entry, Pins, Sets};

use eg::{image::*, pixelcolor::*, prelude::*, primitives::*, style::*, fonts::*};

static UART: Mutex<
    RefCell<
        Option<UART2<Sercom2Pad1<Pb27<PfC>>, Sercom2Pad0<Pb26<PfC>>, (), ()>>,
    >,
> = Mutex::new(RefCell::new(None));

fn print(text: &str) {
    interrupt::free(|cs| {
        if let Some(ref mut serial) = UART.borrow(cs).borrow_mut().deref_mut() {
            writeln!(serial, "{}", text).unwrap();
        }
    });
}

#[entry]
fn main() -> ! {
    /*** Initialize ***/
    let core = CorePeripherals::take().unwrap();
    let mut peripherals = Peripherals::take().unwrap();

    /* Initialize Clocks */
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    /* Initialize UART */
    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );
    interrupt::free(|cs| UART.borrow(cs).replace(Some(serial)));

    /* Get delay driver based on SysTick */
    let mut delay = wio::hal::delay::Delay::new(core.SYST, &mut clocks);

    /* Initialize LCD */
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            58.mhz(),
            &mut delay,
        )
        .unwrap();


    print("Hello World");

    Text::new("Hello World", Point::new(10, 10))
        .into_styled(TextStyle::new(Font12x16, Rgb565::GREEN))
        .draw(&mut display).unwrap();
    let raw = ImageRawLE::new(include_bytes!("../assets/ferris.raw"), 86, 64);
    loop {
        for y in (0..240).step_by(64) {
            for x in (0..320).step_by(86) {
                let image = Image::new(&raw, Point::new(x, y));
                image.draw(&mut display).unwrap();
            }
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::free(|cs| {
        if let Some(ref mut serial) = UART.borrow(cs).borrow_mut().deref_mut() {
            let _ = writeln!(serial, "panic: {}", info);
        }
    });
    loop {}
}

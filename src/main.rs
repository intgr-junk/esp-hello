mod errors;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use log::info;
use crate::errors::AnyErr;

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting {} v{}...", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"));

    blinker().unwrap();

    panic!("This should not be reached");
}

fn blinker() -> Result<(), AnyErr> {
    esp_idf_hal::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio15)?;

    loop {
        led.set_high()?;
        info!("on");
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(1000);

        led.set_low()?;
        info!("off");
        FreeRtos::delay_ms(1000);
    }
}

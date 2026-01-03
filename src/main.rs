mod errors;

use crate::errors::AnyErr;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver};
use esp_idf_hal::peripherals::Peripherals;
use log::info;
use std::f32::consts::PI;
use std::time::Instant;

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting {} v{}...", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"));

    blinker().unwrap();

    panic!("This should not be reached");
}

fn blinker() -> Result<(), AnyErr> {
    esp_idf_hal::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default().frequency(25_000.into()),
    )?;
    let mut driver =
        LedcDriver::new(peripherals.ledc.channel0, timer_driver, peripherals.pins.gpio15)?;
    let max_duty = driver.get_max_duty();

    let start_time = Instant::now();
    info!("Blinker starting @ {:?}", start_time);

    loop {
        FreeRtos::delay_ms(20);

        let secs = start_time.elapsed().as_secs_f32();
        let activation = ((secs * PI).sin() + 1.0) * 0.5;
        let duty = max_duty * ((activation * 65536.0) as u32) / 65536;
        driver.set_duty(duty)?;
        // info!("{} -> {}, duty: {}", secs, activation, duty);
    }
}

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::CpuClock, gpio::{Input, Level, Output}, rtc_cntl::Rtc};
use esp_println::println;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);
    let mut trig = Output::new(peripherals.GPIO2, Level::Low);
    let mut echo  = Input::new(peripherals.GPIO0, esp_hal::gpio::Pull::Up);
    let rtc = Rtc::new(peripherals.LPWR);
    let _ = spawner;
    loop {
        let distance = get_soner(&mut trig, &mut echo, &rtc, 340.0).await;
        println!("Distance: {:.2} cm", distance);
        Timer::after(Duration::from_secs(1)).await;
    }

}

async fn get_soner(trig: &mut Output<'_>, echo: &mut Input<'_>,
rtc: &Rtc<'_>, sound_velocity: f64) -> f64 {
    trig.set_high();
    handle_delay(embassy_time::Delay, 10).await;
    trig.set_low();
    echo.wait_for_high().await;
    let start = rtc.current_time();
    echo.wait_for_low().await;
    let end = rtc.current_time();
    let result = end - start;
    let distance = (result.num_microseconds().unwrap() as f64) * sound_velocity / 2.0 / 10_000.0;
    return distance

}

async fn handle_delay<D: embedded_hal_async::delay::DelayNs>(mut delay: D, value: u32) {
    delay.delay_us(value).await;
}
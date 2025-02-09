#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    gpio::{Input, Level, Output},
    rtc_cntl::Rtc,
};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

extern crate alloc;

// const PI: f64 = 3.14159265358979323846264338327950288;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();
    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);
    // let ledc = Ledc::new(peripherals.LEDC);
    // let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    // lstimer0
    //     .configure(timer::config::Config {
    //         duty: timer::config::Duty::Duty14Bit,
    //         clock_source: timer::LSClockSource::APBClk,
    //         frequency: 2000_u32.Hz(),
    //     })
    //     .unwrap();
    // let mut channel = ledc.channel(channel::Number::Channel0, peripherals.GPIO3);
    // channel
    //     .configure(channel::config::Config {
    //         timer: &lstimer0,
    //         duty_pct: 10,
    //         pin_config: channel::config::PinConfig::PushPull,
    //     })
    //     .unwrap();
    let mut buzzer = Output::new(peripherals.GPIO3, Level::Low);
    let mut trig = Output::new(peripherals.GPIO1, Level::Low);
    let mut echo = Input::new(peripherals.GPIO0, esp_hal::gpio::Pull::Up);
    let rtc = Rtc::new(peripherals.LPWR);
    let _ = spawner;
    loop {
        let distance = get_soner(&mut trig, &mut echo, &rtc, 340.0).await;
        if distance < 40.0 {
            buzzer.set_high();
        } else {
            buzzer.set_low();
        }
        Timer::after(Duration::from_secs(1)).await;
    }
}

async fn get_soner(
    trig: &mut Output<'_>,
    echo: &mut Input<'_>,
    rtc: &Rtc<'_>,
    sound_velocity: f64,
) -> f64 {
    trig.set_high();
    handle_delay(embassy_time::Delay, 10).await;
    trig.set_low();
    echo.wait_for_high().await;
    let start = rtc.current_time();
    echo.wait_for_low().await;
    let end = rtc.current_time();
    let result = end - start;
    (result.num_microseconds().unwrap() as f64) * sound_velocity / 2.0 / 10_000.0
}

async fn handle_delay<D: embedded_hal_async::delay::DelayNs>(mut delay: D, value: u32) {
    delay.delay_us(value).await;
}

// void alert() {
// float sinVal; // Define a variable to save sine value
// int toneVal; // Define a variable to save sound frequency
// for (int x = 0; x < 360; x += 10) { // X from 0 degree->360 degree
// sinVal = sin(x * (PI / 180)); // Calculate the sine of x
// toneVal = 2000 + sinVal * 500; //Calculate sound frequency according to the sine of x
// ledcWriteTone(PIN_BUZZER, toneVal);
// delay(10);
// 29
// }
// 30
// }

// fn alert(channel: &mut Channel<'_, LowSpeed>) {
//     let mut i = 0;
//     loop {
//         if i >= 360 {
//             break;
//         }
//         let sin_val = libm::sin((i as f64 ) * (PI / 180.0));
//         let tone_val = 2000.0 + sin_val * 500.0;
//         println!("{}, {}", tone_val, tone_val as u32);
//         channel.set_duty_hw(tone_val as u32);
//         i += 10
//     }
// }

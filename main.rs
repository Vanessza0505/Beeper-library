#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]


use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;

use embassy_executor::Spawner;

use esp_println as _;
use defmt::info;
use defmt::error;


use embassy_time::Timer;
use Beeper_library::Beeper;
use esp_hal::gpio::{Output, Level, OutputConfig};



#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");
    let _ = spawner;
    
    let mut buzzer = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());


    let mut beeper = Beeper::new(buzzer, 2, true, 250).await;
    
    beeper.beep_n_times().await;

    // wait 1 second until the next beep
    Timer::after_millis(1000).await;

    loop {}
}

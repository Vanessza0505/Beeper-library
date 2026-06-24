#![no_std]

use embassy_time::Timer;


pub async fn beep(beeper: &mut esp_hal::gpio::Output<'_>, status: bool, beep_number: u32){
    if status == false{
    }
    else{
        for _ in 0..beep_number{
            beeper.set_high();
            Timer::after(Duration::from_millis(1000)).await;
            beeper.set_low();
            Timer::after(Duration::from_millis(1000)).await;
        }
    }
}



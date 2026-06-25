#![no_std]

use embassy_time::Timer;




pub struct Beeper{
    beeper: esp_hal::gpio::Output<'static>,
    beep_number: u32,
    status: bool,
    duration: u64,
}

impl Beeper{
    pub async fn new(
    buzzer: esp_hal::gpio::Output<'static>,
    beep_counter: u32,
    on_or_off: bool,
    duration: u64,
    ) -> Self{
        Self{
        beeper:buzzer,beep_number:beep_counter,
        status:on_or_off,
        duration,
        }
    }


    // smooth beep function
    pub async fn smooth_beep(&mut self){
        self.beeper.set_high();
        Timer::after_millis(self.duration).await;
        self.beeper.set_low();
        Timer::after_millis(self.duration).await;
    }

    // Is it okay to beep? (true = free)
    pub fn free(&self) -> bool {
        self.status
    }

    pub async fn wait_until_it_free(&mut self) {
        while !self.free() {
            Timer::after_millis(10).await;
        }
    }

    // It doesn't beep until it's allowed and it beeping n times
    pub async fn beep_n_times(&mut self) {
        self.wait_until_it_free().await;

        // set close
        self.status = false;

        for _ in 0..self.beep_number {

            self.smooth_beep().await;
        }

        // set free
        self.status = true;
    }

} 

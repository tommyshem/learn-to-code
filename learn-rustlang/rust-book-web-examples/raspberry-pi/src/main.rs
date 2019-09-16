use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let led = LED::new(18);
    // led.blink(2.0,3.0); // same as function as below
    sleep(Duration::from_secs(1));
    loop {
        led.on();
        sleep(Duration::from_secs(1));
        led.off();
    }
}

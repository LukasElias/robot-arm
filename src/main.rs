#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;

use embedded_hal::delay::DelayNs;

use microbit::{
    board::Board,
    hal::{Rng, gpio::Level, pwm::Pwm, time::Hertz, timer::Timer},
};

use robot_arm::ServoInitializinator;

const SERVO_MAX_DEGREES: f32 = 180.0;
const MAX_PULSE_US: f32 = 544.0;
const MIN_PULSE_US: f32 = 2400.0;
const PERIOD: Hertz = Hertz(50); // 20 ms or 20000 us

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);

        let mut rng = Rng::new(board.RNG);

        let servo_0_pin = board.edge.e00.into_push_pull_output(Level::Low).degrade();

        let pwm = Pwm::new(board.PWM0);

        let mut servo_initializinator = ServoInitializinator::new(pwm, SERVO_MAX_DEGREES, MAX_PULSE_US, MIN_PULSE_US, PERIOD);

        let servo_0 = servo_initializinator
            .new_servo(servo_0_pin)
            .expect("Added a servo too much");

        let mut servo_steerinator = servo_initializinator.init();

        let mut last_angle = 0;
        loop {
            let mut new_angle = rng.random_u16() % 180;

            while new_angle - last_angle < 40 {
                new_angle = rng.random_u16() % 180;
            }

            let pause = rng.random_u16() % 1400 + 100;

            servo_steerinator.set_servo_degrees(servo_0, new_angle as f32).unwrap();
            timer.delay_ms(pause as u32);

            last_angle = new_angle;
        }
    }
    panic!("End");
}

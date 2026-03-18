#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;

use embedded_hal::delay::DelayNs;

use microbit::{
    board::Board,
    hal::{gpio::Level, pwm::Pwm, time::Hertz, timer::Timer},
};

use robot_arm::ServoInitializinator;

const SERVO_MAX_DEGREES: f32 = 180.0;
const MAX_PULSE_US: f32 = 1000.0;
const MIN_PULSE_US: f32 = 2000.0;
const PERIOD: Hertz = Hertz(50); // 20 ms or 20000 us

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);

        let servopin = board.edge.e00.into_push_pull_output(Level::Low).degrade();

        let pwm = Pwm::new(board.PWM0);

        let mut servo_initializinator = ServoInitializinator::new(pwm, SERVO_MAX_DEGREES, MAX_PULSE_US, MIN_PULSE_US, PERIOD);

        let channel_id = servo_initializinator
            .new_servo(servopin)
            .expect("Added a servo too much");

        let mut servo_steerinator = servo_initializinator.init();

        loop {
            servo_steerinator.set_servo_degrees(channel_id, 0.0).unwrap();
            timer.delay_ms(1000);

            servo_steerinator.set_servo_degrees(channel_id, 45.0).unwrap();
            timer.delay_ms(1000);

            servo_steerinator.set_servo_degrees(channel_id, 90.0).unwrap();
            timer.delay_ms(1000);

            servo_steerinator.set_servo_degrees(channel_id, 135.0).unwrap();
            timer.delay_ms(1000);

            servo_steerinator.set_servo_degrees(channel_id, 180.0).unwrap();
            timer.delay_ms(1000);
        }
    }
    panic!("End");
}

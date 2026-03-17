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
const MAX_DUTY_PERCENT: f32 = 1.0 / 10.0;
const MIN_DUTY_PERCENT: f32 = 1.0 / 20.0;

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);

        let servopin = board.edge.e00.into_push_pull_output(Level::Low).degrade();

        let pwm = Pwm::new(board.PWM0);

        let mut servo_initializinator = ServoInitializinator::new(pwm, SERVO_MAX_DEGREES, MAX_DUTY_PERCENT, MIN_DUTY_PERCENT);

        servo_initializinator.set_period(Hertz(50));

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

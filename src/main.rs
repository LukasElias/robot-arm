#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;

use microbit::{
    board::Board,
    hal::{gpio::Level, pwm::Pwm, time::Hertz},
};
use robot_arm::ServoGroup;

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let servopin = board.edge.e00.into_push_pull_output(Level::Low).degrade();

        let pwm = Pwm::new(board.PWM0);

        let mut servo_group = ServoGroup::new(pwm);

        servo_group.set_period(Hertz(50));

        let channel_id = servo_group
            .new_servo(servopin)
            .expect("Added a servo too much");

        servo_group.channel_set_degrees(channel_id, 35).unwrap();

        loop {}
    }
    panic!("End");
}

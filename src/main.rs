#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;

use microbit::{
    board::Board,
    hal::{
        pwm::{
            Pwm,
            Channel,
        },
        time::Hertz,
        gpio::Level,
    },
};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let servopin = board.edge.e00.into_push_pull_output(Level::Low).degrade();

        let pwm = Pwm::new(board.PWM0);
        
        pwm.set_output_pin(Channel::C0, servopin);
        
        pwm.set_period(Hertz(50));

        pwm.set_duty_on(Channel::C0, 1500);

        pwm.enable();
        

        loop {}
    }
    panic!("End");
}

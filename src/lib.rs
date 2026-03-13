#![no_std]
#![no_main]

use microbit::hal::{
    gpio::{Output, Pin, PushPull},
    pwm::{Channel, Instance, Pwm},
    time::Hertz,
};

const SERVO_MAX_DEGREES: u16 = 180;

pub struct ServoGroup<T: Instance> {
    pwm: Pwm<T>,
    len: usize,
}

impl<T: Instance> ServoGroup<T> {
    pub fn new(pwm: Pwm<T>) -> Self {
        Self { pwm, len: 0 }
    }

    pub fn set_period(&self, freq: Hertz) {
        self.pwm.set_period(freq);
    }

    pub fn new_servo(&mut self, pin: Pin<Output<PushPull>>) -> Result<Channel, ()> {
        let channel = match self.len {
            0 => Channel::C0,
            1 => Channel::C1,
            2 => Channel::C2,
            3 => Channel::C3,
            _ => return Err(()),
        };

        self.pwm.set_output_pin(channel, pin);
        self.len += 1;

        Ok(channel)
    }

    pub fn channel_set_degrees(&self, channel: Channel, degrees: u16) -> Result<(), ()> {
        let duty = self.degrees_to_duty(degrees)?;

        self.pwm.set_duty_on(channel, duty);

        self.pwm.enable_channel(channel);

        Ok(())
    }

    fn degrees_to_duty(&self, degrees: u16) -> Result<u16, ()> {
        if degrees > SERVO_MAX_DEGREES {
            return Err(());
        }

        let min_duty = self.pwm.max_duty() / 20;

        Ok(min_duty / SERVO_MAX_DEGREES * degrees + min_duty)
    }
}

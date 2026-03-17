#![no_std]
#![no_main]

use cortex_m::singleton;
use microbit::hal::{
    gpio::{Output, Pin, PushPull},
    pwm::{Channel, CounterMode, Instance, LoadMode, Pwm, PwmSeq},
    time::Hertz,
};

pub struct ServoInitializinator<T: Instance> {
    pwm: Pwm<T>,
    len: usize,
    max_degrees: f32,
}

impl<T: Instance + core::fmt::Debug> ServoInitializinator<T> {
    pub fn new(pwm: Pwm<T>, max_degrees: f32) -> Self {
        pwm.set_counter_mode(CounterMode::Up);
        pwm.set_load_mode(LoadMode::Individual);
        pwm.loop_inf();

        Self {
            pwm,
            len: 0,
            max_degrees,
        }
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
        self.pwm.enable_channel(channel);

        self.len += 1;

        Ok(channel)
    }

    pub fn init(self) -> ServoSteerinator<T> {
        let max_duty = self.pwm.max_duty();

        let seq: &'static mut [u16; 4] = singleton!(: [u16; 4] = [2000, 0, 0, 0]).unwrap();

        let seq_ptr = seq as *mut [u16; 4];

        let pwm_seq = self.pwm.load(Some(&*seq), None::<&'static [u16; 4]>, true).unwrap();

        ServoSteerinator {
            _pwm_seq: pwm_seq,
            seq_ptr,
            max_duty,
            max_degrees: self.max_degrees,
        }
    }
}

pub struct ServoSteerinator<T: Instance> {
    _pwm_seq: PwmSeq<T, &'static [u16; 4], &'static [u16; 4]>,
    seq_ptr: *mut [u16; 4],
    max_duty: u16,
    max_degrees: f32,
}

impl<T: Instance> ServoSteerinator<T> {
    pub fn set_servo_degrees(&mut self, channel: Channel, degrees: f32) -> Result<(), ()> {
        let duty = self.degrees_to_duty(degrees)?;

        unsafe {
            (*self.seq_ptr)[channel as usize] = duty;
        }

        Ok(())
    }

    pub fn degrees_to_duty(&self, degrees: f32) -> Result<u16, ()> {
        if degrees > self.max_degrees {
            return Err(());
        }

        let min_duty = self.max_duty as f32 / 20.0;

        Ok((min_duty / self.max_degrees * degrees + min_duty) as u16)
    }
}

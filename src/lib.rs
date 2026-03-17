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
    max_duty_percent: f32,
    min_duty_percent: f32,
    max_degrees: f32,
}

impl<T: Instance + core::fmt::Debug> ServoInitializinator<T> {
    pub fn new(pwm: Pwm<T>, max_degrees: f32, max_duty_percent: f32, min_duty_percent: f32) -> Self {
        pwm.set_counter_mode(CounterMode::Up);
        pwm.set_load_mode(LoadMode::Individual);
        pwm.loop_inf();

        Self {
            pwm,
            len: 0,
            max_duty_percent,
            min_duty_percent,
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
        let max_servo_duty = self.pwm.max_duty() as f32 * self.max_duty_percent;
        let min_servo_duty = self.pwm.max_duty() as f32 * self.min_duty_percent;

        let seq: &'static mut [u16; 4] = singleton!(: [u16; 4] = [0, 0, 0, 0]).unwrap();

        let seq_ptr = seq as *mut [u16; 4];

        let pwm_seq = self.pwm.load(Some(&*seq), None::<&'static [u16; 4]>, true).unwrap();

        defmt::info!("max: {} min: {}", max_servo_duty, min_servo_duty);

        ServoSteerinator {
            _pwm_seq: pwm_seq,
            seq_ptr,
            max_servo_duty,
            min_servo_duty,
            max_degrees: self.max_degrees,
        }
    }
}

pub struct ServoSteerinator<T: Instance> {
    _pwm_seq: PwmSeq<T, &'static [u16; 4], &'static [u16; 4]>,
    seq_ptr: *mut [u16; 4],
    max_servo_duty: f32,
    min_servo_duty: f32,
    max_degrees: f32,
}

impl<T: Instance> ServoSteerinator<T> {
    pub fn set_servo_degrees(&mut self, channel: Channel, degrees: f32) -> Result<(), ()> {
        let duty = self.degrees_to_duty(degrees)?;

        unsafe {
            (*self.seq_ptr)[channel as usize] = duty & 0x7FFF;
            defmt::info!("{}", *self.seq_ptr);
        }

        Ok(())
    }

    pub fn degrees_to_duty(&self, degrees: f32) -> Result<u16, ()> {
        if degrees > self.max_degrees {
            return Err(());
        }

        Ok(((self.max_servo_duty - self.min_servo_duty) / self.max_degrees * degrees + self.min_servo_duty) as u16)
    }
}

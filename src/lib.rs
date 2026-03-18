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
    max_pulse_us: f32,
    min_pulse_us: f32,
    max_degrees: f32,
}

impl<T: Instance + core::fmt::Debug> ServoInitializinator<T> {
    pub fn new(pwm: Pwm<T>, max_degrees: f32, max_pulse_us: f32, min_pulse_us: f32) -> Self {
        pwm.set_counter_mode(CounterMode::Up);
        pwm.set_load_mode(LoadMode::Individual);
        pwm.loop_inf();

        Self {
            pwm,
            len: 0,
            max_pulse_us,
            min_pulse_us,
            max_degrees,
        }
    }

    pub fn set_period(&self, freq: Hertz) {
        self.pwm.set_prescaler(microbit::hal::pwm::Prescaler::Div16);
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
        let period_us = (1_000_000 / self.pwm.period().0) as f32;

        let seq: &'static mut [u16; 4] = singleton!(: [u16; 4] = [0, 0, 0, 0]).unwrap();

        let seq_ptr = seq as *mut [u16; 4];

        let pwm_seq = self.pwm.load(Some(&*seq), None::<&'static [u16; 4]>, true).unwrap();

        defmt::info!("{}", period_us);

        ServoSteerinator {
            pwm_seq,
            seq_ptr,
            max_pulse_us: self.max_pulse_us,
            min_pulse_us: self.min_pulse_us,
            max_degrees: self.max_degrees,
            max_duty,
            period_us,
        }
    }
}

pub struct ServoSteerinator<T: Instance> {
    pwm_seq: PwmSeq<T, &'static [u16; 4], &'static [u16; 4]>,
    seq_ptr: *mut [u16; 4],
    max_pulse_us: f32,
    min_pulse_us: f32,
    max_degrees: f32,
    max_duty: u16,
    period_us: f32,
}

impl<T: Instance> ServoSteerinator<T> {
    pub fn set_servo_degrees(&mut self, channel: Channel, degrees: f32) -> Result<(), ()> {
        let duty = self.degrees_to_duty(degrees)?;

        defmt::info!("{}", duty);

        self.set_servo_duty(channel, duty);

        Ok(())
    }

    pub fn set_servo_duty(&mut self, channel: Channel, duty: u16) {
        unsafe {
            (*self.seq_ptr)[channel as usize] = duty | 0x8000;
        }

        self.pwm_seq.start_seq(microbit::hal::pwm::Seq::Seq0);
    }

    pub fn degrees_to_duty(&self, degrees: f32) -> Result<u16, ()> {
        if degrees > self.max_degrees {
            return Err(());
        }

        let pulse_us = self.min_pulse_us + (self.max_pulse_us - self.min_pulse_us) * (degrees / self.max_degrees);

        let duty = (pulse_us / self.period_us) * (self.max_duty as f32);

        Ok(duty as u16)
    }
}

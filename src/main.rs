use rppal::pwm::{Channel, Pwm, Polarity};
use std::{thread, time::Duration};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Создаем PWM
    let mut pwm = Pwm::new(Channel::Pwm0)?;

    // Настраиваем параметры
    pwm.set_frequency(100.0, 10.0)?;  // 100 Hz
    pwm.enable()?;

    // Устанавливаем скорость (0.0 - 1.0)
    pwm.set_duty_cycle(0.5)?; // 50% скорости

    thread::sleep(Duration::from_secs(10));
    Ok(())
}
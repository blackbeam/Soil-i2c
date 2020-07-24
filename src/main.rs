extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate rppal;
extern crate stemma_soil_sensor;

fn muxer(channel: u8) -> Result<(), rppal::i2c::Error> {
    use rppal::i2c::I2c;
    const MUXADDR: u16 = 0x70;

    let mut mdev = I2c::new()?;
    mdev.set_slave_address(MUXADDR)?;

    let channelbyte = &[0u8, channel];
    mdev.write(channelbyte)?;

    Ok(())
}

fn sense(interval_ms: u64) {
    use embedded_hal::blocking::delay::DelayMs;
    use linux_embedded_hal::Delay;
    use stemma_soil_sensor::SoilSensor;

    let delay = Delay {};
    let mut sensor = SoilSensor::init(delay).unwrap();

    loop {
        let temp = sensor.get_temp().unwrap();
        let cap = sensor.get_capacitance().unwrap();
        println!("The temperature is: {:.02}", temp);
        println!("The capacitance is: {}", cap);
        let mut delay = Delay {};
        delay.delay_ms(4000u32);
    }
}

fn main() {
    loop {
        muxer(1);
        println!("\nSwitching to multiplex port 1: \n\n");
        sense(2000);

        muxer(8);
        println!("\nSwitching to multiplex port 1: \n\n");
        sense(2000);
    }
}

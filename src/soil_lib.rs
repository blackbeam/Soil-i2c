extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate rppal;
extern crate stemma_soil_sensor;

pub mod i2conn {
    //muxer: Writes a single byte to the first register of the multiplexer,
    //changing the port that it will be listening on.
    //
    pub fn muxer(channel: u8) -> Result<(), rppal::i2c::Error> {
        use soil_lib::rppal::i2c::I2c;
        const MUXADDR: u16 = 0x70;

        let mut mdev = I2c::new()?;
        mdev.set_slave_address(MUXADDR)?;

        let channelbyte = &[0u8, channel];
        mdev.write(channelbyte)?;

        Ok(())
    }
}

pub mod stemconn {
    use soil_lib::embedded_hal::blocking::delay::DelayMs;
    use soil_lib::linux_embedded_hal::Delay;
    use soil_lib::stemma_soil_sensor::SoilSensor;

    //sensetemp/sensecap: Stolen from Carl's example program; designed to open a 
    //connection to a soil sensor, read the temperature/capacitance, return it,
    //then wait for the provided amount of seconds before another action is taken.
    //
    pub fn sensetemp(interval_ms: u32) -> f32 {
        let delay = Delay {};
        let mut sensor = SoilSensor::init(delay).unwrap();

        let temp = sensor.get_temp().unwrap();

        let mut delay = Delay {};
        delay.delay_ms(interval_ms);

        return temp;
    }
    pub fn sensecap(interval_ms: u32) -> u16 {
        let delay = Delay {};
        let mut sensor = SoilSensor::init(delay).unwrap();

        let cap = sensor.get_capacitance().unwrap();

        let mut delay = Delay {};
        delay.delay_ms(interval_ms);

        return cap;
    }
}

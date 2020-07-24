extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate rppal;
extern crate stemma_soil_sensor;

//muxer: Writes a single byte to the first register of the multiplexer,
//changing the port that it will be listening on.
//
fn muxer(channel: u8) -> Result<(), rppal::i2c::Error> {
    use rppal::i2c::I2c;
    const MUXADDR: u16 = 0x70;

    let mut mdev = I2c::new()?;
    mdev.set_slave_address(MUXADDR)?;

    let channelbyte = &[0u8, channel];
    mdev.write(channelbyte)?;

    Ok(())
}

//sense: Stolen from Carl's example program; designed to open a connection
//to a soil sensor, read the temperature/capacitance, print both, then wait
//for the provided amount of seconds before another action is taken.
//
fn sense(interval_ms: u32) {
    use embedded_hal::blocking::delay::DelayMs;
    use linux_embedded_hal::Delay;
    use stemma_soil_sensor::SoilSensor;

    let delay = Delay {};
    let mut sensor = SoilSensor::init(delay).unwrap();

    let temp = sensor.get_temp().unwrap();
    let cap = sensor.get_capacitance().unwrap();

    println!("Soil Temperature: {:.02}", temp);
    println!("Soil Moisture: {}", cap);
    let mut delay = Delay {};
    delay.delay_ms(interval_ms);
}

//main: Puts the previous functions together, swapping multiplexer ports
//then opening a new connection to the 0x36 address.
//
//(Had to do it this way for now because I2C doesn't like it when you
//establish a connection and swap the multiplexer port halfway through.)
//
fn main() {
    muxer(1);
    println!("\nSwitching to multiplex port 1: \n\n");
    sense(2000);

    muxer(2);
    println!("\nSwitching to multiplex port 2: \n\n");
    sense(2000);
}

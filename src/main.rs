mod soil_lib;

pub use crate::soil_lib::i2conn;

//sensor_read: This function is essentially just an alias to combine the muxer
//and sense functions into one, and to handle the error. (Look at me, handling
//errors like a normal programmer :P)
//

fn mx_channel(channel: u8) {
    match i2conn::muxer(channel) {
        Ok(n) => n,
        Err(err) => println!("Error communicating with multiplexer!: {}", err),
    }
}

//main: Wraps the whole thing together. Uses the sensor_read function to bring
//back values from the soil sensors, switching between multiplexer ports 1 and
//2 before each reading.
//
fn main() {
    let mut temp: f32;
    let mut cap: u16;

    let ports_begin: u8 = 1;
    let ports_end: u8 = 2;
    
    for r in 1..11 {
        println!("Pass #{}:",r);
        for i in ports_begin..ports_end + 1 {
            mx_channel(i);
            println!("\nSwitching to multiplex port {}: \n\n", i);
        
            temp = i2conn::sensetemp(500);
            cap = i2conn::sensecap(500);
            println!("Temperature: {}\nCapacitance: {}\n", temp, cap);
        }
    }
}

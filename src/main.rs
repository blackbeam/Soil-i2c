mod soil_lib;

pub use crate::soil_lib::i2conn;
pub use crate::soil_lib::stemconn;

//mx_channel: This is basically just an alias for the i2conn::muxer function that 
//handles the error. (Look at me, handling errors like a normal programmer :P)
//
fn mx_channel(channel: u8) {
    match i2conn::muxer(channel) {
        Ok(n) => n,
        Err(err) => println!("Error communicating with multiplexer!: {}", err),
    }
}

//main: Wraps the whole thing together. Using two constraint variables as my
//sentinel values for swapping multiplexer ports. Takes 10 readings for each
//port in the range of my constraints, fetching both the temperature and 
//capacitance values at a 500ms delay, and prints them both out on one line.
//
//Nothing fancy, but it's pretty flexible.
//Database functionality coming soon! (i.e. whenever I can learn how to implement ODBC...)
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
        
            temp = stemconn::sensetemp(500);
            cap = stemconn::sensecap(500);
            println!("Temperature: {}\nCapacitance: {}\n", temp, cap);
        }
    }
}

extern crate mysql;

mod soil_lib;

pub use crate::soil_lib::i2conn;
pub use crate::soil_lib::stemconn;

mod data_mysql {
    pub fn insert(plnum: u8, tempwrite: f32, moistwrite: u16) -> Result<(), mysql::Error> {
        use mysql::*;
        use mysql::prelude::*;

        let url = "mysql://soil:Test1234@localhost:3307/SoilTest1";
        let pool = Pool::new(url)?;
        let mut conn = pool.get_conn()?;
   
        let plantno = format!("Plant {}", plnum);
        
        conn.exec_drop(
            r"INSERT INTO SoilData (Plant,Readtime,Moisture,Temperature) VALUES (?,CURRENT_TIME,?,?)", (plantno, moistwrite, tempwrite)
        )?;

        Ok(())
    }
}
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
            
            println!("Writing to MySQL database...");
            match data_mysql::insert(i, temp, cap) {
               Ok(m) => m,
               Err(err) => println!("Error writing to database!!! {}", err),
            }
        }
    }
}

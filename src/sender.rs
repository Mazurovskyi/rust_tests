use serial::SystemPort;
use std::io::prelude::Write;
use std::thread;
use std::time::{Duration, SystemTime};

use std::sync::{Arc, Mutex};

use chrono::{offset::Utc, DateTime};

pub fn run(duration: u64, port: Arc<Mutex<SystemPort>>, data: &mut [u8]){
    //reverse(data);
    loop{

        //for el in data{
            dbg!("Writing data..");
            match port.lock().unwrap().write(data){
                Ok(n) => {
                    let datetime: DateTime<Utc> = SystemTime::now().into();
                    println!("Successfully writing {n} bytes.      msg: {data:?}     time: {}", datetime.format("%d/%m/%Y %T"))
                },
                Err(err) => println!("WRITTING ERROR: {err}")
            }
            thread::sleep(Duration::from_millis(duration));
        //}
        println!("")
    }
    //dbg!("Out of loop run scope");
}

fn reverse(data: &mut[u8]){
    for el in data.iter_mut(){
        *el = !*el;
    }
    println!("REVERSE: {:?}",data)
}
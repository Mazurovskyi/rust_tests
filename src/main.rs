mod sender;

extern crate serial;
use serial:: {SystemPort, prelude::SerialPort};

extern crate chrono;
use std::time::Duration;
use std::{thread, env, error::Error};

use std::sync::{Arc, Mutex};

use std::any::Any;
fn main()-> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    let path = args.get(1).ok_or("Arguments error! Path to port")?;
    let duration = args.get(2).ok_or("Arguments error! Message interval (millisec)")?;
    let duration:u64 = duration.parse()?;

    let data:Vec<u8> = (&args[3..11]).iter().map(|el| el.parse().unwrap()).collect();

    let port = config(&path)?;

    if let Err(err) = run(port, duration,data){
        return Err(format!("{err:?}").into())
    }
    dbg!("main end..");
    Ok(())
}



pub fn config(path: &str)->Result<SystemPort, Box<dyn Error>>{

    let mut port = serial::open(path)?;

    port.reconfigure(&|config|{
        config.set_baud_rate(serial::Baud115200)?;
        config.set_char_size(serial::Bits8);
        config.set_parity(serial::ParityNone);
        config.set_stop_bits(serial::Stop1);
        config.set_flow_control(serial::FlowNone);

        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(2))?;   

    Ok(port)
}



pub fn run(port: SystemPort, duration: u64, mut data: Vec<u8>)->Result<(), Box<dyn Any + Send>> {

    //let get_sign =       [0,1,2,3,4,5,6,7];
    //let get_voltage =    new(0x11, 0x03, 0x04, 0x01);
    //let read_dc_status = new(0x11, 0x03, 0x17, 0x01);
    //let handshake =      new(0x01, 0x06, 0x50, 0x00);

    //let data = [data.as_slice()];

    let port = Arc::new(Mutex::new(port));
    let sender_port = Arc::clone(&port);
    
    let sender = move||{
        sender::run(duration, sender_port, data.as_mut_slice())
    };

    let handle = thread::spawn(sender);
    handle.join()?;
    dbg!("Run end");
    Ok(())
}

pub fn new(addr:u8, code:u8, offset:u16, count:u16)->[u8; 8]{
    
    let (offset_h, offset_l) = into_8_bit(offset);
    let (count_h, count_l) = into_8_bit(count);

    let msg = [addr, code, offset_h, offset_l, count_h, count_l];

    let (crc_h, crc_l) = into_8_bit(crc(&msg));

    let msg_new = [addr, code, offset_h, offset_l, count_h, count_l, crc_l, crc_h]; //choose crc bytes priority

    //reverse(& mut msg_new); 

    msg_new
}

fn crc(data: &[u8])->u16{

    let mut crc:u16 = 0xFFFF;

    for el in data.iter(){

        crc^= *el as u16;

        for _i in 8..0{
            if (crc & 0x0001) != 0{
                crc >>= 1;
                crc ^= 0xA001;
            }
            else{
                crc >>=1;
            }
        }
    }
    !crc
}

fn into_8_bit(val: u16)->(u8,u8){
    let high = ((val & (0xFF00 as u16)) >> 8) as u8;
    let low = (val & (0x00FF as u16)) as u8;
    (high, low)
}

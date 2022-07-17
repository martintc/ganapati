use std::time::Duration;
use std::str;

use serialport::DataBits;

fn main() -> ! {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
	println!("{}", p.port_name);
    }
    let mut port = serialport::new("/dev/tty.usbserial-AB0LBV6H", 115_200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    // let mut serial_buf: Vec<u8> = vec![0; 32];
    // port.read(serial_buf.as_mut_slice()).expect("No data found!");
    // let string = match str::from_utf8(&serial_buf) {
    // 	Ok(s) => s,
    // 	Err(_) => "Not able to convert data.",
    // };

    // println!("{:?}", serial_buf);
    // println!("{string}");
    let mut total_bytes = 0;
    // just continually receive
    loop {
	    let bytes: u32 = match port.bytes_to_read() {
	        Ok(b) => b,
	        Err(e) => {
		        println!("Error {:?}", e);
		        continue;
	        }
	    };

        total_bytes += bytes;
        if bytes == 0 {
            //print!(".");
            continue;
        }
        println!("");
        println!("Bytes recieved: {}", bytes);
	    let mut buffer: Vec<u8> = vec![0; bytes.try_into().unwrap()];
    	// let size = port.read(buffer.as_mut_slice()).expect("Was not able to read data from serial port");
        let size = port.read(buffer.as_mut_slice());
        match size {
            Ok(s) => {
                println!("size: {}", s);
            },
            Err(e) => {
                panic!("An error occured: {}", e);
            }
        };
	    for data in buffer.iter() {
            print!("{} ", data);
        }
        print!("\n");
        let string = str::from_utf8(&buffer);
        match string {
            Ok(s) => println!("{}", s),
            Err(r) => panic!("An error occured: {}", r),
        }

        println!("\nTotal bytes recv: {}", total_bytes);
    }
}

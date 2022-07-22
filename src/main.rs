use std::str;
use std::time::Duration;

use serialport::DataBits;
use serialport::SerialPort;

fn communication(port: &mut Box<dyn SerialPort>) {
    port.write("a".as_bytes()).expect("Write failed");
    let mut has_received: bool = false;
    while !has_received {
        let bytes: u32 = match port.bytes_to_read() {
            Ok(b) => b,
            Err(e) => panic!("An error has occured attempting to received: {}", e),
        };
        if bytes == 0 {
            continue;
        }
        let mut buffer: Vec<u8> = vec![0; bytes.try_into().unwrap()];
        port.read(buffer.as_mut_slice()).expect("An error occured trying to read in messagee");
        let recv_data = str::from_utf8(&buffer);
        match recv_data {
            Ok(s) => {
                println!("Message received: {}", s);
                has_received = true;
            },
            Err(e) => panic!("An error occured writing received data: {}", e),
        }
    }
}

fn wait_for_device(port: &mut Box<dyn SerialPort>) {
    let mut data: String = String::from("");
    while data != "Hello" {
        let bytes: u32 = match port.bytes_to_read() {
            Ok(b) => b,
            Err(e) => panic!("An errored occured: {}", e),
        };
        if bytes == 0 {
            continue;
        }
        let mut buffer: Vec<u8> = vec![0; bytes.try_into().unwrap()];
        let _ = port.read(buffer.as_mut_slice());
        let recv_data = str::from_utf8(&buffer);
        match recv_data {
            Ok(s) => {
                println!("{}", s);
                data = s.to_string();
            },
            Err(e) => panic!("An errored occured: {}", e),
        }
    }
}

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
    let mut port = serialport::new("/dev/tty.usbserial-AB0LBV6H", 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    wait_for_device(&mut port);

    for i in 0..5 {
        communication(&mut port);
    }

    println!("End of commnication");

}

use serialport::SerialPort;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = select_port()?;
    let app = birdview::BirdView::new(port)?;
    eframe::run_native(Box::new(app), eframe::NativeOptions::default());

    Ok(())
}

fn select_port() -> serialport::Result<Box<dyn SerialPort>> {
    let ports = serialport::available_ports().expect("Could not get any serial ports");
    println!("Available ports:");
    for (i, p) in ports.iter().enumerate() {
        println!("[{}]: {}", i, p.port_name);
    }

    let mut buf = String::new();

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let port_info = loop {
        buf.clear();
        print!("Select a port: ");
        stdout.flush().unwrap();
        stdin.read_line(&mut buf).unwrap();

        let port_index: usize = match buf.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                eprintln!("Please enter a valid index");
                continue;
            }
        };

        match ports.get(port_index) {
            Some(i) => break i,
            None => {
                eprintln!("Please enter a valid port index");
                continue;
            }
        }

        

    };

    loop {
        buf.clear();
        print!("Enter baud rate: ");
        stdout.flush().unwrap();
        stdin.read_line(&mut buf).unwrap();

        let baud: u32 = match buf.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                eprintln!("Please enter a valid baud rate");
                continue;
            }
        };

        break serialport::new(&port_info.port_name, baud)
            .open();

    }

}
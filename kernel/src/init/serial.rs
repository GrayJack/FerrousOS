use crate::uart::{PortAddress, SerialPort};

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(PortAddress::COM1) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

lazy_static! {
    pub static ref SERIAL2: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(PortAddress::COM2) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

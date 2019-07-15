//! Very basic Universal Asynchronous Receiver-Transmitter 16550 (UART) implementation

// Got from uart_16550 package, but it doesn't have a git repository and hasn't been updated in 8 months

use core::{
    fmt::{self, Write}
};
use bitflags::bitflags;
use x86_64::instructions::port::Port;

/// The port adresses known.
///
/// COM1 and COM2 are for sure the right addresses,
/// but COM3 and COM4 are less reliable, so use with caution.
pub enum PortAddress {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8
}

bitflags! {
    /// Interrupt enable flags
    struct IntEnFlags: u8 {
        const RECEIVED = 1;
        const SENT = 1 << 1;
        const ERRORED = 1 << 2;
        const STATUS_CHANGE = 1 << 3;
        // 4 to 7 are unused
    }
}

bitflags! {
    /// Line status flags
    struct LineStsFlags: u8 {
        const INPUT_FULL = 1;
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5;
        // 6 and 7 unknown
    }
}

/// Serial Port struct
#[derive(Debug, Clone, PartialEq)]
pub struct SerialPort {
    data: Port<u8>,
    int_en: Port<u8>,
    fifo_ctrl: Port<u8>,
    line_ctrl: Port<u8>,
    modem_ctrl: Port<u8>,
    line_sts: Port<u8>,
}

impl SerialPort {
    /// Create a new instance of SerialPort.
    // pub const fn new(base: PortAddress) -> SerialPort {
    pub fn new(base: PortAddress) -> Self {
        let base = base as u16;
        SerialPort {
            data: Port::new(base),
            int_en: Port::new(base + 1),
            fifo_ctrl: Port::new(base + 2),
            line_ctrl: Port::new(base + 3),
            modem_ctrl: Port::new(base + 4),
            line_sts: Port::new(base + 5),
        }
    }

    // Initialize serial port
    pub fn init(&mut self) {
        unsafe {
            self.int_en.write(0x00);
            self.line_ctrl.write(0x80);
            self.data.write(0x03);
            self.int_en.write(0x00);
            self.line_ctrl.write(0x03);
            self.fifo_ctrl.write(0xC7);
            self.modem_ctrl.write(0x0B);
            self.int_en.write(0x01);
        }
    }

    fn line_sts(&mut self) -> LineStsFlags {
        unsafe {
            LineStsFlags::from_bits_truncate(self.line_sts.read())
        }
    }

    /// Put serial port to receive data
    // Not sure if that is right!!!! Remember to look at this later!!
    pub fn receive(&mut self) {
        unsafe {
            while self.line_sts().contains(LineStsFlags::INPUT_FULL) {}
            self.data.read();
        }
    }

    /// Put serial port to send data
    pub fn send(&mut self, data: u8) {
        unsafe {
            match data {
                8 | 0x7F => {
                    while !self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY) {}
                    self.data.write(8);
                    while !self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY) {}
                    self.data.write(b' ');
                    while !self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY) {}
                    self.data.write(8);
                },
                _ => {
                    while !self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY) {}
                    self.data.write(data);
                }
            }
        }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}

//! Traits to describe Serial port (UART) functionality.
//!
//! Particularly useful for embedded devices.

#![no_std]

pub trait BlockingTx {
    /// Write a single octet to the port's transmitter,
    /// blocking until the character can be stored in the buffer
    /// (not necessarily that the character has been transmitted).
    fn putc(&mut self, ch: u8);
}

pub trait NonBlockingTx {
    /// Try and write a single octet to the port's transmitter.
    /// Will return Err(()) if the FIFO/buffer was full
    /// and the character couldn't be stored.
    fn putc_try(&mut self, ch: u8) -> Result<(),()>;
}

pub trait BlockingRx {
    /// Read a single octet from the port's receiver,
    /// blocking until the character can be read from the buffer.
    fn getc(&mut self) -> u8;
}

pub trait BlockingRxWithTimeout<T> {
    /// Read a single octet from the port's receiver,
    /// blocking until the character can be read from the buffer.
    fn getc(&mut self, timeout: T) -> u8;
}

pub trait NonBlockingRx {
    /// Read a single octet from the port's receiver; if the buffer
    /// is empty 
    fn getc_try(&mut self) -> Option<u8>;
}

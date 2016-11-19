//! Traits to describe Serial port (UART) functionality.
//!
//! Particularly useful for embedded devices.

#![no_std]

pub trait BlockingTx {
    type Error;

    /// Write a single octet to the port's transmitter,
    /// blocking until the character can be stored in the buffer
    /// (not necessarily that the character has been transmitted).
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn putc(&mut self, ch: u8) -> Result<(), Self::Error>;
}

pub trait NonBlockingTx {
    type Error;

    /// Try and write a single octet to the port's transmitter.
    /// Will return `Err` if the FIFO/buffer was full
    /// and the character couldn't be stored.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn putc_try(&mut self, ch: u8) -> Result<(), Self::Error>;
}

pub trait BlockingRx {
    type Error;

    /// Read a single octet from the port's receiver,
    /// blocking until the character can be read from the buffer.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn getc(&mut self) -> Result<u8, Self::Error>;
}

pub trait BlockingRxWithTimeout {
    type Timeout;
    type Error;

    /// Read a single octet from the port's receiver,
    /// blocking until the character can be read from the buffer.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn getc(&mut self, timeout: Self::Timeout) -> Result<u8, Self::Error>;
}

pub trait NonBlockingRx {
    type Error;

    /// Read a single octet from the port's receiver; if the buffer
    /// is empty 
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn getc_try(&mut self) -> Result<u8, Self::Error>;
}

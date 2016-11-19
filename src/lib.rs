//! Traits to describe Serial port (UART) functionality.
//!
//! Particularly useful for embedded devices.

#![no_std]
#![feature(never_type)]

pub trait BlockingTx<E> {
    /// Write a single octet to the port's transmitter,
    /// blocking until the character can be stored in the buffer
    /// (not necessarily that the character has been transmitted).
    fn putc(&mut self, ch: u8) -> Result<(), E>;
}

pub trait NonBlockingTx<E> {
    /// Try and write a single octet to the port's transmitter.
    /// Will return Err(()) if the FIFO/buffer was full
    /// and the character couldn't be stored.
    fn putc_try(&mut self, ch: u8) -> Result<(),E>;
}

pub trait BlockingRx<E> {
    /// Read a single octet from the port's receiver,
    /// blocking until the character can be read from the buffer.
    fn getc(&mut self) -> Result<u8, E>;
}

pub trait BlockingRxWithTimeout<T, E> {
    /// Read a single octet from the port's receiver,
    /// blocking until the character can be read from the buffer.
    fn getc(&mut self, timeout: T) -> Result<u8, E>;
}

pub trait NonBlockingRx<E> {
    /// Read a single octet from the port's receiver; if the buffer
    /// is empty 
    fn getc_try(&mut self) -> Result<u8, E>;
}

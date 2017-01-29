//! Traits to describe Serial port (UART) functionality.
//!
//! This crate contains traits that are suitable for embedded development. This allows developers to produce crates that depend upon generic UART functionality (for example, an AT command interface), allowing the application developer to combine the crate with the specific UART available on their board.
//!
//! It is similar to the C idea of using the functions `getc` and `putc` to decouple the IO device from the library, but in a more Rustic fashion.
//!
//! Here's an example with the `BlockingTx` trait.
//!
//! ```
//! struct SomeStruct<T> { uart: T };
//! impl<T> SomeStruct<T> where T: embedded_serial::BlockingTx {
//!     fn new(uart: T) -> SomeStruct<T> {
//!         SomeStruct { uart: uart }
//!     }
//!
//!     fn write_data(&mut self) -> Result<(), <T as embedded_serial::BlockingTx>::Error> {
//!         self.uart.puts(b"AT\n")?;
//!         Ok(())
//!     }
//! }
//! ```
//!
//! Here's an example with the `BlockingTxWithTimeout` trait.
//!
//! ```
//! struct SomeStruct<T> { uart: T };
//! impl<T> SomeStruct<T> where T: embedded_serial::BlockingTxWithTimeout {
//!     fn new(uart: T) -> SomeStruct<T> {
//!         SomeStruct { uart: uart }
//!     }
//!
//!     fn write_data(&mut self, timeout: &<T as embedded_serial::BlockingTxWithTimeout>::Timeout) -> Result<bool, <T as embedded_serial::BlockingTxWithTimeout>::Error> {
//!         let len = self.uart.puts_to(b"AT\n", timeout)?;
//!         Ok(len == 3)
//!     }
//! }
//! ```
//!
//! Here's an example with the `NonBlockingTx` trait. You would call the `write_data` function until it returned `Ok(true)`.
//!
//! ```
//! struct SomeStruct<T> {
//!     sent: Option<usize>,
//!     uart: T
//! };
//! impl<T> SomeStruct<T> where T: embedded_serial::NonBlockingTx {
//!     fn new(uart: T) -> SomeStruct<T> {
//!         SomeStruct { uart: uart, sent: Some(0) }
//!     }
//!
//!     fn write_data(&mut self) -> Result<bool, <T as embedded_serial::NonBlockingTx>::Error> {
//!         let data = b"AT\n";
//!         if let Some(len) = self.sent {
//!             match self.uart.puts_try(&data[len..]) {
//!                 (_, Ok(Some(()))) => { self.sent = None; Ok(true) },
//!                 (sent, Ok(None)) => {
//!                     let total = len + sent;
//!                     if total == data.len() {
//!                         self.sent = None
//!                     } else {
//!                         self.sent = Some(total)
//!                     };
//!                     Ok(false)
//!                 }
//!                 (sent, Err(e)) => {
//!                     let total = len + sent;
//!                     if total == data.len() {
//!                         self.sent = None
//!                     } else {
//!                         self.sent = Some(total)
//!                     };
//!                     Err(e)
//!                 }
//!             }
//!         } else {
//!             Ok(true)
//!         }
//!     }
//! }
//! ```

#![no_std]
#![deny(missing_docs)]

/// Implementors of this trait offer octet based serial data transmission
/// using a blocking API and requiring a mutable reference to self.
pub trait BlockingTx {
    /// The error type returned if a function fails.
    type Error;

    /// Write a single octet to the port's transmitter,
    /// blocking until the character can be stored in the buffer
    /// (not necessarily that the character has been transmitted).
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn putc(&mut self, ch: u8) -> Result<(), Self::Error>;

    /// Write a complete string to the UART.
    /// Returns number of octets written, or an error.
    fn puts(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
        for octet in data {
            self.putc(*octet)?
        }
        Ok(data.len())
    }
}

/// Implementors of this trait offer octet based serial data transmission
/// using a blocking API with an upper bound on blocking time, and requiring a
/// mutable reference to self.
pub trait BlockingTxWithTimeout {
    /// The type used to specify the timeout.
    type Timeout;
    /// The error type returned if a function fails.
    type Error;

    /// Write a single octet to the port's transmitter, blocking until the
    /// character can be stored in the buffer (not necessarily that the
    /// character has been transmitted) or some timeout occurs.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    ///
    /// If it times out, `Ok(None)` is returned.
    /// If it sends the data, `Ok(Some(ch))` is returned.
    /// If it fails, `Err(...)` is returned.
    fn putc_to(&mut self, ch: u8, timeout: &Self::Timeout) -> Result<Option<u8>, Self::Error>;

    /// Write a complete string to the UART.
    /// Returns number of octets written, or an error.
    /// The timeout applies to each octet individually.
    fn puts_to(&mut self, data: &[u8], timeout: &Self::Timeout) -> Result<usize, Self::Error> {
        let mut count: usize = 0;
        for octet in data {
            if self.putc_to(*octet, timeout)?.is_none() {
                break;
            }
            count = count + 1;
        }
        Ok(count)
    }
}

/// Implementors of this trait offer octet based serial data transmission
/// using a non-blocking API and requiring a mutable reference to self.
pub trait NonBlockingTx {
    /// The error type returned if function fails.
    type Error;

    /// Try and write a single octet to the port's transmitter.
    /// Will return `Ok(None)` if the FIFO/buffer was full
    /// and the character couldn't be stored or `Ok(Some(()))`
    /// if it was stored OK.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn putc_try(&mut self, ch: u8) -> Result<Option<()>, Self::Error>;

    /// Write as much of a complete string to the UART as possible.
    /// Returns the number of octets sent, plus the result from the
    /// last `putc_try` call. Aborts early if `putc_try` fails in any way.
    fn puts_try(&mut self, data: &[u8]) -> (usize, Result<Option<()>, Self::Error>) {
        let mut count = 0;
        for octet in data {
            match self.putc_try(*octet) {
                Ok(Some(_)) => { count = count + 1 },
                Ok(None) => return (count, Ok(None)),
                Err(e) => return (count, Err(e)),
            }
        }
        (count, Ok(Some(())))
    }
}

/// Implementors of this trait offer octet based serial data reception
/// using a blocking API and requiring a mutable reference to self.
pub trait BlockingRx {
    /// The error type returned if a function fails.
    type Error;

    /// Read a single octet from the port's receiver,
    /// blocking until the character can be read from the buffer.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn getc(&mut self) -> Result<u8, Self::Error>;
}

/// Implementors of this trait offer octet based serial data reception using a
/// blocking API with an upper bound on blocking time, and requiring a mutable
/// reference to self.
pub trait BlockingRxWithTimeout {
    /// The type used to specify the timeout.
    type Timeout;
    /// The error type returned if `getc` fails.
    type Error;

    /// Read a single octet from the port's receiver,
    /// blocking until the character can be read from the buffer.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    ///
    /// If it times out, Ok(None) is returned.
    /// If it receives data, Ok(Some(data)) is returned.
    /// If it fails, Err(...) is returned.
    fn getc_to(&mut self, timeout: &Self::Timeout) -> Result<Option<u8>, Self::Error>;
}

/// Implementors of this trait offer octet based serial data reception using a
/// non-blocking API, and requiring a mutable reference to self.
pub trait NonBlockingRx {
    /// The error type returned if `getc` fails.
    type Error;

    /// Attempt to read a single octet from the port's receiver; if the buffer
    /// is empty return None.
    ///
    /// In some implementations, this can result in an Error. If not, use
    /// `type Error = !`.
    ///
    /// If it times out, Ok(None) is returned.
    /// If it receives data, Ok(Some(data)) is returned.
    /// If it fails, Err(...) is returned.
    fn getc_try(&mut self) -> Result<Option<u8>, Self::Error>;
}

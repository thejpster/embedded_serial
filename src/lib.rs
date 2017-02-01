//! # Embedded Serial traits
//!
//! Traits to describe Serial port (UART) functionality.
//!
//! A serial port is taken here to mean a device which can send and/or receive
//! data one octet at a time, in order. Octets are represented using the `u8`
//! type. We are careful here to talk only in octets, not characters (although
//! if you ASCII or UTF-8 encode your strings, they become a sequence of
//! octets).
//!
//! This crate contains traits that are suitable for embedded development.
//! This allows developers to produce crates that depend upon generic UART
//! functionality (for example, an AT command interface), allowing the
//! application developer to combine the crate with the specific UART
//! available on their board.
//!
//! It is similar to the C idea of using the functions `getc` and `putc` to
//! decouple the IO device from the library, but in a more Rustic fashion.
//!
//! Here's an example with the `MutBlockingTx` trait.
//!
//! ```
//! use embedded_serial::MutBlockingTx;
//!
//! struct SomeStruct<T> { uart: T };
//!
//! impl<T> SomeStruct<T> where T: MutBlockingTx {
//!     fn new(uart: T) -> SomeStruct<T> {
//!         SomeStruct { uart: uart }
//!     }
//!
//!     fn write_data(&mut self) -> Result<(), <T as MutBlockingTx>::Error> {
//!         self.uart.puts(b"AT\n").map_err(|e| e.1)?;
//!         Ok(())
//!     }
//! }
//! ```
//!
//! Here's an example with the `MutBlockingTxWithTimeout` trait.
//!
//! ```
//! struct SomeStruct<T> { uart: T };
//!
//! use embedded_serial::MutBlockingTxWithTimeout;
//!
//! impl<T> SomeStruct<T> where T: MutBlockingTxWithTimeout {
//!     fn new(uart: T) -> SomeStruct<T> {
//!         SomeStruct { uart: uart }
//!     }
//!
//!     fn write_data(&mut self, timeout: &<T as MutBlockingTxWithTimeout>::Timeout) -> Result<bool, <T as MutBlockingTxWithTimeout>::Error> {
//!         let len = self.uart.puts(b"AT\n", timeout).map_err(|e| e.1)?;
//!         Ok(len == 3)
//!     }
//! }
//! ```
//!
//! Here's an example with the `MutNonBlockingTx` trait. You would call the `write_data` function until it returned `Ok(true)`.
//!
//! ```
//! use embedded_serial::MutNonBlockingTx;
//!
//! struct SomeStruct<T> {
//!     sent: Option<usize>,
//!     uart: T
//! };
//!
//! impl<T> SomeStruct<T> where T: MutNonBlockingTx {
//!
//!     fn new(uart: T) -> SomeStruct<T> {
//!         SomeStruct { uart: uart, sent: Some(0) }
//!     }
//!
//!     fn write_data(&mut self) -> Result<bool, <T as MutNonBlockingTx>::Error> {
//!         let data = b"AT\n";
//!         if let Some(len) = self.sent {
//!             match self.uart.puts(&data[len..]) {
//!                 // Sent some or more of the data
//!                 Ok(sent) => {
//!                     let total = len + sent;
//!                     self.sent = if total == data.len() {
//!                         None
//!                     } else {
//!                         Some(total)
//!                     };
//!                     Ok(false)
//!                 }
//!                 // Sent some of the data but errored out
//!                 Err((sent, e)) => {
//!                     let total = len + sent;
//!                     self.sent = if total == data.len() {
//!                         None
//!                     } else {
//!                         Some(total)
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
//!
//! In this example, we read three octets from a blocking serial port.
//!
//! ```
//! use embedded_serial::MutBlockingRx;
//!
//! pub struct SomeStruct<T> { uart: T }
//!
//! impl<T> SomeStruct<T> where T: MutBlockingRx {
//!     pub fn new(uart: T) -> SomeStruct<T> {
//!         SomeStruct { uart: uart }
//!     }
//!
//!     pub fn read_response(&mut self) -> Result<(), <T as MutBlockingRx>::Error> {
//!         let mut buffer = [0u8; 3];
//!         // If we got an error, we don't care any many we actually received.
//!         self.uart.gets(&mut buffer).map_err(|e| e.1)?;
//!         // process data in buffer here
//!         Ok(())
//!     }
//! }
//! ```
//!
//! In this example, we read three octets from a blocking serial port.
//!
//! ```
//! use embedded_serial::MutBlockingRx;
//!
//! pub struct SomeStruct<T> { uart: T }
//!
//! impl<T> SomeStruct<T> where T: MutBlockingRx {
//!     pub fn new(uart: T) -> SomeStruct<T> {
//!         SomeStruct { uart: uart }
//!     }
//!
//!     pub fn read_response(&mut self) -> Result<(), <T as MutBlockingRx>::Error> {
//!         let mut buffer = [0u8; 3];
//!         // If we got an error, we don't care any many we actually received.
//!         self.uart.gets(&mut buffer).map_err(|e| e.1)?;
//!         // process data in buffer here
//!         Ok(())
//!     }
//! }
//! ```
//!
//! In this example, we read three octets from a blocking serial port, with a timeout.
//!
//! ```
//! use embedded_serial::MutBlockingRxWithTimeout;
//!
//! pub struct SomeStruct<T> { uart: T }
//!
//! impl<T> SomeStruct<T> where T: MutBlockingRxWithTimeout {
//!     pub fn new(uart: T) -> SomeStruct<T> {
//!         SomeStruct { uart: uart }
//!     }
//!
//!     pub fn read_response(&mut self, timeout: &<T as MutBlockingRxWithTimeout>::Timeout) -> Result<bool, <T as MutBlockingRxWithTimeout>::Error> {
//!         let mut buffer = [0u8; 3];
//!         // If we got an error, we don't care any many we actually received.
//!         let len = self.uart.gets(&mut buffer, timeout).map_err(|e| e.1)?;
//!         // process data in buffer here
//!         Ok(len == buffer.len())
//!     }
//! }
//! ```
//!
//! In this example, we read 16 octets from a non-blocking serial port into a
//! vector which grows to contain exactly as much as we have read so far. You
//! would call the `read_data` function until it returned `Ok(true)`. This differs
//! from the other examples in that we have an immutable reference to our UART
//! instead of owning it.
//!
//! ```
//! use embedded_serial::ImmutNonBlockingRx;
//!
//! struct SomeStruct<'a, T> where T: 'a {
//!     buffer: Vec<u8>,
//!     uart: &'a T
//! };
//!
//! const CHUNK_SIZE: usize = 4;
//! const WANTED: usize = 16;
//!
//! impl<'a, T> SomeStruct<'a, T> where T: ImmutNonBlockingRx {
//!
//!     fn new(uart: &T) -> SomeStruct<T> {
//!         SomeStruct { uart: uart, buffer: Vec::new() }
//!     }
//!
//!     fn read_data(&mut self) -> Result<bool, <T as ImmutNonBlockingRx>::Error> {
//!         let mut buffer = [0u8; CHUNK_SIZE];
//!         if self.buffer.len() < WANTED {
//!             let needed = WANTED - self.buffer.len();
//!             let this_time = if needed < CHUNK_SIZE { needed } else { CHUNK_SIZE };
//!             match self.uart.gets(&mut buffer[0..needed]) {
//!                 // Read some or more of the data
//!                 Ok(read) => {
//!                     self.buffer.extend(&buffer[0..read]);
//!                     Ok(self.buffer.len() == WANTED)
//!                 }
//!                 // Sent some of the data but errored out
//!                 Err((read, e)) => {
//!                     self.buffer.extend(&buffer[0..read]);
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

// Earlier names for the traits, which assume mutability.
pub use MutBlockingTx as BlockingTx;
pub use MutBlockingTxWithTimeout as BlockingTxWithTimeout;
pub use MutNonBlockingTx as NonBlockingTx;
pub use MutBlockingRx as BlockingRx;
pub use MutBlockingRxWithTimeout as BlockingRxWithTimeout;
pub use MutNonBlockingRx as NonBlockingRx;

/// Implementors of this trait offer octet based serial data transmission
/// using a blocking API and requiring a mutable reference to self.
pub trait MutBlockingTx {
    /// The error type returned if a function fails.
    type Error;

    /// Write a single octet to the port's transmitter,
    /// blocking until the octet can be stored in the buffer
    /// (not necessarily that the octet has been transmitted).
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn putc(&mut self, ch: u8) -> Result<(), Self::Error>;

    /// Write a complete string to the UART.
    /// If this returns `Ok(())`, all the data was sent.
    /// Otherwise you get number of octets sent and the error.
    fn puts(&mut self, data: &[u8]) -> Result<(), (usize, Self::Error)> {
        let mut count:usize = 0;
        for octet in data {
            self.putc(*octet).map_err(|e| (count, e))?;
            count = count + 1;
        }
        Ok(())
    }
}

/// Implementors of this trait offer octet based serial data transmission
/// using a blocking API with an upper bound on blocking time, and requiring a
/// mutable reference to self.
pub trait MutBlockingTxWithTimeout {
    /// The type used to specify the timeout.
    type Timeout;
    /// The error type returned if a function fails.
    type Error;

    /// Write a single octet to the port's transmitter, blocking until the
    /// octet can be stored in the buffer (not necessarily that the
    /// octet has been transmitted) or some timeout occurs.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    ///
    /// If it times out, `Ok(None)` is returned.
    /// If it sends the data, `Ok(Some(ch))` is returned.
    /// If it fails, `Err(...)` is returned.
    fn putc_wait(&mut self, ch: u8, timeout: &Self::Timeout) -> Result<Option<u8>, Self::Error>;

    /// Attempts to write a complete string to the UART.
    /// Returns number of octets written, or an error and the number of octets written.
    /// The timeout applies to each octet individually.
    ///
    /// A result of `Ok(data.len())` means all the data was sent.
    /// A result of `Ok(size < data.len())` means only some of the data was sent then there was a timeout.
    /// A result of `Err(size, e)` means some (or all) of the data was sent then there was an error.
    fn puts_wait(&mut self, data: &[u8], timeout: &Self::Timeout) -> Result<usize, (usize, Self::Error)> {
        let mut count: usize = 0;
        for octet in data {
            // If we get an error, return it (with the number of bytes sent),
            // else if we get None, we timed out so abort.
            if self.putc_wait(*octet, timeout).map_err(|e| (count, e))?.is_none() {
                break;
            }
            count = count + 1;
        }
        Ok(count)
    }
}

/// Implementors of this trait offer octet based serial data transmission
/// using a non-blocking API and requiring a mutable reference to self.
pub trait MutNonBlockingTx {
    /// The error type returned if function fails.
    type Error;

    /// Try and write a single octet to the port's transmitter.
    /// Will return `Ok(None)` if the FIFO/buffer was full
    /// and the octet couldn't be stored or `Ok(Some(ch))`
    /// if it was stored OK.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn putc_try(&mut self, ch: u8) -> Result<Option<u8>, Self::Error>;

    /// Write as much of a complete string to the UART as possible.
    /// Returns the number of octets sent, plus the result from the
    /// last `putc` call. Aborts early if `putc` fails in any way.
    fn puts_try(&mut self, data: &[u8]) -> Result<usize, (usize, Self::Error)> {
        let mut count = 0;
        for octet in data {
            // If we get an error, return it (with the number of bytes sent),
            // else if we get None, we timed out so abort.
            if self.putc_try(*octet).map_err(|e| (count, e))?.is_none() {
                break;
            }
            count = count + 1;
        }
        Ok(count)
    }
}

/// Implementors of this trait offer octet based serial data reception
/// using a blocking API and requiring a mutable reference to self.
pub trait MutBlockingRx {
    /// The error type returned if a function fails.
    type Error;

    /// Read a single octet from the port's receiver,
    /// blocking until the octet can be read from the buffer.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn getc(&mut self) -> Result<u8, Self::Error>;

    /// Read a specified number of octets into the given buffer, blocking
    /// until that many have been read.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn gets(&mut self, buffer: &mut [u8]) -> Result<(), (usize, Self::Error)> {
        let mut count:usize = 0;
        for space in buffer {
            *space = self.getc().map_err(|e| (count, e))?;
            count = count + 1;
        }
        Ok(())
    }
}

/// Implementors of this trait offer octet based serial data reception using a
/// blocking API with an upper bound on blocking time, and requiring a mutable
/// reference to self.
pub trait MutBlockingRxWithTimeout {
    /// The type used to specify the timeout.
    type Timeout;
    /// The error type returned if `getc` fails.
    type Error;

    /// Read a single octet from the port's receiver,
    /// blocking until the octet can be read from the buffer.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    ///
    /// If it times out, Ok(None) is returned.
    /// If it receives data, Ok(Some(data)) is returned.
    /// If it fails, Err(...) is returned.
    fn getc_wait(&mut self, timeout: &Self::Timeout) -> Result<Option<u8>, Self::Error>;

    /// Read a specified number of octets into the given buffer, blocking
    /// until that many have been read or a timeout occurs.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    ///
    /// If the result is `Ok(size)` but `size <= buffer.len()`, you had a timeout.
    fn gets_wait(&mut self, buffer: &mut [u8], timeout: &Self::Timeout) -> Result<usize, (usize, Self::Error)> {
        let mut count:usize = 0;
        for space in buffer {
            *space = match self.getc_wait(timeout) {
                Err(e) => return Err((count, e)),
                Ok(None) => return Ok(count),
                Ok(Some(ch)) => ch,
            };
            count = count + 1;
        }
        Ok(count)
    }
}

/// Implementors of this trait offer octet based serial data reception using a
/// non-blocking API, and requiring a mutable reference to self.
pub trait MutNonBlockingRx {
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

    /// Read a specified number of octets into the given buffer, or until the
    /// data runs out.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    ///
    /// If the result is `Ok(size)` but `size <= buffer.len()`, you ran out of data.
    fn gets_try(&mut self, buffer: &mut [u8]) -> Result<usize, (usize, Self::Error)> {
        let mut count:usize = 0;
        for space in buffer {
            *space = match self.getc_try() {
                Err(e) => return Err((count, e)),
                Ok(None) => return Ok(count),
                Ok(Some(ch)) => ch,
            };
            count = count + 1;
        }
        Ok(count)
    }
}

/// Implementors of this trait offer octet based serial data transmission
/// using a blocking API and only requiring an immutable reference to self.
pub trait ImmutBlockingTx {
    /// The error type returned if a function fails.
    type Error;

    /// Write a single octet to the port's transmitter,
    /// blocking until the octet can be stored in the buffer
    /// (not necessarily that the octet has been transmitted).
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn putc(&self, ch: u8) -> Result<(), Self::Error>;

    /// Write a complete string to the UART.
    /// If this returns `Ok(())`, all the data was sent.
    /// Otherwise you get number of octets sent and the error.
    fn puts(&self, data: &[u8]) -> Result<(), (usize, Self::Error)> {
        let mut count:usize = 0;
        for octet in data {
            self.putc(*octet).map_err(|e| (count, e))?;
            count = count + 1;
        }
        Ok(())
    }
}

/// Implementors of this trait offer octet based serial data transmission
/// using a blocking API with an upper bound on blocking time, and requiring a
/// mutable reference to self.
pub trait ImmutBlockingTxWithTimeout {
    /// The type used to specify the timeout.
    type Timeout;
    /// The error type returned if a function fails.
    type Error;

    /// Write a single octet to the port's transmitter, blocking until the
    /// octet can be stored in the buffer (not necessarily that the
    /// octet has been transmitted) or some timeout occurs.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    ///
    /// If it times out, `Ok(None)` is returned.
    /// If it sends the data, `Ok(Some(ch))` is returned.
    /// If it fails, `Err(...)` is returned.
    fn putc_wait(&self, ch: u8, timeout: &Self::Timeout) -> Result<Option<u8>, Self::Error>;

    /// Attempts to write a complete string to the UART.
    /// Returns number of octets written, or an error and the number of octets written.
    /// The timeout applies to each octet individually.
    ///
    /// A result of `Ok(data.len())` means all the data was sent.
    /// A result of `Ok(size < data.len())` means only some of the data was sent then there was a timeout.
    /// A result of `Err(size, e)` means some (or all) of the data was sent then there was an error.
    fn puts_wait(&self, data: &[u8], timeout: &Self::Timeout) -> Result<usize, (usize, Self::Error)> {
        let mut count: usize = 0;
        for octet in data {
            // If we get an error, return it (with the number of bytes sent),
            // else if we get None, we timed out so abort.
            if self.putc_wait(*octet, timeout).map_err(|e| (count, e))?.is_none() {
                break;
            }
            count = count + 1;
        }
        Ok(count)
    }
}

/// Implementors of this trait offer octet based serial data transmission
/// using a non-blocking API and requiring a mutable reference to self.
pub trait ImmutNonBlockingTx {
    /// The error type returned if function fails.
    type Error;

    /// Try and write a single octet to the port's transmitter.
    /// Will return `Ok(None)` if the FIFO/buffer was full
    /// and the octet couldn't be stored or `Ok(Some(ch))`
    /// if it was stored OK.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn putc_try(&self, ch: u8) -> Result<Option<u8>, Self::Error>;

    /// Write as much of a complete string to the UART as possible.
    /// Returns the number of octets sent, plus the result from the
    /// last `putc` call. Aborts early if `putc` fails in any way.
    fn puts_try(&self, data: &[u8]) -> Result<usize, (usize, Self::Error)> {
        let mut count = 0;
        for octet in data {
            // If we get an error, return it (with the number of bytes sent),
            // else if we get None, we timed out so abort.
            if self.putc_try(*octet).map_err(|e| (count, e))?.is_none() {
                break;
            }
            count = count + 1;
        }
        Ok(count)
    }
}

/// Implementors of this trait offer octet based serial data reception
/// using a blocking API and requiring a mutable reference to self.
pub trait ImmutBlockingRx {
    /// The error type returned if a function fails.
    type Error;

    /// Read a single octet from the port's receiver,
    /// blocking until the octet can be read from the buffer.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn getc(&self) -> Result<u8, Self::Error>;

    /// Read a specified number of octets into the given buffer, blocking
    /// until that many have been read.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    fn gets(&self, buffer: &mut [u8]) -> Result<(), (usize, Self::Error)> {
        let mut count:usize = 0;
        for space in buffer {
            *space = self.getc().map_err(|e| (count, e))?;
            count = count + 1;
        }
        Ok(())
    }
}

/// Implementors of this trait offer octet based serial data reception using a
/// blocking API with an upper bound on blocking time, and requiring a mutable
/// reference to self.
pub trait ImmutBlockingRxWithTimeout {
    /// The type used to specify the timeout.
    type Timeout;
    /// The error type returned if `getc` fails.
    type Error;

    /// Read a single octet from the port's receiver,
    /// blocking until the octet can be read from the buffer.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    ///
    /// If it times out, Ok(None) is returned.
    /// If it receives data, Ok(Some(data)) is returned.
    /// If it fails, Err(...) is returned.
    fn getc_wait(&self, timeout: &Self::Timeout) -> Result<Option<u8>, Self::Error>;

    /// Read a specified number of octets into the given buffer, blocking
    /// until that many have been read or a timeout occurs.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    ///
    /// If the result is `Ok(size)` but `size <= buffer.len()`, you had a timeout.
    fn gets_wait(&self, buffer: &mut [u8], timeout: &Self::Timeout) -> Result<usize, (usize, Self::Error)> {
        let mut count:usize = 0;
        for space in buffer {
            *space = match self.getc_wait(timeout) {
                Err(e) => return Err((count, e)),
                Ok(None) => return Ok(count),
                Ok(Some(ch)) => ch,
            };
            count = count + 1;
        }
        Ok(count)
    }
}

/// Implementors of this trait offer octet based serial data reception using a
/// non-blocking API, and requiring a mutable reference to self.
pub trait ImmutNonBlockingRx {
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
    fn getc_try(&self) -> Result<Option<u8>, Self::Error>;

    /// Read a specified number of octets into the given buffer, or until the
    /// data runs out.
    ///
    /// In some implementations, this can result in an Error.
    /// If not, use `type Error = !`.
    ///
    /// If the result is `Ok(size)` but `size <= buffer.len()`, you ran out of data.
    fn gets_try(&self, buffer: &mut [u8]) -> Result<usize, (usize, Self::Error)> {
        let mut count:usize = 0;
        for space in buffer {
            *space = match self.getc_try() {
                Err(e) => return Err((count, e)),
                Ok(None) => return Ok(count),
                Ok(Some(ch)) => ch,
            };
            count = count + 1;
        }
        Ok(count)
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************

# Embedded Serial Traits

## Introduction

This crate contains traits that are suitable for embedded development. This allows developers to produce crates that depend upon generic UART functionality (for example, an AT command interface), allowing the application developer to combine the crate with the specific UART available on their board.

It is similar to the C idea of using the functions `getc` and `putc` to decouple the IO device from the library, but in a more Rustic fashion.

There are variants of the traits for Receive and Transmit, and for blocking, blocking with timeout and non-blocking.

## Unanswered Questions

- Would it be better if the different traits used the same function names?

//! This crate guarantees that your application is free of panicking branches
//!
//! If your *binary* crate contains at least one panicking branch *linking* will fail.
//!
//! This crate has no effect on libraries. It's meant to be a direct dependency of binary crates.
//!
//! This crate and your program *must* be compiled using the release profile or your crate will
//! always fail to link.
//!
//! # Example
//!
//! ```
//! #![no_main]
//! #![no_std]
//!
//! #[cfg(debug_assertions)]
//! use panic_halt as _;
//!
//! // only link this crate when using the release profile
//! #[cfg(not(debug_assertions))]
//! use panic_never as _;
//! ```
//!
//! If your program contains at least one panicking branch you'll see the following linker error:
//!
//! ``` text
//! (..)
//! error: linking with `rust-lld` failed: exit code: 1
//! (..)
//!   = note: rust-lld: error: undefined symbol:
//!           error(panic-never): your program contains at least one panicking branch
//! ```
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.31 and up. It *might* compile on older
//! versions but that may change in any new patch release.

#![deny(missing_docs)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    extern "Rust" {
        #[link_name = "\nerror(panic-never): your program contains at least one panicking branch"]
        fn undefined() -> !;
    }

    unsafe { undefined() }
}

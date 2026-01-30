//! `main_thread` is a single function crate that provides a function to check if the calling thread is the main thread.
//!
//! # Example
//! ```rust
//! use main_thread::IsMainThread;
//!
//! fn some_function() {
//!     match main_thread::is_main_thread() {
//!         IsMainThread::MainThread => println!("Its the main thread"),
//!         IsMainThread::OtherThread => println!("Its not the main thread"),
//!         IsMainThread::Unknown => println!("We don't know if its the main thread or not on this platform/target"),
//!     }
//! }
//! ```
#![deny(clippy::correctness)]
#![deny(
    clippy::perf,
    clippy::complexity,
    clippy::style,
    clippy::nursery,
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::decimal_literal_representation,
    clippy::float_cmp_const,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::unwrap_used,
    clippy::cargo_common_metadata,
    clippy::used_underscore_binding
)]
#![no_std]

/// Platform-specific implementation
mod platform_impl;

use core::fmt::{Display, Formatter};
use platform_impl::platform;

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum IsMainThread {
    MainThread,
    OtherThread,
    #[default]
    Unknown,
}

impl Display for IsMainThread {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MainThread => f.write_str("main thread"),
            Self::OtherThread => f.write_str("other thread"),
            Self::Unknown => f.write_str("unknown thread"),
        }
    }
}

/// Returns the "main thread" characteristics of the calling thread.
#[must_use]
pub fn is_main_thread() -> IsMainThread {
    match platform::is_main_thread() {
        None => IsMainThread::Unknown,
        Some(true) => IsMainThread::MainThread,
        Some(false) => IsMainThread::OtherThread,
    }
}

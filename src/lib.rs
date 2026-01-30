//! main_thread is a single function crate that provides a function to check if the calling thread is the main thread.
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

mod platform_impl;

use platform_impl::platform;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum IsMainThread {
    MainThread,
    OtherThread,
    #[default]
    Unknown,
}

impl Display for IsMainThread {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IsMainThread::MainThread => f.write_str("main thread"),
            IsMainThread::OtherThread => f.write_str("other thread"),
            IsMainThread::Unknown => f.write_str("unknown thread"),
        }
    }
}

/// Returns true if the calling thread is the main thread or not.
///
/// If the current platform does not support this then 'None' is returned.
pub fn is_main_thread() -> IsMainThread {
    match platform::is_main_thread() {
        None => IsMainThread::Unknown,
        Some(true) => IsMainThread::MainThread,
        Some(false) => IsMainThread::OtherThread,
    }
}

/// Windows impl
#[cfg(target_os = "windows")]
#[path = "windows.rs"]
pub mod platform;

/// Linux impl
#[cfg(target_os = "linux")]
#[path = "linux.rs"]
pub mod platform;

/// BSD impl
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd", target_os = "macos"))]
#[path = "bsd.rs"]
pub mod platform;

/// Netbsd impl
#[cfg(target_os = "netbsd")]
#[path = "netbsd.rs"]
pub mod platform;

/// ios impl
#[cfg(target_os = "ios")]
#[path = "macos.rs"]
pub mod platform;

/// Android noop impl
#[cfg(target_os = "android")]
#[path = "android.rs"]
pub mod platform;

/// wasm32 noop impl
#[cfg(target_arch = "wasm32")]
#[path = "wasm.rs"]
pub mod platform;

/// Everything else
#[cfg(all(
    not(target_os = "ios"),
    not(target_os = "windows"),
    not(target_os = "linux"),
    not(target_os = "macos"),
    not(target_os = "android"),
    not(target_os = "dragonfly"),
    not(target_os = "freebsd"),
    not(target_os = "netbsd"),
    not(target_os = "openbsd"),
    not(target_arch = "wasm32"),
))]
pub mod platform {
    pub fn is_main_thread() -> Option<bool> {
        None
    }
}

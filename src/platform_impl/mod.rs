#[cfg(target_os = "windows")]
#[path = "windows.rs"]
pub(crate) mod platform;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
pub(crate) mod platform;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
#[path = "bsd.rs"]
pub(crate) mod platform;

#[cfg(target_os = "netbsd")]
#[path = "netbsd.rs"]
pub(crate) mod platform;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "macos.rs"]
pub(crate) mod platform;

#[cfg(target_os = "android")]
#[path = "android.rs"]
pub(crate) mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
pub(crate) mod platform;

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
pub(crate) mod platform {
    pub fn is_main_thread() -> Option<bool> {
        None
    }
}

/// Linux specific impl
#[allow(clippy::unnecessary_wraps)]
pub fn is_main_thread() -> Option<bool> {
    Some(is_mt())
}

/// actual impl for `x86_64`
#[cfg(all(target_env = "gnu", target_arch = "x86_64"))]
#[inline]
fn is_mt() -> bool {
    // gettid requires gnu c library version 2.30. This is not ideal.
    // we can just "rawdog" this syscall for x86 to reduce the gnu c library requirement down to 2.17.
    // for musl, we don't care as rusts musl targets usually statically links the c runtime.
    // other targets would require careful testing that I don't want to do for now.
    unsafe { libc::syscall(libc::SYS_gettid) == (libc::getpid().into()) }
}

/// actual impl for `x86`
#[cfg(all(target_env = "gnu", target_arch = "x86"))]
#[inline]
fn is_mt() -> bool {
    unsafe { libc::syscall(libc::SYS_gettid) == (libc::getpid()) }
}

/// actual impl for all other targets
#[cfg(not(all(target_env = "gnu", any(target_arch = "x86_64", target_arch = "x86"))))]
#[inline]
fn is_mt() -> bool {
    unsafe { libc::gettid() == libc::getpid() }
}

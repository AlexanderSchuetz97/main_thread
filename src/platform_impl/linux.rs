pub fn is_main_thread() -> Option<bool> {
    Some(is_main_thread_internal())
}

#[cfg(target_os = "linux")]
fn is_main_thread_internal() -> bool {
    use libc::{c_long, getpid, syscall, SYS_gettid};

    unsafe { syscall(SYS_gettid) == getpid() as c_long }
}

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
fn is_main_thread_internal() -> bool {
    use libc::pthread_main_np;

    unsafe { pthread_main_np() == 1 }
}

#[cfg(target_os = "netbsd")]
fn is_main_thread_internal() -> bool {
    use libc::_lwp_self;

    unsafe { _lwp_self() == 1 }
}
use libc::{c_long, getpid, syscall};

pub fn is_main_thread() -> Option<bool> {
    Some(unsafe { syscall(SYS_gettid) == getpid() as c_long })
}

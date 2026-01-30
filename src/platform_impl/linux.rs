pub fn is_main_thread() -> Option<bool> {
    Some(unsafe { libc::gettid() == libc::getpid() })
}

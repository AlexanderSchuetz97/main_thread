use objc2_foundation::NSThread;

/// Macos and ios specific implementation.
/// I am unsure if just calling `libc::pthread_main_np` would not be better.
/// objc2_foundation takes ages to compile.
pub fn is_main_thread() -> Option<bool> {
    Some(NSThread::currentThread().isMainThread())
}

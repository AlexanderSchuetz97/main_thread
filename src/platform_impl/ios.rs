use objc2_foundation::NSThread;

/// Ios specific implementation.
/// This implementation was only tested on macOS.
/// We could use this implementation for macOS too, but objc2_foundation takes ages to compile.
pub fn is_main_thread() -> Option<bool> {
    Some(NSThread::currentThread().isMainThread())
}

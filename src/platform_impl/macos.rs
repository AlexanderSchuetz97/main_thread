use objc2_foundation::NSThread;

pub fn is_main_thread() -> Option<bool> {
    Some(NSThread::currentThread().isMainThread())
}

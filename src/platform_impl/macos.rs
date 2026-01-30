use objc_rs::*;

pub fn is_main_thread() -> Option<bool> {
    unsafe { Some(msg_send![class!(NSThread), isMainThread]) }
}
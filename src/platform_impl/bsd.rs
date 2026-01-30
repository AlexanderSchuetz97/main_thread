fn is_main_thread() -> Option<bool> {
    use libc::pthread_main_np;

    //OpenBSD manpage:
    //The pthread_main_np() function returns:
    //
    // 1 if the calling thread is the main thread
    // 0 if the calling thread is not the main thread
    // -1 if the thread initialization has not completed
    match unsafe { pthread_main_np() } {
        1 => Some(true),
        0 => Some(false),
        _ => None,
    }
}

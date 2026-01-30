pub fn is_main_thread() -> Option<bool> {
    unsafe {
        let self_id = libc::_lwp_self();
        if self_id == 1 {
            // Based on https://stackoverflow.com/questions/4867839/how-can-i-tell-if-pthread-self-is-the-main-first-thread-in-the-process
            // This information is probably horribly outdated or may even be wrong entirely?
            // On modern NetBSD systems _lwp_self()
            // only appears to return 1 in the main thread of the process with the tid 1.
            // since that means that if it does indeed return 1 then it MUST be the main thread even on new netbsd versions.
            // This allows for backwards compatability
            return Some(true);
        }

        //This works on "modern" netbsd
        Some(self_id as i128 == libc::getpid() as i128)
    }
}

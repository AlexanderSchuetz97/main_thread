use main_thread::IsMainThread;

#[unsafe(no_mangle)]
pub extern "C" fn is_this_the_main_thread() -> u32 {
    println!("DLL INVOKED");
    match main_thread::is_main_thread() {
        IsMainThread::MainThread => 1,
        IsMainThread::OtherThread => 2,
        IsMainThread::Unknown => 0,
    }
}

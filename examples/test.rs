use main_thread::{is_main_thread, IsMainThread};

fn main() {
    let n = unsafe { libc::getpid() };
    println!("pthread_self = {}", n as isize);

    let is_main = std::thread::spawn(|| is_main_thread()).join().unwrap();

    assert_ne!(is_main, IsMainThread::MainThread);

    let is_main = is_main_thread();

    assert_ne!(is_main, IsMainThread::OtherThread);

    let is_main = std::thread::spawn(|| is_main_thread()).join().unwrap();

    assert_ne!(is_main, IsMainThread::MainThread);
}

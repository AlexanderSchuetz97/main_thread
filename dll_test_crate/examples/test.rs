use std::{thread};

#[cfg(all(not(windows), not(target_vendor = "apple")))]
pub fn main() {
    unsafe {
        let lib = thread::spawn(|| {libloading::Library::new("target/debug/libdll_test_crate.so").unwrap()}).join().unwrap();

        let symbol = lib.get::<extern "C" fn() -> u32>("is_this_the_main_thread").unwrap().try_as_raw_ptr().unwrap() as usize;
        std::mem::forget(lib);

        assert_eq!(1, std::mem::transmute::<usize, extern "C" fn() -> u32>(symbol)());

        let x2 = thread::spawn(move || {
            std::mem::transmute::<usize, extern "C" fn() -> u32>(symbol)()
        }).join().unwrap();
        assert_eq!(2, x2);
    }
}

#[cfg(target_vendor = "apple")]
pub fn main() {
    unsafe {
        let lib = thread::spawn(|| {libloading::Library::new("target/debug/libdll_test_crate.dylib").unwrap()}).join().unwrap();

        let symbol = lib.get::<extern "C" fn() -> u32>("is_this_the_main_thread").unwrap().try_as_raw_ptr().unwrap() as usize;
        std::mem::forget(lib);

        assert_eq!(1, std::mem::transmute::<usize, extern "C" fn() -> u32>(symbol)());

        let x2 = thread::spawn(move || {
            std::mem::transmute::<usize, extern "C" fn() -> u32>(symbol)()
        }).join().unwrap();
        assert_eq!(2, x2);
    }
}

#[cfg(target_os = "windows")]
pub fn main() {
    unsafe {
        let lib = if std::fs::exists("target/x86_64-pc-windows-gnu/debug/dll_test_crate.dll").unwrap_or_default() {
            //Wine with bimfmt misc
            println!("using GNU");
            thread::spawn(|| {
                libloading::Library::new("target/x86_64-pc-windows-gnu/debug/dll_test_crate.dll").unwrap()
            }).join().unwrap()
        } else {
            thread::spawn(|| {
                libloading::Library::new("target/debug/dll_test_crate.dll").unwrap()
            }).join().unwrap()
        };

        let symbol = lib.get::<extern "C" fn() -> u32>("is_this_the_main_thread").unwrap().try_as_raw_ptr().unwrap() as usize;
        std::mem::forget(lib);

        assert_eq!(1, std::mem::transmute::<usize, extern "C" fn() -> u32>(symbol)());

        let x2 = thread::spawn(move || {
            std::mem::transmute::<usize, extern "C" fn() -> u32>(symbol)()
        }).join().unwrap();
        assert_eq!(2, x2);
    }
}
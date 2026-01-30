use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Mutex;
use defer_heavy::defer;
use winapi::shared::minwindef::{DWORD, FILETIME};
use winapi::um::processthreadsapi;
use winapi::um::processthreadsapi::{GetCurrentProcessId, GetThreadTimes, OpenThread};
use winapi::um::tlhelp32::THREADENTRY32;

pub fn is_main_thread() -> Option<bool> {
    let current_thread_id = unsafe { processthreadsapi::GetCurrentThreadId() };
    let mut main_thread_id = tlhelp32_main_thread_id();
    if main_thread_id == 0 {
        //Tool-api had access denied.
        main_thread_id = xcu_main_thread_id();
    }

    if main_thread_id == 0 {
        //xcu hook was not called.
        return None;
    }

    Some(current_thread_id == main_thread_id)

}

static MUTEX: Mutex<()> = Mutex::new(());

fn tlhelp32_main_thread_id() -> DWORD {
    static TL_MAIN_THREAD_ID: AtomicU64 = AtomicU64::new(u64::MAX);
    let value = TL_MAIN_THREAD_ID.load(SeqCst);
    if value != u64::MAX {
        return value as _;
    }

    let _guard = MUTEX.lock();

    let value = TL_MAIN_THREAD_ID.load(SeqCst);
    if value != u64::MAX {
        return value as _;
    }

    unsafe {
        let current_proc = GetCurrentProcessId();
        let snapshot = winapi::um::tlhelp32::CreateToolhelp32Snapshot(winapi::um::tlhelp32::TH32CS_SNAPTHREAD, current_proc);
        if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            TL_MAIN_THREAD_ID.store(0, SeqCst);
            return 0;
        }

        defer! {
            winapi::um::handleapi::CloseHandle(snapshot);
        }


        let mut entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as _,
            cntUsage: 0,
            th32ThreadID: 0,
            th32OwnerProcessID: 0,
            tpBasePri: 0 as _,
            tpDeltaPri: 0 as _,
            dwFlags: 0,
        };

        let mut main_thread_id = 0;
        let mut main_thread_time = u128::MAX;
        let mut first = true;

        loop {
            if first {
                first = false;
                if winapi::um::tlhelp32::Thread32First(snapshot, &mut entry) == 0 {
                    TL_MAIN_THREAD_ID.store(0, SeqCst);
                    return 0;
                }
            } else if winapi::um::tlhelp32::Thread32Next(snapshot, &mut entry) == 0 {
                if winapi::um::errhandlingapi::GetLastError() != winapi::shared::winerror::ERROR_NO_MORE_FILES {
                    TL_MAIN_THREAD_ID.store(0, SeqCst);
                    return 0;
                }

                TL_MAIN_THREAD_ID.store(main_thread_id, SeqCst);
                return main_thread_id as _;
            }


            if entry.th32OwnerProcessID != current_proc {
                continue;
            }

            let thread = OpenThread(winapi::um::winnt::THREAD_QUERY_INFORMATION, 1, entry.th32ThreadID);
            if thread == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                continue;
            }

            defer! {
                winapi::um::handleapi::CloseHandle(thread);
            }

            let mut creation_time: FILETIME = FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            };

            let mut dummy_time: FILETIME = FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            };

            if GetThreadTimes(thread, &mut creation_time, &mut dummy_time, &mut dummy_time, &mut dummy_time) == 0 {
                continue;
            }

            let inspected_thread_time = ((creation_time.dwHighDateTime as u128) << (std::mem::size_of::<DWORD>() * 8)) | (creation_time.dwLowDateTime as u128);

            if inspected_thread_time < main_thread_time {
                main_thread_time = inspected_thread_time;
                main_thread_id = entry.th32ThreadID as _;
            }
        }
    }
}

/// This is only the fallback code path, because this function MIGHT NOT WORK PROPERLY
/// if the rust code is compiled into a cdylib that is loaded by LoadLibraryA.
/// Because the XCU hook is invoked by which ever thread called LoadLibraryA.
/// This might not be the main thread!
///
/// This is still better than nothing in case tlhelp32 is not available.
fn xcu_main_thread_id() -> DWORD {
    static XCU_MAIN_THREAD_ID: AtomicU64 = AtomicU64::new(0);
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".CRT$XCU"]
    static INIT_MAIN_THREAD_ID: unsafe extern "C" fn() = {
        unsafe extern "C" fn initer() {
            XCU_MAIN_THREAD_ID.store(processthreadsapi::GetCurrentThreadId() as _, SeqCst);
        }
        initer
    };

    XCU_MAIN_THREAD_ID.load(SeqCst) as _
}
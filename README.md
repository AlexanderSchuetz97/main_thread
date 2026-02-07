# main_thread 
Cross-platform library to check if the current thread is the main thread.

## Example
```rust
use main_thread::IsMainThread;

fn some_function() {
     match main_thread::is_main_thread() {
         IsMainThread::MainThread => println!("Its the main thread"),
         IsMainThread::OtherThread => println!("Its not the main thread"),
         IsMainThread::Unknown => println!("We don't know if its the main thread or not on this platform/target"),
     }
}
```

## Supported platforms
| Platform        | Supported | CI-Tested | no-std  |
|-----------------|-----------|-----------|---------|
| Linux Gnu       | &check;   | &check;   | &check; |
| Linux Musl      | &check;   | &cross;   | &check; |
| Windows Msvc    | &check;   | &check;   | &check; |
| Windows Gnu     | &check;   | &cross;   | &check; |
| Windows on Wine | &check;   | &cross;   |         |
| macOS           | &check;   | &check;   | &check; |
| FreeBSD         | &check;   | &check;   | &check; |
| OpenBSD         | &check;   | &check;   | &check; |
| NetBSD          | &check;   | &check;   | &check; |
| Dragonfly BSD   | ?         | &cross;   | &check; |
| ios             | ?         | &cross;   | &cross; |
| android         | &cross;   | &cross;   |         |
| wasm            | &cross;   | &cross;   |         |

Supported '?' means that code exists that handles the platform. 
It's guaranteed that the code compiles, but it has never been tested, not even manually.

### Gnu C library version (only linux gnu targets)
| Arch       | Version |
|------------|---------|
| x86_64     | 2.17    |
| i686       | 2.17    |
| i586       | 2.17    |
| all others | 2.30    |

If you require this for older versions of the gnu c library than 2.30, 
for different architectures then open a GitHub issue.

### When is IsMainThread::Unknown returned exactly?
For all unsupported platforms IsMainThread::Unknown is always returned.

The Windows implementation will return unknown if the
process does not have permission to use tlhelp32.h library functions on itself
and the C-Runtime did not execute the ".CRT$XCU" initializers.
As of 2026 this is never the case, 
but you never know what microsoft decides to do in the future.

The FreeBSD, OpenBSD and Dragonfly BSD implementation will return
IsMainThread::Unknown if the "the thread initialization has not completed" yet.
Normal applications are unlikely to encounter this.

On all other platforms IsMainThread::Unknown is never returned.

## This is a fork
Thanks to "Matthew" for providing the crate "is_main_thread".
(https://gitlab.com/CyanPhoton/is_main_thread/)
This crate is a fork of his work.

### Reasons for forking
* "Matthew" provided no way of contacting him
* "is_main_thread" appears to no longer be maintained
* "is_main_thread" does not compile for macOS targets using a modern rust compiler, fixing the compilation errors resulted in runtime panics (I am reasonably convinced that it never worked)
* "is_main_thread" windows-specific implementation was wrong for all crates that had the crate-type "cdylib" and relied on undefined behavior for all other crate types.
* "is_main_thread" netbsd-specific implementation does not work on newer versions of netbsd. (Possibly on no version that rust supports, it is based on outdated info from an ancient stackoverflow post.)

This fork addresses all these issues.

## License
This crate is released under the Apache-2 license.
See the LICENSE file for more details.

This is the same license as Matthew's "is_main_thread" crate.
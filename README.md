# main_thread 
Single function crate that provides a function to check if the calling thread is the main thread.

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
| Platform      | Supported | Tested  |
|---------------|-----------|---------|
| Linux         | &check;   | &check; |
| Windows       | &check;   | &check; |
| macOS         | &check;   | &check; |
| FreeBSD       | &check;   | &check; |
| OpenBSD       | &check;   | &check; |
| NetBSD        | &check;   | &check; |
| Dragonfly BSD | &check;   | &cross; |
| ios           | &check;   | &cross; |
| android       | &cross;   | &cross; |
| wasm          | &cross;   | &cross; |

### When is IsMainThread::Unknown returned exactly?
For all unsupported platforms it is always returned.

The windows implementation will return unknown if the
process does not have permission to use tlhelp32.dll library functions on itself
and the C-Runtime did not execute the ".CRT$XCU" initializers.
As of 2026 this is never the case, 
but you never know what microsoft decides to do in the future.

On all other platforms IsMainThread::Unknown is never returned.

## This is a fork
Thanks to "Matthew" for providing the crate "is_main_thread".
(https://gitlab.com/CyanPhoton/is_main_thread/)
This crate is a fork of his work.

### Reasons for forking
* "Matthew" provided no way of contacting him
* "is_main_thread" appears to no longer be maintained
* "is_main_thread" does not compile for macOS targets using a modern rust compiler since it is missing "unsafe" statements in the osx specific implementation.
* "is_main_thread" windows specific implementation was wrong for all crates that had the crate-type "cdylib" and relied on undefined behavior for all other crate types.

This fork addresses all these issues.

## License
This crate is released under the Apache-2 license.
See the LICENSE file for more details.

This is the same license as Matthew's "is_main_thread" crate.
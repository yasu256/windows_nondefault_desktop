# Windows non-default desktop

Some applications (e.g. GUI apps) only work on the active [Windows desktop](https://learn.microsoft.com/en-us/windows/win32/winstation/desktops). However, Rust application is run on the desktop "Default" even when run from non-default desktops. This crate is the solution of this problem.

## Usage

The usage is simple:

```rust
fn main() {
    windows_nondefault_desktop::assume_active_desktop();
    // The application is assumed to be running on the active desktop

    // then perform main processing
}
```

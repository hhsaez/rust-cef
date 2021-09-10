#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::all
)]
#[cfg(target_os = "linux")]
#[link(name = "cef", kind = "dynamic")]
mod bindings_linux;

#[cfg(target_os = "linux")]
pub use bindings_linux::*;

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::all
)]
#[cfg(target_os = "macos")]
#[link(name = "cef", kind = "dynamic")]
mod bindings_macos;

#[cfg(target_os = "macos")]
pub use bindings_macos::*;

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::all
)]
#[cfg(target_os = "windows")]
#[link(name = "cef", kind = "dynamic")]
mod bindings_msvc;

#[cfg(target_os = "windows")]
pub use bindings_msvc::*;

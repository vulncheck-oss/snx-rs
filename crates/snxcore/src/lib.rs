#[cfg(not(any(target_os = "linux", target_os = "windows")))]
compile_error!(
    "snx-rs only supports Linux and Windows. There is no platform implementation for this target \
     (e.g. macOS). Build for a supported target, for example: cargo build --target x86_64-unknown-linux-gnu"
);

// Gate every module on supported platforms so that on an unsupported target the
// `compile_error!` above is the only error emitted, instead of a cascade of
// missing-symbol errors from the platform-dependent code.
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod browser;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod controller;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod model;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod otp;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod platform;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod profiles;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod prompt;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod server;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod sexpr;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod tunnel;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod util;

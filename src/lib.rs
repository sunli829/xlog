//! Xlog can add key/value pairs to your log lines.
//!
//! It's fully compatible with `log` crate, only extended support for key/value pairs, and is rustfmt friendly.
//!
//! # Target syntax
//!
//! There is no difference between the following two lines of code, the first is for compatibility with log crates, and the second is friendly to rustfmt.
//!
//! ```rust
//! use xlog::info;
//! info!(target: "a", "hello");
//! info!(target = "a", "hello");
//! ```
//!
//! # Error Key
//!
//! ```rust
//! use xlog::error;
//! use std::io::{Error, ErrorKind};
//! error!("Failed to open database", error = Error::from(ErrorKind::InvalidData).to_string())
//! ```
//!
//! # Examples
//! ```ignore
//! use xlog::{info, error};
//!
//! pub fn serve(bind_addr: &str) {
//!     info!(target: "http", "Start server", bind_addr = bind_addr);
//!     if let Some(err) = start_http_server(bind_addr) {
//!         error!(target: "http", "Failed to start server", error = err.to_string());
//!     }
//! }
//! ```

#[doc(hidden)]
pub use log as _log;

pub use _log::log_enabled;
pub use _log::Level;
pub use _log::LevelFilter;

#[doc(hidden)]
#[macro_export]
macro_rules! msg_and_kvs {
    // Format start
    ($fmt:literal, $($tail:tt)+) => {
        xlog::msg_and_kvs!(@fmt $fmt @args [] $($tail)+)
    };

    // Literal only
    ($fmt:literal $(,)*) => {
        xlog::_log::Record::builder().args(format_args!($fmt))
    };

    // First kv param
    (@fmt $fmt:literal @args [$($args:expr,)*] $key:ident = $value:expr, $($tail:tt)+) => {
        xlog::msg_and_kvs!(@fmt $fmt @args [$($args,)*] @kvs [$key = $value,] $($tail)+)
    };

    // Following KV params
    (@fmt $fmt:literal @args [$($args:expr,)*] @kvs [$($pkey:ident = $pvalue:expr,)*] $key:ident = $value:expr, $($tail:tt)+) => {
        xlog::msg_and_kvs!(@fmt $fmt @args [$($args,)*] @kvs [$($pkey = $pvalue,)* $key = $value,] $($tail)+)
    };

    // Last KV param
    (@fmt $fmt:literal @args [$($args:expr,)*] $key:ident = $value:expr $(,)*) => {
        xlog::msg_and_kvs!(@finish @fmt $fmt @args [$($args,)*] @kvs [$key = $value,])
    };

    // Last KV param
    (@fmt $fmt:literal @args [$($args:expr,)*] @kvs [$($pkey:ident = $pvalue:expr,)*] $key:ident = $value:expr $(,)*) => {
        xlog::msg_and_kvs!(@finish @fmt $fmt @args [$($args,)*] @kvs [$($pkey = $pvalue,)* $key = $value,])
    };

    // Format params
    (@fmt $fmt:literal @args [$($args:expr,)*] $value:expr, $($tail:tt)+) => {
        xlog::msg_and_kvs!(@fmt $fmt @args [$($args,)* $value,] $($tail)+)
    };

    // Last format param
    (@fmt $fmt:literal @args [$($args:expr,)*] $value:expr $(,)*) => {
        xlog::msg_and_kvs!(@finish @fmt $fmt @args [$($args,)* $value,] @kvs [])
    };

    // Finish
    (@finish @fmt $fmt:literal @args [$($args:expr,)*] @kvs [$($key:ident = $value:expr,)*]) => {
        xlog::_log::Record::builder()
            .args(format_args!($fmt, $($args),*))
            .key_values(xlog::msg_and_kvs!(@kvs [$($key = $value,)*]))
    };

    (@kvs []) => { &Option::<&dyn xlog::_log::kv::Source>::None };

    (@kvs [$($key:ident = $value:expr,)*]) => { &Some(&vec![$((stringify!($key), &$value)),*]) };
}

/// The standard logging macro.
#[macro_export]
macro_rules! log {
    (target: $target:expr, $level:expr, $($args:tt)*) => {
        {
            xlog::_log::logger().log(
                &xlog::msg_and_kvs!($($args)*)
                    .level($level)
                    .target($target)
                    .module_path_static(Some(module_path!()))
                    .file_static(Some(file!()))
                    .line(Some(line!()))
                    .build()
                );
        }
    };
    (target = $target:expr, $level:expr, $($args:tt)*) => {
        xlog::log!(target: $target, $level, $($args)+)
    };
    ($level:expr, $($args:tt)*) => {
        xlog::log!(target: module_path!(), $level, $($args)+)
    };
}

/// Logs a message at the trace level.
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($args:tt)*) => {
        xlog::log!(target: $target, xlog::Level::Trace, $($args)+)
    };
    (target = $target:expr, $($args:tt)*) => {
        xlog::log!(target: $target, xlog::Level::Trace, $($args)+)
    };
    ($($args:tt)*) => {
        xlog::log!(xlog::Level::Trace, $($args)+)
    };
}

/// Logs a message at the debug level.
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($args:tt)*) => {
        xlog::log!(target: $target, xlog::Level::Debug, $($args)+)
    };
    (target = $target:expr, $($args:tt)*) => {
        xlog::log!(target: $target, xlog::Level::Debug, $($args)+)
    };
    ($($args:tt)*) => {
        xlog::log!(xlog::Level::Debug, $($args)+)
    };
}

/// Logs a message at the info level.
#[macro_export]
macro_rules! info {
    (target: $target:expr, $($args:tt)*) => {
        xlog::log!(target: $target, xlog::Level::Info, $($args)+)
    };
    (target = $target:expr, $($args:tt)*) => {
        xlog::log!(target: $target, xlog::Level::Info, $($args)+)
    };
    ($($args:tt)*) => {
        xlog::log!(xlog::Level::Info, $($args)+)
    };
}

/// Logs a message at the warn level.
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($args:tt)*) => {
        xlog::log!(target: $target, xlog::Level::Warn, $($args)+)
    };
    (target = $target:expr, $($args:tt)*) => {
        xlog::log!(target: $target, xlog::Level::Warn, $($args)+)
    };
    ($($args:tt)*) => {
        xlog::log!(xlog::Level::Warn, $($args)+)
    };
}

/// Logs a message at the error level.
#[macro_export]
macro_rules! error {
    (target: $target:expr, $($args:tt)*) => {
        xlog::log!(target: $target, xlog::Level::Error, $($args)+)
    };
    (target = $target:expr, $($args:tt)*) => {
        xlog::log!(target: $target, xlog::Level::Error, $($args)+)
    };
    ($($args:tt)*) => {
        xlog::log!(xlog::Level::Error, $($args)+)
    };
}

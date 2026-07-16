//! This modules defines macros for optional logging
//!

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        #[cfg(feature = "log")]
        log::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "log")]
        log::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        #[cfg(feature = "log")]
        log::error!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "log")]
        log::debug!($($arg)*);
    };
}
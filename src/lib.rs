//! A Dijkstra's algorithm implementation that aims to be simple to use and fast to run
//!
//! ** simple.** nodes id and its cost are defined by yuor own types
//!
//! ```toml
//! [dependencies]
//! dijkstra-suite = "0.1.0-alpha.0"
//! ```
//!
//! ## Usage
//!
//! Just import the `add()` function and call it
//!
//! ```rust
//! use dijkstra_suite::add;
//!
//! let sum = add(400, 20);
//!
//! # assert_eq!(sum, 420);
//! ```

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://docs.rs/nanoid"
)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//! # Gjallarhorn Binary
//!
//! Entry point for the executable. Delegates code to the library.

fn main() -> Result<(), slint::PlatformError> {
    gjallarhorn::run()
}

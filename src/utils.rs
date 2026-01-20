//! # Utility Functions Module
//!
//! This module provides shared helper functions used throughout the application.
//! Key utilities include:
//! - `generate_path`: A highly optimized function to generate SVG path commands from a history buffer.
//!   it pre-allocates strings to minimize heap churn during real-time updates.
//! - `hex_to_color` / `brush_to_hex`: Functions to convert between string representations of colors (for storage) and Slint types (for UI).

use slint::SharedString;

/// Helper function to convert a hex string (e.g., "#RRGGBB") to a `slint::Color`.
/// Returns a default gray color if parsing fails or format is invalid.
pub fn hex_to_color(hex: &str) -> slint::Color {
    if hex.len() == 7 && hex.starts_with('#') {
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
        slint::Color::from_rgb_u8(r, g, b)
    } else {
        slint::Color::from_rgb_u8(100, 100, 100) // Fallback
    }
}

/// Helper function to convert a `slint::Brush` (assuming solid color) back to a hex string.
/// Used for saving the current color state to the configuration file.
pub fn brush_to_hex(brush: slint::Brush) -> String {
    let color = brush.color();
    format!(
        "#{:02x}{:02x}{:02x}",
        color.red(),
        color.green(),
        color.blue()
    )
}

/// Returns a `SharedString` containing the SVG `d` attribute commands (M, L).
pub fn generate_path(history: &[f32], max_val: f32, max_history_len: usize) -> SharedString {
    if history.is_empty() {
        return "".into();
    }

    // Pre-allocate to reduce reallocations.
    // Approx 20 bytes per point
    let mut path = String::with_capacity(history.len() * 20);

    let normalize_y = |val: f32| -> f32 { 100.0 - (val.min(max_val) / max_val * 100.0) };

    // Normalize X to fit in 60 units (matching the viewbox-width of 60 in appwindow.slint)
    // Step is calculated based on the MAXIMUM history capacity, ensuring 1 unit of X always equals 1 unit of time.
    let width = 60.0;
    let step_x = width / ((max_history_len.max(2) - 1) as f32);

    use std::fmt::Write;
    let _ = write!(path, "M 0 {:.2}", normalize_y(history[0]));

    for (i, val) in history.iter().enumerate().skip(1) {
        let x = i as f32 * step_x;
        let _ = write!(path, " L {:.2} {:.2}", x, normalize_y(*val));
    }

    path.into()
}

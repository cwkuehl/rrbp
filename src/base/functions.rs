/// The function does nothing and always returns 0.
pub fn mach_nichts() -> i32 {
    0
}

/// Convert string to i32.
/// * s: Affected string.
pub fn to_i32(s: &str) -> i32 {
    let x = s.parse::<i32>();
    if let Ok(i) = x {
        return i;
    }
    0
}

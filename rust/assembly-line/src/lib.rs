// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        s if s <= 4 => (s as f64 * 221.0),
        s if s > 4 && s <= 8 => (s as f64 * 221.0) * (0.9),
        s if s > 8 && s <= 10 => (s as f64 * 221.0) * (0.77),
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

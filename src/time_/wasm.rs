use js_sys::Date;

pub fn precise_time_s() -> f64 {
    Date::new_0().now() / 1000 // Date::now() returns milliseconds, we need seconds
}

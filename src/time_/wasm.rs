use js_sys::Date;

pub fn precise_time_s() -> f64 {
    Date::now() / 1000f64 // Date::now() returns milliseconds, we need seconds
}

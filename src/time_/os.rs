use time::precise_time_s as os_time;

pub fn precise_time_s() -> f64 {
    os_time()
}

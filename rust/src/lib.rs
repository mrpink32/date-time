use std::time::{SystemTime, UNIX_EPOCH};


const YEAR_OFFSET: u64 = 1970;


struct DateTime {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    seconds: u64,
}


pub fn get_time() -> String {
    let time_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get time since epoch!")
        .as_secs_f64();
    let seconds: f64 = time_since_epoch % 60.0;
    let minutes: f64 = f64::floor(time_since_epoch / 60.0 % 60.0);
    let hours: f64 = f64::floor(time_since_epoch / 60.0 / 60.0 % 24.0);
    let days: f64 = time_since_epoch / 60.0 / 60.0 / 24.0;
    let months: f64 = time_since_epoch / 2629743.0;
    let years: f64 = f64::floor(time_since_epoch / 31556926.0);
    return format!(
        "{}/{}/{}/{}:{}:{}",
        (years + YEAR_OFFSET as f64),
        (f64::ceil(months) - (years * 12 as f64)),
        days,
        hours,
        minutes,
        seconds,
        );
}
pub fn get_time_2() -> String {
    let time_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get time since epoch!")
        .as_secs_f64();
    let seconds: f64 = time_since_epoch % 60.0;
    let minutes: f64 = f64::floor(time_since_epoch / 60.0 % 60.0);
    let hours: f64 = f64::floor(time_since_epoch / 60.0 / 60.0 % 24.0);
    let days: f64 = time_since_epoch / 60.0 / 60.0 / 24.0;
    let months: f64 = time_since_epoch / 2629743.0;
    let years: f64 = f64::floor(time_since_epoch / 31556926.0);
    return format!(
        "{}/{}/{}/{}:{}:{}",
        (years + YEAR_OFFSET as f64),
        (f64::ceil(months) - (years * 12 as f64)),
        days,
        hours,
        minutes,
        seconds,
        );
}

pub mod extra_stuff {
    pub fn sleep_secs(time: u64) {
        let start: std::time::Instant = std::time::Instant::now();
        loop {
            let elapsed_time: std::time::Duration = start.elapsed();
            if elapsed_time.as_secs() > time {
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: String = get_time();
        dbg!(result);
        todo!("write a test here!");
    }
}

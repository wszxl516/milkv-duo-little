use crate::{reg_read_p};
use crate::config::SYS_CLOCK_HZ;

pub fn get_sys_time() -> u64 {
    let sys_tick = reg_read_p!(time) as u64;
    return sys_tick;
}
pub fn arch_usleep(us: u64) -> u64 {
    let start_time = get_sys_time();
    let end_time = start_time + us * (SYS_CLOCK_HZ / 1000000);
    loop {
        let run_time = get_sys_time();
        if run_time > end_time {
            break;
        }
    }
    return us;
}
pub fn sleep_ns(seconds: u64) {
    arch_usleep(seconds * 1000);
}
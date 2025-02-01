use crate::constant::LOG_PREFIX;

fn format_msg(msg: &str) -> String {
    format!("{} {}", LOG_PREFIX, msg)
}

pub fn log_exit(msg: &str) {
    println!("{}, exiting...", format_msg(msg))
}

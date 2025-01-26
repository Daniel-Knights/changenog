pub const LOG_PREFIX: &str = "\x1b[33m[changenog]\x1b[0m";

fn format_msg(msg: &str) -> String {
    format!("{} {}", LOG_PREFIX, msg)
}

pub fn log_exit(msg: &str) {
    println!("{}, exiting...", format_msg(msg))
}

pub fn log_warn(msg: &str) {
    println!("{}", format_msg(&("warn: ".to_string() + msg)))
}

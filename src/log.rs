use crate::constant::LOG_PREFIX;

pub fn log_exit(msg: &str, success: bool) {
    let formatted_msg = format!("{}.  exiting...", format_msg(msg));

    if success {
        println!("{formatted_msg}")
    } else {
        eprintln!("{formatted_msg}")
    }
}

fn format_msg(msg: &str) -> String {
    format!("{} {}", LOG_PREFIX, msg)
}

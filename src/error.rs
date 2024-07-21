use crate::HAD_ERROR;

pub fn error(line: u16, message: &str) {
    report(line, "", message);
}

pub fn report(line: u16, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    *HAD_ERROR.lock().unwrap() = true;
}
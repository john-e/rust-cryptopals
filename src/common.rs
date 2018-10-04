use std::process;

pub fn _fail_exit(code: i32) {
    process::exit(code);
}

#[macro_export]
macro_rules! fail_exit {
    ($code:expr) => { common::_fail_exit($code) };
    () => { common::_fail_exit(-1) };
}

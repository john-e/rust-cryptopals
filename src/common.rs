#[macro_export]
macro_rules! fail_exit {
    ($code:expr) => { std::process::exit($code) };
    () => { std::process::exit(-1) };
}

#[macro_export]
macro_rules! errexit {
    ($error:expr) => ({
        writeln!(&mut stderr(), "{}\n", $error).unwrap();
        process::exit(255);
    });
}

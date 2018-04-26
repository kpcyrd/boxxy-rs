#[macro_export]
macro_rules! shprintln {
    ($dst:expr, $fmt:expr) => ({
        use std::io::Write;
        writeln!($dst, $fmt).unwrap();
        $dst.flush().unwrap();
    });
    ($dst:expr, $fmt:expr, $($arg:tt)*) => ({
        use std::io::Write;
        writeln!($dst, $fmt, $($arg)*).unwrap();
        $dst.flush().unwrap();
    });
}

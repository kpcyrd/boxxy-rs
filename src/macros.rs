#[macro_export]
macro_rules! shprintln {
    ($dst:expr, $fmt:expr) => ({
        use std::io::Write;
        write!($dst, concat!($fmt, "\n")).unwrap();
        $dst.flush().unwrap();
    });
    ($dst:expr, $fmt:expr, $($arg:tt)*) => ({
        use std::io::Write;
        write!($dst, concat!($fmt, "\n"), $($arg)*).unwrap();
        $dst.flush().unwrap();
    });
}

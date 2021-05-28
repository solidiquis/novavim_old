#[macro_export]
macro_rules! flush_print {
    ( $($t:tt)* ) => {
        {
            use std::io;
            use std::io::Write;
            let mut h = io::stdout();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}


#[macro_export]
macro_rules! read_json {
    ($filename:expr) => {{
        use std::fs::File;
        use std::io::Read;
        use serde::Deserialize;
        let mut file = File::open($filename).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file")
    }};
}
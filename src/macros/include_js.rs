#[macro_export]
macro_rules! include_js {
    ($($tt:tt)*) => {
        {
            let text = stringify!({
                $($tt)*
            }).to_string();
            let re = regex::Regex::new(r#"b"((?:\\"|[^"])*)""#).unwrap();
            let modified_text = re.replace_all(&text, "`$1`").to_string();
            modified_text
        }
    };
}


#[macro_export]
macro_rules! js_context {
    ($js:tt, $json:tt) => {
        {
            use serde_json::json;
            use crate::include_js;
            format!("((context) => {{{}}})({})", include_js!($js), json!($json))
        }
    };
}

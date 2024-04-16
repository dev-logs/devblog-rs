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


/// Allow writing Javascript syntax in rust file with parameters support.
///
/// 1. Capabilities:
/// - Writing Javascript code in macro with respect on the IDE code high lighting.
/// - Serialize parameters from outside world with serde json.
///
/// 2. Some restrictions:
/// - ! Must include semicolon (;) at the end of every instruction.
/// - ! Can not use backtick (`), replace any backtick block (``) with (b"").
/// - ! When use string literal, the code inside block ${} can not contain double quote "".
/// - ! JS code might be not parsed correctly.
///
/// 3. Example:
/// ```rust
/// use web_sys::js_sys::eval;
/// use devblog_rs::js_context;
///
/// let js_code = js_context! ({
///     const {str1, str2} = context;
///     const combinedText = b"${param1}${param2}";
///     return combinedText;
/// }, {"str1": "combine", "str2": "text"});
///
/// let result = eval(js_code.as_str());
/// assert_eq!(result.unwrap().as_string(), String::from("combinetext"));
/// ```
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

use interop;
use js::jsapi::JSErrorReport;
use std::ops::Add;

pub struct WrappedError {
    pub line: u32,
    pub column: u32,
    pub filename: String,
    pub message: String,
}

impl WrappedError {
    pub fn from_jserr(source: JSErrorReport) -> WrappedError {
        let filename = interop::char_ptr_to_str_or(source.filename, "anonymous file");
        let message = interop::null_term_ptr_to_string(source.ucmessage);

        WrappedError {
            filename: String::new().add(filename),
            message,
            column: source.column,
            line: source.lineno,
        }
    }
}
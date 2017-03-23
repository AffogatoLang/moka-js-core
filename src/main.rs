#[macro_use]
extern crate js;
extern crate libc;
extern crate docopt;
extern crate moka_runner;

use js::jsapi::CallArgs;
use js::jsapi::JSContext;
use js::jsapi::JS_EncodeStringToUTF8;
use js::jsapi::JSAutoCompartment;
use js::jsapi::JS_ReportError;
use js::jsapi::Value;
use js::jsval::{ UndefinedValue };

use moka_runner::util;
use moka_runner::interop::{ static_to_char_ptr };
use moka_runner::runtime::errors::print_pending_exception_with_snippet;
use moka_runner::runtime::Container;

use std::ffi::CStr;
use std::str;
use std::path::Path;

use docopt::Docopt;

const USAGE: &'static str = "Usage: runner <path>";

fn main() {
    let args = Docopt::new(USAGE)
                .and_then(|d| d.parse())
                .unwrap_or_else(|e| e.exit());

    let path = args.get_str("<path>");

    let file = util::files::read_as_file(Path::new(path)).unwrap();

    let name = file.0;
    let contents = file.1.as_str();

    let container = Container::new();
    rooted!(in(container.context) let rooted = container.global);
    let root_handle = rooted.handle();
    let _ac = JSAutoCompartment::new(container.context, rooted.get());

    container.declare_global(root_handle, "log\0", Some(puts), 1);

    match container.exec(root_handle, contents, name.as_str()) {
        Ok(_) => (),
        Err(_) => print_pending_exception_with_snippet(container.context, contents)
    }
}

unsafe extern "C" fn puts(context: *mut JSContext, argc: u32, vp: *mut Value) -> bool {
    let args = CallArgs::from_vp(vp, argc);

    if args._base.argc_ != 1 {
        JS_ReportError(context, static_to_char_ptr("log requires exactly 1 argument\0"));
        return false;
    }

    let arg = args.get(0);
    let js = js::rust::ToString(context, arg);
    rooted!(in(context) let message_root = js);
    let message = JS_EncodeStringToUTF8(context, message_root.handle());
    let message = CStr::from_ptr(message);
    println!("{}", str::from_utf8(message.to_bytes()).unwrap());

    args.rval().set(UndefinedValue());
    return true;
}

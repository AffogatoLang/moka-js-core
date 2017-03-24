use std::str;
use std::ffi::CStr;

use interop::static_to_char_ptr;

use js::jsapi::CallArgs;
use js::jsapi::JSContext;
use js::jsapi::JS_EncodeStringToUTF8;
use js::jsapi::JS_ReportError;
use js::jsapi::Value;
use js::jsval::UndefinedValue;
use js::rust::ToString;

pub unsafe extern "C" fn log(context: *mut JSContext, argc: u32, vp: *mut Value) -> bool {
    let args = CallArgs::from_vp(vp, argc);

    if args._base.argc_ != 1 {
        JS_ReportError(context, static_to_char_ptr("log requires exactly 1 argument\0"));
        return false;
    }

    let arg = args.get(0);
    let js = ToString(context, arg);
    rooted!(in(context) let message_root = js);
    let message = JS_EncodeStringToUTF8(context, message_root.handle());
    let message = CStr::from_ptr(message);
    println!("{}", str::from_utf8(message.to_bytes()).unwrap());

    args.rval().set(UndefinedValue());
    return true;
}

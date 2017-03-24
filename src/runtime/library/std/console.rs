use std::str;
use std::ffi::CStr;
use std::ops::AddAssign;

use js::jsapi::CallArgs;
use js::jsapi::JSContext;
use js::jsapi::JS_EncodeStringToUTF8;
use js::jsapi::Value;
use js::jsval::UndefinedValue;
use js::rust::ToString;

pub unsafe extern "C" fn log(context: *mut JSContext, argc: u32, vp: *mut Value) -> bool {
    let argv = CallArgs::from_vp(vp, argc);
    let args = argv._base.argc_;
    let mut cur = 0;
    let mut buffer = String::new();

    while cur < args {
        let arg = argv.get(cur);
        let js = ToString(context, arg);
        rooted!(in(context) let message_root = js);
        let message = JS_EncodeStringToUTF8(context, message_root.handle());
        let message = CStr::from_ptr(message).to_str().unwrap();
        buffer.add_assign(format!(" {}", message).as_str());
        cur += 1;
    }

    println!("{}", buffer.trim());

    argv.rval().set(UndefinedValue());
    return true;
}

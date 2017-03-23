use std::ptr;

use js::jsapi::JSContext;
use js::jsapi::OnNewGlobalHookOption;
use js::jsapi::CompartmentOptions;
use js::jsapi::JS_NewGlobalObject;
use js::jsapi::JSObject;
use js::rust::SIMPLE_GLOBAL_CLASS;

// TODO: Replace implementation of SIMPLE_GLOBAL_CLASS
pub fn create_global(context: *mut JSContext) -> *mut JSObject {

    let h_option = OnNewGlobalHookOption::FireOnNewGlobalHook;
    let c_option = CompartmentOptions::default();

    let global: *mut JSObject;

    unsafe {
        global = JS_NewGlobalObject(
            context,
            &SIMPLE_GLOBAL_CLASS,
            ptr::null_mut(),
            h_option,
            &c_option
        );
    }

    global
}
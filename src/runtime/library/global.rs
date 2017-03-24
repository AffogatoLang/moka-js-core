use std::ptr;

use js::jsapi::JSContext;
use js::jsapi::OnNewGlobalHookOption;
use js::jsapi::CompartmentOptions;
use js::jsapi::JS_NewGlobalObject;
use js::jsapi::JSObject;
//use js::jsapi::JSClass;
use js::rust::SIMPLE_GLOBAL_CLASS;
//use js::jsapi::{ JSCLASS_RESERVED_SLOTS_SHIFT };
//use js::consts::JSCLASS_RESERVED_SLOTS_MASK;

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

//pub fn create_base_object_class() -> JSClass {
//    JSClass {
//        name: static_to_char_ptr("RootObject\0"),
//        flags: JSCLASS_RESERVED_SLOTS_MASK << JSCLASS_RESERVED_SLOTS_SHIFT,
//        reserved: [0 as *mut _; 3],
//    }
//}
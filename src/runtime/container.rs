use js::jsapi::JSObject;
use js::jsapi::JSContext;
use js::jsapi::JSNative;
use js::jsapi::HandleObject;
use js::jsapi::JSAutoCompartment;
use js::jsapi::JS_DefineFunction;
use js::jsval::UndefinedValue;
use js::rust::Runtime;

use libc::c_uint;
use libc::c_char;

use interop::static_to_char_ptr;
use runtime::create_global;

pub struct Container {
    pub runtime: Runtime,
    pub context: *mut JSContext,
    pub global: *mut JSObject,
    pub root: HandleObject,
}

impl Container {
    pub fn new() -> Container {
        let runtime = Runtime::new();
        let context = runtime.cx();
        let global = create_global(context);
        rooted!(in(context) let root = global);

        Container {
            runtime: runtime,
            context: context,
            global: global,
            root: root.handle(),
        }
    }


    pub fn declare_global(&self, root: HandleObject, name: &'static str, func: JSNative, params: c_uint) {
        unsafe {
            JS_DefineFunction(self.context, root, static_to_char_ptr(name), func, params, 0);
        }
    }
    pub fn declare_global_a(&self, root: HandleObject, name: &[u8], func: JSNative, params: c_uint) {
        unsafe {
            JS_DefineFunction(self.context, root, name.as_ptr() as *const c_char, func, params, 0);
        }
    }

    pub fn exec(&self, root: HandleObject, script: &str, filename: &str) -> Result<(), ()> {
        rooted!(in(self.context) let mut rval = UndefinedValue());
//        rooted!(in(self.context) let rooted = self.global);
        let r = self.runtime.evaluate_script(root, script, filename, 0, rval.handle_mut());

        r
    }
}
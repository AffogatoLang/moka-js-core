use js::jsval::UndefinedValue;
use js::jsapi::JSContext;
use js::jsapi::JSErrorReport;
use js::jsapi::{ Value };
use js::jsapi::{
    JS_IsExceptionPending,
    JS_GetPendingException,
    JS_ErrorFromException
};

use runtime::errors::WrappedError;

use term_painter::ToStyle;
use term_painter::Color::Red;

fn get_error(context: *mut JSContext) -> Option<JSErrorReport> {
    let has_exception: bool;

    unsafe {
        has_exception = JS_IsExceptionPending(context);
    }

    if has_exception {
        rooted!(in(context) let mut rval = UndefinedValue());
        unsafe {
            JS_GetPendingException(context, rval.handle_mut());
        }
        rooted!(in(context) let handler = Value::to_object(&rval.get()));

        let err: JSErrorReport;
        unsafe {
            err = *JS_ErrorFromException(context, handler.handle());
        }

        Some(err)
    } else {
        None
    }
}

pub fn print_pending_exception(context: *mut JSContext)  {
    match get_error(context) {
        Some(error) => {
            let e = WrappedError::from_jserr(error);
            println!(
                "\nException in {} at line {}, column {}:\n\n{}\n",
                e.filename,
                e.line + 1,
                e.column,
                Red.paint(e.message)
            );
        }
        None => ()
    }
}

pub fn print_pending_exception_with_snippet(context: *mut JSContext, script: &str) {
    print_pending_exception(context)
}
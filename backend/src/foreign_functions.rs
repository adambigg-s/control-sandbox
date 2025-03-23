use std::ffi::c_void;

use sokol::app as sap;
use sokol::gfx;

use crate::GlobalState;
use crate::simulation::{Control, State};

pub fn call_controller(state: &State) -> Control {
    unsafe { controller(state) }
}

unsafe extern "C" {
    fn controller(state: &State) -> Control;
}

pub extern "C" fn ffi_cb_init(user_data: *mut c_void) {
    let global_state: &mut GlobalState;
    unsafe {
        global_state = &mut *(user_data as *mut GlobalState);
    }
    global_state.callback_init(user_data);
}

pub extern "C" fn ffi_cb_event(raw_event: *const sap::Event, user_data: *mut c_void) {
    let event: &sap::Event;
    let global_state: &mut GlobalState;
    unsafe {
        event = &*raw_event;
        global_state = &mut *(user_data as *mut GlobalState);
    }
    global_state.callback_event(event);
}

pub extern "C" fn ffi_cb_frame(user_data: *mut c_void) {
    let global_state: &mut GlobalState;
    unsafe {
        global_state = &mut *(user_data as *mut GlobalState);
    }
    global_state.callback_frame();
}

#[allow(unused_must_use)]
#[allow(clippy::from_raw_with_void_ptr)]
pub extern "C" fn ffi_cb_cleanup(user_data: *mut c_void) {
    gfx::shutdown();
    unsafe {
        if !user_data.is_null() {
            Box::from_raw(user_data);
        }
    }
}

mod simulation;

use std::os::raw::c_void;

use sokol::app as sap;
use sokol::gfx;
use sokol::glue;
use sokol::log;

use simulation::{Control, Simulation, State};

unsafe extern "C" {
    fn controller(state: &State) -> Control;
}

fn call_controller(state: &State) -> Control {
    unsafe { controller(state) }
}

fn main() {
    let global_state = Box::new(GlobalState {
        simulation: Simulation {
            state: State {
                position: 70.,
                delta_time: 0.01,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    let user_data = Box::into_raw(global_state) as *mut c_void;

    sap::run(&sap::Desc {
        user_data,
        init_userdata_cb: Some(ffi_cb_init),
        event_userdata_cb: Some(ffi_cb_event),
        frame_userdata_cb: Some(ffi_cb_frame),
        cleanup_userdata_cb: Some(ffi_cb_cleanup),
        width: 400,
        height: 400,
        window_title: c"control sandbox".as_ptr(),
        fullscreen: false,
        high_dpi: true,
        logger: sap::Logger {
            func: Some(log::slog_func),
            user_data,
        },
        icon: sap::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(Default)]
struct GlobalState {
    simulation: Simulation,
    pipeline: gfx::Pipeline,
    bindings: gfx::Bindings,
}

impl GlobalState {
    fn callback_init(&mut self, self_c_ptr: *mut c_void) {
        gfx::setup(&gfx::Desc {
            environment: glue::environment(),
            logger: gfx::Logger {
                func: Some(log::slog_func),
                user_data: self_c_ptr,
            },
            ..Default::default()
        });
    }

    fn callback_event(&mut self, event: &sap::Event) {
        if event.key_code == sap::Keycode::Escape {
            sap::request_quit();
        }
    }

    fn callback_frame(&mut self) {}
}

extern "C" fn ffi_cb_init(user_data: *mut c_void) {
    let global: &mut GlobalState;
    unsafe {
        global = &mut *(user_data as *mut GlobalState);
    }
    global.callback_init(user_data);
}

extern "C" fn ffi_cb_event(raw_event: *const sap::Event, user_data: *mut c_void) {
    let event: &sap::Event;
    let global: &mut GlobalState;
    unsafe {
        event = &*raw_event;
        global = &mut *(user_data as *mut GlobalState);
    }
    global.callback_event(event);
}

extern "C" fn ffi_cb_frame(user_data: *mut c_void) {
    let global: &mut GlobalState;
    unsafe {
        global = &mut *(user_data as *mut GlobalState);
    }
    global.callback_frame();
}

#[allow(unused_must_use)]
#[allow(clippy::from_raw_with_void_ptr)]
extern "C" fn ffi_cb_cleanup(user_data: *mut c_void) {
    gfx::shutdown();
    unsafe {
        if !user_data.is_null() {
            Box::from_raw(user_data);
        }
    }
}

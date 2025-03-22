mod shaders;
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
    pass_action: gfx::PassAction,
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

        #[rustfmt::skip]
        let vertices: [f32; 18] = [
            0.5, -0.5, 0.,   1., 0., 0.,
            -0.5, -0.5, 0.,  0., 1., 0.,
            0., 0.5, 0.,     0., 0., 1.,
        ];
        self.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&vertices),
            ..Default::default()
        });
        self.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&shaders::simple_shader_desc(gfx::query_backend())),
            layout: {
                let mut layout = gfx::VertexLayoutState::new();
                layout.attrs[shaders::ATTR_SIMPLE_POSITION].format = gfx::VertexFormat::Float3;
                layout.attrs[shaders::ATTR_SIMPLE_V_COLOR].format = gfx::VertexFormat::Float3;
                layout
            },
            ..Default::default()
        });
        self.pass_action.colors[0] = gfx::ColorAttachmentAction {
            load_action: gfx::LoadAction::Clear,
            clear_value: gfx::Color {
                r: 0.3,
                g: 0.6,
                b: 0.9,
                a: 1.,
            },
            ..Default::default()
        };
    }

    fn callback_event(&mut self, event: &sap::Event) {
        if event.key_code == sap::Keycode::Escape {
            sap::request_quit();
        }
    }

    fn callback_frame(&mut self) {
        call_controller(&self.simulation.state);
        self.simulation.step();

        gfx::begin_pass(&gfx::Pass {
            action: self.pass_action,
            swapchain: glue::swapchain(),
            ..Default::default()
        });
        gfx::apply_viewport(0, 0, sap::width(), sap::height(), false);
        gfx::apply_pipeline(self.pipeline);
        gfx::apply_bindings(&self.bindings);
        gfx::draw(0, 3, 1);

        gfx::end_pass();
        gfx::commit();
    }
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

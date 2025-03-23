mod camera;
mod foreign_functions;
mod shaders;
mod simulation;

use std::ffi::c_void;
use std::path::Path;

use camera::Camera;
use camera::Inputs;
use sokol::app as sap;
use sokol::gfx;
use sokol::glue;
use sokol::log;

use glam as glm;

use foreign_functions::*;
use simulation::{Simulation, State};
use sokol::time;

const HEIGHT: i32 = 600;
const WIDTH: i32 = 800;

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
        camera: Camera::new(),
        ..Default::default()
    });
    let user_data = Box::into_raw(global_state) as *mut c_void;

    sap::run(&sap::Desc {
        user_data,
        init_userdata_cb: Some(ffi_cb_init),
        event_userdata_cb: Some(ffi_cb_event),
        frame_userdata_cb: Some(ffi_cb_frame),
        cleanup_userdata_cb: Some(ffi_cb_cleanup),
        width: 800,
        height: 600,
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

struct Transform {
    position: glm::Vec3,
    rotation: glm::Quat,
    scale: glm::Vec3,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: glm::Vec3::ZERO,
            rotation: glm::Quat::IDENTITY,
            scale: glm::Vec3::ONE,
        }
    }
}

impl Transform {
    fn to_matrix(&self) -> glm::Mat4 {
        glm::Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texcoord: [f32; 2],
}

#[derive(Default)]
struct GlobalState {
    simulation: Simulation,
    pipeline: gfx::Pipeline,
    bindings: gfx::Bindings,
    pass_action: gfx::PassAction,
    vertex_count: usize,
    camera: Camera,
    inputs: Inputs,
    transform: Transform,
}

pub fn cube_verts_uv_normal() -> gfx::Buffer {
    #[rustfmt::skip]
    let cube_verts: [f32; 288] = [
        -0.5, -0.5, -0.5,  0.0, 0.0,  0., 0., -1.,
         0.5, -0.5, -0.5,  1.0, 0.0,  0., 0., -1.,
         0.5,  0.5, -0.5,  1.0, 1.0,  0., 0., -1.,
         0.5,  0.5, -0.5,  1.0, 1.0,  0., 0., -1.,
        -0.5,  0.5, -0.5,  0.0, 1.0,  0., 0., -1.,
        -0.5, -0.5, -0.5,  0.0, 0.0,  0., 0., -1.,

        -0.5, -0.5,  0.5,  0.0, 0.0,  0., 0., 1.,
         0.5, -0.5,  0.5,  1.0, 0.0,  0., 0., 1.,
         0.5,  0.5,  0.5,  1.0, 1.0,  0., 0., 1.,
         0.5,  0.5,  0.5,  1.0, 1.0,  0., 0., 1.,
        -0.5,  0.5,  0.5,  0.0, 1.0,  0., 0., 1.,
        -0.5, -0.5,  0.5,  0.0, 0.0,  0., 0., 1.,

        -0.5,  0.5,  0.5,  1.0, 0.0,  -1., 0., 0.,
        -0.5,  0.5, -0.5,  1.0, 1.0,  -1., 0., 0.,
        -0.5, -0.5, -0.5,  0.0, 1.0,  -1., 0., 0.,
        -0.5, -0.5, -0.5,  0.0, 1.0,  -1., 0., 0.,
        -0.5, -0.5,  0.5,  0.0, 0.0,  -1., 0., 0.,
        -0.5,  0.5,  0.5,  1.0, 0.0,  -1., 0., 0.,

         0.5,  0.5,  0.5,  1.0, 0.0,  1., 0., 0.,
         0.5,  0.5, -0.5,  1.0, 1.0,  1., 0., 0.,
         0.5, -0.5, -0.5,  0.0, 1.0,  1., 0., 0.,
         0.5, -0.5, -0.5,  0.0, 1.0,  1., 0., 0.,
         0.5, -0.5,  0.5,  0.0, 0.0,  1., 0., 0.,
         0.5,  0.5,  0.5,  1.0, 0.0,  1., 0., 0.,

        -0.5, -0.5, -0.5,  0.0, 1.0,  -1., 0., 0.,
         0.5, -0.5, -0.5,  1.0, 1.0,  -1., 0., 0.,
         0.5, -0.5,  0.5,  1.0, 0.0,  -1., 0., 0.,
         0.5, -0.5,  0.5,  1.0, 0.0,  -1., 0., 0.,
        -0.5, -0.5,  0.5,  0.0, 0.0,  -1., 0., 0.,
        -0.5, -0.5, -0.5,  0.0, 1.0,  -1., 0., 0.,

        -0.5,  0.5, -0.5,  0.0, 1.0,  0., 1., 0.,
         0.5,  0.5, -0.5,  1.0, 1.0,  0., 1., 0.,
         0.5,  0.5,  0.5,  1.0, 0.0,  0., 1., 0.,
         0.5,  0.5,  0.5,  1.0, 0.0,  0., 1., 0.,
        -0.5,  0.5,  0.5,  0.0, 0.0,  0., 1., 0.,
        -0.5,  0.5, -0.5,  0.0, 1.0,  0., 1., 0.,
    ];
    gfx::make_buffer(&gfx::BufferDesc {
        data: gfx::slice_as_range(&cube_verts),
        label: c"square texture verts".as_ptr(),
        ..Default::default()
    })
}

impl GlobalState {
    fn callback_init(&mut self, self_c_ptr: *mut c_void) {
        time::setup();
        gfx::setup(&gfx::Desc {
            environment: glue::environment(),
            logger: gfx::Logger {
                func: Some(log::slog_func),
                user_data: self_c_ptr,
            },
            ..Default::default()
        });

        let (models, _materials) = tobj::load_obj(
            Path::new("./vendor/f35/f35.obj"),
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )
        .expect("model loading failure");

        let mut vertices = Vec::new();
        for model in &models {
            let mesh = &model.mesh;
            for i in 0..mesh.indices.len() {
                let idx = mesh.indices[i] as usize;
                vertices.push(Vertex {
                    position: [
                        mesh.positions[idx * 3],
                        mesh.positions[idx * 3 + 1],
                        mesh.positions[idx * 3 + 2],
                    ],
                    normal: if !mesh.normals.is_empty() {
                        [
                            mesh.normals[idx * 3],
                            mesh.normals[idx * 3 + 1],
                            mesh.normals[idx * 3 + 2],
                        ]
                    } else {
                        [0., 0., 1.]
                    },
                    texcoord: if !mesh.texcoords.is_empty() {
                        [mesh.texcoords[idx * 2], mesh.texcoords[idx * 2 + 1]]
                    } else {
                        [0., 0.]
                    },
                });
            }
        }
        self.vertex_count = vertices.len();

        let vertex_size = std::mem::size_of::<Vertex>();
        let mut vertex_data = Vec::with_capacity(vertices.len() * vertex_size);
        for vertex in &vertices {
            for &coord in &vertex.position {
                vertex_data.extend_from_slice(&coord.to_ne_bytes());
            }
            for &coord in &vertex.normal {
                vertex_data.extend_from_slice(&coord.to_ne_bytes());
            }
            for &coord in &vertex.texcoord {
                vertex_data.extend_from_slice(&coord.to_ne_bytes());
            }
        }
        self.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&vertex_data),
            ..Default::default()
        });

        // let buffer = cube_verts_uv_normal();
        // self.bindings.vertex_buffers[0] = buffer;
        // self.vertex_count = 36;

        let img = image::open(Path::new("./vendor/f35/f35_texture.jpg"))
            .expect("failed to load texture")
            .flipv()
            .to_rgba8();
        let (width, height) = img.dimensions();
        let raw_image = img.into_raw();
        let texture = gfx::make_image(&gfx::ImageDesc {
            width: width as i32,
            height: height as i32,
            pixel_format: gfx::PixelFormat::Rgba8,
            data: {
                let mut subimage = gfx::ImageData::new();
                subimage.subimage[0][0] = gfx::slice_as_range(&raw_image);
                subimage
            },
            ..Default::default()
        });
        self.bindings.images[shaders::IMG_TEX] = texture;
        let sampler = gfx::make_sampler(&gfx::SamplerDesc {
            min_filter: gfx::Filter::Linear,
            mag_filter: gfx::Filter::Linear,
            ..Default::default()
        });
        self.bindings.samplers[shaders::SMP_SAMP] = sampler;
        self.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&shaders::texture_shader_desc(gfx::query_backend())),
            primitive_type: gfx::PrimitiveType::Triangles,
            cull_mode: gfx::CullMode::None,
            depth: gfx::DepthState {
                compare: gfx::CompareFunc::Less,
                write_enabled: true,
                ..Default::default()
            },
            layout: {
                let mut layout = gfx::VertexLayoutState::new();
                layout.attrs[shaders::ATTR_TEXTURE_POSITION].format = gfx::VertexFormat::Float3;
                layout.attrs[shaders::ATTR_TEXTURE_V_NORMAL].format = gfx::VertexFormat::Float3;
                layout.attrs[shaders::ATTR_TEXTURE_V_TEXCOORD].format = gfx::VertexFormat::Float2;
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
        println!("\x1b[2J");
    }

    fn callback_event(&mut self, event: &sap::Event) {
        self.inputs.get_inputs(event);
    }

    fn callback_frame(&mut self) {
        call_controller(&self.simulation.state);
        self.simulation.step();
        self.camera.update(&mut self.inputs, 0.01);

        let projection = self.camera.projection_matrix();
        let view = self.camera.view_matrix();
        let vs_params = [self.transform.to_matrix(), view, projection];
        let delta_rotation = glm::Quat::from_rotation_x(0.01);
        self.transform.rotation = delta_rotation * self.transform.rotation;

        gfx::begin_pass(&gfx::Pass {
            action: self.pass_action,
            swapchain: glue::swapchain(),
            ..Default::default()
        });
        gfx::apply_viewport(0, 0, sap::width(), sap::height(), false);
        gfx::apply_pipeline(self.pipeline);
        gfx::apply_bindings(&self.bindings);
        gfx::apply_uniforms(shaders::UB_VS_PARAMS, &gfx::slice_as_range(&vs_params));
        gfx::draw(0, self.vertex_count, 1);

        gfx::end_pass();
        gfx::commit();
    }
}

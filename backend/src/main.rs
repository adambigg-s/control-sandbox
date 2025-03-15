const M: f32 = 10000.;
const G: f32 = 9.81;
const RHO: f32 = 1.25;
const S: f32 = 50.;
const B: f32 = 10.;
const C: f32 = 5.;
const T_MAX: f32 = 100000.;
const IXX: f32 = 10000.;
const IYY: f32 = 50000.;
const IZZ: f32 = 50000.;
const C_DO: f32 = 0.02;
const K: f32 = 0.05;
const C_L_ALPHA: f32 = 5.;
const C_Y_RUD: f32 = 0.1;
const C_L_AIL: f32 = 0.1;
const C_L_ELEV: f32 = 0.5;
const C_N_RUD: f32 = 0.1;

fn euler_to_rotation_matrix(phi: f32, theta: f32, psi: f32) -> [[f32; 3]; 3] {
    let (cphi, sphi) = phi.sin_cos();
    let (ctheta, stheta) = theta.sin_cos();
    let (cpsi, spsi) = psi.sin_cos();

    [
        [
            ctheta * cpsi,
            sphi * stheta * cpsi - cphi * spsi,
            cphi * stheta * cpsi + sphi * spsi,
        ],
        [
            ctheta * spsi,
            sphi * stheta * spsi + cphi * cpsi,
            cphi * stheta * spsi - sphi * cpsi,
        ],
        [-stheta, sphi * ctheta, cphi * ctheta],
    ]
}

fn transpose(m: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
    [
        [m[0][0], m[1][0], m[2][0]],
        [m[0][1], m[1][1], m[2][1]],
        [m[0][2], m[1][2], m[2][2]],
    ]
}

fn cross_product(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[2],
    ]
}

fn mat_vec_mult(m: [[f32; 3]; 3], v: [f32; 3]) -> [f32; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

fn compute_derivatives(state: &State, control: &Control) -> State {
    let phi = state.phi;
    let theta = state.theta;
    let psi = state.psi;

    let r = euler_to_rotation_matrix(phi, theta, psi);

    let v_b = [state.u, state.v, state.w];
    let v_w = mat_vec_mult(r, v_b);

    let tan_theta = theta.tan();
    let cos_theta = theta.cos();
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let phi = state.p + state.q * sin_phi * tan_theta + state.r * cos_phi * tan_theta;
    let theta = state.q * cos_phi - state.r * sin_phi;
    let psi = if cos_theta != 0.0 {
        (state.q * sin_phi + state.r * cos_phi) / cos_theta
    } else {
        0.0
    };

    let v = (state.u * state.u + state.v * state.v + state.w * state.w).sqrt();
    let alpha = if state.u != 0.0 {
        (state.w / state.u).atan()
    } else {
        0.0
    };
    let q_dyn = 0.5 * RHO * v * v;

    let c_l = C_L_ALPHA * alpha + C_L_ELEV * control.elevator;
    let c_d = C_DO + K * c_l * c_l;
    let c_y = C_Y_RUD * control.rudder;
    let c_l_ail = C_L_AIL * control.aileron;
    let c_m = C_L_ELEV * control.elevator;
    let c_n = C_N_RUD * control.rudder;

    let f_aero = [-c_d * q_dyn * S, c_y * q_dyn * S, -c_l * q_dyn * S];

    let m_aero = [
        c_l_ail * q_dyn * S * B,
        c_m * q_dyn * S * C,
        c_n * q_dyn * S * B,
    ];

    let thrust = [T_MAX * control.throttle, 0.0, 0.0];

    let g_w = [0.0, 0.0, G];
    let g_b = mat_vec_mult(transpose(r), g_w);

    let f_b = [
        f_aero[0] + thrust[0] + M * g_b[0],
        f_aero[1] + M * g_b[1],
        f_aero[2] + M * g_b[2],
    ];

    let omega = [state.p, state.q, state.r];
    let omega_cross_v = cross_product(omega, v_b);
    let u = f_b[0] / M - omega_cross_v[0];
    let v = f_b[1] / M - omega_cross_v[1];
    let w = f_b[2] / M - omega_cross_v[2];

    let i_omega = [IXX * state.p, IYY * state.q, IZZ * state.r];
    let omega_cross_i_omega = cross_product(omega, i_omega);
    let m_net = [
        m_aero[0] - omega_cross_i_omega[0],
        m_aero[1] - omega_cross_i_omega[1],
        m_aero[2] - omega_cross_i_omega[2],
    ];
    let p = m_net[0] / IXX;
    let q = m_net[1] / IYY;
    let r = m_net[2] / IZZ;

    State {
        x: v_w[0],
        y: v_w[1],
        z: v_w[2],
        phi,
        theta,
        psi,
        u,
        v,
        w,
        p,
        q,
        r,
    }
}

fn integrate(state: &State, deriv: &State, dt: f32) -> State {
    State {
        x: state.x + deriv.x * dt,
        y: state.y + deriv.y * dt,
        z: state.z + deriv.z * dt,
        phi: state.phi + deriv.phi * dt,
        theta: state.theta + deriv.theta * dt,
        psi: state.psi + deriv.psi * dt,
        u: state.u + deriv.u * dt,
        v: state.v + deriv.v * dt,
        w: state.w + deriv.w * dt,
        p: state.p + deriv.p * dt,
        q: state.q + deriv.q * dt,
        r: state.r + deriv.r * dt,
    }
}

#[repr(C)]
#[derive(Default)]
struct State {
    x: f32,
    y: f32,
    z: f32,
    phi: f32,
    theta: f32,
    psi: f32,
    u: f32,
    v: f32,
    w: f32,
    p: f32,
    q: f32,
    r: f32,
}

#[repr(C)]
struct Control {
    aileron: f32,
    elevator: f32,
    rudder: f32,
    throttle: f32,
}

fn main() {
    let mut state = State {
        u: 100.,
        ..Default::default()
    };
    let dt = 0.01;
    loop {
        let control = unsafe { controller(&state) };
        let deriv = compute_derivatives(&state, &control);
        state = integrate(&state, &deriv, dt);

        println!(
            "Pos: ({:.2}, {:.2}, {:.2}), Angles: ({:.2}, {:.2}, {:.2}), Velocity: ({:.2}, {:.2}, {:.2})",
            state.x,
            state.y,
            state.z,
            state.phi.to_degrees(),
            state.theta.to_degrees(),
            state.psi.to_degrees(),
            state.u,
            state.v,
            state.w,
        );

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[link(name = "controller")]
unsafe extern "C" {
    fn controller(state: &State) -> Control;
}

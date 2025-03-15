
typedef struct State {
    float x, y, z;
    float phi, theta, psi;
    float u, v, w;
    float p, q, r;
} State;

typedef struct Control {
    float aileron;
    float elevator;
    float rudder;
    float throttle;
} Control;

void clamp(float* target, float min, float max) {
    if (*target < min) {
        *target = min;
    }
    if (*target > max) {
        *target = max;
    }
}

Control controller(const State *state) {
    Control controller = {
        .aileron = 0.,
        .elevator = -0.1,
        .rudder = 0.,
        .throttle = 0.5,
    };
    return controller;
}

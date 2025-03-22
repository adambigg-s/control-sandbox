typedef struct State {
    float position;
    float velocity;
    int tick;
    float delta_time;
} State;

typedef struct Control {
    float force;
} Control;

static float integral = 0.;

Control controller(const State *state) {
    Control controller = {.force = 0};

    float kp = 7.;
    float kd = 3.;
    float ki = 0.5;

    if (state->tick % 100 == 0) {
        integral = 0;
    }
    integral += state->position * 0.01;

    controller.force += -kp * state->position;
    controller.force += -kd * state->velocity;
    controller.force += -ki * integral;

    return controller;
}


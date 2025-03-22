typedef struct State {
    float position;
    float velocity;
    int tick;
    float delta_time;
} State;

typedef struct Control {
    float force;
} Control;

Control controller(const State *state) {
    Control controller;
    
    return controller;
}

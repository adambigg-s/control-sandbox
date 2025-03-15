#[link(name = "controller")]
unsafe extern "C" {
    fn controller(state: &State) -> Control;
}

#[repr(C)]
struct State {

}

#[repr(C)]
struct Control {

}

fn main() {
    let state = State {};
    loop {
        let control = unsafe { controller(&state) };
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
